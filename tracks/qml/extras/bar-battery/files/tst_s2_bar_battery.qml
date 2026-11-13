// Stage 2 of the status bar project. Don't edit this file: `learn`
// overwrites it. Your code goes in Battery.qml.

import QtQuick
import QtTest

TestCase {
    id: testCase
    name: "BarBattery"

    Component {
        id: batteryComponent
        Battery {}
    }

    function make(percent, status) {
        const b = createTemporaryObject(batteryComponent, testCase, { percent: percent, status: status })
        verify(b !== null, "could not create Battery")
        return b
    }

    function test_c07_level_rounds_and_clamps_percent() {
        const b = make(57.6, Battery.Discharging)
        compare(b.level, 58)
        b.percent = 101
        compare(b.level, 100, "above 100")
        b.percent = -1
        compare(b.level, 0, "below 0")
        b.percent = 42.4
        compare(b.level, 42, "the binding should follow percent")
    }

    function test_c08_status_is_an_enum_and_the_label_shows_it() {
        verify(Battery.Unknown !== Battery.Charging && Battery.Charging !== Battery.Discharging && Battery.Discharging !== Battery.Full,
               "Battery.Unknown, Charging, Discharging and Full should be four different values")
        const b = make(57, Battery.Unknown)
        compare(b.label, "", "Unknown")
        b.status = Battery.Discharging
        compare(b.label, "57%", "Discharging")
        b.status = Battery.Charging
        compare(b.label, "57% charging", "Charging")
        b.status = Battery.Full
        compare(b.label, "Full", "Full")
    }

    function test_c09_icon_follows_level_and_status() {
        const b = make(95, Battery.Discharging)
        const cases = [[95, "battery-full"], [90, "battery-full"], [89, "battery-good"], [50, "battery-good"],
                       [49, "battery-low"], [20, "battery-low"], [19, "battery-empty"], [0, "battery-empty"]]
        for (const [p, expected] of cases) {
            b.percent = p
            compare(b.icon, expected, "at " + p + "%")
        }
        b.status = Battery.Charging
        compare(b.icon, "battery-charging", "charging at 0%")
        b.status = Battery.Full
        b.percent = 80
        compare(b.icon, "battery-full", "status Full at 80%")
    }

    function test_c10_low_at_or_below_the_threshold_unless_charging() {
        const b = make(21, Battery.Discharging)
        compare(b.low, false, "21% with threshold 20")
        b.percent = 20
        compare(b.low, true, "20%")
        b.status = Battery.Charging
        compare(b.low, false, "20% while charging")
        b.status = Battery.Discharging
        b.lowThreshold = 10
        compare(b.low, false, "20% with threshold 10")
    }

    function test_c11_lowbattery_fires_once_per_drop() {
        const b = make(30, Battery.Discharging)
        const seen = []
        b.lowBattery.connect(level => seen.push(level))
        for (const p of [25, 20, 18, 15, 22, 24])
            b.percent = p
        compare(seen, [20], "dropping to 20 and below, then up to 24")
        b.percent = 26
        b.percent = 19
        compare(seen, [20, 19], "after recovering above threshold + 5")
        b.status = Battery.Charging
        b.status = Battery.Discharging
        compare(seen, [20, 19, 19], "charging re-arms the warning")
    }

    function test_c12_parseuevent_reads_sysfs_lines() {
        const b = make(0, Battery.Unknown)
        const text = "POWER_SUPPLY_NAME=BAT0\nPOWER_SUPPLY_STATUS=Discharging\nPOWER_SUPPLY_PRESENT=1\n"
                   + "POWER_SUPPLY_CAPACITY=57\nPOWER_SUPPLY_ENERGY_NOW=30000000\n"
                   + "POWER_SUPPLY_ENERGY_FULL=50000000\nPOWER_SUPPLY_POWER_NOW=12000000\n"
        compare(b.parseUevent(text), 5, "lines used")
        compare([b.status, b.level, b.energyNow, b.energyFull, b.powerNow],
                [Battery.Discharging, 57, 30000000, 50000000, 12000000], "status, level and energy")
        b.parseUevent("POWER_SUPPLY_STATUS=Not charging")
        compare(b.status, Battery.Unknown, "a status it doesn't know")
        compare(b.parseUevent("garbage\n\nPOWER_SUPPLY_CAPACITY=80"), 1, "lines without = are skipped")
        compare(b.level, 80)
    }

    function test_c13_remaining_estimates_time_to_empty_or_full() {
        const b = make(60, Battery.Discharging)
        b.energyNow = 30000000
        b.energyFull = 50000000
        b.powerNow = 12000000
        compare(b.remaining, "2 h 30 min left", "30 Wh at 12 W")
        b.powerNow = 40000000
        compare(b.remaining, "45 min left", "30 Wh at 40 W")
        b.status = Battery.Charging
        b.powerNow = 15000000
        compare(b.remaining, "1 h 20 min to full", "20 Wh to go at 15 W")
        b.powerNow = 0
        compare(b.remaining, "", "no power reading")
        b.powerNow = 15000000
        b.status = Battery.Full
        compare(b.remaining, "", "full")
    }
}
