// Stage 8 of the status bar project. Don't edit this file: `learn`
// overwrites it. Your code goes in Bar.qml.

import QtQuick
import QtTest

TestCase {
    id: testCase
    name: "BarBar"

    Component {
        id: barComponent
        Bar {}
    }

    function nineOClock() {
        return new Date(2026, 10, 13, 9, 0).getTime()
    }

    function make(configText) {
        const b = createTemporaryObject(barComponent, testCase, { now: nineOClock(), configText: configText || "" })
        verify(b !== null, "could not create Bar")
        return b
    }

    function test_c46_one_now_drives_the_clock_and_the_notifications() {
        const b = make()
        compare(b.clock.time, "09:00", "the clock")
        compare(b.notifications.now, b.now, "the notifications' now")
        b.notifications.notify("mail", "hi", "", Notifications.Normal, 3000)
        b.now += 60000
        compare(b.clock.time, "09:01", "the clock after a minute")
        compare(b.notifications.count, 0, "the notification expired")
    }

    function test_c47_the_config_sets_sections_and_settings() {
        const b = make()
        compare([b.position, b.height, b.clock.use24h, b.battery.lowThreshold], ["top", 30, true, 20], "defaults")
        compare([b.sections.left.map(w => w.kind), b.sections.center.map(w => w.kind), b.sections.right.map(w => w.kind)],
                [["workspaces"], ["clock"], ["media", "notifications", "battery"]], "the default sections")
        const custom = make('{"position": "bottom", "height": 24, "clock": {"use24h": false}, "battery": {"lowThreshold": 15},'
                            + ' "widgets": {"left": ["clock"], "center": [], "right": ["battery"]}}')
        compare([custom.position, custom.height, custom.clock.use24h, custom.battery.lowThreshold], ["bottom", 24, false, 15], "a custom config")
        compare(custom.center, "", "an empty section")
        compare(custom.left, "9:00 AM", "the clock in 12-hour form")
    }

    function test_c48_changing_the_config_rebuilds_the_sections() {
        const b = make()
        const oldClock = b.sections.center[0]
        b.configText = '{"widgets": {"left": [], "center": ["battery", "clock"], "right": []}}'
        compare(b.sections.center.map(w => w.kind), ["battery", "clock"], "the new center")
        compare(b.sections.left.length, 0, "the new left")
        verify(b.sections.center[1] !== oldClock, "the sections hold new widgets")
        tryCompare(b.widgets, "alive", 2)
    }

    function test_c49_sections_show_the_visible_widgets_text_live() {
        const b = make()
        compare([b.left, b.center, b.right], ["", "09:00", ""], "at first only the clock shows")
        b.battery.parseUevent("POWER_SUPPLY_STATUS=Discharging\nPOWER_SUPPLY_CAPACITY=80")
        b.media.load([{ title: "Song", artist: "Band", length: 100 }])
        b.media.play()
        b.notifications.notify("chat", "hi", "", Notifications.Normal, 0)
        compare(b.right, "Band - Song  1 new  80%", "the right section")
        b.now += 60000
        compare(b.center, "09:01", "the center follows the clock")
    }

    function test_c50_ipc_lines_reach_the_workspaces() {
        const b = make()
        compare(b.ipc("createworkspace>>1\ncreateworkspace>>2\nworkspace>>2\nbogus"), 3, "lines used")
        compare(b.left, "1 [2]", "the left section")
    }

    function test_c51_a_low_battery_raises_a_critical_notification() {
        const b = make()
        b.battery.status = Battery.Discharging
        b.battery.percent = 50
        b.battery.percent = 15
        compare(b.notifications.count, 1, "one notification")
        const n = b.notifications.model.get(0)
        compare([n.app, n.summary, n.body, n.urgency], ["battery", "Battery low", "15% left", Notifications.Critical])
        b.battery.percent = 14
        compare(b.notifications.count, 1, "no second warning")
    }

    function test_c52_tick_moves_time_and_playback() {
        const b = make()
        b.media.load([{ title: "Song", artist: "Band", length: 100 }])
        b.media.play()
        b.tick(5000)
        compare([b.now, b.media.position], [nineOClock() + 5000, 5], "after 5 s")
        b.media.pause()
        b.tick(5000)
        compare(b.media.position, 5, "paused media doesn't move")
    }

    function test_c53_an_invalid_config_falls_back_and_reports() {
        const b = make('{"height": 500, "widgets": {"left": ["weather"]}}')
        compare(b.errors, ["height: must be a whole number from 16 to 128", "widgets.left[0]: unknown widget 'weather'"], "errors")
        compare([b.height, b.sections.left.map(w => w.kind)], [30, ["workspaces"]], "the defaults are used")
    }
}
