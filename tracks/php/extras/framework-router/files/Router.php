<?php
// Which code answers which URL.

declare(strict_types=1);

require_once __DIR__ . '/Http.php';

/** Anything that turns a request into a response. */
interface Handler
{
    public function handle(Request $request): Response;
}

final class Router implements Handler
{
    public function get(string $pattern, callable $handler): self
    {
        throw new RuntimeException('TODO: implement Router::get');
    }

    public function post(string $pattern, callable $handler): self
    {
        throw new RuntimeException('TODO: implement Router::post');
    }

    /** Add a route; returns $this so calls chain. */
    public function add(Method $method, string $pattern, callable $handler): self
    {
        throw new RuntimeException('TODO: implement Router::add');
    }

    /**
     * The first route matching the method and the whole path. Its handler gets
     * the request (with a "params" attribute) and the parameters array, and may
     * return a Response, a string (a text response) or an array (JSON).
     * "{name}" matches one path segment; "{id:\d+}" matches that regex.
     * No route for the path: 404 "Not Found". The path matches, but only for
     * other methods: 405 "Method Not Allowed" with an Allow header listing
     * them, in the order they were added ("POST, GET").
     */
    public function handle(Request $request): Response
    {
        throw new RuntimeException('TODO: implement Router::handle');
    }
}
