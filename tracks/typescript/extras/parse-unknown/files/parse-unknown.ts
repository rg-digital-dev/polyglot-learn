export interface User {
  name: string;
  age: number;
  tags: string[];
  email?: string;
}

/**
 * A type guard: return true when `value` is a valid User. After
 * `if (isUser(x))`, TypeScript treats `x` as a User. Valid means: a non-null,
 * non-array object with a string `name`, a number `age`, a `tags` array of
 * strings, and an `email` that is either missing or a string.
 */
export function isUser(value: unknown): value is User {
  throw new Error("TODO: implement isUser");
}

/**
 * Parse JSON text into a User. Return a new object with only the User fields
 * (drop any others). Throw an Error for malformed JSON, for a value that isn't
 * an object, and for a bad field; for a bad field the message starts with the
 * field's name, e.g. "age: expected a number".
 */
export function parseUser(json: string): User {
  throw new Error("TODO: implement parseUser");
}

/**
 * Parse JSON text holding an array of users. Throw an Error when it isn't an
 * array, or when an entry is invalid; then the message starts with the
 * entry's index in brackets, e.g. "[1] age: expected a number".
 */
export function parseUsers(json: string): User[] {
  throw new Error("TODO: implement parseUsers");
}
