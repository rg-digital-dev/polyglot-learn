// Stage 5 of the status bar project. Don't edit this file: `learn`
// overwrites it. Your code goes in Media.qml.

import QtQuick
import QtTest

TestCase {
    id: testCase
    name: "BarMedia"

    Component {
        id: mediaComponent
        Media {}
    }

    function tracks() {
        return [
            { title: "Intro", artist: "Band", length: 60 },
            { title: "Song", artist: "Band", length: 200 },
            { title: "Outro", artist: "Other", length: 30 }
        ]
    }

    function make() {
        const m = createTemporaryObject(mediaComponent, testCase)
        verify(m !== null, "could not create Media")
        m.load(tracks())
        return m
    }

    function test_c27_load_sets_the_queue_and_stops() {
        const m = make()
        compare([m.index, m.position, m.playing, m.status], [0, 0, false, "stopped"])
        compare(m.track.title, "Intro", "track")
        m.load([])
        compare([m.index, m.track, m.status], [-1, null, "stopped"], "an empty queue")
        m.play()
        compare(m.playing, false, "play with nothing to play")
    }

    function test_c28_play_pause_toggle_and_stop_change_the_status() {
        const m = make()
        m.play()
        compare(m.status, "playing", "after play")
        m.position = 10
        m.pause()
        compare(m.status, "paused", "after pause")
        m.toggle()
        compare(m.status, "playing", "after toggle")
        m.stop()
        compare([m.status, m.position], ["stopped", 0], "after stop")
        m.toggle()
        compare(m.status, "playing", "toggle from stopped")
        m.pause()
        compare(m.status, "stopped", "paused at 0:00 counts as stopped")
    }

    function test_c29_the_icon_follows_the_state() {
        verify(hasGroup(), "Media should have a StateGroup in a property named group")
        const m = make()
        compare(m.icon, "media-playback-stop", "stopped")
        m.play()
        compare([m.group.state, m.icon], ["playing", "media-playback-start"], "playing")
        m.position = 5
        m.pause()
        compare([m.group.state, m.icon], ["paused", "media-playback-pause"], "paused")
        m.stop()
        compare([m.group.state, m.icon], ["", "media-playback-stop"], "stopped again: the default state")
    }

    function hasGroup() {
        const m = createTemporaryObject(mediaComponent, testCase)
        return m !== null && m.group !== undefined && m.group !== null && m.group.states.length === 2
    }

    function test_c30_next_and_previous_change_track() {
        const m = make()
        m.next()
        compare(m.track.title, "Song", "next")
        m.position = 10
        m.previous()
        compare([m.track.title, m.position], ["Song", 0], "previous after 10 s restarts the track")
        m.previous()
        compare(m.track.title, "Intro", "previous at the start goes back")
        m.previous()
        compare(m.index, 0, "previous on the first track stays")
        m.seek(500)
        compare(m.position, 60, "seek stays within the track")
    }

    function test_c31_advance_moves_on_through_the_queue() {
        const m = make()
        m.advance(30)
        compare(m.position, 0, "nothing moves while stopped")
        m.play()
        m.advance(45)
        compare([m.index, m.position], [0, 45], "45 s into the first track")
        m.advance(25)
        compare([m.index, m.position], [1, 10], "the 10 s left over carry into the next track")
        compare(m.status, "playing", "still playing")
    }

    function test_c32_the_end_of_the_queue_stops_or_repeats() {
        const m = make()
        m.play()
        m.advance(60 + 200 + 30)
        compare([m.index, m.position, m.status], [2, 0, "stopped"], "the end of the last track")
        m.repeat = true
        m.play()
        m.advance(35)
        compare([m.index, m.position, m.status], [0, 5, "playing"], "with repeat")
    }

    function test_c33_label_and_progress_describe_the_track() {
        const m = make()
        m.next()
        m.seek(65)
        compare(m.label, "Band - Song")
        compare(m.progress, "1:05 / 3:20")
        m.load([])
        compare([m.label, m.progress], ["", ""], "with no track")
    }
}
