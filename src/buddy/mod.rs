use std::path::PathBuf;
use derive_more::{Deref, From};
use serde::{Deserialize, Serialize};
use crate::ais::assts::{AsstId, ThreadId};
use crate::ais::OaClient;
use crate::buddy::config::Config;
use crate::Result;


mod config;

const BUDDY_TOML:&str = "buddy.toml";

pub struct Buddy {
    dir: PathBuf,
    oac: OaClient,
    asst_id :AsstId,
    config:Config
}


#[derive(Debug,From,Deref,Deserialize,Serialize)]
pub struct Conv {
    thread_id:ThreadId,

}

