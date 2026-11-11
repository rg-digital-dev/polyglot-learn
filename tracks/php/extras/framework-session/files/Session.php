<?php
// Per-visitor state, one-time messages, and protection against forged forms.

declare(strict_types=1);

require_once __DIR__ . '/Middleware.php';

interface Session
{
    public function get(string $key, mixed $default = null): mixed;
    public function set(string $key, mixed $value): void;
    public function remove(string $key): void;
    /** Keep $value for the next takeFlash($key) only. */
    public function flash(string $key, mixed $value): void;
    /** The flashed value, removed as it's read. */
    public function takeFlash(string $key, mixed $default = null): mixed;
}

/** A session in memory: for tests, and for one request in a CLI. A stored null counts as stored. */
final class ArraySession implements Session
{
    public function get(string $key, mixed $default = null): mixed
    {
        throw new RuntimeException('TODO: implement ArraySession::get');
    }

    public function set(string $key, mixed $value): void
    {
        throw new RuntimeException('TODO: implement ArraySession::set');
    }

    public function remove(string $key): void
    {
        throw new RuntimeException('TODO: implement ArraySession::remove');
    }

    public function flash(string $key, mixed $value): void
    {
        throw new RuntimeException('TODO: implement ArraySession::flash');
    }

    public function takeFlash(string $key, mixed $default = null): mixed
    {
        throw new RuntimeException('TODO: implement ArraySession::takeFlash');
    }
}

final class Csrf
{
    public function __construct(private readonly Session $session)
    {
    }

    /**
     * The session's token: 64 hex characters from a cryptographically secure
     * random source, made the first time and then reused.
     */
    public function token(): string
    {
        throw new RuntimeException('TODO: implement Csrf::token');
    }

    /** Whether $given matches the session's token, compared in constant time. No token yet: false. */
    public function valid(?string $given): bool
    {
        throw new RuntimeException('TODO: implement Csrf::valid');
    }
}

/**
 * Safe methods pass. Others need the token, in an X-CSRF-Token header or a
 * _token field of a form body; without a valid one the answer is
 * 419 "CSRF token mismatch".
 */
final class CsrfMiddleware implements Middleware
{
    public function __construct(private readonly Csrf $csrf)
    {
    }

    public function process(Request $request, Handler $next): Response
    {
        throw new RuntimeException('TODO: implement CsrfMiddleware::process');
    }
}
