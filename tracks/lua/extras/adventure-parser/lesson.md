# Parsing commands with patterns

Lua has no regular expressions in its standard library. It has *patterns*, a smaller language that fits in a few hundred lines of C. They cover most of what a command parser needs: character classes, repetition, anchors and captures.

| Pattern | Means |
|---|---|
| `%a` `%d` `%w` `%s` `%p` | a letter, digit, letter or digit, space, punctuation |
| `%S` `%A` … | the opposite of each class (upper case) |
| `[...]` `[^...]` | a set, or anything but the set; `%-` is a literal `-` inside it |
| `+` `*` `-` `?` | one or more, zero or more (longest), zero or more (shortest), optional |
| `^` `$` | the start and the end of the string, when first or last |
| `( )` | a capture |

```lua
local line = "  Give the OLD map to Captain Flint!  "

-- string.lower, then replace punctuation with spaces; gsub returns the new
-- string and a count, so keep only the first value.
local clean = line:lower():gsub("%p", " ")

-- gmatch iterates over every match: here, each run of non-space characters.
local words = {}
for word in clean:gmatch("%S+") do
  words[#words + 1] = word
end
print(table.concat(words, "|"))            --> give|the|old|map|to|captain|flint

-- A capture returns just the part in parentheses.
print(("go north"):match("^go%s+(%a+)$"))  --> north
print(("look"):match("^go%s+(%a+)$"))      --> nil

-- find with plain = true searches for text, not a pattern.
print(("3.5 kg"):find(".", 1, true))       --> 2   2
```

A parser for a game doesn't need to understand English. It needs to throw away noise ("the", "a", punctuation, case), map many ways of saying something onto one verb, and find the boundary between *what* and *where*: the first preposition in "put the lamp into the box".

<!-- from:typescript -->
## Coming from TypeScript
- `s:gmatch("%S+")` is like `s.split(/\s+/)` without the empty strings at either end.
- `%` is the escape character, not `\`: `%d` is `\d` and `%.` is a literal dot. There is no `|` alternation and no `{2,3}` counts.
- `s:match(p)` returns the captures as separate values (or the whole match without captures), not an array. It returns `nil` when nothing matches.
- `gsub` returns two values, the string and the number of replacements. Wrap the call in parentheses, or call a method on it, to keep only the string.
<!-- /from -->

<!-- from:default -->
## Key points
- Clean the input first: lower case, punctuation to spaces, then split with `gmatch("%S+")`.
- Data tables (synonyms, phrases, directions) keep the parsing code short. Adding a word adds no code.
- `find(s, text, 1, true)` searches for plain text. Without `true`, characters like `.` and `-` are pattern syntax.
<!-- /from -->

## Common mistake
Searching for text that contains a magic character with a pattern function:

```lua
print(("v1.2"):find("."))        --> 1   1      "." is any character
print(("50%"):gsub("%", ""))     --> error: malformed pattern (ends with '%')
```

The characters `^$()%.[]*+-?` mean something in a pattern. To search for them as text, pass `true` as `find`'s fourth argument (`s:find(".", 1, true)`), or escape each one with `%`: `"%."`, `"%%"`.
## In this stage
In `parser.lua`, implement `words` and `parse`. The tables of verbs, synonyms, phrases, directions and prepositions are given; use them rather than writing the words into your code.