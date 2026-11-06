// TODO: these are plain aliases, so the type checker can't tell them apart
// (branded-ids.types.ts checks that it can). Turn each into a branded type.
export type UserId = string;
export type OrderId = string;
export type Cents = number;

/** A user ID looks like "u_" followed by lowercase letters and digits. Throw "not a user id: <raw>" otherwise. */
export function userId(raw: string): UserId {
  throw new Error("TODO: implement userId");
}

/** An order ID looks like "o_" followed by digits. Throw "not an order id: <raw>" otherwise. */
export function orderId(raw: string): OrderId {
  throw new Error("TODO: implement orderId");
}

/** A whole, non-negative number of cents. Throw "not an amount in cents: <amount>" otherwise. */
export function cents(amount: number): Cents {
  throw new Error("TODO: implement cents");
}

/** The sum of two amounts, still in cents. */
export function addCents(a: Cents, b: Cents): Cents {
  throw new Error("TODO: implement addCents");
}

/** Euros with two decimals: 1234 cents is "€12.34", 5 is "€0.05". */
export function formatCents(amount: Cents): string {
  throw new Error("TODO: implement formatCents");
}

// TODO: the annotation makes the keys plain strings, so priceOf("cake")
// compiles. Check the values with `satisfies` instead, to keep the keys.
export const PRICES: Record<string, Cents> = {
  coffee: 250 as Cents,
  tea: 180 as Cents,
};

/** The price of an item on the menu. */
export function priceOf(item: keyof typeof PRICES): Cents {
  throw new Error("TODO: implement priceOf");
}
