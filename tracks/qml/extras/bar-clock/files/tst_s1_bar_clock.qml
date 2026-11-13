// Stage 1 of the status bar project. Don't edit this file: `learn`
// overwrites it. Your code goes in Clock.qml.

import QtQuick
import QtTest

TestCase {
    id: testCase
    name: "BarClock"

    Component {
        id: clockComponent
        Clock {}
    }

    function make() {
        const c = createTemporaryObject(clockComponent, testCase)
        verify(c !== null, "could not create Clock")
        return c
    }

    // Milliseconds for a local date and time; months count from 1 here.
    function at(year, month, day, hour, minute, second) {
        return new Date(year, month - 1, day, hour, minute, second || 0).getTime()
    }

    function test_c01_time_follows_now_in_24_hour_form() {
        const c = make()
        c.now = at(2026, 11, 13, 9, 5)
        compare(c.time, "09:05")
        c.now = at(2026, 11, 13, 23, 59, 30)
        compare(c.time, "23:59", "the binding should follow a new now")
        c.showSeconds = true
        compare(c.time, "23:59:30", "with showSeconds")
    }

    function test_c02_the_12_hour_form() {
        const c = make()
        c.use24h = false
        const cases = [[9, 5, "9:05 AM"], [12, 0, "12:00 PM"], [0, 30, "12:30 AM"], [18, 45, "6:45 PM"]]
        for (const [h, m, expected] of cases) {
            c.now = at(2026, 11, 13, h, m)
            compare(c.time, expected, h + ":" + m)
        }
        c.showSeconds = true
        c.now = at(2026, 11, 13, 7, 8, 9)
        compare(c.time, "7:08:09 AM", "with seconds")
    }

    function test_c03_date_uses_day_and_month_names() {
        const c = make()
        c.now = at(2026, 11, 13, 12, 0)
        compare(c.date, "Fri 13 Nov")
        c.now = at(2027, 1, 3, 0, 1)
        compare(c.date, "Sun 3 Jan", "a new now")
    }

    function test_c04_greeting_changes_with_the_hour() {
        const c = make()
        const cases = [[4, "Good night"], [5, "Good morning"], [11, "Good morning"], [12, "Good afternoon"],
                       [17, "Good afternoon"], [18, "Good evening"], [22, "Good evening"], [23, "Good night"]]
        for (const [h, expected] of cases) {
            c.now = at(2026, 11, 13, h, 30)
            compare(c.greeting, expected, "at " + h + ":30")
        }
    }

    function test_c05_relative_says_how_far_away_a_time_is() {
        const c = make()
        c.now = at(2026, 11, 13, 12, 0)
        const minute = 60000
        compare(c.relative(c.now + 30000), "now", "30 seconds ahead")
        compare(c.relative(c.now - 59000), "now", "59 seconds ago")
        compare(c.relative(c.now + 5 * minute), "in 5 min")
        compare(c.relative(c.now + 125 * minute + 30000), "in 2 h 5 min")
        compare(c.relative(c.now + 180 * minute), "in 3 h")
        compare(c.relative(c.now - 5 * minute), "5 min ago")
        compare(c.relative(c.now - 70 * minute), "1 h 10 min ago")
    }

    function test_c06_override_breaks_the_label_binding_and_follow_restores_it() {
        const c = make()
        c.now = at(2026, 11, 13, 9, 5)
        compare(c.label, "09:05", "label follows time at first")
        c.override("Meeting!")
        c.now = at(2026, 11, 13, 9, 6)
        compare(c.label, "Meeting!", "after override, a new time doesn't change the label")
        c.follow()
        compare(c.label, "09:06", "follow shows the time again")
        c.now = at(2026, 11, 13, 9, 7)
        compare(c.label, "09:07", "and keeps following it")
    }
}
