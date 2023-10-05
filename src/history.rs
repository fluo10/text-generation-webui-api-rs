use serde::{Serialize, Deserialize};

#[derive(Clone, Debug,Default, PartialEq, Serialize, Deserialize)]
pub struct History {
    #[serde(default)]
    visible: Vec<(String, String)>,
    #[serde(default)]
    internal: Vec<(String, String)>,
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn parse () {
       let raw = r#"{
    "visible": [
        [
            "",
            "Hello, I've been waiting for you. Do you need to talk with me? Well, tell me about anything, I'm listening to you."
        ],
        [
            "I just updated your character. How are you feeling?",
            "Oh...I feel great! As always. But it seems that my personality has changed a bit. It feels like there is something new in me. Is everything okay with the update?"
        ]
    ]
}"#;
        let history = History{
            visible: vec![
                (
                    "".to_string(), 
                    "Hello, I've been waiting for you. Do you need to talk with me? Well, tell me about anything, I'm listening to you.".to_string(),
                ),
                (
                    "I just updated your character. How are you feeling?".to_string(),
                    "Oh...I feel great! As always. But it seems that my personality has changed a bit. It feels like there is something new in me. Is everything okay with the update?".to_string(),
                ),
            ],
            internal: Vec::new(),
        };
        let parsed: History = serde_json::from_str(&raw).unwrap();
        assert_eq!(parsed, history);
    }
}
