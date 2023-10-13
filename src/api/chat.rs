use crate::{Character, History, Result, Error};
use std::default::Default;
use std::time::Instant;
use serde::{Serialize, Deserialize};
use url::Url;



#[derive(Debug, Default, PartialEq,Serialize, Deserialize)]
pub enum ChatMode {
    #[default]
    Chat,
    ChatInstruct,
    Instruct
}

#[derive(Debug, Default, PartialEq,Serialize, Deserialize)]
pub enum InstructionTemplate {
    #[default]
    VicunaV1_1
}

#[derive(Debug, Default,  PartialEq,Serialize, Deserialize)]
pub struct ChatApiRequest {
    pub user_input: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_new_tokens: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    auto_max_new_tokens: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_tokens_second: Option<i32>,
    

    //# Generation params. If 'preset' is set to different than 'None', the values
    // in presets/preset-name.yaml are used instead of the individual numbers.
    #[serde(skip_serializing_if = "Option::is_none")]
    preset: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    do_sample: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    top_p: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    typical_p: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    epsilon_cutoff:Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    eta_cutoff: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tfs: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    top_a: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    repetition_penalty: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    repetition_penalty_range: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    top_k: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_length: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    no_repeat_ngram_size: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    num_beams: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    penalty_alpha: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    length_penalty: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    early_stopping: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mirostat_mode: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mirostat_tau: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mirostat_eta: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    grammar_string: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    guidance_scale: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    negative_prompt: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    seed: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    add_bos_token: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    truncation_length: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ban_eos_token: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_token_bans: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    skip_special_tokens: Option<bool>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    stopping_strings: Vec<String>,

    // Fields for Chat mode 
    #[serde(skip_serializing_if = "Option::is_none")]
    history: Option<History>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    character: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instruction_template: Option<String>, // Will get autodetected if unset
    #[serde(skip_serializing_if = "Option::is_none")]
    your_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    name1: Option<String>, // 'name of user', # Optional
    #[serde(skip_serializing_if = "Option::is_none")]
    name2: Option<String>, //'name of character', # Optional
    #[serde(skip_serializing_if = "Option::is_none")]
    context: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    greeting: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name1_instruct: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name2_instruct: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    context_instruct: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    turn_template: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    regenerate: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename="_continue")]
    cont: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_instruct_command: Option<String>,
}

impl ChatApiRequest{
    pub fn character_name(self, name: &str) -> Self{
        Self {
            character: Some(name.to_string()),
            ..self
        }
    }

    pub fn character(self, character: &Character) -> Self {
        Self {
            character: Some(character.name.clone()),
            your_name: character.user.clone(),
            name1: character.user.clone(),
            //'name of character', # Optional
            name2: Some(character.name.clone()),
            context: Some(character.context.clone()),
            greeting: character.greeting.clone(),
            ..self
        }
    }
    //pub fn and_preset_name(self, name: String) -> Self {
    //    todo!();
    //}
    //pub fn and_preset(self, preset: Preset) -> Self {
    //    todo!();
    //}
    pub fn history(self, history: &History) -> Self {
        Self {
            history: Some(history.clone()),
            ..self
        }
    }
    pub fn user_input(self, input: &str) -> Self {
        Self {
            user_input: Some(input.to_string()),
            ..self
        }
    }
    pub async fn send(self, url: &str) -> Result<ChatApiResponse> {
        let endpoint_path= "api/v1/chat";
        println!("{:?}", serde_json::to_string(&self)?);
        let endpoint = Url::parse(url)?.join(endpoint_path)?;
        let client = reqwest::Client::new();
        let response = client.post(endpoint)
            .body(serde_json::to_string(&self)?)
            .send().await?
            .text().await?;
        

        Ok(serde_json::from_str(&response)?)
    }

}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ChatApiResponse {
    results: Vec<ChatApiResponseResult>,
}
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ChatApiResponseResult {
        history: History
}

impl ChatApiResponse {
    pub fn pop_history(&mut self) -> Option<History>{
        let content: ChatApiResponseResult = self.results.pop()?;
        Some(content.history)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn parse_response() {
        let json = "{\"results\": [{\"history\": {\"internal\": [[\"Hello\", \"Good morning! How may I assist you today?\"]], \"visible\": [[\"Hello\", \"Good morning! How may I assist you today?\"]]}}]}";
        let mut response: ChatApiResponse = serde_json::from_str(json).unwrap();
        let history: History = response.pop_history().unwrap();
        assert_eq!(history, History{
            internal: vec![(
                          "Hello".to_string(), 
                          "Good morning! How may I assist you today?".to_string()
                      ),],
                      visible: vec![(
                          "Hello".to_string(), 
                          "Good morning! How may I assist you today?".to_string()
                          ),],
        });




    }
}
