use std::{
    io::Write,
    process::{Command, Stdio},
};

use crate::error::Result;

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

    child.stdin.as_mut().unwrap().write_all(pdf)?;
    let output = child.wait_with_output().unwrap();
    let mut splits = output.stdout.split(|&x| x == b'\n');

    let page_count = splits
        .clone()
        .find(|line| line.starts_with(b"Pages:"))
        .map(|line| {
            let line = std::str::from_utf8(line).unwrap();
            line.split_whitespace().last().unwrap().parse().unwrap()
        })
        .unwrap();

    let encrypted = splits
        .find(|line| line.starts_with(b"Encrypted:"))
        .map(|line| {
            let line = std::str::from_utf8(line).unwrap();
            match line.split_whitespace().last().unwrap() {
                "yes" => true,
                "no" => false,
                _ => panic!("Unexpected value for Encrypted: {}", line),
            }
        })
        .unwrap();

    Ok((page_count, encrypted))
}
