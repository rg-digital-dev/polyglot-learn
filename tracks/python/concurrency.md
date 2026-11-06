# Concurrency in Python

Python has threads (`threading`), processes (`multiprocessing`) and an event loop (`asyncio`). In standard CPython, a lock called the GIL lets only one thread run Python code at a time, so threads don't speed up computation; but a thread that waits (for the network, the disk, or `time.sleep`) releases it, so waiting work runs in parallel fine. `concurrent.futures.ThreadPoolExecutor` is the simple way in: `pool.map(fn, items)` runs `fn` on every item using the pool's threads and returns the results in input order. Use it in a `with` block, which waits for all of them.

```python
import time
from concurrent.futures import ThreadPoolExecutor


def fetch(url: str) -> int:
    time.sleep(0.05)          # stands in for a network call
    return len(url)


urls = ["a.example", "bb.example", "ccc.example"]
start = time.perf_counter()
with ThreadPoolExecutor(max_workers=len(urls)) as pool:
    sizes = list(pool.map(fetch, urls))
print(sizes, round(time.perf_counter() - start, 2))   # [9, 10, 11] about 0.05, not 0.15
```

<!-- from:typescript -->
## Coming from TypeScript
- Real threads instead of one event loop; `pool.map` is like `Promise.all(items.map(fn))` for blocking code.
- Python also has `async`/`await` (`asyncio.gather`), but it needs async libraries all the way down.
- Threads share memory: use a `threading.Lock` when several of them change the same data.
<!-- /from -->

<!-- from:default -->
## Key points
- Threads help with waiting, not with CPU work (the GIL).
- `ThreadPoolExecutor().map(fn, items)` keeps the input order.
- `with` waits for the pool to finish.
<!-- /from -->

## Common mistake
Calling the function instead of passing it:

```python
pool.map(task(d), delays)   # runs task once, here, and passes its result
pool.map(task, delays)      # fixed: the pool calls task for each delay
```

## In this exercise
Implement `run_all(delays)` in `run-all/run_all.py`. The provided `task(ms)` sleeps; run all of them at the same time and return the results in input order. The tests time the call.
