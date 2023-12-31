// region: --- Modules
pub mod asst;

use async_openai::Client;
use async_openai::config::OpenAIConfig;
use crate::{OPENAI_API_KEY, Result};
// endregion: --- Modules

// region: --- Client


pub type OaClient = Client<OpenAIConfig>;

pub fn new_oa_client() ->Result<OaClient> {
    if OPENAI_API_KEY.len() > 0 {
        Ok(Client::new())
    } else {
        println!("No {OPENAI_API_KEY} env variable, Please set it.");
        Err("No openai api key in env".into())
    }
}

// endregion: ---Client

