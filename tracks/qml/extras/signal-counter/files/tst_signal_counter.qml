// Part of the Deeper QML module (tracks/qml/extras/signal-counter). Don't edit
// this file: `learn` overwrites it. Your code goes in SignalCounter.qml.

import QtQuick
import QtTest

TestCase {
    id: testCase
    name: "SignalCounter"

    Component {
        id: subjectComponent
        SignalCounter {}
    }

    Component {
        id: spyComponent
        SignalSpy {}
    }

    // A fresh counter for every test, destroyed when the test ends.
    function make(props) {
        const c = createTemporaryObject(subjectComponent, testCase, props || {})
        verify(c !== null, "could not create SignalCounter")
        return c
    }

    // A SignalSpy records every emission of one signal: spy.count is how many,
    // spy.signalArguments[i] holds the arguments of emission i.
    function spyOn(target, signalName) {
        const spy = createTemporaryObject(spyComponent, testCase, { target: target, signalName: signalName })
        verify(spy.valid, "SignalCounter has no signal " + signalName)
        return spy
    }

    // The arguments of emission i as a plain array.
    function args(spy, i) {
        return Array.from(spy.signalArguments[i])
    }

    function test_c01_increment_adds_one() {
        const c = make()
        c.increment()
        compare(c.count, 1)
        c.increment()
        compare(c.count, 2)
    }

    function test_c02_increment_emits_stepped_with_old_and_new_count() {
        const c = make()
        const stepped = spyOn(c, "stepped")
        c.increment()
        c.increment()
        compare(stepped.count, 2, "stepped emissions")
        compare(args(stepped, 0), [0, 1], "first emission")
        compare(args(stepped, 1), [1, 2], "second emission")
    }

    function test_c03_decrement_emits_stepped_too() {
        const c = make({ count: 5 })
        const stepped = spyOn(c, "stepped")
        c.decrement()
        compare(c.count, 4)
        compare(stepped.count, 1, "stepped emissions")
        compare(args(stepped, 0), [5, 4])
    }

    function test_c04_decrement_stops_at_zero() {
        const c = make({ count: 1 })
        const stepped = spyOn(c, "stepped")
        c.decrement()
        c.decrement()
        compare(c.count, 0)
        compare(stepped.count, 1, "no stepped when nothing changes")
    }

    function test_c05_increment_stops_at_the_limit() {
        const c = make({ limit: 3 })
        const stepped = spyOn(c, "stepped")
        for (let i = 0; i < 5; i++)
            c.increment()
        compare(c.count, 3)
        compare(stepped.count, 3, "no stepped when nothing changes")
    }

    function test_c06_limitreached_is_emitted_once_with_the_limit() {
        const c = make({ limit: 2 })
        const reached = spyOn(c, "limitReached")
        c.increment()
        compare(reached.count, 0, "not at the limit yet")
        c.increment()
        c.increment()
        compare(reached.count, 1, "limitReached emissions")
        compare(args(reached, 0), [2])
    }

    function test_c07_reset_does_not_emit_stepped() {
        const c = make()
        c.increment()
        c.increment()
        const stepped = spyOn(c, "stepped")
        const countChanged = spyOn(c, "countChanged")
        c.reset()
        compare(c.count, 0)
        compare(stepped.count, 0, "stepped emissions")
        compare(countChanged.count, 1, "countChanged still fires")
    }

    function test_c08_oncountchanged_counts_every_change() {
        const c = make()
        c.increment()
        c.increment()
        c.reset()
        c.reset()
        compare(c.changes, 3, "a reset at 0 is not a change")
    }

    function test_c09_onstepped_keeps_a_log() {
        const c = make()
        c.increment()
        c.increment()
        c.decrement()
        c.reset()
        compare(c.log, ["0->1", "1->2", "2->1"])
    }
}
