<?php
// Requests and responses: plain values that never change once made.

declare(strict_types=1);

enum Method: string
{
    case Get = 'GET';
    case Head = 'HEAD';
    case Post = 'POST';
    case Put = 'PUT';
    case Patch = 'PATCH';
    case Delete = 'DELETE';
    case Options = 'OPTIONS';

    /** Safe methods only read: GET, HEAD and OPTIONS. */
    public function isSafe(): bool
    {
        throw new RuntimeException('TODO: implement Method::isSafe');
    }
}

// TODO: once made, a request must not change: make every property readonly.
final class Request
{
    /**
     * @param array<string, string> $query
     * @param array<string, string> $headers names in lower case
     * @param array<string, mixed> $attributes values added on the way, such as route parameters
     */
    public function __construct(
        public Method $method,
        public string $path,
        public array $query = [],
        public array $headers = [],
        public string $body = '',
        public array $attributes = [],
    ) {
    }

    /**
     * A request from a method name in any case, a URI and headers. The path is
     * decoded ("/caf%C3%A9" is "/café"), "/" if empty; the query string becomes
     * $query (parse_str); header names are stored lower-case. An unknown method
     * throws InvalidArgumentException("unknown method: <method>").
     */
    public static function create(string $method, string $uri, array $headers = [], string $body = ''): self
    {
        throw new RuntimeException('TODO: implement Request::create');
    }

    /** The header's value, the name matched in any case; null if absent. */
    public function header(string $name): ?string
    {
        throw new RuntimeException('TODO: implement Request::header');
    }

    public function attribute(string $name, mixed $default = null): mixed
    {
        throw new RuntimeException('TODO: implement Request::attribute');
    }

    /** A copy of the request with one attribute added or replaced. */
    public function withAttribute(string $name, mixed $value): self
    {
        throw new RuntimeException('TODO: implement Request::withAttribute');
    }
}

// TODO: readonly too.
final class Response
{
    /** @param array<string, string> $headers names as given; look them up with header() */
    public function __construct(
        public int $status = 200,
        public array $headers = [],
        public string $body = '',
    ) {
    }

    /** Content-Type "text/plain; charset=utf-8". */
    public static function text(string $body, int $status = 200): self
    {
        throw new RuntimeException('TODO: implement Response::text');
    }

    /** Content-Type "text/html; charset=utf-8". */
    public static function html(string $body, int $status = 200): self
    {
        throw new RuntimeException('TODO: implement Response::html');
    }

    /** Content-Type "application/json"; slashes and Unicode not escaped; invalid data throws. */
    public static function json(mixed $data, int $status = 200): self
    {
        throw new RuntimeException('TODO: implement Response::json');
    }

    /** A Location header and no body; 302 unless told otherwise. */
    public static function redirect(string $to, int $status = 302): self
    {
        throw new RuntimeException('TODO: implement Response::redirect');
    }

    /** A copy with another status: 100 to 599, else InvalidArgumentException. */
    public function withStatus(int $status): self
    {
        throw new RuntimeException('TODO: implement Response::withStatus');
    }

    /** A copy with this header set, replacing one with the same name in any case. */
    public function withHeader(string $name, string $value): self
    {
        throw new RuntimeException('TODO: implement Response::withHeader');
    }

    /** The header's value, the name matched in any case; null if absent. */
    public function header(string $name): ?string
    {
        throw new RuntimeException('TODO: implement Response::header');
    }
}
