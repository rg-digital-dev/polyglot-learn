// A window to see the bar: `qml Preview.qml` (Qt's `qml` tool, next to
// qmltestrunner). It feeds the bar made-up events so there's something to show.

import QtQuick
import QtQuick.Window

Window {
    id: window
    width: 900
    height: bar.height
    visible: true
    color: "#1e1e2e"
    title: "Status bar preview"

    Bar {
        id: bar
        now: Date.now()
        configText: '{"clock": {"showSeconds": true}}'
        Component.onCompleted: {
            ipc("createworkspace>>1\ncreateworkspace>>2\ncreateworkspace>>3\nworkspace>>2\nopenwindow>>a1,3,chat,x\nurgent>>a1")
            battery.parseUevent("POWER_SUPPLY_STATUS=Discharging\nPOWER_SUPPLY_CAPACITY=64\nPOWER_SUPPLY_ENERGY_NOW=32000000\nPOWER_SUPPLY_POWER_NOW=9000000")
            media.load([{ title: "Night Drive", artist: "Synthwave Co.", length: 214 }])
            media.play()
            notifications.notify("mail", "2 new messages", "", Notifications.Normal, 0)
        }
    }

    Timer {
        interval: 1000
        running: true
        repeat: true
        onTriggered: bar.tick(1000)
    }

    Text {
        anchors.left: parent.left
        anchors.leftMargin: 12
        anchors.verticalCenter: parent.verticalCenter
        color: "#cdd6f4"
        text: bar.left
    }
    Text {
        anchors.centerIn: parent
        color: "#cdd6f4"
        font.bold: true
        text: bar.center
    }
    Text {
        anchors.right: parent.right
        anchors.rightMargin: 12
        anchors.verticalCenter: parent.verticalCenter
        color: "#cdd6f4"
        text: bar.right
    }
}
