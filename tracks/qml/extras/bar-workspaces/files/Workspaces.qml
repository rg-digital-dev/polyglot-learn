import QtQuick

// The compositor's workspaces, kept up to date from Hyprland's event socket
// (socket2): one "EVENT>>DATA" line per event.
QtObject {
    id: root

    // One element per workspace, sorted by wsId:
    // { wsId: int, name: string, windows: int, urgent: bool }
    property ListModel model: ListModel {}

    // The id of the focused workspace; 0 before the first focus.
    property int active: 0

    // TODO: make this a binding on the model's count.
    readonly property int count: 0

    // Window address -> the id of the workspace it is on.
    property var windowsAt: ({})

    // Emitted with the id when a workspace event changes the focus.
    signal activated(int wsId)

    // The model index of a workspace, or -1.
    function indexOf(wsId: int): int {
        throw new Error("TODO: implement indexOf")
    }

    // Add a workspace in id order; `name` defaults to the id as text.
    // Adding an id that exists changes nothing. Returns its index.
    function add(wsId: int, name: string): int {
        throw new Error("TODO: implement add")
    }

    // Remove a workspace; false if there was none.
    function remove(wsId: int): bool {
        throw new Error("TODO: implement remove")
    }

    // Handle one event line. Returns false for an unknown event or bad data.
    //   workspace>>3                 focus 3 (adding it if needed), clear its urgent flag
    //   createworkspace>>5           add 5
    //   destroyworkspace>>5          remove 5
    //   renameworkspace>>3,code      rename 3 (the new name may contain commas)
    //   openwindow>>ADDR,3,CLASS,TITLE   a window on 3 (the title may contain commas)
    //   closewindow>>ADDR            that window is gone
    //   movewindow>>ADDR,4           that window moved to 4
    //   urgent>>ADDR                 mark that window's workspace urgent
    // A window can open on a workspace that isn't in the model yet: remember
    // where it is, but there is no count to change.
    function handle(line: string): bool {
        throw new Error("TODO: implement handle")
    }

    // Handle every line of a chunk read from the socket; blank lines are
    // skipped. Returns how many lines were handled.
    function feed(text: string): int {
        throw new Error("TODO: implement feed")
    }

    // The workspace names, in order.
    function names(): var {
        throw new Error("TODO: implement names")
    }
}
