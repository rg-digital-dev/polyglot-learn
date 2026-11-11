<?php
// Stage 5 of the web framework project. Don't edit this file: `learn`
// overwrites it. Your code goes in View.php.

declare(strict_types=1);

require __DIR__ . '/../harness.php';
require_once __DIR__ . '/View.php';

/** A fresh template folder with these files. */
function templates(array $files): View
{
    $dir = sys_get_temp_dir() . '/framework-s5-' . getmypid() . '-' . bin2hex(random_bytes(4));
    mkdir($dir);
    foreach ($files as $name => $text) {
        file_put_contents("$dir/$name", $text);
    }
    return new View($dir);
}

$t = new Harness();

$t->scenario('c25_e_escapes_html', function () {
    Harness::expect(e('<script>alert("x")</script>'), '&lt;script&gt;alert(&quot;x&quot;)&lt;/script&gt;', 'tags and double quotes');
    Harness::expect(e("it's & co"), 'it&#039;s &amp; co', 'single quotes and ampersands');
    Harness::expect(e(42), '42', 'numbers');
    Harness::expect(e("bad \xC3\x28 byte"), "bad \u{FFFD}( byte", 'broken UTF-8 is replaced, not dropped');
});

$t->scenario('c26_render_fills_in_variables', function () {
    $v = templates(['hello.php' => '<h1>Hello, <?= e($name) ?></h1><?php foreach ($tags as $tag): ?><i><?= e($tag) ?></i><?php endforeach ?>']);
    Harness::expect($v->render('hello.php', ['name' => 'Ada', 'tags' => ['a', 'b']]), '<h1>Hello, Ada</h1><i>a</i><i>b</i>', 'output');
});

$t->scenario('c27_escaping_stops_injected_html', function () {
    $v = templates(['comment.php' => '<p><?= e($text) ?></p>']);
    Harness::expect($v->render('comment.php', ['text' => '<img src=x onerror="steal()">']), '<p>&lt;img src=x onerror=&quot;steal()&quot;&gt;</p>', 'output');
});

$t->scenario('c28_a_template_can_use_a_layout', function () {
    $v = templates([
        'page.php' => '<?php $view->layout("layout.php", ["title" => "Posts"]) ?><ul><li><?= e($first) ?></li></ul>',
        'layout.php' => '<title><?= e($title) ?></title><main><?= $content ?></main>',
    ]);
    Harness::expect($v->render('page.php', ['first' => 'A & B']), '<title>Posts</title><main><ul><li>A &amp; B</li></ul></main>', 'output');
    Harness::expect($v->render('layout.php', ['title' => 'T', 'content' => 'c']), '<title>T</title><main>c</main>', 'the layout alone');
});

$t->scenario('c29_render_returns_output_instead_of_printing_it', function () {
    $v = templates(['x.php' => 'printed?']);
    ob_start();
    $got = $v->render('x.php');
    $leaked = ob_get_clean();
    Harness::expect([$got, $leaked], ['printed?', ''], 'returned, and nothing echoed');
});

$t->scenario('c30_a_failing_template_leaves_no_buffers_behind', function () {
    $v = templates(['bad.php' => 'half <?php throw new RuntimeException("template broke");']);
    $level = ob_get_level();
    try {
        $v->render('bad.php');
        Harness::check(false, 'the exception should come out of render');
    } catch (RuntimeException $e) {
        Harness::expect($e->getMessage(), 'template broke', 'the exception');
    }
    Harness::expect(ob_get_level(), $level, 'output buffering level');
    try {
        $v->render('missing.php');
        Harness::check(false, 'a missing template should throw');
    } catch (RuntimeException $e) {
        Harness::expect($e->getMessage(), 'no template missing.php', 'message');
    }
});

$t->finish();
