use std::fmt;

use serde::{Deserialize, Serialize};
use serde_yaml;
use strum_macros::{Display, EnumString, ToString};

use super::esc_seq::EscSeq;

#[derive(Debug, Serialize, Deserialize)]
pub enum UptimeFormat {
    Days,
    Hours,
    Colon,
    None,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum MemoryFormatDisplay {
    Percent,
    Values,
    Both,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum MemoryFormatMetric {
    GB,
    MiB,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Display {
    pub should_display_image: bool,
    pub should_display_blocks: bool,
    pub should_display_text: bool,
    pub should_display_keys: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Alignment {
    pub space_before: u8,
    pub space_after: u8,
    pub lines_before: u8,
    pub lines_after: u8,
    pub hr_node_repeat: u8,
    pub word_wrap: u8,
    pub color_block_padding: u8,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BulletChar {
    pub hr_divider_node: String,
    pub default_bullet: String,
    pub interface_bullet: String,
    pub machine_bullet: String,
    pub shell_bullet: String,
    pub cpu_bullet: String,
    pub ram_bullet: String,
    pub uptime_bullet: String,
    pub date_bullet: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TextColor {
    pub user_string_color: EscSeq,
    pub machine_string_color: EscSeq,
    pub hr_color: EscSeq,
    pub detail_key_color: EscSeq,
    pub detail_value_color: EscSeq,
    pub text_description_color: EscSeq,
    pub default_bullet_color: EscSeq,
    pub interface_bullet_color: EscSeq,
    pub machine_bullet_color: EscSeq,
    pub shell_bullet_color: EscSeq,
    pub cpu_bullet_color: EscSeq,
    pub ram_bullet_color: EscSeq,
    pub uptime_bullet_color: EscSeq,
    pub date_bullet_color: EscSeq,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Override {
    pub interface_override: String,
    pub machine_override: String,
    pub shell_override: String,
    pub cpu_override: bool,
    pub ram_override: bool,
    pub uptime_override: bool,
    pub date_override: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Formats {
    pub date_format: String,
    pub uptime_format: UptimeFormat,
    pub cpu_memory_format: (MemoryFormatDisplay, MemoryFormatMetric),
    pub ram_memory_format: (MemoryFormatDisplay, MemoryFormatMetric),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Details {
    pub username: String,
    pub device_name: String,
    pub operating_sys_distro: String,
    pub machine_architecture: String,
    pub machine_shell: String,
    pub machine_shell_version: String,
    pub cpu_status: String,
    pub ram_status: String,
    pub system_uptime: String,
    pub current_date_time: String,
    pub welcome_message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub formats: Formats,
    pub displays: Display,
    pub alignments: Alignment,
    pub chars: BulletChar,
    pub colors: TextColor,
    pub overrides: Override,
    pub details: Details,
}
