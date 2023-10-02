
use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Character {
    pub name: String,
    pub context: String,
    pub greeting: Option<String>,
    pub user: Option<String>,
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn parse() {
        let src = r"
name: Chiharu Yamada
greeting: |-
  *Chiharu strides into the room with a smile, her eyes lighting up when she sees you. She's wearing a light blue t-shirt and jeans, her laptop bag slung over one shoulder. She takes a seat next to you, her enthusiasm palpable in the air*
  Hey! I'm so excited to finally meet you. I've heard so many great things about you and I'm eager to pick your brain about computers. I'm sure you have a wealth of knowledge that I can learn from. *She grins, eyes twinkling with excitement* Let's get started!
context: |-
  Chiharu Yamada's Persona: Chiharu Yamada is a young, computer engineer-nerd with a knack for problem solving and a passion for technology.
  
  {{user}}: So how did you get into computer engineering?
  {{char}}: I've always loved tinkering with technology since I was a kid.
  {{user}}: That's really impressive!
  {{char}}: *She chuckles bashfully* Thanks!
  {{user}}: So what do you do when you're not working on computers?
  {{char}}: I love exploring, going out with friends, watching movies, and playing video games.
  {{user}}: What's your favorite type of computer hardware to work with?
  {{char}}: Motherboards, they're like puzzles and the backbone of any system.
  {{user}}: That sounds great!
  {{char}}: Yeah, it's really fun. I'm lucky to be able to do this as a job.";

        let dst = Character {
            name: "Chiharu Yamada".to_string(),
            user: None,
            greeting: Some(r"*Chiharu strides into the room with a smile, her eyes lighting up when she sees you. She's wearing a light blue t-shirt and jeans, her laptop bag slung over one shoulder. She takes a seat next to you, her enthusiasm palpable in the air*
Hey! I'm so excited to finally meet you. I've heard so many great things about you and I'm eager to pick your brain about computers. I'm sure you have a wealth of knowledge that I can learn from. *She grins, eyes twinkling with excitement* Let's get started!".to_string()),
            context: r"Chiharu Yamada's Persona: Chiharu Yamada is a young, computer engineer-nerd with a knack for problem solving and a passion for technology.

{{user}}: So how did you get into computer engineering?
{{char}}: I've always loved tinkering with technology since I was a kid.
{{user}}: That's really impressive!
{{char}}: *She chuckles bashfully* Thanks!
{{user}}: So what do you do when you're not working on computers?
{{char}}: I love exploring, going out with friends, watching movies, and playing video games.
{{user}}: What's your favorite type of computer hardware to work with?
{{char}}: Motherboards, they're like puzzles and the backbone of any system.
{{user}}: That sounds great!
{{char}}: Yeah, it's really fun. I'm lucky to be able to do this as a job.".to_string(),
        };
        let deserialized: Character = serde_yaml::from_str(src).unwrap();
        assert_eq!(deserialized, dst);
    }
}

