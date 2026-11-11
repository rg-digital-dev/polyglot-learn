<?php
// Code that runs around every request: before the router, after it, or instead of it.

declare(strict_types=1);

require_once __DIR__ . '/Router.php';

interface Middleware
{
    /** Handle the request, usually by passing it on with $next->handle(). */
    public function process(Request $request, Handler $next): Response;
}

/** A Handler made from a function. */
final class CallableHandler implements Handler
{
    public function __construct(private readonly Closure $fn)
    {
    }

    public function handle(Request $request): Response
    {
        throw new RuntimeException('TODO: implement CallableHandler::handle');
    }
}

/** The core handler wrapped in middleware; the first one is outermost. */
final class Pipeline implements Handler
{
    public function __construct(private readonly Handler $core, Middleware ...$middleware)
    {
        throw new RuntimeException('TODO: implement Pipeline');
    }

    public function handle(Request $request): Response
    {
        throw new RuntimeException('TODO: implement Pipeline::handle');
    }
}

/** Any exception becomes a 500: "Internal Server Error", or "<class>: <message>" when debugging. */
final class ErrorMiddleware implements Middleware
{
    public function __construct(private readonly bool $debug = false)
    {
    }

    public function process(Request $request, Handler $next): Response
    {
        throw new RuntimeException('TODO: implement ErrorMiddleware::process');
    }
}

/** Adds "Server-Timing: app;dur=<milliseconds, one decimal>". */
final class TimingMiddleware implements Middleware
{
    public function process(Request $request, Handler $next): Response
    {
        throw new RuntimeException('TODO: implement TimingMiddleware::process');
    }
}

/** "/posts/" redirects (301) to "/posts", keeping the query string; "/" itself is left alone. */
final class TrailingSlashMiddleware implements Middleware
{
    public function process(Request $request, Handler $next): Response
    {
        throw new RuntimeException('TODO: implement TrailingSlashMiddleware::process');
    }
}
