// Stage 6 of the status bar project. Don't edit this file: `learn`
// overwrites it. Your code goes in config.mjs and Config.qml.

import QtQuick
import QtTest
import "config.mjs" as Lib

TestCase {
    id: testCase
    name: "BarConfig"

    Component {
        id: configComponent
        Config {}
    }

    function make(text) {
        const c = createTemporaryObject(configComponent, testCase, { text: text })
        verify(c !== null, "could not create Config")
        return c
    }

    function test_c34_merge_combines_nested_objects_without_changing_them() {
        const defaults = { a: 1, inner: { x: 1, y: 2 }, list: [1, 2, 3] }
        const overrides = { inner: { y: 5, z: 6 }, list: [9], extra: true }
        const merged = Lib.merge(defaults, overrides)
        compare(merged, { a: 1, inner: { x: 1, y: 5, z: 6 }, list: [9], extra: true })
        compare(defaults, { a: 1, inner: { x: 1, y: 2 }, list: [1, 2, 3] }, "defaults unchanged")
        compare(overrides, { inner: { y: 5, z: 6 }, list: [9], extra: true }, "overrides unchanged")
        merged.inner.x = 100
        compare(defaults.inner.x, 1, "the result shares nothing with defaults")
    }

    function test_c35_the_defaults_are_valid() {
        compare(Lib.validate(Lib.copy(Lib.DEFAULTS)), [])
        compare(Lib.WIDGETS, ["clock", "battery", "workspaces", "notifications", "media"], "WIDGETS")
        compare(Lib.DEFAULTS.widgets.right, ["media", "notifications", "battery"], "the default right section")
    }

    function test_c36_validate_names_the_path_of_each_problem() {
        const config = Lib.merge(Lib.DEFAULTS, {
            colour: "red",
            position: "left",
            height: 12.5,
            clock: { use24h: "yes", zone: "UTC" },
            battery: { lowThreshold: 150 },
            widgets: { left: "clock", right: ["battery", "weather"] }
        })
        compare(Lib.validate(config), [
            "clock.zone: unknown setting",
            "colour: unknown setting",
            "position: must be top or bottom",
            "height: must be a whole number from 16 to 128",
            "clock.use24h: must be true or false",
            "battery.lowThreshold: must be a whole number from 0 to 100",
            "widgets.left: must be a list",
            "widgets.right[1]: unknown widget 'weather'"
        ])
    }

    function test_c37_parse_falls_back_to_the_defaults() {
        compare(Lib.parse("  "), { config: Lib.DEFAULTS, errors: [] }, "blank text")
        const good = Lib.parse('{"position": "bottom", "clock": {"showSeconds": true}}')
        compare(good.errors, [], "a good file's errors")
        compare([good.config.position, good.config.clock.showSeconds, good.config.clock.use24h], ["bottom", true, true], "a good file")
        const broken = Lib.parse('{"position": ')
        compare(broken.config, Lib.DEFAULTS, "broken JSON gives the defaults")
        compare(broken.errors.length, 1, "one error")
        verify(broken.errors[0].indexOf("not a JSON object: ") === 0, "the error starts with 'not a JSON object: ', got: " + broken.errors[0])
        compare(Lib.parse("[1, 2]"), { config: Lib.DEFAULTS, errors: ["not a JSON object"] }, "a list")
        compare(Lib.parse('{"height": 4}'), { config: Lib.DEFAULTS, errors: ["height: must be a whole number from 16 to 128"] }, "an invalid file")
    }

    function test_c38_config_parses_its_text_through_a_binding() {
        const c = make("")
        compare([c.valid, c.config.position], [true, "top"], "empty text")
        c.text = '{"position": "bottom"}'
        compare([c.valid, c.config.position], [true, "bottom"], "after setting text")
        c.text = '{"position": "middle"}'
        compare([c.valid, c.errors, c.config.position], [false, ["position: must be top or bottom"], "top"], "invalid text")
    }

    function test_c39_get_reads_a_dotted_path() {
        const c = make('{"clock": {"use24h": false}}')
        compare(c.get("clock.use24h"), false)
        compare(c.get("height"), 30)
        compare(c.get("widgets.center"), ["clock"])
        compare(c.get("clock.nothing"), undefined, "a missing key")
        compare(c.get("height.deeper"), undefined, "a path through a number")
    }
}
