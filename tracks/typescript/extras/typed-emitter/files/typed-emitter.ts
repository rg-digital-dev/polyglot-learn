/** A listener for an event whose payload is the tuple `Args`. */
export type Listener<Args extends unknown[]> = (...args: Args) => void;

/**
 * A mapped type: one optional key per event name in `Events`, holding the
 * listeners for that event. For `{ tick: [number] }` it is
 * `{ tick?: Listener<[number]>[] }`.
 */
export type ListenerMap<Events extends Record<string, unknown[]>> = {
  [K in keyof Events]?: Listener<Events[K]>[];
};

/**
 * An event emitter whose events and payloads are fixed by `Events`, e.g.
 * `new Emitter<{ login: [user: string]; logout: [] }>()`.
 */
export class Emitter<Events extends Record<string, unknown[]>> {
  private listeners: ListenerMap<Events> = {};

  /**
   * Add `listener` for `event`. It runs after the listeners added before it.
   * Return a function that removes it again (calling that twice is harmless).
   */
  on<K extends keyof Events>(event: K, listener: Listener<Events[K]>): () => void {
    throw new Error("TODO: implement Emitter.on");
  }

  /** Remove `listener` from `event`. Does nothing if it isn't registered. */
  off<K extends keyof Events>(event: K, listener: Listener<Events[K]>): void {
    throw new Error("TODO: implement Emitter.off");
  }

  /**
   * Call every listener of `event` with `args`, in the order they were added,
   * and return how many ran. A listener removed during the emit still runs
   * this time; one added during the emit waits for the next one.
   */
  emit<K extends keyof Events>(event: K, ...args: Events[K]): number {
    throw new Error("TODO: implement Emitter.emit");
  }
}
