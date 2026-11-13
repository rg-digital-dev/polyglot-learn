import QtQuick

// The whole bar: its parts, the config that arranges them, and the three
// lines of text it shows.
QtObject {
    id: bar

    // Milliseconds since the epoch, for the clock and the notifications.
    property real now: 0

    // TODO: make configText an alias of the Config's text, and errors,
    // position and height bindings on the Config.
    property string configText: ""
    readonly property var errors: []
    readonly property string position: ""
    readonly property int height: 0

    // TODO: the parts, each held in a property of its own type:
    //   config         a Config
    //   clock          a Clock on bar.now, with use24h and showSeconds from the config
    //   battery        a Battery with lowThreshold from the config
    //   workspaces     a Workspaces
    //   notifications  a Notifications on bar.now
    //   media          a Media
    //   widgets        a Widgets whose sources are the five parts above

    // The widgets of each section, made from the config's widgets lists:
    // built when the bar is complete and rebuilt whenever the config changes.
    property var sections: ({ left: [], center: [], right: [] })

    // The visible widgets' text per section (Widgets.textOf), live.
    // TODO: make these bindings.
    readonly property string left: ""
    readonly property string center: ""
    readonly property string right: ""

    // TODO: when the battery emits lowBattery(level), notify from "battery"
    // with the summary "Battery low", the body "15% left", Critical urgency
    // and the default timeout.

    // Lines from the compositor's event socket. Returns how many were used.
    function ipc(text: string): int {
        throw new Error("TODO: implement ipc")
    }

    // Time passes: move now and, while playing, the media position.
    function tick(ms: real) {
        throw new Error("TODO: implement tick")
    }
}
