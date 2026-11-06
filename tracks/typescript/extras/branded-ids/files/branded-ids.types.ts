// Part of the TypeScript in Practice module (tracks/typescript/extras/branded-ids). Don't
// edit this file: `learn` overwrites it. Your code goes in branded-ids.ts.
//
// Never run, only type-checked: this is case 1, "types check". Each
// `@ts-expect-error` line MUST be a type error. If your types let it through,
// tsc reports "Unused '@ts-expect-error' directive" for that line.

import { PRICES, addCents, cents, orderId, priceOf, userId } from "./branded-ids.ts";
import type { Cents, OrderId, UserId } from "./branded-ids.ts";

function loadUser(id: UserId): UserId {
  return id;
}
function loadOrder(id: OrderId): OrderId {
  return id;
}

const user = userId("u_ada");
const order = orderId("o_1");
loadUser(user);
loadOrder(order);

// @ts-expect-error a plain string isn't a UserId
loadUser("u_ada");
// @ts-expect-error an OrderId isn't a UserId
loadUser(order);
// @ts-expect-error a UserId isn't an OrderId
loadOrder(user);

// A branded value is still usable as what it's built on.
const text: string = user;
const shout: string = user.toUpperCase();

const total: Cents = addCents(cents(250), cents(180));
// @ts-expect-error a plain number isn't Cents
addCents(250, cents(180));
// @ts-expect-error adding with + gives a plain number, not Cents
const sum: Cents = total + total;
const plain: number = total;

const coffee: Cents = priceOf("coffee");
const tea: Cents = PRICES.tea;
// @ts-expect-error there's no cake on the menu
priceOf("cake");
// @ts-expect-error PRICES only has coffee and tea
PRICES.cake;

export { coffee, plain, shout, sum, tea, text };
