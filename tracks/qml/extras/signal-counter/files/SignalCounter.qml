import QtQuick

QtObject {
    id: root

    property int count: 0
    property int limit: 10

    // How many times count has changed. Increase it in an onCountChanged handler.
    property int changes: 0

    // Every step as "old->new", e.g. ["0->1", "1->2"]. Fill it in an onStepped handler.
    property var log: []

    // Emitted by increment() and decrement() whenever count actually changes.
    signal stepped(int oldValue, int newValue)

    // Emitted once, when increment() makes count equal to limit.
    signal limitReached(int value)

    // Add one to count, unless it is already at the limit.
    function increment() {
        throw new Error("TODO: implement increment")
    }

    // Take one from count, unless it is already 0.
    function decrement() {
        throw new Error("TODO: implement decrement")
    }

    // Set count back to 0 without emitting stepped.
    function reset() {
        throw new Error("TODO: implement reset")
    }
}
