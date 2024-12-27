use anyhow::Result;
use std::fs::canonicalize;
use webbrowser;

pub struct Bitchat;

impl Bitchat {
  pub fn run() -> Result<Option<Box<dyn super::Output>>> {
    // Path to the HTML file
    let html_path = "static/bitchat.html";

    // Resolve the absolute path of the file
    let absolute_path = canonicalize(html_path)?.to_str().unwrap().to_string();

    // Attempt to open the file in the default web browser
    if webbrowser::open(&absolute_path).is_ok() {
      println!("Bitchat opened successfully in your default browser.");
    } else {
      eprintln!("Failed to open Bitchat in your browser.");
    }

    // No output to return
    Ok(None)
  }
}
