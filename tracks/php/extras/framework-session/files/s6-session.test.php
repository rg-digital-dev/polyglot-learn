<?php
// Stage 6 of the web framework project. Don't edit this file: `learn`
// overwrites it. Your code goes in Session.php.

declare(strict_types=1);

require __DIR__ . '/../harness.php';
require_once __DIR__ . '/Session.php';

function protectedApp(Csrf $csrf): Pipeline
{
    $router = (new Router())
        ->get('/form', fn() => 'form')
        ->post('/comments', fn() => Response::text('saved', 201));
    return new Pipeline($router, new CsrfMiddleware($csrf));
}

$t = new Harness();

$t->scenario('c31_a_session_stores_values', function () {
    $s = new ArraySession();
    Harness::expect([$s->get('user'), $s->get('user', 'guest')], [null, 'guest'], 'missing keys');
    $s->set('user', 'ada');
    $s->set('cart', [1, 2]);
    $s->set('nothing', null);
    Harness::expect([$s->get('user'), $s->get('cart'), $s->get('nothing', 'default')], ['ada', [1, 2], null], 'stored values, null included');
    $s->remove('user');
    Harness::expect($s->get('user'), null, 'removed');
    Harness::check($s instanceof Session, 'ArraySession implements Session');
});

$t->scenario('c32_a_flash_message_is_read_once', function () {
    $s = new ArraySession();
    $s->flash('notice', 'Saved!');
    Harness::expect([$s->takeFlash('notice'), $s->takeFlash('notice'), $s->takeFlash('notice', 'gone')], ['Saved!', null, 'gone'], 'read once');
});

$t->scenario('c33_a_csrf_token_is_random_and_kept', function () {
    $a = new Csrf(new ArraySession());
    $token = $a->token();
    Harness::check(preg_match('/^[0-9a-f]{64}$/', $token) === 1, "64 hex characters, got $token");
    Harness::expect($a->token(), $token, 'the same token on the next call');
    Harness::check((new Csrf(new ArraySession()))->token() !== $token, 'another session gets another token');
});

$t->scenario('c34_only_the_session_token_is_valid', function () {
    $csrf = new Csrf(new ArraySession());
    Harness::check(!$csrf->valid('anything'), 'no token made yet: nothing is valid');
    $token = $csrf->token();
    Harness::expect([$csrf->valid($token), $csrf->valid(strtoupper($token)), $csrf->valid(''), $csrf->valid(null)], [true, false, false, false], 'valid()');
});

$t->scenario('c35_unsafe_requests_need_the_token', function () {
    $csrf = new Csrf(new ArraySession());
    $app = protectedApp($csrf);
    $token = $csrf->token();
    Harness::expect($app->handle(Request::create('GET', '/form'))->body, 'form', 'GET needs no token');
    $none = $app->handle(Request::create('POST', '/comments', [], 'text=hi'));
    Harness::expect([$none->status, $none->body], [419, 'CSRF token mismatch'], 'POST without a token');
    Harness::expect($app->handle(Request::create('POST', '/comments', [], "text=hi&_token=wrong"))->status, 419, 'a wrong token');
    Harness::expect($app->handle(Request::create('POST', '/comments', [], "text=hi&_token=$token"))->status, 201, 'a form field');
    Harness::expect($app->handle(Request::create('POST', '/comments', ['X-CSRF-Token' => $token]))->status, 201, 'a header');
});

$t->scenario('c36_valid_compares_in_constant_time', function () {
    $source = (string) file_get_contents(__DIR__ . '/Session.php');
    Harness::check(str_contains($source, 'hash_equals('), 'compare tokens with hash_equals(), not === or ==');
    Harness::check(str_contains($source, 'random_bytes('), 'make tokens with random_bytes()');
});

$t->finish();
