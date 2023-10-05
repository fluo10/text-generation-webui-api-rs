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

#[derive(Debug, Default, PartialEq,Serialize, Deserialize)]
pub struct ChatApiBuilder {
    pub user_input: Option<String>,
    max_new_tokens: Option<i32>,
    auto_max_new_tokens: Option<bool>,
    max_tokens_second: Option<i32>,
    

    //# Generation params. If 'preset' is set to different than 'None', the values
    // in presets/preset-name.yaml are used instead of the individual numbers.
    preset: Option<String>,
    do_sample: Option<bool>,
    temperature: Option<f32>,
    top_p: Option<f32>,
    typical_p: Option<f32>,
    epsilon_cutoff:Option<f32>,
    eta_cutoff: Option<f32>,
    tfs: Option<f32>,
    top_a: Option<f32>,
    repetition_penalty: Option<f32>,
    repetition_penalty_range: Option<i32>,
    top_k: Option<i32>,
    min_length: Option<i32>,
    no_repeat_ngram_size: Option<i32>,
    num_beams: Option<i32>,
    penalty_alpha: Option<f32>,
    length_penalty: Option<f32>,
    early_stopping: Option<bool>,
    mirostat_mode: Option<i32>,
    mirostat_tau: Option<f32>,
    mirostat_eta: Option<f32>,
    grammar_string: Option<String>,
    guidance_scale: Option<f32>,
    negative_prompt: Option<String>,

    seed: Option<i32>,
    add_bos_token: Option<bool>,
    truncation_length: Option<i32>,
    ban_eos_token: Option<bool>,
    custom_token_bans: Option<String>,
    skip_special_tokens: Option<bool>,
    stopping_strings: Vec<String>,

    // Fields for Chat mode 
    history: Option<History>,
    mode: Option<ChatMode>,
    character: Option<String>,
    instruction_template: Option<InstructionTemplate>, // Will get autodetected if unset
    your_name: Option<String>,

    name1: Option<String>, // 'name of user', # Optional
    name2: Option<String>, //'name of character', # Optional
    context: Option<String>,
    greeting: Option<String>,
    name1_instruct: Option<String>,
    name2_instruct: Option<String>,
    context_instruct: Option<String>,
    turn_template: Option<String>,
    regenerate: Option<bool>,
    #[serde(rename="_continue")]
    cont: Option<bool>,
    chat_instruct_command: Option<String>,
}

impl ChatApiBuilder{
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
    #[tokio::main]
    pub async fn request(self, url: &str) -> Result<reqwest::Response> {
        let endpoint_path= "api/v1/chat";
        let endpoint = Url::parse(url)?.join(endpoint_path)?;

        let client = reqwest::Client::new();
        let response = client.post(endpoint)
            .body(serde_json::to_string(&self)?)
            .send().await?;
        Ok(response)
    }

}
