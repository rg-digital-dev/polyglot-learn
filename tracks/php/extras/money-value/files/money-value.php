<?php

declare(strict_types=1);

/**
 * An amount of money in whole cents (no floats) plus an ISO currency code.
 * Once built, a Money never changes: every operation returns a new one.
 */
final class Money
{
    /**
     * @throws InvalidArgumentException unless $currency is three uppercase letters A-Z
     */
    public function __construct(
        public readonly int $cents,
        public readonly string $currency,
    ) {
        throw new RuntimeException('TODO: implement Money::__construct');
    }

    /** A copy with a different amount and the same currency. */
    public function withCents(int $cents): Money
    {
        throw new RuntimeException('TODO: implement Money::withCents');
    }

    /** @throws InvalidArgumentException when the currencies differ */
    public function add(Money $other): Money
    {
        throw new RuntimeException('TODO: implement Money::add');
    }

    /** Same amount and same currency. */
    public function equals(Money $other): bool
    {
        throw new RuntimeException('TODO: implement Money::equals');
    }

    /** "EUR 12.34", "EUR 0.05", "EUR -0.05": currency, space, two decimals. */
    public function format(): string
    {
        throw new RuntimeException('TODO: implement Money::format');
    }

    /**
     * Split into $parts amounts that add up exactly to this one. Spread the
     * leftover cents one each over the first parts: 100 in 3 is 34, 33, 33.
     *
     * @return list<Money>
     * @throws InvalidArgumentException when $parts is less than 1
     */
    public function allocate(int $parts): array
    {
        throw new RuntimeException('TODO: implement Money::allocate');
    }
}
