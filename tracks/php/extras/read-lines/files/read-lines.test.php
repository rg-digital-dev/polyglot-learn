<?php
// Tests for the read-lines exercise. Don't edit this file: it is overwritten
// on every run. Your code goes in read-lines.php.

declare(strict_types=1);

require __DIR__ . '/../harness.php';
require_once __DIR__ . '/read-lines.php';

/** A generator over $items that counts in $produced how many it has handed out. */
function counting(array $items, int &$produced): Generator
{
    foreach ($items as $item) {
        $produced++;
        yield $item;
    }
}

$t = new Harness();
$t->scenario('c01_readlines_yields_numbered_lines', function () {
    $g = readLines("alpha\nbeta\ngamma");
    Harness::check($g instanceof Generator, 'readLines should return a Generator');
    Harness::expect(iterator_to_array($g), [1 => 'alpha', 2 => 'beta', 3 => 'gamma'], 'lines');
});
$t->scenario('c02_readlines_drops_line_endings_and_a_final_newline', function () {
    Harness::expect(iterator_to_array(readLines("a\r\nb\r\n")), [1 => 'a', 2 => 'b'], 'Windows endings');
    Harness::expect(iterator_to_array(readLines("a\n\nb\n")), [1 => 'a', 2 => '', 3 => 'b'], 'empty line in the middle');
    Harness::expect(iterator_to_array(readLines("\n")), [1 => ''], 'a single newline');
});
$t->scenario('c03_readlines_of_an_empty_string_yields_nothing', function () {
    Harness::expect(iterator_to_array(readLines('')), [], 'lines');
});
$t->scenario('c04_readlines_returns_the_line_count', function () {
    $g = readLines("one\ntwo\nthree\n");
    foreach ($g as $_) {
    }
    Harness::expect($g->getReturn(), 3, 'getReturn() after three lines');
    $g = readLines('');
    foreach ($g as $_) {
    }
    Harness::expect($g->getReturn(), 0, 'getReturn() for an empty string');
});
$t->scenario('c05_nonblank_skips_blank_lines_keeps_line_numbers', function () {
    $lines = nonBlank(readLines("title\n\n  \nbody\n\t\nend"));
    Harness::expect(iterator_to_array($lines), [1 => 'title', 4 => 'body', 6 => 'end'], 'lines');
});
$t->scenario('c06_nonblank_also_accepts_an_array', function () {
    Harness::expect(iterator_to_array(nonBlank([10 => 'x', 11 => ' ', 12 => 'y'])), [10 => 'x', 12 => 'y'], 'lines');
});
$t->scenario('c07_concatlines_yields_every_line_in_order', function () {
    $g = concatLines(["a\nb", '', "c\nd\ne\n"]);
    Harness::expect(iterator_to_array($g, false), ['a', 'b', 'c', 'd', 'e'], 'lines');
    $keys = [];
    foreach (concatLines(["a\nb", "c"]) as $k => $_) {
        $keys[] = $k;
    }
    Harness::expect($keys, [1, 2, 1], 'keys (line numbers within each text)');
});
$t->scenario('c08_concatlines_returns_the_total_line_count', function () {
    $g = concatLines(["a\nb", '', "c\nd\ne\n"]);
    foreach ($g as $_) {
    }
    Harness::expect($g->getReturn(), 5, 'getReturn()');
});
$t->scenario('c09_take_reads_an_infinite_generator_lazily', function () {
    Harness::expect(iterator_to_array(take(naturals(5), 4), false), [5, 6, 7, 8], 'take(naturals(5), 4)');
    Harness::expect(iterator_to_array(take(readLines("a\nb\nc"), 2)), [1 => 'a', 2 => 'b'], 'take keeps keys');
});
$t->scenario('c10_take_stops_without_pulling_extra_items', function () {
    $produced = 0;
    $out = iterator_to_array(take(counting(['a', 'b', 'c', 'd'], $produced), 2), false);
    Harness::expect($out, ['a', 'b'], 'take 2');
    Harness::expect($produced, 2, 'items pulled from the source for take 2');
    $produced = 0;
    Harness::expect(iterator_to_array(take(counting(['a'], $produced), 0)), [], 'take 0');
    Harness::expect($produced, 0, 'items pulled from the source for take 0');
    Harness::expect(iterator_to_array(take(['x', 'y'], 5)), ['x', 'y'], 'take more than there is');
});
$t->finish();
