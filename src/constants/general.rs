use serde::{Deserialize, Serialize};
use serde_yaml;
use strum_macros::Display;

#[derive(Debug, Serialize, Deserialize, Display, PartialEq)]
pub enum BlockType {
    Light,
    Normal
}