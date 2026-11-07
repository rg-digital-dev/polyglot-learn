# Error strategies in TypeScript

A `Record<string, string>` is a plain object, so it inherits from `Object.prototype`. `settings["constructor"]` is a function, not `undefined`, and `settings[key] ?? fallback` doesn't fall back for it. `Object.hasOwn(settings, key)` asks only about the object's own keys (a `Map` with `.has` avoids the question altogether). JavaScript's falsy values make the other check easy to get wrong too: `0`, `""` and `NaN` all count as missing for `||`.

For several errors at once there's `AggregateError`. It keeps the errors in `.errors`, but its `.message` is only the text you pass, so pass a message that lists them:

```ts
const problems: string[] = [];
if (name.length < 3) problems.push("name: too short");
if (!email.includes("@")) problems.push("email: missing @");
if (problems.length > 0) {
  throw new AggregateError(problems.map((p) => new Error(p)), problems.join("; "));
}
```

<!-- from:javascript -->
## Coming from JavaScript
- The same runtime: `Object.hasOwn`, `AggregateError` and the falsy values all behave as they do in plain JavaScript.
- The types won't catch `settings[key]` finding an inherited property: `Record<string, string>` says every key is a string.
- `catch (e)` gives `unknown`; check `e instanceof Error` before reading `.message`.
<!-- /from -->

<!-- from:default -->
## Key points
- `Object.hasOwn(obj, key)`, not `obj[key] !== undefined`, to see whether a key is there.
- `||` falls back for `0` and `""`; `??` only for `null` and `undefined`.
- `AggregateError`'s message doesn't list its errors unless you put them there.
<!-- /from -->

## Common mistake
Converting with `Number` or `parseInt` and falling back on anything odd:

```ts
return parseInt(settings[key]) || fallback; // "0" gives the fallback, "12abc" gives 12
Number("");                                 // 0, not NaN
```

## In these exercises
`intSetting` checks `Object.hasOwn` first, then tests the value with `/^[0-9]+$/` before converting, and throws with the key in the message. `validateSignup` pushes each problem to an array and throws once with all of them in the message.
