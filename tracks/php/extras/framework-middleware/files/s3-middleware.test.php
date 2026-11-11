<?php
// Stage 3 of the web framework project. Don't edit this file: `learn`
// overwrites it. Your code goes in Middleware.php.

declare(strict_types=1);

require __DIR__ . '/../harness.php';
require_once __DIR__ . '/Middleware.php';

/** Records its name before and after the rest of the pipeline. */
final class Tracer implements Middleware
{
    public static array $log = [];

    public function __construct(private string $name)
    {
    }

    public function process(Request $request, Handler $next): Response
    {
        self::$log[] = "{$this->name}:in";
        $response = $next->handle($request);
        self::$log[] = "{$this->name}:out";
        return $response->withHeader('X-' . $this->name, '1');
    }
}

/** Answers without calling the rest. */
final class Blocker implements Middleware
{
    public function process(Request $request, Handler $next): Response
    {
        return Response::text('blocked', 403);
    }
}

function core(): Handler
{
    return (new Router())
        ->get('/ok', function () {
            Tracer::$log[] = 'core';
            return Response::text('ok');
        })
        ->get('/boom', fn() => throw new LogicException('the database is gone'));
}

$t = new Harness();

$t->scenario('c13_the_first_middleware_is_outermost', function () {
    Tracer::$log = [];
    $r = (new Pipeline(core(), new Tracer('A'), new Tracer('B')))->handle(Request::create('GET', '/ok'));
    Harness::expect(Tracer::$log, ['A:in', 'B:in', 'core', 'B:out', 'A:out'], 'order');
    Harness::expect([$r->body, $r->header('X-A'), $r->header('X-B')], ['ok', '1', '1'], 'each could change the response');
});

$t->scenario('c14_a_middleware_can_answer_by_itself', function () {
    Tracer::$log = [];
    $r = (new Pipeline(core(), new Tracer('A'), new Blocker(), new Tracer('B')))->handle(Request::create('GET', '/ok'));
    Harness::expect([$r->status, $r->body], [403, 'blocked'], 'response');
    Harness::expect(Tracer::$log, ['A:in', 'A:out'], 'B and the core never ran');
    Harness::expect((new Pipeline(core()))->handle(Request::create('GET', '/ok'))->body, 'ok', 'no middleware at all');
});

$t->scenario('c15_errors_become_a_500', function () {
    $r = (new Pipeline(core(), new ErrorMiddleware()))->handle(Request::create('GET', '/boom'));
    Harness::expect([$r->status, $r->body], [500, 'Internal Server Error'], 'no details by default');
    $d = (new Pipeline(core(), new ErrorMiddleware(debug: true)))->handle(Request::create('GET', '/boom'));
    Harness::expect([$d->status, $d->body], [500, 'LogicException: the database is gone'], 'details when debugging');
    $ok = (new Pipeline(core(), new ErrorMiddleware()))->handle(Request::create('GET', '/ok'));
    Harness::expect($ok->status, 200, 'a working request passes through');
});

$t->scenario('c16_timing_adds_a_server_timing_header', function () {
    $r = (new Pipeline(core(), new TimingMiddleware()))->handle(Request::create('GET', '/ok'));
    Harness::check(preg_match('/^app;dur=\d+\.\d$/', $r->header('Server-Timing') ?? '') === 1, 'expected "app;dur=<ms with one decimal>", got ' . var_export($r->header('Server-Timing'), true));
});

$t->scenario('c17_trailing_slashes_redirect_with_a_301', function () {
    $p = new Pipeline(core(), new TrailingSlashMiddleware());
    $r = $p->handle(Request::create('GET', '/ok/?a=1&b=2'));
    Harness::expect([$r->status, $r->header('Location')], [301, '/ok?a=1&b=2'], 'redirect keeps the query');
    Harness::expect($p->handle(Request::create('GET', '/ok//'))->header('Location'), '/ok', 'several slashes');
    Harness::expect($p->handle(Request::create('GET', '/'))->status, 404, '/ itself is not redirected');
});

$t->scenario('c18_a_callable_handler_wraps_a_function', function () {
    $h = new CallableHandler(fn(Request $r) => Response::text($r->path));
    Harness::check($h instanceof Handler, 'CallableHandler is a Handler');
    Harness::expect((new Pipeline($h, new Tracer('A')))->handle(Request::create('GET', '/x'))->body, '/x', 'body');
});

$t->finish();
