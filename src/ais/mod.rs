// regio n: --- Modules
pub mod assts;
pub mod msg;

use crate::{Result, OPENAI_API_KEY};
use async_openai::config::OpenAIConfig;
use async_openai::Client;
// endregion: --- Modules

// region: --- Client

pub type OaClient = Client<OpenAIConfig>;

pub fn new_oa_client() -> Result<OaClient> {
        Ok(Client::new())
}

// endregion: ---Client
