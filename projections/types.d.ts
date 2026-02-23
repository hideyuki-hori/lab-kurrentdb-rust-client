interface ProjectionEvent<T> {
  body: T
  bodyRaw: string
  streamId: string
  eventType: string
  sequenceNumber: number
  metadataRaw: Record<string, unknown>
  isJson: boolean
  partition: string
}

interface WhenHandlers<S> {
  $init?: () => S
  $any?: (state: S, event: ProjectionEvent<unknown>) => void
  $deleted?: (state: S, event: ProjectionEvent<unknown>) => void
  [eventType: string]: ((state: S, event: ProjectionEvent<any>) => void) | (() => S) | undefined
}

interface ProjectionBuilder<S = unknown> {
  when(handlers: WhenHandlers<S>): ProjectionChain
  foreachStream(): { when(handlers: WhenHandlers<S>): ProjectionChain }
  outputState(): void
}

interface ProjectionChain {
  outputState(): ProjectionChain
  transformBy(handler: (state: any) => any): ProjectionChain
  filterBy(handler: (state: any) => boolean): ProjectionChain
}

declare function fromAll(): ProjectionBuilder<any>
declare function fromCategory(name: string): ProjectionBuilder<any>
declare function fromStream(name: string): ProjectionBuilder<any>
declare function fromStreams(streams: string[]): ProjectionBuilder<any>
declare function emit(streamId: string, eventType: string, eventBody: unknown, metadata?: unknown): void
declare function linkTo(streamId: string, event: unknown, metadata?: unknown): void
declare function log(message: string): void
