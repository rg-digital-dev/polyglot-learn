<?php

declare(strict_types=1);

/** Remembers when an object was first saved and last changed. */
trait HasTimestamps
{
    private ?int $createdAt = null;
    private ?int $updatedAt = null;

    /** The class that uses this trait says what time it is. */
    abstract protected function now(): int;

    /** Record a save at now(): sets createdAt the first time, updatedAt every time. */
    public function touch(): void
    {
        throw new RuntimeException('TODO: implement HasTimestamps::touch');
    }

    public function createdAt(): ?int
    {
        throw new RuntimeException('TODO: implement HasTimestamps::createdAt');
    }

    public function updatedAt(): ?int
    {
        throw new RuntimeException('TODO: implement HasTimestamps::updatedAt');
    }

    /** "never saved", or "created 100, updated 250". */
    public function describe(): string
    {
        throw new RuntimeException('TODO: implement HasTimestamps::describe');
    }
}

/** A list of tags without duplicates, in the order they were first added. */
trait HasTags
{
    /** @var list<string> */
    private array $tags = [];

    /** Add the tags that aren't there yet, and return $this for chaining. */
    public function tag(string ...$tags): static
    {
        throw new RuntimeException('TODO: implement HasTags::tag');
    }

    /** @return list<string> */
    public function tags(): array
    {
        throw new RuntimeException('TODO: implement HasTags::tags');
    }

    /** "no tags", or "tags: home, todo". */
    public function describe(): string
    {
        throw new RuntimeException('TODO: implement HasTags::describe');
    }
}

/** Uses one trait. Nothing to change here. */
final class Bookmark
{
    use HasTags;

    public function __construct(public readonly string $url)
    {
    }
}

final class Note
{
    // TODO: use both traits. describe() must come from HasTimestamps, and the
    // HasTags version must be available as describeTags(). Then implement the
    // trait's abstract now() by calling $this->clock.

    /** @param Closure(): int $clock returns the current time */
    public function __construct(
        public readonly string $title,
        private Closure $clock,
    ) {
    }

    /** "Groceries (tags: home, todo; created 100, updated 250)" */
    public function summary(): string
    {
        throw new RuntimeException('TODO: implement Note::summary');
    }
}
