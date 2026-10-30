# List models

Views such as `ListView` and `Repeater` can show a JavaScript array, but they rebuild every row whenever it changes. They work best with a **model**. `ListModel` is the built-in one. Each element is an object of named roles (`{ name: "Ada", score: 3 }`), and the model tells views exactly which rows were inserted, removed, moved or changed, so only those rows are redrawn. You change it with its own methods: `append(obj)`, `insert(index, obj)`, `remove(index, n)`, `move(from, to, n)`, `setProperty(index, role, value)` and `clear()`. You read it with `count` and `get(index)`. The model emits `countChanged` when rows come or go and `dataChanged` when a role of an existing row changes, so other code can stay in sync.

```qml
import QtQuick

QtObject {
    id: board

    property ListModel players: ListModel {
        onCountChanged: board.updateTotal()
        onDataChanged: board.updateTotal()
    }
    property int total: 0

    function updateTotal() {
        let sum = 0
        for (let i = 0; i < players.count; i++)
            sum += players.get(i).score
        total = sum
    }

    function join(name: string) {
        players.append({ name: name, score: 0 })
    }

    function score(index: int, points: int) {
        players.setProperty(index, "score", players.get(index).score + points)
    }

    function leaders(min: int): var {
        const out = []
        for (let i = 0; i < players.count; i++)
            if (players.get(i).score >= min)
                out.push(players.get(i).name)
        return out
    }
}
// board.join("Ada"); board.join("Lin"); board.score(1, 5)
// board.total -> 5, board.leaders(1) -> ["Lin"]
// board.players.move(1, 0, 1) -> Lin is first
```

<!-- from:typescript -->
## Coming from TypeScript
- A `ListModel` is not an array: no `length`, no indexing with `[]`, no `map` or `filter`. Use `count`, `get(i)` and a loop.
- `get(i)` returns a live view of the row, not a copy; read the values you need, or change them with `setProperty`.
- The first element appended fixes the roles' types. Keep every element the same shape.
- Copy into a JS array when you want array methods for a computed result.
<!-- /from -->

<!-- from:default -->
## Key points
- Change a `ListModel` only through its methods so views get notified.
- `countChanged` and `dataChanged` let you keep derived values in sync.
- Remove while looping by walking backwards from `count - 1`.
<!-- /from -->

## Common mistake
Treating the model like an array:

```qml
console.log(items.length)        // undefined
items[0].title                   // TypeError: Cannot read property 'title' of undefined
items.get(0).title               // fixed
```

## In this exercise
Edit `todo-model/TodoModel.qml`. Implement the list operations on `items` with the `ListModel` methods, return titles as JS arrays from `titles()` and `pending()`, and keep `remaining` equal to the number of open items. One test changes `items` directly, without your functions, so update `remaining` from the model's signals.
