// The bar's settings: plain JavaScript in an ECMAScript module, so it can be
// imported into QML (`import "config.mjs" as Lib`) and has no QML in it.

// The widgets a section can list.
export const WIDGETS = ["clock", "battery", "workspaces", "notifications", "media"]

export const DEFAULTS = {
    position: "top",
    height: 30,
    clock: { use24h: true, showSeconds: false },
    battery: { lowThreshold: 20 },
    widgets: {
        left: ["workspaces"],
        center: ["clock"],
        right: ["media", "notifications", "battery"],
    },
}

// A deep copy of plain data.
export function copy(value) {
    return JSON.parse(JSON.stringify(value))
}

// `defaults` with `overrides` laid over it: objects merge key by key, at any
// depth; anything else (arrays included) replaces. Neither input changes,
// and the result shares no objects with them.
export function merge(defaults, overrides) {
    throw new Error("TODO: implement merge")
}

// Every problem in a merged config, as "path: message", in this order:
//   unknown keys, anywhere, in the config's own key order
//                                  "clock.zone: unknown setting", "colour: unknown setting"
//   position not "top"/"bottom"    "position: must be top or bottom"
//   height not an integer 16..128  "height: must be a whole number from 16 to 128"
//   use24h, showSeconds not bool   "clock.use24h: must be true or false"
//   lowThreshold not integer 0..100  "battery.lowThreshold: must be a whole number from 0 to 100"
//   a section not a list           "widgets.left: must be a list"
//   an unknown widget              "widgets.right[1]: unknown widget 'weather'"
// An empty list means the config is valid.
export function validate(config) {
    throw new Error("TODO: implement validate")
}

// Read a config file's text: { config, errors }. Blank text is the defaults.
// Text that isn't JSON gives ["not a JSON object: <JSON.parse's message>"],
// valid JSON that isn't an object ["not a JSON object"], and an invalid
// config validate's errors. With any error, config is the defaults.
export function parse(text) {
    throw new Error("TODO: implement parse")
}
