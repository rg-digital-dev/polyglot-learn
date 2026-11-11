<?php
// Stage 4 of the web framework project. Don't edit this file: `learn`
// overwrites it. Your code goes in Validation.php.

declare(strict_types=1);

require __DIR__ . '/../harness.php';
require_once __DIR__ . '/Validation.php';

final class SignupForm
{
    public function __construct(
        #[Required, Length(min: 3, max: 20)]
        public readonly string $name,
        #[Required, Email]
        public readonly string $email,
        #[Length(max: 10)]
        public readonly string $bio = 'none yet',
        public readonly int $age = 0,
    ) {
    }
}

function check(array $input): ValidationResult
{
    return (new Validator())->validate(SignupForm::class, $input);
}

$t = new Harness();

$t->scenario('c19_valid_input_builds_the_object', function () {
    $r = check(['name' => ' ada ', 'email' => 'ada@example.com', 'bio' => 'maths', 'age' => '36', 'extra' => 'ignored']);
    Harness::check($r->ok(), 'expected no errors, got ' . Harness::show($r->errors));
    Harness::check($r->value instanceof SignupForm, 'value should be a SignupForm');
    Harness::expect([$r->value->name, $r->value->email, $r->value->bio, $r->value->age], ['ada', 'ada@example.com', 'maths', 36], 'fields, trimmed and converted');
});

$t->scenario('c20_missing_required_fields_only_say_so', function () {
    $r = check(['name' => '   ']);
    Harness::check(!$r->ok() && $r->value === null, 'invalid input gives no value');
    Harness::expect($r->errors, ['name' => ['is required'], 'email' => ['is required']], 'errors');
});

$t->scenario('c21_each_rule_reports_its_own_message', function () {
    $r = check(['name' => 'al', 'email' => 'not-an-email', 'bio' => 'far too long a bio']);
    Harness::expect($r->errors, [
        'name' => ['must be at least 3 characters'],
        'email' => ['must be an email address'],
        'bio' => ['must be at most 10 characters'],
    ], 'errors');
    Harness::expect(check(['name' => str_repeat('é', 20), 'email' => 'a@b.co'])->errors, [], 'length counts characters, not bytes');
});

$t->scenario('c22_optional_fields_fall_back_to_their_defaults', function () {
    $r = check(['name' => 'grace', 'email' => 'g@example.com', 'bio' => '']);
    Harness::expect([$r->value?->bio, $r->value?->age], ['none yet', 0], 'defaults');
});

$t->scenario('c23_an_int_field_needs_a_whole_number', function () {
    Harness::expect(check(['name' => 'grace', 'email' => 'g@example.com', 'age' => 'old'])->errors, ['age' => ['must be a whole number']], 'age');
    Harness::expect(check(['name' => 'grace', 'email' => 'g@example.com', 'age' => '4.5'])->errors, ['age' => ['must be a whole number']], '4.5');
});

$t->scenario('c24_rules_are_attributes_that_implement_rule', function () {
    $attrs = (new ReflectionParameter([SignupForm::class, '__construct'], 'name'))->getAttributes();
    Harness::expect(array_map(fn($a) => $a->getName(), $attrs), ['Required', 'Length'], 'attributes on $name');
    $length = $attrs[1]->newInstance();
    Harness::check($length instanceof Rule, 'Length implements Rule');
    Harness::expect([$length->check('ab'), $length->check('abc')], ['must be at least 3 characters', null], 'Length::check');
    Harness::expect([(new Required())->check(''), (new Email())->check('x@y.org')], ['is required', null], 'Required and Email');
});

$t->finish();
