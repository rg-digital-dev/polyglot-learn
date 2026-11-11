<?php
// Stage 7 of the web framework project. Don't edit this file: `learn`
// overwrites it. Your code goes in Api.php.

declare(strict_types=1);

require __DIR__ . '/../harness.php';
require_once __DIR__ . '/Api.php';

function api(): Pipeline
{
    $router = (new Router())
        ->post('/notes', fn(Request $r) => Response::json(['saved' => json_body($r)], 201))
        ->get('/numbers', fn(Request $r) => paginate(range(1, 25), $r));
    return new Pipeline($router, new ProblemMiddleware());
}

function json(Response $r): mixed
{
    return json_decode($r->body, true);
}

$t = new Harness();

$t->scenario('c37_problem_builds_rfc_9457_details', function () {
    $p = problem(404, 'Not Found', 'no note 7', ['note' => 7]);
    Harness::expect([$p->status, $p->header('Content-Type')], [404, 'application/problem+json'], 'status and type');
    Harness::expect(json($p), ['type' => 'about:blank', 'title' => 'Not Found', 'status' => 404, 'detail' => 'no note 7', 'note' => 7], 'body');
    Harness::expect(array_key_exists('detail', json(problem(500, 'Oops'))), false, 'no detail when none is given');
});

$t->scenario('c38_a_json_body_is_read', function () {
    $r = api()->handle(Request::create('POST', '/notes', ['Content-Type' => 'application/json; charset=utf-8'], '{"text": "hi", "tags": ["a"]}'));
    Harness::expect([$r->status, json($r)], [201, ['saved' => ['text' => 'hi', 'tags' => ['a']]]], 'response');
});

$t->scenario('c39_bad_json_bodies_are_problems', function () {
    $send = fn(string $body, string $type = 'application/json') => api()->handle(Request::create('POST', '/notes', ['Content-Type' => $type], $body));
    $broken = $send('{"text": ');
    Harness::expect([$broken->status, json($broken)['title']], [400, 'Invalid JSON'], 'broken JSON');
    Harness::check(($broken->header('Content-Type') ?? '') === 'application/problem+json', 'as a problem response');
    Harness::check(str_contains(json($broken)['detail'] ?? '', 'Syntax error'), 'detail has the parser message');
    Harness::expect(json($send('[1, 2]'))['title'], 'Expected a JSON object', 'a list');
    Harness::expect(json($send('{}'))['title'] ?? 'saved', 'saved', 'an empty object is fine');
    Harness::expect($send('text=hi', 'application/x-www-form-urlencoded')->status, 415, 'not JSON at all');
});

$t->scenario('c40_paginate_returns_a_page_and_totals', function () {
    $r = api()->handle(Request::create('GET', '/numbers?page=2'));
    Harness::expect(json($r), ['data' => range(11, 20), 'page' => 2, 'pages' => 3, 'total' => 25], 'page 2');
    Harness::expect(json(api()->handle(Request::create('GET', '/numbers')))['data'], range(1, 10), 'page 1 by default');
    Harness::expect(json(api()->handle(Request::create('GET', '/numbers?page=9')))['data'], [], 'past the end');
});

$t->scenario('c41_link_headers_point_to_the_next_and_previous_page', function () {
    $link = fn(string $uri) => api()->handle(Request::create('GET', $uri))->header('Link');
    Harness::expect($link('/numbers?page=2&sort=desc'), '</numbers?page=3&sort=desc>; rel="next", </numbers?page=1&sort=desc>; rel="prev"', 'page 2 of 3');
    Harness::expect($link('/numbers'), '</numbers?page=2>; rel="next"', 'page 1');
    Harness::expect($link('/numbers?page=3'), '</numbers?page=2>; rel="prev"', 'the last page');
});

$t->scenario('c42_a_bad_page_number_is_a_400', function () {
    foreach (['0', '-1', 'two', '1.5'] as $page) {
        $r = api()->handle(Request::create('GET', "/numbers?page=$page"));
        Harness::expect([$r->status, json($r)['title']], [400, 'Invalid page'], "page=$page");
    }
});

$t->finish();
