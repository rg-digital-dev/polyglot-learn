// Stage 4 of the status bar project. Don't edit this file: `learn`
// overwrites it. Your code goes in Notifications.qml.

import QtQuick
import QtTest

TestCase {
    id: testCase
    name: "BarNotifications"

    Component {
        id: notificationsComponent
        Notifications {}
    }

    function make() {
        const n = createTemporaryObject(notificationsComponent, testCase, { now: 1000 })
        verify(n !== null, "could not create Notifications")
        return n
    }

    function summaries(n) {
        const out = []
        for (let i = 0; i < n.model.count; i++)
            out.push(n.model.get(i).summary)
        return out
    }

    function test_c21_notify_adds_the_newest_first() {
        const n = make()
        compare(n.notify("mail", "2 new messages", "", Notifications.Normal, -1), 1, "the first id")
        compare(n.notify("chat", "Ann: hi", "how are you?", Notifications.Low, -1), 2, "the second id")
        compare(summaries(n), ["Ann: hi", "2 new messages"], "newest first")
        const top = n.model.get(0)
        compare([top.nid, top.app, top.body, top.urgency, top.at, top.expires],
                [2, "chat", "how are you?", Notifications.Low, 1000, 6000], "the newest one's fields")
        compare(n.count, 2, "count")
    }

    function test_c22_notifications_expire_as_now_passes() {
        const n = make()
        const gone = []
        n.expired.connect(nid => gone.push(nid))
        n.notify("a", "default", "", Notifications.Normal, -1)    // 1: expires at 6000
        n.notify("b", "short", "", Notifications.Normal, 2000)    // 2: at 3000
        n.notify("c", "never", "", Notifications.Normal, 0)       // 3: never
        n.notify("d", "critical", "", Notifications.Critical, 10) // 4: never
        n.now = 2999
        compare(summaries(n), ["critical", "never", "short", "default"], "at 2999")
        n.now = 3000
        compare(summaries(n), ["critical", "never", "default"], "at 3000")
        n.now = 100000
        compare(summaries(n), ["critical", "never"], "much later")
        compare(gone, [2, 1], "expired ids, in the order they expired")
    }

    function test_c23_dismiss_and_clear_remove_notifications() {
        const n = make()
        n.notify("mail", "m1", "", Notifications.Normal, 0)
        n.notify("chat", "c1", "", Notifications.Normal, 0)
        n.notify("mail", "m2", "", Notifications.Normal, 0)
        compare(n.dismiss(2), true, "dismiss 2")
        compare(n.dismiss(2), false, "dismiss 2 again")
        n.notify("chat", "c2", "", Notifications.Normal, 0)
        compare(n.clear("mail"), 2, "clear mail")
        compare(summaries(n), ["c2"])
        compare(n.clear(""), 1, "clear everything")
        compare(n.count, 0)
    }

    function test_c24_do_not_disturb_holds_back_popups() {
        const n = make()
        const popped = []
        n.popup.connect(nid => popped.push(nid))
        n.notify("a", "one", "", Notifications.Normal, 0)
        n.doNotDisturb = true
        n.notify("a", "two", "", Notifications.Normal, 0)
        n.notify("a", "three", "", Notifications.Critical, 0)
        compare(popped, [1, 3], "popups")
        compare(n.count, 3, "every notification is still kept")
    }

    function test_c25_groups_counts_per_app_latest_first() {
        const n = make()
        n.notify("mail", "m1", "", Notifications.Normal, 0)
        n.notify("chat", "c1", "", Notifications.Normal, 0)
        n.notify("mail", "m2", "", Notifications.Normal, 0)
        n.notify("mail", "m3", "", Notifications.Normal, 0)
        compare(n.groups(), [{ app: "mail", count: 3, latest: "m3" }, { app: "chat", count: 1, latest: "c1" }])
        n.clear("")
        compare(n.groups(), [], "no notifications")
    }

    function test_c26_badge_counts_for_the_bar() {
        const n = make()
        compare(n.badge, "", "none")
        n.notify("a", "x", "", Notifications.Normal, 0)
        compare(n.badge, "1", "one")
        for (let i = 0; i < 9; i++)
            n.notify("a", "x", "", Notifications.Normal, 0)
        compare(n.badge, "9+", "ten")
        n.dismiss(1)
        compare(n.badge, "9", "nine")
    }
}
