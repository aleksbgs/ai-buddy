// region --- Modules
mod error;
mod ais;
mod buddy;

use crate::ais::asst::CreateConfig;
use crate::ais::{asst, new_oa_client};
pub use self::error::{Error, Result};

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

async fn start()->Result<()>{
    let oac = new_oa_client()?;


    let asst_config = CreateConfig {
        name: "buddy-01".to_string(),
        model: "gpt-3.5-turbo-1106".to_string(),
    };

    let asst_id = asst::create(&oac,asst_config).await?;

    println!("->> asst_id: {asst_id:?}");
    Ok(())
}
