# A JSON API: bodies, problems and pages

An API speaks JSON in both directions, and its errors should be JSON too: a client parsing the response shouldn't get an HTML error page. RFC 9457, "Problem Details for HTTP APIs", standardises the shape: `{"type", "title", "status", "detail"}` with the content type `application/problem+json`.

```php
<?php
declare(strict_types=1);

foreach (['{"name": "Ada"}', '{"name": ', 'null', '[1, 2]'] as $body) {
    try {
        $data = json_decode($body, false, 64, JSON_THROW_ON_ERROR);
        echo $body, ' -> ', $data instanceof stdClass ? 'an object' : gettype($data), "\n";
    } catch (JsonException $e) {
        echo $body, ' -> ', $e->getMessage(), "\n";   // Syntax error
    }
}

// Pages: slice the list, and link to the neighbours.
$items = range(1, 25);
$page = 2;
$perPage = 10;
$query = ['sort' => 'desc'];
echo json_encode(array_slice($items, ($page - 1) * $perPage, $perPage)), "\n";
echo '</items?' . http_build_query(['page' => $page + 1] + $query) . '>; rel="next"', "\n";
```

Without `JSON_THROW_ON_ERROR`, `json_decode` returns `null` for broken input, the same as for the valid JSON `null`, and you have to remember to call `json_last_error()`. With it, broken input throws a `JsonException` with a message saying why.

Decoding to objects (`false` as the second argument) tells `{}` apart from `[]`, which both become an empty array when decoded to arrays.

Throwing an `HttpError` deep inside a handler and turning it into a problem response in one middleware keeps every handler simple. The handler says what went wrong; the middleware decides what that looks like on the wire.

`Link` headers (RFC 8288) are how APIs such as GitHub's point to the next and previous pages, so clients don't build page URLs themselves.

<!-- from:typescript -->
## Coming from TypeScript
- `JSON.parse` always throws on bad input; PHP's `json_decode` only does with `JSON_THROW_ON_ERROR`.
- PHP has no distinction between a JS array and object until you decode to objects: associative arrays play both roles.
- `['page' => 3] + $query` is like `{ page: 3, ...query }` with the left side winning: `+` keeps the left array's keys.
<!-- /from -->

<!-- from:default -->
## Key points
- Always decode with `JSON_THROW_ON_ERROR`, and check the shape you got.
- Return errors as `application/problem+json`, from one middleware.
- Validate query parameters like any other input; a bad page number is a 400.
<!-- /from -->

## Common mistake
`(int) $request->query['page']` and moving on. `(int) "two"` is 0 and `(int) "1.5"` is 1, so bad input quietly turns into a different page instead of an error the client can see and fix.

## In this stage
In `Api.php`, implement `problem`, `json_body`, `ProblemMiddleware` and `paginate`. `HttpError` is given.
