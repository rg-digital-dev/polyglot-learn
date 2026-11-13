# A ListModel fed by events

Hyprland, like most compositors, tells clients what happens on a socket, one line per event: `workspace>>3` when the focus moves, `openwindow>>5a1f,3,kitty,~/src` when a window opens. A bar keeps a model of the workspaces and updates it line by line.

`ListModel` is QML's list of records. Views like `Repeater` and `ListView` watch it and update only the rows that changed. You change it through its methods, so it can tell them what happened:

```qml
import QtQuick

QtObject {
    property ListModel people: ListModel {}

    function addSorted(name, age) {
        let i = 0
        while (i < people.count && people.get(i).name < name)
            i++
        people.insert(i, { name: name, age: age })      // one row, in place
    }

    function birthday(name) {
        for (let i = 0; i < people.count; i++) {
            if (people.get(i).name === name) {
                people.setProperty(i, "age", people.get(i).age + 1)
                return true
            }
        }
        return false
    }
}
```

`count`, `get(i)`, `append`, `insert`, `remove`, `move` and `setProperty` cover almost everything. A row's fields must keep their types: a field that starts as a number stays a number.

Parsing the event lines is plain JavaScript string work: `indexOf(">>")`, `slice`, `split(",")` and `parseInt`. One detail matters: the last field of some events (a window title, a new workspace name) can contain commas itself. Split, then join the remainder back together.

<!-- from:typescript -->
## Coming from TypeScript
- A `ListModel` is an observable array of plain records; `insert(i, obj)` is `splice(i, 0, obj)` with change notifications.
- `delete obj[key]` and object literals used as maps (`windowsAt[address]`) work as in any JavaScript.
- `parseInt("x")` is `NaN`, and `NaN === NaN` is false. Test with `isNaN`.
<!-- /from -->

<!-- from:default -->
## Key points
- Change a ListModel with its methods: `insert`, `remove`, `setProperty`.
- Split protocol lines once, at the first separator; rejoin fields that may contain the separator.
- Keep side tables (window to workspace) for events that only name an address.
<!-- /from -->

## Common mistake
Splitting a whole line on commas and trusting the field count:

```js
const [addr, ws, cls, title] = "5a2b,1,firefox,News, weather, and more".split(",")
// title is "News"; " weather" and " and more" are lost
```

## In this stage
In `Workspaces.qml`, implement `indexOf`, `add`, `remove`, `handle`, `feed` and `names`, and make `count` a binding. In Hyprland the events come from `$XDG_RUNTIME_DIR/hypr/$HYPRLAND_INSTANCE_SIGNATURE/.socket2.sock`. Here the tests play them to you.