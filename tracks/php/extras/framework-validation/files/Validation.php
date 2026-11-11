<?php
// Checking form input against rules written as attributes on a class.

declare(strict_types=1);

interface Rule
{
    /** null if the value is fine, or what's wrong with it ("is required"). */
    public function check(mixed $value): ?string;
}

// TODO: Required, Length and Email are used as attributes, #[Required], on
// constructor parameters (and so on promoted properties). Declare each one as
// an attribute class that can target both.

/** "is required" for null or "". */
final class Required implements Rule
{
    public function check(mixed $value): ?string
    {
        throw new RuntimeException('TODO: implement Required::check');
    }
}

/**
 * "must be at least <min> characters" or "must be at most <max> characters",
 * counting characters, not bytes ("é" is one). Without the mbstring
 * extension, preg_match_all('/./su', $text) counts them.
 */
final class Length implements Rule
{
    public function __construct(public readonly int $min = 0, public readonly ?int $max = null)
    {
    }

    public function check(mixed $value): ?string
    {
        throw new RuntimeException('TODO: implement Length::check');
    }
}

/** "must be an email address" unless filter_var accepts it as one. */
final class Email implements Rule
{
    public function check(mixed $value): ?string
    {
        throw new RuntimeException('TODO: implement Email::check');
    }
}

final class ValidationResult
{
    /** @param array<string, list<string>> $errors per field, in the class's order */
    public function __construct(public readonly ?object $value, public readonly array $errors)
    {
    }

    public function ok(): bool
    {
        return $this->errors === [];
    }
}

final class Validator
{
    /**
     * Check $input against the Rule attributes on $class's constructor
     * parameters and, if all pass, build the object with them.
     *
     * - Strings are trimmed first. Input keys the class doesn't have are ignored.
     * - A missing value (absent, null or "") only gets the Required check: a
     *   missing optional field uses its default.
     * - An int parameter takes a numeric string with no fraction, else
     *   "must be a whole number".
     * - Errors are keyed by field, each a list of messages.
     */
    public function validate(string $class, array $input): ValidationResult
    {
        throw new RuntimeException('TODO: implement Validator::validate');
    }
}
