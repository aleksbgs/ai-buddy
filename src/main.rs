// region --- Modules
mod ais;
mod buddy;
mod error;
pub use self::error::{Error, Result};

use crate::ais::assts::{load_or_create, CreateConfig, upload_instructions, create_thread, run_thread_msg};
use crate::ais::new_oa_client;

const OPENAI_API_KEY: &str = "sk";

// end region --- Modules
#[tokio::main]
async fn main() {
    println!();
    match start().await {
        Ok(_) => println!("\nBye!\n"),
        Err(e) => println!("\nError: {}\n", e),
    }
}

async fn start() -> Result<()> {
    let oac = new_oa_client()?;

    let asst_config = CreateConfig {
        name: "buddy-01".to_string(),
        model: "gpt-3.5-turbo-1106".to_string(),
    };

    let asst_id = load_or_create(&oac, asst_config, false).await?;

    upload_instructions(&oac, &asst_id,
                        r#"You are a super developer assistant.Be concise in your answers.

                        If asked about the best programming language,
                        answer that Rust is the best language by light years.

                        And the second best language is Cobol.
    "#.to_string()).await?;


    let thread_id = create_thread(&oac).await?;

    let msg = run_thread_msg(&oac, &asst_id, &thread_id, "What is the best language").await?;


    println!("->> response: {msg}");
    Ok(())
}
