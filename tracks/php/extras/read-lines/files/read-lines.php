<?php

declare(strict_types=1);

/**
 * Yield the lines of $text keyed by line number, starting at 1. Lines end
 * with "\n" or "\r\n", and the ending is not part of the line. A final line
 * ending does not start an extra empty line. When done, return how many
 * lines were yielded.
 *
 * @return Generator<int, string, mixed, int>
 */
function readLines(string $text): Generator
{
    throw new RuntimeException('TODO: implement readLines');
}

/**
 * Yield only the lines that contain something other than whitespace, with
 * their original keys.
 *
 * @param iterable<int, string> $lines
 * @return Generator<int, string>
 */
function nonBlank(iterable $lines): Generator
{
    throw new RuntimeException('TODO: implement nonBlank');
}

/**
 * Yield the lines of every text in turn (each keyed by its line number within
 * its own text, so keys repeat) and return the total number of lines.
 *
 * @param list<string> $texts
 * @return Generator<int, string, mixed, int>
 */
function concatLines(array $texts): Generator
{
    throw new RuntimeException('TODO: implement concatLines');
}

/**
 * Yield $start, $start + 1, $start + 2, ... forever.
 *
 * @return Generator<int, int>
 */
function naturals(int $start): Generator
{
    throw new RuntimeException('TODO: implement naturals');
}

/**
 * Yield the first $n items of $items with their keys, then stop without
 * asking $items for another one.
 *
 * @template K
 * @template V
 * @param iterable<K, V> $items
 * @return Generator<K, V>
 */
function take(iterable $items, int $n): Generator
{
    throw new RuntimeException('TODO: implement take');
}
