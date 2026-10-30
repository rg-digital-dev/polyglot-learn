<?php
// Tests for the timestamps-trait exercise. Don't edit this file: it is
// overwritten on every run. Your code goes in timestamps-trait.php.

declare(strict_types=1);

require __DIR__ . '/../harness.php';
require_once __DIR__ . '/timestamps-trait.php';

/** A fake clock: set $time, and the closure returns it. */
final class FakeClock
{
    public int $time = 100;

    public function closure(): Closure
    {
        return fn(): int => $this->time;
    }
}

$t = new Harness();
$t->scenario('c01_a_new_note_has_no_timestamps', function () {
    $note = new Note('Groceries', (new FakeClock())->closure());
    Harness::expect($note->createdAt(), null, 'createdAt');
    Harness::expect($note->updatedAt(), null, 'updatedAt');
});
$t->scenario('c02_touch_sets_both_timestamps_the_first_time', function () {
    $note = new Note('Groceries', (new FakeClock())->closure());
    $note->touch();
    Harness::expect($note->createdAt(), 100, 'createdAt');
    Harness::expect($note->updatedAt(), 100, 'updatedAt');
});
$t->scenario('c03_touch_again_moves_only_updatedat', function () {
    $clock = new FakeClock();
    $note = new Note('Groceries', $clock->closure());
    $note->touch();
    $clock->time = 250;
    $note->touch();
    Harness::expect($note->createdAt(), 100, 'createdAt');
    Harness::expect($note->updatedAt(), 250, 'updatedAt');
});
$t->scenario('c04_tag_keeps_the_first_occurrence_of_each_tag', function () {
    $b = new Bookmark('https://php.net');
    $b->tag('docs', 'php');
    $b->tag('php', 'reference', 'docs');
    Harness::expect($b->tags(), ['docs', 'php', 'reference'], 'tags');
});
$t->scenario('c05_tag_returns_the_same_object_for_chaining', function () {
    $b = new Bookmark('https://php.net');
    $same = $b->tag('a')->tag('b');
    Harness::check($same === $b, 'tag() should return $this');
    Harness::expect($b->tags(), ['a', 'b'], 'tags');
});
$t->scenario('c06_each_object_has_its_own_tags', function () {
    $a = new Bookmark('https://a.example');
    $b = new Bookmark('https://b.example');
    $a->tag('first');
    Harness::expect($a->tags(), ['first'], 'tags of the first bookmark');
    Harness::expect($b->tags(), [], 'tags of the second bookmark');
});
$t->scenario('c07_note_uses_both_traits', function () {
    $used = array_keys(class_uses(Note::class));
    sort($used);
    Harness::expect($used, ['HasTags', 'HasTimestamps'], 'traits used by Note');
    $note = new Note('Groceries', (new FakeClock())->closure());
    Harness::expect($note->tag('home')->tags(), ['home'], 'tags of a note');
});
$t->scenario('c08_describe_comes_from_hastimestamps', function () {
    $clock = new FakeClock();
    $note = new Note('Groceries', $clock->closure());
    $note->tag('home');
    Harness::expect($note->describe(), 'never saved', 'before touch');
    $note->touch();
    $clock->time = 250;
    $note->touch();
    Harness::expect($note->describe(), 'created 100, updated 250', 'after two touches');
});
$t->scenario('c09_the_tags_describe_is_available_as_describetags', function () {
    $note = new Note('Groceries', (new FakeClock())->closure());
    Harness::expect($note->describeTags(), 'no tags', 'without tags');
    $note->tag('home', 'todo');
    Harness::expect($note->describeTags(), 'tags: home, todo', 'with two tags');
    Harness::expect((new Bookmark('https://php.net'))->tag('docs')->describe(), 'tags: docs', 'Bookmark::describe');
});
$t->scenario('c10_summary_combines_title_tags_and_timestamps', function () {
    $clock = new FakeClock();
    $note = new Note('Groceries', $clock->closure());
    Harness::expect($note->summary(), 'Groceries (no tags; never saved)', 'new note');
    $note->tag('home', 'todo')->touch();
    $clock->time = 250;
    $note->touch();
    Harness::expect($note->summary(), 'Groceries (tags: home, todo; created 100, updated 250)', 'saved note');
});
$t->finish();
