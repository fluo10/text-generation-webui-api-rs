use serde::{Serialize, Deserialize};
use std::ops::{Deref, DerefMut};

#[derive(Clone, Debug,Default, PartialEq, Serialize, Deserialize)]
pub struct History {
    #[serde(default)]
    pub visible: Vec<(String, String)>,
    #[serde(default)]
    pub internal: Vec<(String, String)>,
}

impl History {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(&mut self, value: (&str, &str)) {
        let (user, agent) = value;
        self.visible.push((user.to_string(),agent.to_string()));
        self.internal.push((user.to_string(),agent.to_string()));
    }

    pub fn pop(&mut self) -> Option<(String,String)> {
        let _ = self.internal.pop();
        self.visible.pop()
    }

    pub fn last_prompt(&self) -> Option<String> {
        Some(self.visible.last()?.0.clone())
    }

    pub fn last_reply(&self) -> Option<String> {
        Some(self.visible.last()?.1.clone())
    }
}

impl Deref for History {
    type Target = [(String, String)];
    fn deref(&self) -> &Self::Target {
        self.visible.deref()
    }
}

impl DerefMut for History {
    fn deref_mut(&mut self) -> &mut [(String, String)] {
        self.visible.deref_mut()
    }
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
