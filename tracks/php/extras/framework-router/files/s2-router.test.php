<?php
// Stage 2 of the web framework project. Don't edit this file: `learn`
// overwrites it. Your code goes in Router.php.

declare(strict_types=1);

require __DIR__ . '/../harness.php';
require_once __DIR__ . '/Router.php';

function blog(): Router
{
    $router = new Router();
    $router->get('/', fn() => Response::text('home'))
        ->get('/posts/{id:\d+}', fn(Request $r, array $p) => Response::text("post {$p['id']}"))
        ->get('/users/{name}/posts/{slug}', fn(Request $r, array $p) => Response::text("{$p['name']}: {$p['slug']}"))
        ->post('/posts', fn() => Response::text('created', 201))
        ->get('/posts', fn() => Response::text('list'));
    return $router;
}

function get(string $uri, string $method = 'GET'): Response
{
    return blog()->handle(Request::create($method, $uri));
}

$t = new Harness();

$t->scenario('c07_static_routes_match_by_method_and_path', function () {
    Harness::expect([get('/')->body, get('/posts')->body, get('/posts', 'POST')->body, get('/posts', 'POST')->status], ['home', 'list', 'created', 201], 'bodies');
});

$t->scenario('c08_parameters_are_captured_by_name', function () {
    Harness::expect(get('/posts/42')->body, 'post 42', '/posts/42');
    Harness::expect(get('/users/ada/posts/hello-world')->body, 'ada: hello-world', 'two parameters');
    $seen = null;
    $router = (new Router())->get('/tags/{tag}', function (Request $r) use (&$seen) {
        $seen = $r->attribute('params');
        return Response::text('ok');
    });
    $router->handle(Request::create('GET', '/tags/php'));
    Harness::expect($seen, ['tag' => 'php'], 'the params attribute');
});

$t->scenario('c09_a_pattern_constrains_a_parameter', function () {
    Harness::expect(get('/posts/abc')->status, 404, '/posts/abc does not match {id:\d+}');
    Harness::expect(get('/posts/42/edit')->status, 404, 'a route matches the whole path');
    Harness::expect(get('/users/ada/posts/a/b')->status, 404, '{slug} matches one segment only');
});

$t->scenario('c10_no_matching_path_is_a_404', function () {
    $r = get('/nowhere');
    Harness::expect([$r->status, $r->body], [404, 'Not Found'], '404');
});

$t->scenario('c11_a_path_with_other_methods_is_a_405_with_allow', function () {
    $r = get('/posts', 'DELETE');
    Harness::expect([$r->status, $r->header('Allow')], [405, 'POST, GET'], '405 and Allow');
    Harness::expect(get('/posts/1', 'POST')->header('Allow'), 'GET', 'Allow for /posts/1');
});

$t->scenario('c12_handlers_may_return_a_string_or_an_array', function () {
    $router = (new Router())
        ->get('/text', fn() => 'plain')
        ->get('/data', fn() => ['ok' => true])
        ->get('/method', fn(Request $r) => $r->method->value);
    $text = $router->handle(Request::create('GET', '/text'));
    $data = $router->handle(Request::create('GET', '/data'));
    Harness::expect([$text->body, $text->header('Content-Type')], ['plain', 'text/plain; charset=utf-8'], 'a string');
    Harness::expect([$data->body, $data->header('Content-Type')], ['{"ok":true}', 'application/json'], 'an array');
    Harness::expect($router->handle(Request::create('GET', '/method'))->body, 'GET', 'the handler gets the request');
});

$t->finish();
