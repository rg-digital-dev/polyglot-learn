import QtQuick

// The clock: everything the bar shows about the time, as bindings on `now`.
QtObject {
    id: root

    // Milliseconds since the epoch. The bar sets it; nothing here reads the
    // system clock, so the same `now` always shows the same text.
    property real now: 0
    property bool use24h: true
    property bool showSeconds: false

    readonly property var dayNames: ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"]
    readonly property var monthNames: ["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"]

    // The local time: "09:05" or "09:05:07"; in 12-hour form "9:05 AM",
    // "12:00 PM" (noon), "12:30 AM" (just after midnight).
    // TODO: make this a binding on now, use24h and showSeconds.
    readonly property string time: ""

    // The local date: "Fri 13 Nov".
    // TODO: make this a binding on now.
    readonly property string date: ""

    // By the local hour: 5 to 11 "Good morning", 12 to 17 "Good afternoon",
    // 18 to 22 "Good evening", otherwise "Good night".
    // TODO: make this a binding on now.
    readonly property string greeting: ""

    // What the bar shows. It follows `time` until someone assigns to it.
    // TODO: make this a binding on time.
    property string label: ""

    // How far `ms` is from now, in whole minutes rounded down: "now" (under a
    // minute either way), "in 5 min", "in 2 h 5 min", "in 3 h", "5 min ago",
    // "1 h 10 min ago".
    function relative(ms: real): string {
        throw new Error("TODO: implement relative")
    }

    // Show a fixed text instead of the time: assigning replaces the binding.
    function override(text: string) {
        throw new Error("TODO: implement override")
    }

    // Follow the time again: give label a new binding.
    function follow() {
        throw new Error("TODO: implement follow")
    }
}
