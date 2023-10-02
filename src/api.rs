use crate::{Character, History};
use std::default::Default;
use serde::{Serialize, Deserialize};

#[derive(PartialEq,Serialize, Deserialize)]
pub enum ChatMode {
    Chat,
    ChatInstruct,
    Instruct
}

#[derive(PartialEq,Serialize, Deserialize)]
pub enum InstructionTemplate {
    VicunaV1_1
}
#[derive(PartialEq,Serialize, Deserialize)]
pub struct RequestApi {
    pub user_input: String,
    max_new_tokens: i16,
    auto_max_new_tokens: bool,
    max_tokens_second: i16,
    history: History,
    mode: ChatMode,
    character: String,
    instruction_template: Option<InstructionTemplate>, // Will get autodetected if unset
    your_name: String,
    // 'name of user', # Optional
    name1: Option<String>,
        //'name of character', # Optional
        name2: Option<String>,
        context: Option<String>,
        greeting: Option<String>,
        name1_instruct: Option<String>,
        name2_instruct: Option<String>,
        context_instruct: Option<String>,
        turn_template: Option<String>,
        regenerate: bool,
        #[serde(rename="_continue")]
        cont: bool,
        chat_instruct_command: String,

        //# Generation params. If 'preset' is set to different than 'None', the values
        // in presets/preset-name.yaml are used instead of the individual numbers.
        preset: String,
        do_sample: bool,
        temperature: f32,
        top_p: f32,
        typical_p: i8,
        epsilon_cutoff:i32,
        eta_cutoff: i32,
        tfs: i32,
        top_a: u32,
        repetition_penalty: f32,
        repetition_penalty_range: u32,
        top_k: u32,
        min_length: u32,
        no_repeat_ngram_size: u32,
        num_beams: u32,
        penalty_alpha: u32,
        length_penalty: u32,
        early_stopping: bool,
        mirostat_mode: u32,
        mirostat_tau: u32,
        mirostat_eta: f32,
        grammar_string: String,
        guidance_scale: u32,
        negative_prompt: String,

        seed: i32,
        add_bos_token: bool,
        truncation_length: u32,
        ban_eos_token: bool,
        custom_token_bans: String,
        skip_special_tokens: bool,
        stopping_strings: Vec<String>,
}
impl Default for RequestApi{
    fn default() -> Self {
        Self {
            user_input: "".to_string(),
            max_new_tokens: 250,
            auto_max_new_tokens: false,
            max_tokens_second: 0,
            history: History::default(),
            mode: ChatMode::ChatInstruct,
            character: "Example".to_string(),
            instruction_template: Some(InstructionTemplate::VicunaV1_1),  // Will get autodetected if unset
            your_name: "You".to_string(),
            name1: None,
            name2: None,
            context: None,
            greeting: None,
            name1_instruct: None,
            name2_instruct: None,
            context_instruct: None,
            turn_template: None,
            regenerate: false,
            cont: false,
            chat_instruct_command:r#"'Continue the chat dialogue below. Write a single reply for the character "<|character|>".\n\n<|prompt|>',"#.to_string(),

            preset: "None".to_string(),
            do_sample: true,
            temperature: 0.7,
            top_p: 0.1,
            typical_p: 1,
            epsilon_cutoff: 0, 
            eta_cutoff: 0,
            tfs: 1,
            top_a: 0,
            repetition_penalty: 1.18,
            repetition_penalty_range: 0,
            top_k: 40,
            min_length: 0,
            no_repeat_ngram_size: 0,
            num_beams: 1,
            penalty_alpha: 0,
            length_penalty: 1,
            early_stopping: false,
            mirostat_mode: 0,
            mirostat_tau: 5,
            mirostat_eta: 0.1,
            grammar_string: "".to_string(),
            guidance_scale: 1,
            negative_prompt: "".to_string(),
            seed: -1,
            add_bos_token: true,
            truncation_length: 2048,
            ban_eos_token: false,
            custom_token_bans: "".to_string(),
            skip_special_tokens: true,
            stopping_strings: Vec::new()
        }
    }
}

impl RequestApi{
    pub fn and_character(self, character: Character) -> Self {
        Self {
    character: character.name.clone(),
    your_name: character.user.clone().unwrap_or(Self::default().your_name),
    // 'name of user', # Optional
    name1: character.user.clone(),
        //'name of character', # Optional
        name2: Some(character.name.clone()),
        context: Some(character.context),
        greeting: character.greeting,
        ..self
        }

    }
    pub fn and_history(self, history: History) -> Self {
        Self {
            history: history,
            ..self
        }
    }

}
