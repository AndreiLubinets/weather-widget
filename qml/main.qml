import QtQuick.Controls 2.12
import QtQuick.Window 2.12
import QtQuick.Controls.Material 2.12

// This must match the qml_uri and qml_version
// specified with the #[cxx_qt::qobject] macro in Rust.
import demo 1.0

Window {
    visible: true
    height: 300
    width: 400
    color: "#2a2a3e"

    AppState {
        id: state
    }


    Column {
        anchors.horizontalCenter: parent.horizontalCenter
        anchors.verticalCenter: parent.verticalCenter
        /* space between widget */
        spacing: 10

        Text {
            font.bold: true
            font.pointSize: 18
            color: "white"
            horizontalAlignment: Text.AlignHCenter
            text: state.location
        }

        Image {
            source: "http:" + state.image
        }   

        Text {
            horizontalAlignment: Text.AlignHCenter
            color: "white"
            text: state.temp + "°C"
        }

        Button {
            text: "Say Hello!"
            onClicked: state.getWeather()
        }
    }
}