//! Observe OpenAI provider-native hosted tool events during streaming.
//!
//! Provider-hosted tools such as web search are executed by OpenAI, not by
//! Rig's function tool executor. Rig surfaces their lifecycle as opaque
//! provider events so applications can render progress while preserving the
//! raw provider payload.
//!
//! `OPENAI_API_KEY=... cargo run --example openai_provider_events`

use futures::StreamExt;
use rig::client::{CompletionClient, ProviderClient};
use rig::providers::openai;
use rig::streaming::{StreamedAssistantContent, StreamingPrompt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = openai::Client::from_env()?;
    let agent = client
        .agent(openai::GPT_4O_MINI)
        .additional_params(serde_json::json!({
            "tools": [{ "type": "web_search" }]
        }))
        .build();

    let mut stream = agent
        .stream_prompt("Find one current source about Rust async streams.")
        .await;

    while let Some(choice) = stream.next().await {
        match choice? {
            StreamedAssistantContent::ProviderEvent(event) if event.provider == "openai" => {
                tracing::debug!(
                    event_type = %event.event_type,
                    raw = ?event.raw,
                    "provider-native hosted tool event"
                );
            }
            StreamedAssistantContent::Text(text) => print!("{}", text.text),
            StreamedAssistantContent::Final(_) => println!(),
            _ => {}
        }
    }

    Ok(())
}
