#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use openai_api_rs::v1::api::Client;
use openai_api_rs::v1::chat_completion::{self, ChatCompletionRequest};

use std::fs::{File, remove_file};
use std::io::Write;

use std::process::Command;

const TEMP_FILE: &'static str = "_temp.md";
const MODEL: &'static str = "gpt-3.5-turbo";

#[tauri::command]
async fn generate_notebook(path: &str, description: &str, client: tauri::State<'_, Client>) -> Result<bool, ()> {

    let content = generate_notebook_content(description, client)
        .await
        .unwrap()
        .replace("```python", "```code");

    {
        let mut file = File::create(TEMP_FILE).unwrap();
        file.write_all(content.as_bytes()).unwrap();
    }

    Command::new("pandoc")
        .args(&[TEMP_FILE, "-o", path])
        .spawn()
        .unwrap()
        .wait()
        .expect("Failed to use pandoc to convert markdown to notebook");

    remove_file(TEMP_FILE).ok();

    Ok(true)
}

async fn generate_notebook_content(description: &str, client: tauri::State<'_, Client>) -> Result<String, ()> {

    let model = String::from(MODEL);

    let prompt = format!(
        "You have been integrated into a software application as an advanced large language model \
        (LLM). Your primary function is to autonomously generate content for a Jupyter Notebook using \
        the widely accepted Pandoc Markdown syntax (include Python code). You will produce high-quality, \
        structured Markdown content adhering to the specific conventions and standards of the Pandoc \
        format, facilitating seamless integration into Jupyter Notebooks. The description of the \
        notebooks is {description}."
    );

    let req = ChatCompletionRequest {
        model,
        messages: vec![chat_completion::ChatCompletionMessage {
            role: chat_completion::MessageRole::user,
            content: prompt.to_string(),
        }],
    };

    let result = client.chat_completion(req).await.unwrap();
    let response = result.choices[0].message.content.to_string();

    Ok(response)
}

fn main() {
    let api_token = std::env::var("OPENAI_TOKEN").expect("OPENAI_TOKEN not set");

    tauri::Builder::default()
        .manage(Client::new(api_token.to_string()))
        .invoke_handler(tauri::generate_handler![generate_notebook])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
