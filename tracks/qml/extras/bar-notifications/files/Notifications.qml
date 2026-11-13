import QtQuick

// The notification center: what apps sent, newest first, until it expires
// or is dismissed.
QtObject {
    id: root

    enum Urgency {
        Low,
        Normal,
        Critical
    }

    // Milliseconds; the bar moves it forward. Expiry follows it.
    property real now: 0
    property bool doNotDisturb: false
    property int defaultTimeout: 5000

    // Newest first: { nid: int, app: string, summary: string, body: string,
    // urgency: int, at: real, expires: real } where at is the now it arrived
    // and expires 0 means never.
    property ListModel model: ListModel {}

    property int nextId: 1

    // TODO: make count and badge bindings.
    readonly property int count: 0

    // For the bar: "" with none, "1" to "9", then "9+".
    readonly property string badge: ""

    // Emitted for a notification that should pop up on screen: every one,
    // except that do-not-disturb holds back all but critical ones.
    signal popup(int nid)
    // Emitted when a notification expires (not when it is dismissed).
    signal expired(int nid)

    // TODO: when now changes, remove the notifications whose time has come
    // (expires > 0 and expires <= now), emitting expired for each, oldest first.

    // Add a notification and return its id (1, 2, 3, ...). `timeout` in ms:
    // -1 uses defaultTimeout, 0 never expires. Critical ones never expire.
    function notify(app: string, summary: string, body: string, urgency: int, timeout: int): int {
        throw new Error("TODO: implement notify")
    }

    // Remove one notification; false if there is no such id.
    function dismiss(nid: int): bool {
        throw new Error("TODO: implement dismiss")
    }

    // Remove every notification, or only those of `app` when it isn't "".
    // Returns how many were removed.
    function clear(app: string): int {
        throw new Error("TODO: implement clear")
    }

    // One entry per app: [{ app, count, latest }] where latest is the summary
    // of its newest notification; the app with the newest notification first.
    function groups(): var {
        throw new Error("TODO: implement groups")
    }
}
