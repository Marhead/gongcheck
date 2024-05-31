use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct MarkdownFile {
    pub path: Option<String>,
    pub contents: String
}

#[derive(Deserialize, Serialize)]
pub struct MarkdownPath {
    pub path: String
}

pub const DOCS_KEY: &'static str = "gongcheck_about.md";
pub const DOCS_STR: &'static str = r#"# Gongcheck

Gongcheck is a free and [open-source](https://github.com/Marehad) markdown editor.

Here needs testing text in here"#;