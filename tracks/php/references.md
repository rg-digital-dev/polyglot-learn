# Values and references in PHP

PHP arrays are *values*: `$b = $a` copies the array, and so does passing it to a function or returning it. (PHP only does the work when one side is changed, so the copy is cheap.) A function that changes its array parameter changes its own copy. Objects are different: `$b = $a` makes a second handle to the same object, and `clone $a` copies it (one level). `&` creates a real reference to a variable when you need one.

```php
<?php
$a = [[1, 2], [3]];
$b = $a;          // a copy, inner arrays included
$a[0][] = 9;
print_r($b[0]);   // [1, 2]

$o = new ArrayObject([1, 2]);
$p = $o;          // the same object
$p[] = 3;
echo count($o), "\n";   // 3

function addOne(array $xs): array { $xs[] = 1; return $xs; }
$c = [0];
addOne($c);
echo count($c), "\n";   // 1: the function changed its own copy
```

<!-- from:typescript -->
## Coming from TypeScript
- Arrays don't behave like JavaScript arrays here: assignment and arguments *copy* them.
- Objects do behave like JavaScript objects: they're shared, and `clone` is a shallow `{ ...o }`.
- `&$x` (a reference parameter or `foreach ($a as &$v)`) opts into sharing a variable.
<!-- /from -->

<!-- from:default -->
## Key points
- Arrays are copied by assignment, arguments and return values.
- Objects are shared; `clone` copies one level.
- `&` makes two names for one variable.
<!-- /from -->

## Common mistake
A `foreach` by reference leaves the reference behind:

```php
$xs = [1, 2, 3];
foreach ($xs as &$x) { $x *= 2; }
foreach ($xs as $x) { }   // writes into $xs[2] through the old reference
print_r($xs);             // [2, 4, 4]

foreach ($xs as &$x) { $x *= 2; }
unset($x);                // fixed: drop the reference after the loop
```

## In these exercises
Both are short in PHP, because arrays can't share: `array_fill` can repeat one row safely, and `addBonus` may change `$scores` and return it, since it's already a copy. Compare with the other tracks to see what PHP's value arrays save you from.
