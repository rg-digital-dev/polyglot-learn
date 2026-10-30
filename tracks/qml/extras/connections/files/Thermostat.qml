import QtQuick

// A thermostat that watches a Sensor (see Sensor.qml). The root is an Item,
// not a QtObject, so it can hold child objects such as a Connections element.
Item {
    id: root

    // The sensor to watch. It can be replaced at any time, or be null.
    property Sensor sensor: null

    property real target: 21

    // True while the sensor's reading is below target. Re-check it whenever
    // the reading or the target changes.
    property bool heating: false

    // Forward every alarm of the watched sensor (and of the sensors passed to
    // listen()) as this signal, with the same reason.
    signal warning(string reason)

    // TODO: add a Connections element here that targets root.sensor.

    // Also forward other's alarms as warning, until ignore(other) is called.
    function listen(other: Sensor) {
        throw new Error("TODO: implement listen")
    }

    // Stop forwarding other's alarms.
    function ignore(other: Sensor) {
        throw new Error("TODO: implement ignore")
    }
}
