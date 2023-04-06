use std::process::{Command, Output};

pub fn get_shell_version(shell: String) -> Option<String> {
    let output: Output = Command::new(shell).arg("--version").output().ok()?;
    let version: String = extract_version_from_output(output).ok()?;
    Some(version)
}

fn extract_version_from_output(output: Output) -> Result<String, String> {
    let output_str: String =
        String::from_utf8(output.stdout).map_err(|_| "Invalid UTF-8".to_string())?;
    let output_lines: Vec<&str> = output_str.split('\n').collect();
    if let Some(first_line) = output_lines.get(0) {
        Ok(first_line.to_string())
    } else {
        Err("Output is empty".to_string())
    }
}
