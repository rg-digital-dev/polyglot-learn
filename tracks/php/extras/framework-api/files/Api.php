<?php
// A JSON API: reading JSON bodies, errors as problem details, pages of results.

declare(strict_types=1);

require_once __DIR__ . '/Middleware.php';

/** An error with an HTTP status, turned into a problem response by ProblemMiddleware. */
final class HttpError extends RuntimeException
{
    public function __construct(public readonly int $status, string $title, public readonly ?string $detail = null)
    {
        parent::__construct($title);
    }
}

/**
 * An RFC 9457 problem details response, Content-Type application/problem+json:
 * {"type": "about:blank", "title", "status", "detail" (only if given)} plus $extra.
 */
function problem(int $status, string $title, ?string $detail = null, array $extra = []): Response
{
    throw new RuntimeException('TODO: implement problem');
}

/**
 * The request's JSON object as an array. A Content-Type without
 * application/json: HttpError 415 "Unsupported Media Type". Broken JSON:
 * HttpError 400 "Invalid JSON", the parser's message as the detail (use
 * JSON_THROW_ON_ERROR). JSON that isn't an object: 400 "Expected a JSON object".
 */
function json_body(Request $request): array
{
    throw new RuntimeException('TODO: implement json_body');
}

/** Turns an HttpError from further in into a problem response. */
final class ProblemMiddleware implements Middleware
{
    public function process(Request $request, Handler $next): Response
    {
        throw new RuntimeException('TODO: implement ProblemMiddleware::process');
    }
}

/**
 * One page of $items as JSON {"data", "page", "pages", "total"}. The page
 * comes from ?page= (1 if absent); anything but a positive whole number is
 * HttpError 400 "Invalid page". A Link header points to the next page and the
 * previous one ('</path?page=3&sort=desc>; rel="next", </path?page=1&sort=desc>; rel="prev"'),
 * page first, keeping the other query parameters; none when there's neither.
 */
function paginate(array $items, Request $request, int $perPage = 10): Response
{
    throw new RuntimeException('TODO: implement paginate');
}
