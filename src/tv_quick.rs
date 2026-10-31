//! Adapter between Quick Settings and the documented webOS TV Luna methods.
//! UI code asks for an action; it does not name service URIs.
use serde_json::{Value, json};

/// System settings app launched from All settings.
/// `applicationManager/launch` takes this id; if the TV has no such app, the launch fails.
pub const SETTINGS_APP: &str = "com.webos.app.settings";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Action {
    VolumeUp,
    VolumeDown,
    Mute,
    Unmute,
    Network,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Outcome {
    Audio,
    Network(Option<bool>),
}

/// Documented webOS TV calls only.
/// Audio: <https://webostv.developer.lge.com/develop/references/audio>
/// Network: <https://webostv.developer.lge.com/develop/references/connection-manager>
pub fn request(action: Action) -> (&'static str, Value) {
    match action {
        Action::VolumeUp => ("luna://com.webos.audio/volumeUp", json!({})),
        Action::VolumeDown => ("luna://com.webos.audio/volumeDown", json!({})),
        Action::Mute => ("luna://com.webos.audio/setMuted", json!({"muted": true})),
        Action::Unmute => ("luna://com.webos.audio/setMuted", json!({"muted": false})),
        Action::Network => (
            "luna://com.palm.connectionmanager/getStatus",
            json!({"subscribe": false}),
        ),
    }
}

pub fn interpret(action: Action, value: &Value) -> Outcome {
    match action {
        Action::VolumeUp | Action::VolumeDown | Action::Mute | Action::Unmute => Outcome::Audio,
        Action::Network => Outcome::Network(
            value
                .get("isInternetConnectionAvailable")
                .and_then(Value::as_bool),
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn audio_calls_match_the_documented_methods() {
        let (up, up_body) = request(Action::VolumeUp);
        let (down, down_body) = request(Action::VolumeDown);
        let (mute, mute_body) = request(Action::Mute);
        assert_eq!(up, "luna://com.webos.audio/volumeUp");
        assert_eq!(down, "luna://com.webos.audio/volumeDown");
        assert_eq!(mute, "luna://com.webos.audio/setMuted");
        assert_eq!(up_body, json!({}));
        assert_eq!(down_body, json!({}));
        assert_eq!(mute_body, json!({"muted": true}));
        assert_eq!(
            interpret(Action::VolumeUp, &json!({"returnValue": true})),
            Outcome::Audio
        );
    }

    #[test]
    fn network_status_uses_the_documented_connection_flag() {
        let (uri, body) = request(Action::Network);
        assert_eq!(uri, "luna://com.palm.connectionmanager/getStatus");
        assert_eq!(body, json!({"subscribe": false}));
        assert_eq!(
            interpret(
                Action::Network,
                &json!({"returnValue": true, "isInternetConnectionAvailable": true})
            ),
            Outcome::Network(Some(true))
        );
    }
}
