# Building a report with f-strings

A report is text built from data, and f-strings do most of the work. After the colon inside `{…}` comes a format spec: width, alignment, grouping and precision.

```python
from decimal import ROUND_HALF_UP, Decimal

amount = Decimal("1234567.891")
print(f"{amount:,.2f}")            # 1,234,567.89: commas and two decimals
print(f"[{'left':<8}]")            # [left    ]
print(f"[{'right':>8}]")           # [   right]
print("ab".ljust(5, "."), "ab".rjust(5, "."))   # ab... ...ab

# Round half up, the way people expect money to round:
print(Decimal("2.345").quantize(Decimal("0.01"), rounding=ROUND_HALF_UP))   # 2.35
print(round(2.345, 2))             # 2.35 or 2.34, depending on the float: avoid for money

rows = [("games", "1,000.00"), ("books", "234.50")]
width = max(len(name) for name, _ in rows)
for name, money in rows:
    print(f"| {name:<{width}} | {money:>8} |")   # widths can come from variables
```

Markdown tables are plain text, so they're easy to generate and read fine before they're rendered. Padding every column to its widest cell keeps the raw file readable too. `:---` and `---:` in the line under the headers set each column's alignment once rendered.

Build the report as a list of parts joined at the end, rather than adding strings piece by piece. Blank lines between parts are then exactly where the `join` puts them.

<!-- from:typescript -->
## Coming from TypeScript
- f-strings are template literals, `f"{x}"` for `` `${x}` ``. The format spec after `:` replaces `toFixed`, `padStart`, `padEnd` and `toLocaleString`.
- `"\n\n".join(parts)` is `parts.join("\n\n")`, called on the separator rather than the list.
- `quantize` with `ROUND_HALF_UP` is the dependable money rounding that `toFixed` isn't: `(2.345).toFixed(2)` gives "2.35" or "2.34" depending on the float.
<!-- /from -->

<!-- from:default -->
## Key points
- Format specs: `<` `>` for alignment, `,` for thousands, `.2f` for two decimals.
- Round money with `Decimal.quantize(…, ROUND_HALF_UP)`, not `round` on floats.
- Collect the parts in a list and `join` them once.
<!-- /from -->

## Common mistake
Padding cells to a fixed width such as `{name:<10}`. The first long product name breaks the table. Measure the widest cell in each column first.

## In this stage
In `report.py`, implement `money`, `table` and `render`. Their docstrings show the exact layout.
