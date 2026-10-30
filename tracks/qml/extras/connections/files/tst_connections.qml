// Part of the Deeper QML module (tracks/qml/extras/connections). Don't edit
// this file: `learn` overwrites it. Your code goes in Thermostat.qml.

import QtQuick
import QtTest

TestCase {
    id: testCase
    name: "Thermostat"

    Component {
        id: sensorComponent
        Sensor {}
    }

    Component {
        id: thermostatComponent
        Thermostat {}
    }

    Component {
        id: spyComponent
        SignalSpy {}
    }

    // Fresh objects for every test, destroyed when the test ends.
    function makeSensor() {
        const s = createTemporaryObject(sensorComponent, testCase)
        verify(s !== null, "could not create Sensor")
        return s
    }

    function makeThermostat(sensor) {
        const t = createTemporaryObject(thermostatComponent, testCase, { sensor: sensor })
        verify(t !== null, "could not create Thermostat")
        return t
    }

    function spyOnWarning(thermostat) {
        return createTemporaryObject(spyComponent, testCase, { target: thermostat, signalName: "warning" })
    }

    function test_c01_heats_when_the_reading_drops_below_target() {
        const s = makeSensor()
        const t = makeThermostat(s)
        s.report(18)
        compare(t.heating, true, "reading 18, target 21")
    }

    function test_c02_stops_heating_at_the_target() {
        const s = makeSensor()
        const t = makeThermostat(s)
        s.report(15)
        compare(t.heating, true, "reading 15, target 21")
        s.report(21)
        compare(t.heating, false, "reading 21, target 21")
    }

    function test_c03_a_new_target_rechecks_the_last_reading() {
        const s = makeSensor()
        const t = makeThermostat(s)
        s.report(20)
        t.target = 22
        compare(t.heating, true, "reading 20, target raised to 22")
        t.target = 19
        compare(t.heating, false, "reading 20, target lowered to 19")
    }

    function test_c04_follows_a_replaced_sensor() {
        const first = makeSensor()
        const second = makeSensor()
        const t = makeThermostat(first)
        t.sensor = second
        second.report(10)
        compare(t.heating, true, "the new sensor reports 10")
        first.report(30)
        compare(t.heating, true, "the old sensor is no longer watched")
    }

    function test_c05_forwards_sensor_alarms_as_warnings() {
        const s = makeSensor()
        const t = makeThermostat(s)
        const warning = spyOnWarning(t)
        s.fail("battery low")
        compare(warning.count, 1, "warning emissions")
        compare(Array.from(warning.signalArguments[0]), ["battery low"])
    }

    function test_c06_listen_forwards_another_sensor_s_alarms() {
        const s = makeSensor()
        const other = makeSensor()
        const t = makeThermostat(s)
        const warning = spyOnWarning(t)
        t.listen(other)
        other.fail("door open")
        compare(warning.count, 1, "warning emissions")
        compare(Array.from(warning.signalArguments[0]), ["door open"])
    }

    function test_c07_ignore_disconnects_that_sensor() {
        const s = makeSensor()
        const other = makeSensor()
        const t = makeThermostat(s)
        const warning = spyOnWarning(t)
        t.listen(other)
        other.fail("first")
        t.ignore(other)
        other.fail("second")
        compare(warning.count, 1, "only the alarm before ignore() is forwarded")
        s.fail("main sensor")
        compare(warning.count, 2, "the main sensor is still forwarded")
    }
}
