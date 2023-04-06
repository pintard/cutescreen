use std::{
    cmp,
    env::{consts, current_dir, var},
    fs::{read_to_string, File},
    path::PathBuf,
    process::id,
    str::SplitWhitespace,
};

use chrono::{DateTime, Local};
use psutil::memory::{self, VirtualMemory};
use whoami;

use crate::{
    constants::{
        config::{Alignment, Config, Details, Display, Formats, TextColor},
        esc_seq::EscSeq::{self, End},
        general::BlockType,
    },
    utils::get_shell_version::get_shell_version,
};

pub fn base_command() {
    assemble_base();
}

fn assemble_base() -> Vec<String> {
    let mut base_details: Vec<String> = Vec::new();

    /* Load yaml config */
    let config_path: PathBuf = current_dir().unwrap().join("src/configs/.config.yaml");
    let config_file: File = File::open(config_path).expect("Unable to open file");
    let config: Config = serde_yaml::from_reader(config_file).expect("Unable to read values");

    /* Deconstruct Config preferences */
    let Display {
        should_display_text,
        should_display_blocks,
        should_display_image,
        should_display_keys,
    } = config.displays;

    let Alignment {
        space_before,
        space_after,
        hr_node_repeat,
        lines_before,
        lines_after,
        word_wrap,
        color_block_padding,
    } = config.alignments;

    let TextColor {
        user_string_color,
        machine_string_color,
        hr_color,
        detail_key_color,
        detail_value_color,
        text_description_color,
        default_bullet_color,
        interface_bullet_color,
        machine_bullet_color,
        shell_bullet_color,
        cpu_bullet_color,
        ram_bullet_color,
        uptime_bullet_color,
        date_bullet_color,
    } = config.colors;

    let Formats { date_format, .. } = config.formats;

    let Details {
        welcome_message, ..
    } = config.details;

    /* Output assembly functions */

    /**
     * Takes a color block type and stringifies the ASCII color background grid
     * @return String the color blocks
     */
    let assemble_blocks = |block_type: BlockType| -> String {
        let mut blocks = String::new();
        let x: u8 = if block_type == BlockType::Light {
            10
        } else {
            4
        };

        for i in 0..9 {
            let color: String = format!("\x1b[{}{}m", x, i);
            let padding: String = " ".repeat(color_block_padding.into());
            let end_seq: String = End.to_string();
            let block: String = color + &padding + &end_seq;
            blocks.push_str(&block);
        }

        println!("{}", blocks);
        blocks
    };

    /**
     * Takes welcome message from supplied configuration object and wrap the lines
     * at a word_wrap rate also found in the config object
     * @return Vec<String> the welcome message split into multiple strings in a vector
     */
    let assemble_message = || -> Vec<String> {
        trait TextHandling {
            fn split_at_closest(&self, idx: u8) -> (String, String);
        }

        impl TextHandling for String {
            /**
             * Splits a string at the closest space given a target index
             */
            fn split_at_closest(&self, idx: u8) -> (String, String) {
                let mut str: String = String::from(&self.clone());
                let mut aggr: String = String::new();
                if str.len() as u8 > idx {
                    while (aggr.len() as u8) < idx {
                        let mut iter: SplitWhitespace = str.split_whitespace();
                        let word: &str = iter.next().unwrap();
                        let chunk: &str = &(word.to_owned() + " ");
                        aggr.push_str(chunk);
                        str = iter.map(|x| x.to_owned() + " ").collect::<String>();
                    }
                } else {
                    aggr = str;
                    str = String::from("");
                }
                (aggr, str)
            }
        }

        let mut wrapped_text: Vec<String> = vec![];
        let mut text: String = String::from(welcome_message);
        let word_wrap: usize = config.alignments.word_wrap.into();

        if text.len() > word_wrap {
            while !text.is_empty() {
                let (chunk, rest) = text.split_at_closest(cmp::min(word_wrap, text.len()) as u8);
                wrapped_text.push(chunk.to_string());
                text = rest.to_string();
            }
        } else {
            wrapped_text = Vec::from([text]);
        }

        println!("{:?}", wrapped_text);
        wrapped_text
    };

    let assemble_details = || -> Vec<String> {
        let mut details: Vec<String> = Vec::new();

        // OS User and machine name
        let username: String = whoami::username();
        let device_name: String = whoami::devicename();
        println!("{} {}", username, device_name);

        // Operating system and version
        let operating_sys_distro: String = whoami::distro();
        println!("{}", operating_sys_distro);

        // Machine type / architecture
        let machine_architecture: String = consts::ARCH.to_string();
        println!("{}", machine_architecture);

        // Shell emulator name and version
        let machine_shell: String = var("SHELL")
            .unwrap()
            .split("/")
            .last()
            .unwrap_or_else(|| "BASH")
            .to_uppercase();
        let machine_shell_version =
            get_shell_version(machine_shell).unwrap_or_else(|| "unknown".to_string());

        println!("{}", machine_shell_version);

        // Ram usage in percent or value
        // let virtual_memory: VirtualMemory = memory::virtual_memory().ok().unwrap();
        // let memory_used = virtual_memory.used();
        // let memory_total = virtual_memory.available();
        // let memory_percentage = virtual_memory.percent();

        // let memory_percent_test = (memory_used as f64 / memory_total as f64) * 100 as f64;

        // let memory_values: String = format!(
        //     "used:{}\ntotal:{}\npercent:{}\npercent_test:{}",
        //     memory_used, memory_total, memory_percentage, memory_percent_test
        // );

        // println!("{}", memory_values,);

        // CPU usage in percent or value

        // System uptime
        // let system_uptime =

        // let boot_time = boot_time().unwrap();
        // let uptime = time::now().duration_since(boot_time).unwrap();
        // println!("System uptime: {}", uptime);

        // Calendar day
        let curr_time: DateTime<Local> = Local::now();
        let formatted_curr_time: String = curr_time.format(&date_format).to_string();
        println!("{}", formatted_curr_time);

        details
    };

    let color_string = |color: EscSeq, str: &str| -> String { format!("{}{}{}", color, str, End) };

    /* Match details to see if should be pushed to details list, from yaml map using config struct */
    assemble_blocks(BlockType::Normal);
    assemble_blocks(BlockType::Light);
    assemble_details();
    assemble_message();

    println!("{}", color_string(user_string_color, "Hello"));

    base_details
}
