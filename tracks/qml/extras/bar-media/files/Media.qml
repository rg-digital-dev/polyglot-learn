import QtQuick

// A media player's state: a queue of tracks, what plays, where it is.
QtObject {
    id: root

    // [{ title: string, artist: string, length: seconds }]
    property var queue: []
    property int index: -1
    property bool playing: false
    // Seconds into the current track.
    property real position: 0
    // At the end of the queue: start again from the first track.
    property bool repeat: false

    // TODO: make track, status, label and progress bindings.
    // queue[index], or null when index is outside the queue.
    readonly property var track: null

    // "playing", "paused" or "stopped": the state of the group below, where
    // its default state "" means stopped.
    readonly property string status: "stopped"

    // Set by the states: "media-playback-start" while playing,
    // "media-playback-pause" while paused, "media-playback-stop" otherwise.
    property string icon: "media-playback-stop"

    // "Artist - Title", or "" with no track.
    readonly property string label: ""

    // "1:05 / 3:20", or "" with no track.
    readonly property string progress: ""

    // A QtObject has no `state` of its own (Item does), so the states live in
    // a StateGroup. TODO: add two states:
    //   "playing" when playing and there is a track,
    //   "paused"  when not playing, there is a track and position > 0,
    // each setting root.icon with PropertyChanges.
    property StateGroup group: StateGroup {
        id: group
        states: []
    }

    // A new queue: stopped at its first track (index -1 when it's empty).
    function load(tracks: var) {
        throw new Error("TODO: implement load")
    }

    // Play, when there is a track.
    function play() {
        throw new Error("TODO: implement play")
    }

    function pause() {
        throw new Error("TODO: implement pause")
    }

    function toggle() {
        throw new Error("TODO: implement toggle")
    }

    // Stop: not playing, back to the start of the track.
    function stop() {
        throw new Error("TODO: implement stop")
    }

    // The next track from its start; at the end of the queue, the first track
    // with repeat, otherwise stop on the last one.
    function next() {
        throw new Error("TODO: implement next")
    }

    // More than 3 seconds in: back to the start of this track. Otherwise
    // the previous track (staying on the first).
    function previous() {
        throw new Error("TODO: implement previous")
    }

    // Jump to a position, kept within the track.
    function seek(seconds: real) {
        throw new Error("TODO: implement seek")
    }

    // Time passes while playing: move the position, going on to the next
    // track (or stopping) when this one ends. Time left over carries on into
    // the next track.
    function advance(seconds: real) {
        throw new Error("TODO: implement advance")
    }
}
