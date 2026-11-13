import QtQuick
import "config.mjs" as Lib

// The bar's config file, parsed by config.mjs whenever its text changes.
QtObject {
    id: root

    // The file's text; the bar reads it from disk.
    property string text: ""

    // Lib.parse(text) as a binding, so it runs again when text changes:
    // { config, errors }.
    // TODO: make parsed, config, errors and valid bindings.
    readonly property var parsed: ({ config: {}, errors: [] })
    readonly property var config: ({})
    readonly property var errors: []
    readonly property bool valid: false

    // The value at a dotted path, such as "clock.use24h"; undefined when
    // the path doesn't exist.
    function get(path: string): var {
        throw new Error("TODO: implement get")
    }
}
