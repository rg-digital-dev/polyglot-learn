import QtQuick

// The pieces of the bar: one small object per widget, made on demand from
// the names in the config, each showing text from its source.
QtObject {
    id: root

    // What each kind of widget shows: { clock: Clock, battery: Battery,
    // workspaces: Workspaces, notifications: Notifications, media: Media }.
    property var sources: ({})

    // How many widgets exist right now: created and not yet destroyed.
    property int alive: 0

    // TODO: declare the widgets as inline components in this file.
    //
    // Widget, the base of every widget, a QtObject with
    //   required property string kind
    //   required property var source
    //   property var owner        the Widgets that made it (null by default);
    //                             when the widget is destroyed, it lowers owner.alive
    //   property string text      ("" by default)
    //   property bool visible     (true by default)
    //
    // Then one per kind, each built on Widget and setting its kind:
    //   ClockWidget          text: the clock's time
    //   BatteryWidget        text: the battery's label; hidden while its status is Battery.Unknown
    //   WorkspacesWidget     text: the names separated by spaces, the active one in
    //                        brackets, urgent ones followed by "!": "1 [2] 3!";
    //                        hidden with no workspaces. Keep it up to date when
    //                        the model or the active workspace changes.
    //   NotificationsWidget  text: "<badge> new" ("3 new"); hidden with no notifications
    //   MediaWidget          text: the media's label, plus " (paused)" while paused;
    //                        hidden while stopped

    // A Component that makes a plain Widget: Component { Widget {} }.
    property Component widgetComponent: null

    // A Component per kind: { clock: Component { ClockWidget {} }, ... }.
    readonly property var components: ({})

    // Make a widget of this kind on its source, with this object as its
    // owner and parent, and count it in alive.
    // Throws "unknown widget 'weather'" or "no source for 'clock'".
    function create(kind: string): var {
        throw new Error("TODO: implement create")
    }

    // Make the widgets for a section, in order, after destroying the ones in
    // `old` (the list a previous build returned). Returns the new list.
    function build(names: var, old: var): var {
        throw new Error("TODO: implement build")
    }

    // The text of the visible widgets in a list with text, separated by two
    // spaces.
    function textOf(list: var): string {
        throw new Error("TODO: implement textOf")
    }
}
