// Provided, don't change: `learn` overwrites this file. Your code goes in
// Thermostat.qml.

import QtQuick

QtObject {
    // The last temperature reported, in degrees Celsius.
    property real reading: 20

    // Emitted when the sensor has a problem.
    signal alarm(string reason)

    function report(value: real) {
        reading = value
    }

    function fail(reason: string) {
        alarm(reason)
    }
}
