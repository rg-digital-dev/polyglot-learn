# Lists, Maps and Tuples in PHP

PHP has one structure, the **array**, and it is both a list and a map. `[1, 2, 3]` is a list with keys `0, 1, 2`. `["alice" => 3]` is a map. Add to the end with `$xs[] = 4;`, count with `count($xs)`, and read with `$xs[0]` or `$m["alice"]`. Arrays are copied when assigned, so changing a copy leaves the original alone. There is no tuple: use a small array and unpack it with `[$name, $score] = $pair;`. Reading a missing key gives `null` and a warning, so use `??` or `isset` when a key may be absent.

```php
<?php
declare(strict_types=1);

$xs = [1, 2, 3];
$xs[] = 4;
echo $xs[0], count($xs), "\n";                // --> 14

$m = ["alice" => 3, "bob" => 5];
$m["carol"] = 7;
var_dump($m["dave"] ?? 0);                    // --> int(0)
var_dump(array_key_exists("bob", $m));        // --> bool(true)

$copy = $xs;
$copy[0] = 99;
echo $xs[0], "\n";                            // --> 1

[$name, $score] = ["alice", 3];
echo "$name $score\n";                        // --> alice 3
foreach ($m as $k => $v) { echo "$k=$v,"; }   // --> alice=3,bob=5,carol=7,
echo "\n";
```

<!-- from:python -->
## Coming from Python
- A PHP array is a `list` and a `dict` in one. Both use `[]`: `["a" => 1]` is the map form.
- Missing key: a warning and `null`, not a `KeyError`. `$m["k"] ?? 0` works like `m.get("k", 0)`.
- Arrays are copied on assignment, unlike Python lists, which are shared.
- `count($xs)` is `len(xs)`, and `$xs[] = 4;` is `append`.
<!-- /from -->

<!-- from:typescript -->
## Coming from TypeScript
- Lists and maps are the same value, and a map is not an object.
- Assigning copies the array, so `$b = $a; $b[] = 1;` leaves `$a` alone.
- `count($xs)` replaces `.length`, and `$xs[] = 4;` replaces `push`.
<!-- /from -->

<!-- from:lua -->
## Coming from Lua
- Arrays count from `0`, not `1`.
- Like a Lua table, one array is both list and map. Assigning copies it, though, where a Lua table is shared.
- A missing key is `null` plus a warning, not a silent `nil`.
<!-- /from -->

<!-- from:default -->
## Key points
- One structure, the array, serves as list and map.
- Positions start at `0`. `$xs[] = v;` appends.
- A missing key warns and gives `null`. Use `??` to supply a default.
- Assigning an array copies it.
<!-- /from -->

## Common mistake
Removing an item with `unset($xs[1]);` and then expecting the list to close the gap. The keys stay `0, 2`, so `$xs[1]` is gone and `array_is_list($xs)` is false.
