# Lists, Maps and Tuples in Python

Python has all three shapes built in. A list is `[1, 2, 3]`: ordered, changeable, counted from 0, and negative indexes count from the end (`xs[-1]` is the last item). A dict is `{"alice": 3}`: reading a missing key with `m["bob"]` raises `KeyError`, while `m.get("bob")` gives `None`, or a default you pass as the second argument. A tuple is `(1, 2)`: fixed, and a one-item tuple needs a trailing comma, `(1,)`. Slices such as `xs[1:3]` give a new list. `for x in xs:` walks a list, and `for k, v in m.items():` walks a dict.

```python
xs = [10, 20, 30]
xs.append(40)
print(xs[0], xs[-1], len(xs))       # --> 10 40 4
print(xs[1:3])                      # --> [20, 30]

m = {"alice": 3}
m["bob"] = 5
print(m.get("zed"), m.get("zed", 0))  # --> None 0
for name, score in m.items():
    print(name, score)              # --> alice 3   then bob 5

pair = ("alice", 3)
who, score = pair
print(who, score, type((1,)).__name__)  # --> alice 3 tuple
```

<!-- from:typescript -->
## Coming from TypeScript
- `m["bob"]` on a missing key raises `KeyError`; it does not give `undefined`. Use `m.get("bob")`.
- `len(xs)` replaces `xs.length`, and `xs.append(x)` replaces `push`.
- `xs[-1]` is the last item.
<!-- /from -->

<!-- from:lua -->
## Coming from Lua
- Lists count from 0, not 1. `xs[len(xs) - 1]` or `xs[-1]` is the last item.
- Lists and dicts are separate types. A dict does not have a list part.
- `xs[5]` past the end raises `IndexError`, not `nil`.
<!-- /from -->

<!-- from:default -->
## Key points
- List `[1, 2]`, dict `{"k": v}`, tuple `(1, 2)`. Lists and dicts can be changed; tuples cannot.
- Index from 0; `xs[-1]` is the last item; `len(xs)` is the length.
- `m[k]` raises `KeyError` when missing; `m.get(k)` gives `None`.
- `(1,)` is a one-item tuple; `(1)` is just `1`.
<!-- /from -->

## Common mistake
Forgetting that assigning a list copies nothing. `ys = xs` makes two names for one list, so `ys.append(1)` changes `xs` too. Use `xs.copy()` or `list(xs)` for a separate list.
