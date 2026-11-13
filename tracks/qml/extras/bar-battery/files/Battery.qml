import QtQuick

// The battery, as Linux reports it in /sys/class/power_supply/BAT0/uevent.
QtObject {
    id: root

    // From anywhere: Battery.Unknown, Battery.Charging, ...
    enum Status {
        Unknown,
        Charging,
        Discharging,
        Full
    }

    // The raw charge, which can be off: 57.6, 101 or -1.
    property real percent: 0
    property int status: Battery.Unknown
    property int lowThreshold: 20
    // As sysfs reports them: energy in µWh, power in µW. 0 means unknown.
    property real energyNow: 0
    property real energyFull: 0
    property real powerNow: 0

    // percent rounded, and kept between 0 and 100.
    // TODO: make this and the four properties below bindings.
    readonly property int level: 0

    // At or below the threshold, unless charging.
    readonly property bool low: false

    // "battery-charging" while charging; otherwise by level: "battery-full"
    // (90 and up, or status Full), "battery-good" (50 and up), "battery-low"
    // (20 and up), "battery-empty".
    readonly property string icon: ""

    // "57%", "57% charging", "Full", or "" while the status is Unknown.
    readonly property string label: ""

    // While discharging: "2 h 30 min left" (energyNow / powerNow). While
    // charging: "1 h 5 min to full" ((energyFull - energyNow) / powerNow).
    // Whole minutes, rounded down; "" when powerNow is 0 or nothing applies.
    readonly property string remaining: ""

    // Emitted with the level when the battery becomes low. Once warned, it
    // stays quiet until the battery recovers: charging, or a level above
    // lowThreshold + 5.
    // TODO: emit it from change handlers (onLowChanged, ...).
    signal lowBattery(int level)

    // Read the KEY=value lines of a uevent file and set the properties they
    // cover: POWER_SUPPLY_STATUS (Charging, Discharging, Full, anything else
    // is Unknown), POWER_SUPPLY_CAPACITY, POWER_SUPPLY_ENERGY_NOW,
    // POWER_SUPPLY_ENERGY_FULL and POWER_SUPPLY_POWER_NOW. Other lines are
    // ignored. Returns how many lines were used.
    function parseUevent(text: string): int {
        throw new Error("TODO: implement parseUevent")
    }
}
