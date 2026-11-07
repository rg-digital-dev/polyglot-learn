# Error strategies in PHP

`array_key_exists($key, $settings)` asks whether the key is there; `isset` also says no for a `null` value, and `??` falls back for both. PHP's loose conversions are the bigger risk. The string `"0"` is falsy (`empty("0")` is `true`), so `$settings[$key] ?: $fallback` replaces a real `0`. And `(int)` never fails: `(int)"eighty"` and `(int)""` are `0`, and `(int)"8_080"` is `8`. Check with `ctype_digit` before converting.

PHP has no built-in "many errors" exception. Collect the messages in an array and throw once:

```php
$problems = [];
if (strlen($name) < 3) {
    $problems[] = 'name: too short';
}
if (!str_contains($email, '@')) {
    $problems[] = 'email: missing @';
}
if ($problems !== []) {
    throw new InvalidArgumentException(implode('; ', $problems));
}
```

<!-- from:typescript -->
## Coming from TypeScript
- `array_key_exists` is `Object.hasOwn`; arrays have no prototype, so `"constructor"` is just a key.
- `"0"` is falsy in PHP, unlike in JavaScript, so `?:` is riskier than `||`.
- `(int)` is like `parseInt` but never gives `NaN`: anything unreadable becomes `0`.
<!-- /from -->

<!-- from:default -->
## Key points
- `array_key_exists` for "is it there?"; `?:` and `empty()` treat `"0"` as missing.
- `ctype_digit` before `(int)`, which turns anything into a number.
- Collect messages in an array and throw one exception.
<!-- /from -->

## Common mistake
Falling back with `?:`:

```php
return (int) ($settings[$key] ?? '') ?: $fallback; // "0" and "eighty" both give the fallback
```

## In these exercises
`intSetting` checks `array_key_exists`, then `ctype_digit`, and throws `InvalidArgumentException` with the key in the message. `validateSignup` counts characters with `preg_match_all('/./su', …)`, which needs no mbstring, collects the problems, and throws once with `implode('; ', …)`.
