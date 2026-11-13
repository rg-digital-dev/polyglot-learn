// Stage 3 of the status bar project. Don't edit this file: `learn`
// overwrites it. Your code goes in Workspaces.qml.

import QtQuick
import QtTest

TestCase {
    id: testCase
    name: "BarWorkspaces"

    Component {
        id: workspacesComponent
        Workspaces {}
    }

    function make() {
        const w = createTemporaryObject(workspacesComponent, testCase)
        verify(w !== null, "could not create Workspaces")
        return w
    }

    // The model as [id, name, windows, urgent] rows.
    function rows(w) {
        const out = []
        for (let i = 0; i < w.model.count; i++) {
            const e = w.model.get(i)
            out.push([e.wsId, e.name, e.windows, e.urgent])
        }
        return out
    }

    function test_c14_add_keeps_workspaces_sorted_by_id() {
        const w = make()
        compare(w.add(3, ""), 0, "index of the first workspace")
        compare(w.add(1, "web"), 0, "1 goes before 3")
        compare(w.add(2, ""), 1, "2 goes between")
        compare(w.add(3, "again"), 2, "an id that exists keeps its index")
        compare(rows(w), [[1, "web", 0, false], [2, "2", 0, false], [3, "3", 0, false]])
        compare(w.count, 3, "count")
        compare(w.names(), ["web", "2", "3"], "names()")
        compare([w.indexOf(2), w.indexOf(9)], [1, -1], "indexOf")
    }

    function test_c15_remove_takes_a_workspace_out() {
        const w = make()
        w.add(1, "")
        w.add(2, "")
        compare(w.remove(1), true)
        compare(w.remove(1), false, "removing it again")
        compare(rows(w), [[2, "2", 0, false]])
    }

    function test_c16_workspace_events_move_the_focus() {
        const w = make()
        const seen = []
        w.activated.connect(id => seen.push(id))
        compare(w.handle("workspace>>2"), true)
        compare(w.active, 2, "active")
        compare(rows(w), [[2, "2", 0, false]], "focusing a workspace that isn't known adds it")
        w.handle("workspace>>5")
        w.handle("workspace>>5")
        compare(seen, [2, 5], "activated fires when the focus changes")
    }

    function test_c17_create_destroy_and_rename_events() {
        const w = make()
        verify(w.handle("createworkspace>>4"))
        verify(w.handle("createworkspace>>1"))
        verify(w.handle("renameworkspace>>4,code, mostly"))
        compare(rows(w), [[1, "1", 0, false], [4, "code, mostly", 0, false]], "after create and rename")
        verify(w.handle("destroyworkspace>>1"))
        compare(rows(w), [[4, "code, mostly", 0, false]], "after destroy")
        compare(w.handle("destroyworkspace>>1"), false, "destroying it again")
        compare(w.handle("renameworkspace>>9,x"), false, "renaming a workspace that isn't there")
    }

    function test_c18_window_events_count_windows_per_workspace() {
        const w = make()
        w.handle("createworkspace>>1")
        w.handle("createworkspace>>2")
        verify(w.handle("openwindow>>5a1f,1,kitty,~/src"))
        verify(w.handle("openwindow>>5a2b,1,firefox,News, weather, and more"))
        verify(w.handle("openwindow>>5a3c,2,mpv,song.mp3"))
        compare(rows(w).map(r => r[2]), [2, 1], "after three windows open")
        verify(w.handle("movewindow>>5a1f,2"))
        compare(rows(w).map(r => r[2]), [1, 2], "after moving one")
        verify(w.handle("closewindow>>5a3c"))
        compare(rows(w).map(r => r[2]), [1, 1], "after closing one")
        compare(w.handle("closewindow>>5a3c"), false, "closing it again")
        compare(w.handle("movewindow>>ffff,1"), false, "moving a window that isn't known")
    }

    function test_c19_urgent_marks_a_workspace_until_it_is_focused() {
        const w = make()
        w.handle("workspace>>1")
        w.handle("openwindow>>beef,3,chat,Ping")
        w.handle("createworkspace>>3")
        verify(w.handle("urgent>>beef"))
        compare(rows(w)[1][3], true, "workspace 3 is urgent")
        w.handle("workspace>>3")
        compare(rows(w)[1][3], false, "focusing it clears the flag")
        compare(w.handle("urgent>>cafe"), false, "an unknown window")
    }

    function test_c20_feed_handles_many_lines_and_skips_the_rest() {
        const w = make()
        const chunk = "createworkspace>>1\nworkspace>>1\n\nactivewindow>>kitty,~\nnonsense\nworkspace>>x\ncreateworkspace>>2\n"
        compare(w.feed(chunk), 3, "handled lines")
        compare([w.count, w.active], [2, 1], "count and active")
    }
}
