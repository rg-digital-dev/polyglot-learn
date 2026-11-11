<?php
// Putting it together: a container that builds objects, and the app itself.

declare(strict_types=1);

require_once __DIR__ . '/Middleware.php';

final class ContainerError extends RuntimeException
{
}

/** Builds objects and their dependencies, once each. */
final class Container
{
    /** Build $id with $factory($container) when it's first asked for. */
    public function set(string $id, Closure $factory): void
    {
        throw new RuntimeException('TODO: implement Container::set');
    }

    /** Whether get($id) has something to try: a factory, a built value, or a class. */
    public function has(string $id): bool
    {
        throw new RuntimeException('TODO: implement Container::has');
    }

    /**
     * The one $id, built the first time and kept: from its factory, or by
     * building the class, asking the container for each constructor parameter
     * typed with a class or interface and using defaults for the rest.
     * Errors are ContainerError:
     *   "can't build <id>: no such class"
     *   "can't build <id>: it isn't a concrete class"      (an interface)
     *   "can't build <id>: nothing to pass as $<param>"
     *   "circular dependency: First -> Second -> First"
     */
    public function get(string $id): mixed
    {
        throw new RuntimeException('TODO: implement Container::get');
    }
}

final class App implements Handler
{
    public readonly Router $router;

    /** The router comes from the container. */
    public function __construct(public readonly Container $container = new Container())
    {
        throw new RuntimeException('TODO: implement App');
    }

    public function get(string $pattern, callable $handler): self
    {
        throw new RuntimeException('TODO: implement App::get');
    }

    public function post(string $pattern, callable $handler): self
    {
        throw new RuntimeException('TODO: implement App::post');
    }

    /** Add middleware, outermost first: an object, or a class name the container builds when a request comes. */
    public function add(Middleware|string $middleware): self
    {
        throw new RuntimeException('TODO: implement App::add');
    }

    public function handle(Request $request): Response
    {
        throw new RuntimeException('TODO: implement App::handle');
    }
}

/**
 * Send $response: "HTTP/1.1 <status>", then "<Name>: <value>" for each
 * header, both through $header (PHP's header() when null), then echo the body.
 */
function emit(Response $response, ?callable $header = null): void
{
    throw new RuntimeException('TODO: implement emit');
}
