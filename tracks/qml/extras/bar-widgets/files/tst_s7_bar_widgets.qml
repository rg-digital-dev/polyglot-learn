// Stage 7 of the status bar project. Don't edit this file: `learn`
// overwrites it. Your code goes in Widgets.qml.

import QtQuick
import QtTest

TestCase {
    id: testCase
    name: "BarWidgets"

    Component { id: widgetsComponent; Widgets {} }
    Component { id: clockComponent; Clock {} }
    Component { id: batteryComponent; Battery {} }
    Component { id: workspacesComponent; Workspaces {} }
    Component { id: notificationsComponent; Notifications {} }
    Component { id: mediaComponent; Media {} }

    // Widgets with a fresh source of every kind.
    function make() {
        const sources = {
            clock: createTemporaryObject(clockComponent, testCase, { now: new Date(2026, 10, 13, 9, 5).getTime() }),
            battery: createTemporaryObject(batteryComponent, testCase, { percent: 57, status: Battery.Discharging }),
            workspaces: createTemporaryObject(workspacesComponent, testCase),
            notifications: createTemporaryObject(notificationsComponent, testCase),
            media: createTemporaryObject(mediaComponent, testCase)
        }
        const w = createTemporaryObject(widgetsComponent, testCase, { sources: sources })
        verify(w !== null, "could not create Widgets")
        return w
    }

    function test_c40_create_makes_a_widget_bound_to_its_source() {
        const w = make()
        const clock = w.create("clock")
        verify(clock !== null, "create(\"clock\") returned null")
        compare([clock.kind, clock.text, clock.visible], ["clock", "09:05", true], "the clock widget")
        w.sources.clock.now = new Date(2026, 10, 13, 9, 6).getTime()
        compare(clock.text, "09:06", "its text follows the clock")
        const battery = w.create("battery")
        compare([battery.kind, battery.text], ["battery", "57%"], "the battery widget")
        w.sources.battery.percent = 56
        compare(battery.text, "56%", "its text follows the battery")
        compare(w.alive, 2, "alive after creating two")
    }

    function test_c41_unknown_kinds_and_missing_sources_throw() {
        const w = make()
        let message = ""
        try { w.create("weather") } catch (e) { message = e.message }
        compare(message, "unknown widget 'weather'")
        w.sources = { battery: w.sources.battery }
        message = ""
        try { w.create("clock") } catch (e) { message = e.message }
        compare(message, "no source for 'clock'")
    }

    function test_c42_a_widget_requires_its_kind_and_source() {
        const w = make()
        verify(w.widgetComponent !== null && w.widgetComponent !== undefined, "Widgets should have a widgetComponent property: Component { Widget {} }")
        ignoreWarning(/Required property kind was not initialized/)
        ignoreWarning(/Required property source was not initialized/)
        compare(w.widgetComponent.createObject(testCase), null, "creating a Widget without kind and source should fail")
        const plain = w.widgetComponent.createObject(testCase, { kind: "custom", source: {} })
        verify(plain !== null, "a Widget with kind and source")
        compare([plain.kind, plain.text, plain.visible], ["custom", "", true])
        plain.destroy()
    }

    function test_c43_visible_follows_the_source() {
        const w = make()
        const battery = w.create("battery")
        const notifications = w.create("notifications")
        const media = w.create("media")
        compare([battery.visible, notifications.visible, media.visible], [true, false, false], "at first")
        w.sources.battery.status = Battery.Unknown
        w.sources.notifications.notify("mail", "hi", "", Notifications.Normal, 0)
        w.sources.notifications.notify("mail", "hi again", "", Notifications.Normal, 0)
        w.sources.media.load([{ title: "Song", artist: "Band", length: 100 }])
        w.sources.media.play()
        compare([battery.visible, notifications.visible, media.visible], [false, true, true], "after the sources change")
        compare([notifications.text, media.text], ["2 new", "Band - Song"], "texts")
        w.sources.media.position = 10
        w.sources.media.pause()
        compare(media.text, "Band - Song (paused)", "paused")
    }

    function test_c44_workspaces_widget_marks_active_and_urgent() {
        const w = make()
        const ws = w.create("workspaces")
        compare(ws.visible, false, "no workspaces yet")
        const source = w.sources.workspaces
        source.feed("createworkspace>>1\ncreateworkspace>>2\ncreateworkspace>>3\nworkspace>>2\nopenwindow>>a1,3,chat,x\nurgent>>a1")
        compare([ws.visible, ws.text], [true, "1 [2] 3!"], "after the events")
        source.handle("renameworkspace>>1,web")
        compare(ws.text, "web [2] 3!", "after a rename")
        source.handle("workspace>>3")
        compare(ws.text, "web 2 [3]", "after focusing 3")
    }

    function test_c45_build_replaces_a_section_and_destroys_the_old_widgets() {
        const w = make()
        const first = w.build(["clock", "battery"], [])
        compare(first.map(x => x.kind), ["clock", "battery"], "the first build")
        compare(w.textOf(first), "09:05  57%", "textOf")
        const second = w.build(["media", "clock"], first)
        compare(second.map(x => x.kind), ["media", "clock"], "the second build")
        tryCompare(w, "alive", 2)
        compare(w.textOf(second), "09:05", "textOf skips hidden widgets")
    }
}
