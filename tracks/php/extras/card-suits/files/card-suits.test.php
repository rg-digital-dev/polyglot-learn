<?php
// Tests for the card-suits exercise. Don't edit this file: it is overwritten
// on every run. Your code goes in card-suits.php.

declare(strict_types=1);

require __DIR__ . '/../harness.php';
require_once __DIR__ . '/card-suits.php';

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

$t = new Harness();
$t->scenario('c01_suit_colour_comes_from_a_match', function () {
    Harness::check(Suit::Hearts->color() === Color::Red, 'Hearts should be Red');
    Harness::check(Suit::Diamonds->color() === Color::Red, 'Diamonds should be Red');
    Harness::check(Suit::Clubs->color() === Color::Black, 'Clubs should be Black');
    Harness::check(Suit::Spades->color() === Color::Black, 'Spades should be Black');
});
$t->equals('c02_suit_symbols', fn() => array_map(fn(Suit $s) => $s->symbol(), Suit::cases()), ['♥', '♦', '♣', '♠']);
$t->scenario('c03_fromletter_accepts_either_case_or_returns_null', function () {
    Harness::check(Suit::fromLetter('h') === Suit::Hearts, "fromLetter('h') should be Hearts");
    Harness::check(Suit::fromLetter('S') === Suit::Spades, "fromLetter('S') should be Spades");
    Harness::check(Suit::fromLetter('d') === Suit::Diamonds, "fromLetter('d') should be Diamonds");
    Harness::expect(Suit::fromLetter('x'), null, "fromLetter('x')");
    Harness::expect(Suit::fromLetter(''), null, "fromLetter('')");
});
$t->equals('c04_rank_labels', fn() => array_map(fn(Rank $r) => $r->label(), Rank::cases()), ['2', '3', '4', '5', '6', '7', '8', '9', '10', 'J', 'Q', 'K', 'A']);
$t->scenario('c05_card_label_joins_rank_and_symbol', function () {
    Harness::expect((new Card(Rank::Ten, Suit::Hearts))->label(), '10♥', 'ten of hearts');
    Harness::expect((new Card(Rank::Queen, Suit::Spades))->label(), 'Q♠', 'queen of spades');
    Harness::expect((new Card(Rank::Two, Suit::Clubs))->label(), '2♣', 'two of clubs');
});
$t->scenario('c06_parse_reads_rank_and_suit_letters', function () {
    $c = Card::parse('QH');
    Harness::check($c->rank === Rank::Queen && $c->suit === Suit::Hearts, "parse('QH') should be the queen of hearts, got {$c->label()}");
    $c = Card::parse('10s');
    Harness::check($c->rank === Rank::Ten && $c->suit === Suit::Spades, "parse('10s') should be the ten of spades, got {$c->label()}");
    $c = Card::parse('ad');
    Harness::check($c->rank === Rank::Ace && $c->suit === Suit::Diamonds, "parse('ad') should be the ace of diamonds, got {$c->label()}");
});
$t->scenario('c07_parse_rejects_bad_input', function () {
    expectInvalid(fn() => Card::parse(''), "parse('')");
    expectInvalid(fn() => Card::parse('Q'), "parse('Q')");
    expectInvalid(fn() => Card::parse('QX'), "parse('QX')");
    expectInvalid(fn() => Card::parse('1H'), "parse('1H')");
    expectInvalid(fn() => Card::parse('11H'), "parse('11H')");
});
$t->scenario('c08_higher_rank_of_the_same_suit_wins', function () {
    $kingH = new Card(Rank::King, Suit::Hearts);
    $fiveH = new Card(Rank::Five, Suit::Hearts);
    $aceC = new Card(Rank::Ace, Suit::Clubs);
    Harness::check($kingH->beats($fiveH), 'K♥ should beat 5♥');
    Harness::check(!$fiveH->beats($kingH), '5♥ should not beat K♥');
    Harness::check(!$kingH->beats($kingH), 'K♥ should not beat itself');
    Harness::check(!$aceC->beats($fiveH), 'A♣ should not beat 5♥ (different suit, not trump)');
});
$t->scenario('c09_trump_beats_any_other_suit', function () {
    $twoS = new Card(Rank::Two, Suit::Spades);
    $aceH = new Card(Rank::Ace, Suit::Hearts);
    Harness::check($twoS->beats($aceH), '2♠ should beat A♥');
    Harness::check(!$aceH->beats($twoS), 'A♥ should not beat 2♠');
});
$t->scenario('c10_full_deck_has_52_cards_in_order', function () {
    $deck = fullDeck();
    Harness::expect(count($deck), 52, 'number of cards');
    Harness::expect($deck[0]->label(), '2♥', 'first card');
    Harness::expect($deck[12]->label(), 'A♥', 'thirteenth card');
    Harness::expect($deck[13]->label(), '2♦', 'fourteenth card');
    Harness::expect($deck[51]->label(), 'A♠', 'last card');
    $labels = array_map(fn(Card $c) => $c->label(), $deck);
    Harness::expect(count(array_unique($labels)), 52, 'distinct cards');
});
$t->finish();
