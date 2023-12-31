use async_openai::types::{AssistantToolsRetrieval, CreateAssistantRequest};
use derive_more::{Deref, Display, From};
use crate::ais::OaClient;
use crate::Result;

// region: ---Types

pub struct CreateConfig{
    pub name:String,
    pub model:String,
}

#[derive(Debug,From,Deref,Display)]
pub struct AsstId(String);

#[derive(Debug,From,Deref,Display)]
pub struct ThreadId(String);

#[derive(Debug,From,Deref,Display)]
pub struct FileId(String);

// endregion: --- Types

pub async fn create(oac:&OaClient,config:CreateConfig) -> Result<AsstId> {
    let oa_assts = oac.assistants();

    let asst_obj = oa_assts
        .create(CreateAssistantRequest {
            model: config.model,
            name: Some(config.name),
            tools: Some(vec![AssistantToolsRetrieval::default().into()]),
            ..Default::default()
        })
        .await?;
    Ok(asst_obj.id.into())
}
