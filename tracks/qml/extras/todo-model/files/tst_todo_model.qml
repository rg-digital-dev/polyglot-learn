// Part of the Deeper QML module (tracks/qml/extras/todo-model). Don't edit
// this file: `learn` overwrites it. Your code goes in TodoModel.qml.

import QtQuick
import QtTest

TestCase {
    id: testCase
    name: "TodoModel"

    Component {
        id: subjectComponent
        TodoModel {}
    }

    // A fresh, empty list for every test, destroyed when the test ends.
    function make(titles) {
        const m = createTemporaryObject(subjectComponent, testCase)
        verify(m !== null, "could not create TodoModel")
        for (const t of titles || [])
            m.add(t)
        return m
    }

    function test_c01_add_appends_an_open_item() {
        const m = make()
        m.add("milk")
        m.add("bread")
        compare(m.items.count, 2)
        compare(m.items.get(1).title, "bread")
        compare(m.items.get(1).done, false)
    }

    function test_c02_addfirst_inserts_at_the_top() {
        const m = make(["b"])
        m.addFirst("a")
        compare(m.titles(), ["a", "b"])
    }

    function test_c03_toggle_flips_done() {
        const m = make(["a", "b"])
        m.toggle(1)
        compare(m.items.get(1).done, true, "after one toggle")
        compare(m.items.get(0).done, false, "the other item is untouched")
        m.toggle(1)
        compare(m.items.get(1).done, false, "after two toggles")
    }

    function test_c04_removeat_removes_one_item() {
        const m = make(["a", "b", "c"])
        m.removeAt(1)
        compare(m.titles(), ["a", "c"])
    }

    function test_c05_moveup_swaps_with_the_item_above() {
        const m = make(["a", "b", "c"])
        m.moveUp(2)
        compare(m.titles(), ["a", "c", "b"])
        m.moveUp(0)
        compare(m.titles(), ["a", "c", "b"], "index 0 stays put")
    }

    function test_c06_remaining_counts_open_items() {
        const m = make(["a", "b", "c"])
        compare(m.remaining, 3, "three new items")
        m.toggle(0)
        compare(m.remaining, 2, "after toggling one")
        m.removeAt(0)
        compare(m.remaining, 2, "after removing the done one")
        m.removeAt(0)
        compare(m.remaining, 1, "after removing an open one")
    }

    function test_c07_remaining_follows_direct_model_changes() {
        const m = make()
        m.items.append({ title: "x", done: false })
        m.items.append({ title: "y", done: false })
        compare(m.remaining, 2, "after items.append")
        m.items.setProperty(0, "done", true)
        compare(m.remaining, 1, "after items.setProperty")
        m.items.clear()
        compare(m.remaining, 0, "after items.clear")
    }

    function test_c08_pending_lists_open_titles_in_order() {
        const m = make(["a", "b", "c", "d"])
        m.toggle(1)
        m.toggle(3)
        compare(m.pending(), ["a", "c"])
    }

    function test_c09_cleardone_removes_every_finished_item() {
        const m = make(["a", "b", "c", "d", "e"])
        m.toggle(1)
        m.toggle(2)
        m.toggle(4)
        m.clearDone()
        compare(m.titles(), ["a", "d"])
        compare(m.remaining, 2)
    }
}
