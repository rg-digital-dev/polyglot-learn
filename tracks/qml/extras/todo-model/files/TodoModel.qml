import QtQuick

QtObject {
    id: root

    // One element per todo: { title: string, done: bool }.
    property ListModel items: ListModel {}

    // How many items are not done. Keep it in sync with items, including when
    // other code changes items directly (hint: the model has signals).
    property int remaining: 0

    // Add an open todo at the end.
    function add(title: string) {
        throw new Error("TODO: implement add")
    }

    // Add an open todo at the top.
    function addFirst(title: string) {
        throw new Error("TODO: implement addFirst")
    }

    // Flip the done flag of the item at index.
    function toggle(index: int) {
        throw new Error("TODO: implement toggle")
    }

    function removeAt(index: int) {
        throw new Error("TODO: implement removeAt")
    }

    // Swap the item at index with the one above it. Index 0 stays put.
    function moveUp(index: int) {
        throw new Error("TODO: implement moveUp")
    }

    // Every title, in model order, as a JS array.
    function titles(): var {
        throw new Error("TODO: implement titles")
    }

    // The titles of the items that are not done, in model order.
    function pending(): var {
        throw new Error("TODO: implement pending")
    }

    // Remove every item that is done.
    function clearDone() {
        throw new Error("TODO: implement clearDone")
    }
}
