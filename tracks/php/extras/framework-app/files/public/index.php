<?php
// The front controller: every request comes here. Try it:
//   php -S localhost:8000 public/index.php
// then open http://localhost:8000/hello/ada

declare(strict_types=1);

require_once __DIR__ . '/../App.php';
require_once __DIR__ . '/../Api.php';
require_once __DIR__ . '/../View.php';

$app = new App();
$app->add(new ErrorMiddleware(debug: true))
    ->add(TrailingSlashMiddleware::class)
    ->add(TimingMiddleware::class)
    ->add(ProblemMiddleware::class);

$app->get('/', fn() => 'Hello from your framework.');
$app->get('/hello/{name}', fn(Request $r, array $p) => Response::html('<h1>Hello, ' . e($p['name'] ?? '') . '</h1>'));
$app->get('/api/numbers', fn(Request $r) => paginate(range(1, 95), $r));

$uri = $_SERVER['REQUEST_URI'] ?? '/';
$headers = function_exists('getallheaders') ? getallheaders() : [];
emit($app->handle(Request::create($_SERVER['REQUEST_METHOD'] ?? 'GET', $uri, $headers, (string) file_get_contents('php://input'))));
