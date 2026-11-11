<?php
// Stage 8 of the web framework project. Don't edit this file: `learn`
// overwrites it. Your code goes in App.php (and public/index.php).

declare(strict_types=1);

require __DIR__ . '/../harness.php';
require_once __DIR__ . '/App.php';

final class Clock
{
    public function now(): string
    {
        return '2026-11-11';
    }
}

final class Greeter
{
    public function __construct(public readonly Clock $clock, public readonly string $greeting = 'Hello')
    {
    }
}

final class Needy
{
    public function __construct(public readonly string $secret)
    {
    }
}

interface Mailer
{
}

final class First
{
    public function __construct(public readonly Second $second)
    {
    }
}

final class Second
{
    public function __construct(public readonly First $first)
    {
    }
}

final class Stamp implements Middleware
{
    public function __construct(private readonly Clock $clock)
    {
    }

    public function process(Request $request, Handler $next): Response
    {
        return $next->handle($request)->withHeader('X-Date', $this->clock->now());
    }
}

$t = new Harness();

$t->scenario('c43_the_container_builds_classes_and_their_dependenc', function () {
    $c = new Container();
    $g = $c->get(Greeter::class);
    Harness::check($g instanceof Greeter && $g->clock instanceof Clock, 'a Greeter with a Clock');
    Harness::expect($g->greeting, 'Hello', 'a default for the string parameter');
    Harness::check($c->get(Greeter::class) === $g && $c->get(Clock::class) === $g->clock, 'each is built once');
});

$t->scenario('c44_factories_decide_how_to_build', function () {
    $c = new Container();
    $c->set(Needy::class, fn() => new Needy('s3cret'));
    $c->set('greeting', fn(Container $c) => $c->get(Greeter::class)->greeting . ', world');
    Harness::expect([$c->get(Needy::class)->secret, $c->get('greeting')], ['s3cret', 'Hello, world'], 'values');
    Harness::check($c->has(Clock::class) && $c->has('greeting') && !$c->has('nope'), 'has()');
});

$t->scenario('c45_the_container_explains_what_it_cannot_build', function () {
    $c = new Container();
    $message = function (string $id) use ($c): string {
        try {
            $c->get($id);
        } catch (ContainerError $e) {
            return $e->getMessage();
        }
        return 'no ContainerError';
    };
    Harness::expect($message(Needy::class), "can't build Needy: nothing to pass as \$secret", 'a string without a default');
    Harness::expect($message(Mailer::class), "can't build Mailer: it isn't a concrete class", 'an interface');
    Harness::expect($message('Nowhere'), "can't build Nowhere: no such class", 'no class');
    Harness::expect($message(First::class), 'circular dependency: First -> Second -> First', 'a cycle');
});

$t->scenario('c46_the_app_routes_through_its_middleware', function () {
    $app = new App();
    $app->add(new ErrorMiddleware())
        ->add(Stamp::class)
        ->get('/hello/{name}', fn(Request $r, array $p) => "hi {$p['name']}")
        ->get('/fail', fn() => throw new RuntimeException('x'));
    $r = $app->handle(Request::create('GET', '/hello/ada'));
    Harness::expect([$r->body, $r->header('X-Date')], ['hi ada', '2026-11-11'], 'a class name is built by the container');
    Harness::expect($app->handle(Request::create('GET', '/fail'))->status, 500, 'errors are handled');
    Harness::check($app->router === $app->container->get(Router::class), 'the router comes from the container');
});

$t->scenario('c47_emit_sends_the_status_headers_and_body', function () {
    $sent = [];
    ob_start();
    emit(Response::text('done', 201)->withHeader('X-Id', '9'), function (string $line) use (&$sent) {
        $sent[] = $line;
    });
    Harness::expect([$sent, ob_get_clean()], [['HTTP/1.1 201', 'Content-Type: text/plain; charset=utf-8', 'X-Id: 9'], 'done'], 'headers and body');
});

$t->scenario('c48_public_index_php_serves_requests', function () {
    // A small script that fakes one request, run in its own PHP process.
    $runner = sys_get_temp_dir() . '/framework-s8-' . getmypid() . '.php';
    file_put_contents($runner, '<?php $_SERVER["REQUEST_URI"] = "/hello/%3Cb%3E"; $_SERVER["REQUEST_METHOD"] = "GET"; require '
        . var_export(__DIR__ . '/public/index.php', true) . ';');
    $out = [];
    exec(escapeshellarg(PHP_BINARY) . ' ' . escapeshellarg($runner) . ' 2>&1', $out, $code);
    unlink($runner);
    $body = implode("\n", $out);
    Harness::expect($code, 0, "exit code; output: $body");
    Harness::check(str_contains($body, '&lt;b&gt;'), "the name is escaped: $body");
});

$t->finish();
