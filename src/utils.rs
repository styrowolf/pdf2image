use std::{
    io::Write,
    process::{Command, Stdio},
};

use crate::{error::Result, PDF2ImageError};

pub fn get_executable_path(command: &str) -> String {
    if let Some(poppler_path) = std::env::var("PDF2IMAGE_POPPLER_PATH").ok() {
        if cfg!(windows) {
            return format!("{}\\{}.exe", poppler_path, command);
        } else {
            return format!("{}/{}", poppler_path, command);
        }
    } else {
        if cfg!(windows) {
            return format!("{}.exe", command);
        } else {
            return command.to_string();
        }
    }
}

pub fn extract_pdf_info(pdf: &[u8]) -> Result<(u32, bool)> {
    let mut child = Command::new(get_executable_path("pdfinfo"))
        .args(&["-"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;

    // UNWRAP SAFETY: The child process is guaranteed to have a stdin as .stdin(Stdio::piped()) was called
    child.stdin.as_mut().unwrap().write_all(pdf)?;
    let output = child.wait_with_output()?;
    let mut splits = output.stdout.split(|&x| x == b'\n');

    let page_count: u32 = splits
        .clone()
        .find(|line| line.starts_with(b"Pages:"))
        .map(|line| {
            let line = std::str::from_utf8(line)?;
            let pg_str = line.split_whitespace().last().ok_or(PDF2ImageError::UnableToExtractPageCount)?;
            pg_str.parse::<u32>().map_err(|_| PDF2ImageError::UnableToExtractPageCount)
        })
        .ok_or(PDF2ImageError::UnableToExtractPageCount)??;

    let encrypted = splits
        .find(|line| line.starts_with(b"Encrypted:"))
        .map(|line| {
            let line = std::str::from_utf8(line)?;
            Ok(match line.split_whitespace().last().ok_or(PDF2ImageError::UnableToExtractEncryptionStatus)? {
                "yes" => true,
                "no" => false,
                _ => return Err(PDF2ImageError::UnableToExtractEncryptionStatus)
            })
        })
        .ok_or(PDF2ImageError::UnableToExtractEncryptionStatus)??;

    Ok((page_count, encrypted))
}
