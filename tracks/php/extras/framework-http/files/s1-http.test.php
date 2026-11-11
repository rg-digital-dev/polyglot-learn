<?php
// Stage 1 of the web framework project. Don't edit this file: `learn`
// overwrites it. Your code goes in Http.php.

declare(strict_types=1);

require __DIR__ . '/../harness.php';
require_once __DIR__ . '/Http.php';

$t = new Harness();

$t->scenario('c01_create_parses_the_method_path_and_query', function () {
    $r = Request::create('post', '/posts/7?sort=new&tag[]=php&tag[]=web');
    Harness::check($r->method === Method::Post, 'method should be Method::Post');
    Harness::expect($r->path, '/posts/7', 'path');
    Harness::expect($r->query, ['sort' => 'new', 'tag' => ['php', 'web']], 'query');
    Harness::expect(Request::create('GET', '')->path, '/', 'an empty URI is /');
    Harness::expect(Request::create('GET', '/?a=1')->query, ['a' => '1'], 'query on /');
    Harness::expect(Request::create('GET', '/caf%C3%A9/a%20b')->path, '/café/a b', 'the path is decoded');
});

$t->throws('c02_an_unknown_method_is_rejected', fn() => Request::create('BREW', '/'), ['BREW']);

$t->scenario('c03_headers_match_in_any_case', function () {
    $r = Request::create('GET', '/', ['Content-Type' => 'text/html', 'X-Request-ID' => 'abc']);
    Harness::expect($r->header('content-type'), 'text/html', 'content-type');
    Harness::expect($r->header('X-REQUEST-ID'), 'abc', 'x-request-id');
    Harness::expect($r->header('Accept'), null, 'a missing header');
});

$t->scenario('c04_requests_and_responses_are_read_only', function () {
    $r = Request::create('GET', '/a');
    $response = new Response();
    foreach ([fn() => $r->path = '/b', fn() => $response->status = 500] as $i => $change) {
        try {
            $change();
            Harness::check(false, "change $i worked: make the properties readonly");
        } catch (Error $e) {
            Harness::check(str_contains($e->getMessage(), 'readonly'), 'expected a readonly error, got: ' . $e->getMessage());
        }
    }
    $r2 = $r->withAttribute('user', 'ada');
    Harness::expect([$r->attribute('user'), $r2->attribute('user'), $r2->attribute('none', 'x')], [null, 'ada', 'x'], 'withAttribute makes a copy');
    Harness::check(Method::Get->isSafe() && !Method::Post->isSafe() && Method::Head->isSafe() && !Method::Delete->isSafe(), 'isSafe: GET, HEAD and OPTIONS only');
});

$t->scenario('c05_with_methods_return_changed_copies', function () {
    $a = Response::text('hi');
    $b = $a->withStatus(201)->withHeader('X-Id', '7')->withHeader('content-type', 'text/csv');
    Harness::expect([$a->status, $a->header('X-Id'), $a->header('Content-Type')], [200, null, 'text/plain; charset=utf-8'], 'the original');
    Harness::expect([$b->status, $b->header('x-id'), $b->header('Content-Type'), $b->body], [201, '7', 'text/csv', 'hi'], 'the copy');
    Harness::expect(count($b->headers), 2, 'withHeader replaced Content-Type instead of adding a second one');
    try {
        $a->withStatus(99);
        Harness::check(false, 'withStatus(99) should throw InvalidArgumentException');
    } catch (InvalidArgumentException) {
    }
});

$t->scenario('c06_response_shortcuts_set_type_and_status', function () {
    $j = Response::json(['name' => 'Zoë', 'url' => 'a/b'], 201);
    Harness::expect([$j->status, $j->header('Content-Type'), $j->body], [201, 'application/json', '{"name":"Zoë","url":"a/b"}'], 'json');
    $h = Response::html('<p>x</p>');
    Harness::expect($h->header('Content-Type'), 'text/html; charset=utf-8', 'html');
    $r = Response::redirect('/login');
    Harness::expect([$r->status, $r->header('Location'), $r->body], [302, '/login', ''], 'redirect');
    Harness::expect(Response::redirect('/new', 301)->status, 301, 'permanent redirect');
});

$t->finish();
