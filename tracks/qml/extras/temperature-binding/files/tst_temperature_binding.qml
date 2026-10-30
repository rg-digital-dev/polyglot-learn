// Part of the Deeper QML module (tracks/qml/extras/temperature-binding). Don't
// edit this file: `learn` overwrites it. Your code goes in TemperatureBinding.qml.

import QtQuick
import QtTest

TestCase {
    id: testCase
    name: "TemperatureBinding"

    Component {
        id: subjectComponent
        TemperatureBinding {}
    }

    // A fresh instance for every test, destroyed when the test ends.
    function make(celsius) {
        const t = createTemporaryObject(subjectComponent, testCase, { celsius: celsius })
        verify(t !== null, "could not create TemperatureBinding")
        return t
    }

    function test_c01_fahrenheit_follows_celsius() {
        const t = make(100)
        fuzzyCompare(t.fahrenheit, 212, 0.001)
        t.celsius = -40
        fuzzyCompare(t.fahrenheit, -40, 0.001, "after celsius = -40")
        t.celsius = 37
        fuzzyCompare(t.fahrenheit, 98.6, 0.001, "after celsius = 37")
    }

    function test_c02_label_shows_both_scales_with_one_decimal() {
        const t = make(21.5)
        compare(t.label, "21.5 C / 70.7 F")
        t.celsius = 0
        compare(t.label, "0.0 C / 32.0 F", "after celsius = 0")
    }

    function test_c03_freezing_is_true_at_or_below_zero() {
        const t = make(0)
        compare(t.freezing, true, "at 0")
        t.celsius = 0.5
        compare(t.freezing, false, "at 0.5")
        t.celsius = -12
        compare(t.freezing, true, "at -12")
    }

    function test_c04_kelvin_is_readonly_and_follows_celsius() {
        const t = make(0)
        fuzzyCompare(t.kelvin, 273.15, 0.001)
        t.celsius = 100
        fuzzyCompare(t.kelvin, 373.15, 0.001, "after celsius = 100")
        let error = null
        try {
            t.kelvin = 5
        } catch (e) {
            error = e
        }
        verify(error !== null, "assigning kelvin should throw: declare it readonly")
    }

    function test_c05_pin_breaks_the_fahrenheit_binding() {
        const t = make(20)
        t.pin(50)
        fuzzyCompare(t.fahrenheit, 50, 0.001)
        t.celsius = 30
        fuzzyCompare(t.fahrenheit, 50, 0.001, "a pinned fahrenheit ignores celsius")
    }

    function test_c06_label_follows_a_pinned_fahrenheit() {
        const t = make(20)
        t.pin(50)
        compare(t.label, "20.0 C / 50.0 F")
    }

    function test_c07_unpin_restores_the_binding() {
        const t = make(20)
        t.pin(50)
        t.unpin()
        fuzzyCompare(t.fahrenheit, 68, 0.001, "right after unpin")
        t.celsius = 100
        fuzzyCompare(t.fahrenheit, 212, 0.001, "celsius changes are followed again")
        compare(t.label, "100.0 C / 212.0 F")
    }
}
