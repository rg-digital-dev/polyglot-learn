<?php
// Tests for the money-value exercise. Don't edit this file: it is overwritten
// on every run. Your code goes in money-value.php.

declare(strict_types=1);

require __DIR__ . '/../harness.php';
require_once __DIR__ . '/money-value.php';

/** Fails unless $fn throws InvalidArgumentException (the stub's RuntimeException doesn't count). */
function expectInvalid(callable $fn, string $what): void
{
    try {
        $fn();
    } catch (InvalidArgumentException $e) {
        return;
    }
    throw new UnexpectedValueException("$what: expected InvalidArgumentException, got no exception");
}

/** @param list<Money> $list */
function centsOf(array $list): array
{
    return array_map(fn(Money $m) => $m->cents, $list);
}

$t = new Harness();
$t->scenario('c01_constructor_keeps_cents_and_currency', function () {
    $m = new Money(1250, 'EUR');
    Harness::expect($m->cents, 1250, 'cents');
    Harness::expect($m->currency, 'EUR', 'currency');
    Harness::expect((new Money(-5, 'USD'))->cents, -5, 'negative cents');
});
$t->scenario('c02_constructor_rejects_a_bad_currency_code', function () {
    expectInvalid(fn() => new Money(100, 'eur'), "new Money(100, 'eur')");
    expectInvalid(fn() => new Money(100, 'EU'), "new Money(100, 'EU')");
    expectInvalid(fn() => new Money(100, 'EURO'), "new Money(100, 'EURO')");
    expectInvalid(fn() => new Money(100, ''), "new Money(100, '')");
});
$t->scenario('c03_properties_cannot_be_changed_after_construction', function () {
    $m = new Money(100, 'EUR');
    try {
        $m->cents = 999;
    } catch (Error $e) {
        Harness::check(str_contains($e->getMessage(), 'readonly'), 'expected a readonly error, got: ' . $e->getMessage());
        Harness::expect($m->cents, 100, 'cents after the failed write');
        return;
    }
    throw new UnexpectedValueException('assigning $money->cents should throw an Error');
});
$t->scenario('c04_withcents_returns_a_changed_copy', function () {
    $a = new Money(100, 'GBP');
    $b = $a->withCents(250);
    Harness::check($a !== $b, 'withCents should return a new object');
    Harness::expect($a->cents, 100, 'original cents');
    Harness::expect($b->cents, 250, 'copy cents');
    Harness::expect($b->currency, 'GBP', 'copy currency');
});
$t->scenario('c05_equals_compares_by_value', function () {
    $a = new Money(500, 'EUR');
    Harness::check($a->equals(new Money(500, 'EUR')), '500 EUR should equal another 500 EUR');
    Harness::check(!$a->equals(new Money(501, 'EUR')), '500 EUR should not equal 501 EUR');
    Harness::check(!$a->equals(new Money(500, 'USD')), '500 EUR should not equal 500 USD');
});
$t->scenario('c06_add_sums_amounts_of_the_same_currency', function () {
    $a = new Money(1999, 'EUR');
    $sum = $a->add(new Money(1, 'EUR'));
    Harness::expect($sum, new Money(2000, 'EUR'), 'sum');
    Harness::expect($a->cents, 1999, 'left operand after add');
});
$t->scenario('c07_add_rejects_different_currencies', function () {
    expectInvalid(fn() => (new Money(100, 'EUR'))->add(new Money(100, 'USD')), 'EUR + USD');
});
$t->scenario('c08_format_shows_two_decimals', function () {
    Harness::expect((new Money(1234, 'EUR'))->format(), 'EUR 12.34', '1234');
    Harness::expect((new Money(5, 'EUR'))->format(), 'EUR 0.05', '5');
    Harness::expect((new Money(-5, 'EUR'))->format(), 'EUR -0.05', '-5');
    Harness::expect((new Money(0, 'USD'))->format(), 'USD 0.00', '0');
    Harness::expect((new Money(-120000, 'JPY'))->format(), 'JPY -1200.00', '-120000');
});
$t->scenario('c09_allocate_splits_without_losing_a_cent', function () {
    $parts = (new Money(100, 'EUR'))->allocate(3);
    Harness::expect(centsOf($parts), [34, 33, 33], '100 in 3');
    Harness::expect($parts[2]->currency, 'EUR', 'currency of a part');
    Harness::expect(centsOf((new Money(10, 'EUR'))->allocate(4)), [3, 3, 2, 2], '10 in 4');
    Harness::expect(centsOf((new Money(2, 'EUR'))->allocate(5)), [1, 1, 0, 0, 0], '2 in 5');
    Harness::expect(centsOf((new Money(7, 'EUR'))->allocate(1)), [7], '7 in 1');
});
$t->scenario('c10_allocate_rejects_fewer_than_one_part', function () {
    expectInvalid(fn() => (new Money(100, 'EUR'))->allocate(0), 'allocate(0)');
    expectInvalid(fn() => (new Money(100, 'EUR'))->allocate(-2), 'allocate(-2)');
});
$t->finish();
