/** Provided: a step fails if it starts with "!", with the error "step `!x` failed". */
export function checkStep(step: string): void {
  if (step.startsWith("!")) throw new Error(`step \`${step}\` failed`);
}

/** Provided: a name is bad if it starts with "!", with the error "bad name `!x`". */
export function checkName(name: string): void {
  if (name.startsWith("!")) throw new Error(`bad name \`${name}\``);
}

/** Provided: resolve after `ms` milliseconds. */
export function delay(ms: number): Promise<void> {
  return new Promise((resolve) => setTimeout(resolve, ms));
}

/** A transaction that rolls back when it's disposed, unless it was committed. */
export class Transaction implements Disposable {
  name: string;
  log: string[];

  /** Log "begin <name>". */
  constructor(name: string, log: string[]) {
    this.name = name;
    this.log = log;
    throw new Error("TODO: implement the Transaction constructor");
  }

  /** Log "<name>: <entry>". */
  record(entry: string): void {
    throw new Error("TODO: implement record");
  }

  /** Mark as committed; the dispose method then logs "commit" instead of "rollback". */
  commit(): void {
    throw new Error("TODO: implement commit");
  }

  /** Called by `using` at the end of the block: log "commit <name>" or "rollback <name>". */
  [Symbol.dispose](): void {
    throw new Error("TODO: implement Symbol.dispose");
  }
}

/** A connection whose cleanup is asynchronous. */
export class Connection implements AsyncDisposable {
  name: string;
  log: string[];

  private constructor(name: string, log: string[]) {
    this.name = name;
    this.log = log;
  }

  /** Provided: logs "open <name>" after a short wait. */
  static async open(name: string, log: string[]): Promise<Connection> {
    await delay(5);
    log.push(`open ${name}`);
    return new Connection(name, log);
  }

  /** Provided: logs "<name>: <sql>" and returns "rows for <sql>". */
  async query(sql: string): Promise<string> {
    await delay(5);
    this.log.push(`${this.name}: ${sql}`);
    return `rows for ${sql}`;
  }

  /** Called by `await using`: wait 5 ms with `delay`, then log "close <name>". */
  async [Symbol.asyncDispose](): Promise<void> {
    throw new Error("TODO: implement Symbol.asyncDispose");
  }
}

/**
 * Run `steps` in one transaction named "transfer": check each with
 * `checkStep` and record it, then commit. A failing step's error is thrown,
 * and the transaction rolls back by itself.
 */
export function transfer(log: string[], steps: string[]): void {
  throw new Error("TODO: implement transfer");
}

/**
 * Open a connection named `name` with `Connection.open`, run `sql` on it and
 * return the result. The connection must be closed before this returns.
 */
export async function runQuery(log: string[], name: string, sql: string): Promise<string> {
  throw new Error("TODO: implement runQuery");
}

/**
 * Open one transaction per name (check each with `checkName` first) on a
 * DisposableStack and commit them all. If a name is bad, its error is thrown
 * and the transactions opened so far roll back, newest first.
 */
export function openAll(log: string[], names: string[]): void {
  throw new Error("TODO: implement openAll");
}
