import QtQuick

QtObject {
    id: root

    property real celsius: 20

    // Bind each of these to an expression so it updates whenever celsius
    // (or fahrenheit, for label) changes:
    // - fahrenheit: celsius * 9 / 5 + 32
    // - label: both scales with one decimal, e.g. "21.5 C / 70.7 F"
    // - freezing: true at or below 0 degrees
    // - kelvin: celsius + 273.15; keep it readonly
    property real fahrenheit
    property string label
    property bool freezing
    readonly property real kelvin: 0

    // Pin fahrenheit to a fixed value: assign it, which removes its binding.
    function pin(value: real) {
        throw new Error("TODO: implement pin")
    }

    // Make fahrenheit follow celsius again with Qt.binding().
    function unpin() {
        throw new Error("TODO: implement unpin")
    }
}
