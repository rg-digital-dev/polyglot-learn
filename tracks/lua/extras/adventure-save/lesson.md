# Saving as Lua, loading in a sandbox

The simplest save format for a Lua program is Lua itself. A table literal like `return {room = "hall", inventory = {"lamp"}}` is readable, diffable and parsed by `load`, with no JSON library. The catch is that `load` runs code, and a save file can come from anywhere. So the loader has to make sure that running the file can do nothing except build a value.

`load(text, name, mode, env)` compiles a chunk without running it. Its `env` argument becomes the chunk's `_ENV`, the table in which it finds every global name. Pass an empty table and `os`, `io`, `require` and `print` are all nil inside the chunk. Mode `"t"` accepts only source text: precompiled bytecode can crash the interpreter, so it is refused. A debug hook with a count runs every N instructions, which is enough to stop a file that loops forever.

```lua
local function run_data(text)
  local chunk, err = load(text, "=data", "t", {})
  if not chunk then return nil, err end
  debug.sethook(function() error("too long", 0) end, "", 100000)
  local ok, value = pcall(chunk)
  debug.sethook()                         -- always remove the hook
  if not ok then return nil, value end
  return value
end

print(run_data("return {1, 2, three = 3}").three)  --> 3
print(run_data("return os.getenv('HOME')"))       --> nil  data:1: attempt to index a nil value (global 'os')
print(run_data("while true do end"))              --> nil  too long
```

Writing the file is the other half. `string.format("%q", s)` quotes a string so that Lua reads it back exactly: quotes, newlines and zero bytes included. Numbers need care too. Lua 5.4 has integers and floats, so `1` and `1.0` are different values, and `%.17g` prints enough digits for any float to come back unchanged. Lua 5.4's `<close>` attribute closes a file when the variable goes out of scope, even on an early return.

<!-- from:typescript -->
## Coming from TypeScript
- This is `JSON.stringify` and `JSON.parse`, with Lua literals instead of JSON. It handles integer versus float, any string, and non-string keys.
- `load` with an environment is close to running code in a fresh `vm` context with an empty global object. The step limit is the `timeout` option.
- `local f <close> = io.open(...)` is `using f = ...` from explicit resource management: the file is closed when the block ends, however it ends.
<!-- /from -->

<!-- from:default -->
## Key points
- Serialize to a literal; deserialize with `load(text, name, "t", {})`.
- Sort keys when writing, so the same state always gives the same file.
- Treat loaded data as untrusted: no globals, no bytecode, a step limit.
- Save ids, not objects, and look the objects up again when restoring.
<!-- /from -->

## Common mistake
Loading a save with plain `load(text)` or `dofile(path)`. The chunk then runs with every global available, so a save file containing `os.remove("important.txt")` deletes the file. The world files in stage 8 are trusted code that you write yourself. A save file is data, and it is loaded as data.

## In this stage
In `save.lua`, implement `serialize`, `deserialize`, `index`, `snapshot`, `restore`, `save` and `load`. A snapshot holds only ids and plain values. `restore` puts the game's own entity objects back where the snapshot says.