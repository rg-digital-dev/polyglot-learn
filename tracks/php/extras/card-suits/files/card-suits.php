<?php

declare(strict_types=1);

/** A pure enum: cases with no value behind them. */
enum Color
{
    case Red;
    case Black;
}

/** A backed enum: each case has a string value, usable with from()/tryFrom(). */
enum Suit: string
{
    case Hearts = 'H';
    case Diamonds = 'D';
    case Clubs = 'C';
    case Spades = 'S';

    /** The suit that beats every other suit in Card::beats(). */
    public const TRUMP = self::Spades;

    /** Hearts and Diamonds are red, Clubs and Spades are black. */
    public function color(): Color
    {
        throw new RuntimeException('TODO: implement Suit::color');
    }

    /** One of "♥", "♦", "♣", "♠". */
    public function symbol(): string
    {
        throw new RuntimeException('TODO: implement Suit::symbol');
    }

    /** "h" or "H" gives Hearts, and so on. Unknown letters give null. */
    public static function fromLetter(string $letter): ?Suit
    {
        throw new RuntimeException('TODO: implement Suit::fromLetter');
    }
}

enum Rank: int
{
    case Two = 2;
    case Three = 3;
    case Four = 4;
    case Five = 5;
    case Six = 6;
    case Seven = 7;
    case Eight = 8;
    case Nine = 9;
    case Ten = 10;
    case Jack = 11;
    case Queen = 12;
    case King = 13;
    case Ace = 14;

    /** "2" to "10" for number cards, then "J", "Q", "K", "A". */
    public function label(): string
    {
        throw new RuntimeException('TODO: implement Rank::label');
    }
}

final class Card
{
    public function __construct(
        public readonly Rank $rank,
        public readonly Suit $suit,
    ) {
    }

    /** Rank label followed by the suit symbol, e.g. "10♥" or "Q♠". */
    public function label(): string
    {
        throw new RuntimeException('TODO: implement Card::label');
    }

    /**
     * Parse a rank label followed by a suit letter, e.g. "QH", "10s", "2c".
     *
     * @throws InvalidArgumentException for anything else
     */
    public static function parse(string $text): Card
    {
        throw new RuntimeException('TODO: implement Card::parse');
    }

    /**
     * A trump card beats any card of another suit. Otherwise a card beats
     * another of the same suit with a lower rank, and nothing else.
     */
    public function beats(Card $other): bool
    {
        throw new RuntimeException('TODO: implement Card::beats');
    }
}

/**
 * All 52 cards: suits in declaration order, each with ranks Two to Ace.
 *
 * @return list<Card>
 */
function fullDeck(): array
{
    throw new RuntimeException('TODO: implement fullDeck');
}
