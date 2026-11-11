# Storing data in SQLite

Python ships with SQLite, a whole SQL database in one file with no server to run. `sqlite3.connect(path)` opens or creates the file, and `":memory:"` gives a database that disappears when it's closed, which is ideal for tests.

```python
import sqlite3

db = sqlite3.connect(":memory:")
db.execute("CREATE TABLE IF NOT EXISTS scores (name TEXT PRIMARY KEY, points INTEGER)")

rows = [("ada", 30), ("bob", 12), ("ada", 99)]          # ada twice
with db:                                                 # one transaction
    db.executemany("INSERT OR IGNORE INTO scores VALUES (?, ?)", rows)
print(db.execute("SELECT COUNT(*) FROM scores").fetchone()[0])     # 2: the second ada was ignored

try:
    with db:
        db.execute("INSERT INTO scores VALUES ('cy', 5)")
        raise RuntimeError("something went wrong")
except RuntimeError:
    pass
print(db.execute("SELECT name FROM scores WHERE name = ?", ("cy",)).fetchall())  # []: rolled back

for name, total in db.execute("SELECT name, SUM(points) FROM scores GROUP BY name ORDER BY name"):
    print(name, total)
db.close()
```

`with db:` makes a transaction. It commits if the block finishes and rolls back if it raises, so a failed import leaves nothing half-done. `executemany` runs one statement for every tuple in an iterable, and a generator works there too.

Always pass values with `?` placeholders. Building SQL with f-strings lets a customer called `'); DROP TABLE orders; --` do real damage. That's SQL injection.

SQLite has no decimal type, and its `REAL` is a float. Store money as an integer number of cents, so sums are exact, and turn it back into a Decimal when reading. Dates as ISO text (`2026-03-14`) sort and compare correctly as strings.

<!-- from:typescript -->
## Coming from TypeScript
- `sqlite3` is like better-sqlite3 in Node, and it's built in: no package to install, and the calls are synchronous.
- `with db:` is the transaction wrapper you'd write as `db.transaction(() => …)()`.
- A class becomes usable in `with` through `__enter__` and `__exit__`, much like `Symbol.dispose` and `using` in recent TypeScript.
<!-- /from -->

<!-- from:default -->
## Key points
- `with connection:` is a transaction: commit on success, roll back on an exception.
- Use `?` placeholders for values, never string formatting.
- Store money as integer cents and dates as ISO text; convert at the edges.
<!-- /from -->

## Common mistake
Calling `execute` in a loop with a commit after every row. It works, but it's slow (each commit syncs the file to disk), and a failure halfway leaves half an import. One `executemany` inside one `with` is faster and all-or-nothing.

## In this stage
Implement `Store` in `store.py`. `SCHEMA` is given. Make it a context manager (`__enter__` and `__exit__`), and use `with self.db:` in `save`.
