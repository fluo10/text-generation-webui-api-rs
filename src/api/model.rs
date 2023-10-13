use crate::Result;
use serde::{Serialize, Deserialize};
use url::Url;

use std::collections::HashMap;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ModelApiRequestAction {
    #[serde(rename="info")]
    Info,
    #[serde(rename="load")]
    Load,
    #[serde(rename="list")]
    List,
}

#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ModelApiRequest{
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<ModelApiRequestAction>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_name: Option<String>,

}

impl ModelApiRequest {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn info() -> Self {
        Self{
            action: Some(ModelApiRequestAction::Info),
            ..Default::default()
        }
    }
    pub fn list() -> Self {
        Self{
            action: Some(ModelApiRequestAction::List),
            ..Default::default()
        }
    }
    pub fn load(model_name: &str) -> Self {
        Self{
            action: Some(ModelApiRequestAction::Load),
            model_name: Some(model_name.to_string()),
        }
    }

    pub async fn send(self, url: &str) -> Result<reqwest::Response> {
        let endpoint_path = "api/v1/model";
        println!("{:?}", serde_json::to_string(&self)?);
        let endpoint = Url::parse(url)?.join(endpoint_path)?;
        let client = reqwest::Client::new();
        let response = client.post(endpoint)
            .body(serde_json::to_string(&self)?)
            .send().await?;
        Ok(response)
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ModelApiResponse {
    pub result: ModelApiResponseResult,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ModelApiResponseResult {
    List(Vec<String>),
    Info(ModelApiResponseInfo),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ModelApiResponseInfo {
    pub model_name: String,
    pub lora_names: Vec<String>,
   // #[serde(rename = "shared.settings")]
   // shared_settings: String,
   // #[serde(rename = "shared.args")]
   // shared_args: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parse_info_response() {
        let json = r##"{"result": {"model_name": "airoboros-c34b-2.1.Q4_K_M.gguf", "lora_names": [],
        "shared.settings": {"dark_theme": true, "show_controls": true, "start_with": "", "mode": "chat", "chat_style": "cai-chat", "character": "None", "prompt-default": "QA", "prompt-notebook": "QA", "preset": "simple-1", "max_new_tokens": 200, "max_new_tokens_min": 1, "max_new_tokens_max": 4096, "seed": -1, "negative_prompt": "", "truncation_length": 2048, "truncation_length_min": 0, "truncation_length_max": 16384, "custom_stopping_strings": "", "auto_max_new_tokens": false, "max_tokens_second": 0, "ban_eos_token": false, "custom_token_bans": "", "add_bos_token": true, "skip_special_tokens": true, "stream": true, "name1": "You", "name2": "Assistant", "context": "This is a conversation with your Assistant. It is a computer program designed to help you with various tasks such as answering questions, providing recommendations, and helping with decision making. You can ask it anything you want and it will do its best to give you accurate and relevant information.", "greeting": "", "instruction_template": null, "chat-instruct_command": "Continue the chat dialogue below. Write a single reply for the character \"<|character|>\".\n\n<|prompt|>", "autoload_model": false, "default_extensions": ["gallery"]}, 
        "shared.args": {"notebook": false, "chat": false, "multi_user": false, "character": "Rufina", "model": "airoboros-c34b-2.1.Q4_K_M.gguf", "lora": null, "model_dir": "models/", "lora_dir": "loras/", "model_menu": false, "no_stream": false, "settings": null, "extensions": ["api,whisper_stt", "api", "gallery"], "verbose": false, "chat_buttons": false, "loader": "llama.cpp", "cpu": false, "auto_devices": false, "gpu_memory": null, "cpu_memory": null, "disk": false, "disk_cache_dir": "cache", "load_in_8bit": false, "bf16": false, "no_cache": false, "xformers": false, "sdp_attention": false, "trust_remote_code": false, "load_in_4bit": false, "compute_dtype": "float16", "quant_type": "nf4", "use_double_quant": false, "threads": 0, "n_batch": 512, "no_mmap": false, "low_vram": false, "mlock": false, "mul_mat_q": false, "cache_capacity": null, "n_gpu_layers": 50, "tensor_split": "10,1", "n_ctx": 4096, "llama_cpp_seed": 0.0, "wbits": 0, "model_type": null, "groupsize": -1, "pre_layer": null, "checkpoint": null, "monkey_patch": false, "triton": false, "no_inject_fused_attention": false, "no_inject_fused_mlp": false, "no_use_cuda_fp16": false, "desc_act": false, "disable_exllama": false, "gpu_split": "", "max_seq_len": 2048, "cfg_cache": false, "deepspeed": false, "nvme_offload_dir": null, "local_rank": 0, "rwkv_strategy": null, "rwkv_cuda_on": false, "alpha_value": 1, "rope_freq_base": 0, "compress_pos_emb": 1, "listen": true, "listen_host": null, "listen_port": null, "share": false, "auto_launch": false, "gradio_auth": null, "gradio_auth_path": null, "ssl_keyfile": null, "ssl_certfile": null, "api": true, "api_blocking_port": 5000, "api_streaming_port": 5005, "public_api": false, "public_api_id": null, "multimodal_pipeline": null}}}
    "##;
    let response: ModelApiResponse = serde_json::from_str(json).unwrap();
    if let ModelApiResponseResult::Info(x) = response.result {
        assert_eq!(&x.model_name, "airoboros-c34b-2.1.Q4_K_M.gguf");
    } else {
        panic!();
    }
    }
    #[test]
    fn parse_list_response() {
        let json = r#"{"result": ["airoboros-c34b-2.1.Q4_K_M.gguf", "airoboros-l2-70b-2.1.Q4_K_M.gguf", "vicuna-33b-1.3-superhot-8k-GPTQ-4bit--1g.act.order.safetensors"]}"#;
        let response: ModelApiResponse = serde_json::from_str(json).unwrap();
        if let ModelApiResponseResult::List(x) = response.result {
            assert_eq!(x, vec![
                       "airoboros-c34b-2.1.Q4_K_M.gguf".to_string(),
                       "airoboros-l2-70b-2.1.Q4_K_M.gguf".to_string(),
                       "vicuna-33b-1.3-superhot-8k-GPTQ-4bit--1g.act.order.safetensors".to_string(),
            ]);
        };
    }
}




