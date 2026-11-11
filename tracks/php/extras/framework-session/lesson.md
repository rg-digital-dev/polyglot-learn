# Sessions, flash messages and CSRF tokens

HTTP forgets everything between requests, so sessions keep state per visitor: who's logged in, what's in the cart. PHP's own `$_SESSION` is global, and tests and long-running servers hate globals. Hiding it behind an interface lets the framework use an in-memory session in tests and a real one in production.

A flash message is session data that lives for exactly one more read: "Saved!" after a redirect. CSRF protection uses the session too. A forged form on another site can make a logged-in visitor's browser post to yours, cookie and all. What it can't do is read a secret token from your pages, so every form carries one and every unsafe request must send it back.

```php
<?php
declare(strict_types=1);

$session = [];

// One token per session, from the operating system's secure random source.
$session['_csrf'] ??= bin2hex(random_bytes(32));
echo strlen($session['_csrf']), " hex characters\n";   // 64

function tokenOk(array $session, ?string $sent): bool
{
    // hash_equals takes the same time however much matches; === stops early.
    return isset($session['_csrf'], $sent) && hash_equals($session['_csrf'], $sent);
}

var_dump(tokenOk($session, $session['_csrf']));   // true
var_dump(tokenOk($session, 'guess'));             // false
var_dump(tokenOk($session, null));                // false

// Every form carries it in a hidden field named _token, printed with e().
```

`random_bytes` uses the operating system's cryptographically secure generator. `rand`, `mt_rand` and `uniqid` are predictable and must never make secrets.

`hash_equals` compares in constant time. With `===`, a wrong first character fails a little faster than a wrong last one, and an attacker who can time thousands of requests can recover a token a character at a time.

Safe methods (GET, HEAD, OPTIONS) shouldn't change anything, so they need no token. Every other method does. **419** is the status Laravel made popular for a failed token.

<!-- from:typescript -->
## Coming from TypeScript
- This is express-session plus the csurf idea, written out. Modern browsers' `SameSite` cookies help too, but tokens still protect older browsers and same-site attacks.
- `random_bytes(32)` is Node's `crypto.randomBytes(32)`; `hash_equals` is `crypto.timingSafeEqual`.
- `array_key_exists` vs `isset` is like `in` vs `!== undefined`: `isset` treats a stored null as missing.
<!-- /from -->

<!-- from:default -->
## Key points
- Put the session behind an interface; use an in-memory one in tests.
- Make tokens with `random_bytes`, compare them with `hash_equals`.
- Require the token on every unsafe method; let safe ones through.
<!-- /from -->

## Common mistake
Checking the token only on the forms you remembered. Protection that's opt-in per route gets forgotten on the one route that matters. A middleware that guards every unsafe request is safe by default.

## In this stage
In `Session.php`, implement `ArraySession`, `Csrf` and `CsrfMiddleware`.
