# Concurrency

Concurrency means making progress on several tasks over the same stretch of time. If three tasks wait 300, 100 and 200 ms, running them one after another takes 600 ms. Starting all three and then waiting takes about 300 ms, the time of the slowest.

`run_all` checks two things:

- **The results come back in input order**, even though the tasks finish in a different order.
- **The time is about the slowest task, not the sum.** Running the tasks one by one fails, and so does skipping them.

The tools are very different across the six tracks:

| Track | Mechanism | Runs in parallel? | Time in the tests |
|---|---|---|---|
| Rust | OS threads (`std::thread::scope`) | yes | real milliseconds |
| Go | goroutines + `sync.WaitGroup` | yes | real milliseconds |
| TypeScript | promises on one event loop (`Promise.all`) | no: tasks take turns while waiting | real milliseconds |
| PHP | Fibers you resume yourself | no | virtual ticks |
| Lua | coroutines you resume yourself | no | virtual ticks |
| QML | generators you advance yourself | no | virtual ticks |

Plain PHP and Lua have no threads, and QML's JS engine has no `setTimeout`. So those tracks provide a **virtual clock**: a task finishes after `ms` ticks, and you call `tick()` once per scheduling round. Advance every unfinished task once, then tick. Do that until all are done. This round-robin loop is cooperative multitasking. It's what event loops, game engines and FiveM's `Citizen.CreateThread` do under the hood.

## Results in order

Tasks finish in any order. Write each result into its slot by index (`results[i] = ...`), not with `append`/`push` as they come in.

## Common mistake

Waiting inside the loop that starts the tasks: `for d in delays: results.push(await task(d))`. Each task then starts only after the previous one has finished. Start them all first, then wait for all of them.

## Go deeper

- [Concurrency (Wikipedia)](https://en.wikipedia.org/wiki/Concurrency_(computer_science))
- [Coroutine](https://en.wikipedia.org/wiki/Coroutine) and [cooperative multitasking](https://en.wikipedia.org/wiki/Cooperative_multitasking)
