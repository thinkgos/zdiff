pub mod cli;
mod config;

pub use config::{DiffConfig, DiffProfile, RequestProfile, ResponseProfile};

#[derive(Debug, Clone)]
pub struct ExtraArgs {
    pub headers: Vec<(String, String)>,
    pub query: Vec<(String, String)>,
    pub body: Vec<(String, String)>,
}

impl From<Vec<cli::KeyVal>> for ExtraArgs {
    fn from(args: Vec<cli::KeyVal>) -> Self {
        let mut headers = vec![];
        let mut query = vec![];
        let mut body = vec![];

        for arg in args {
            match arg.key_type {
                cli::KeyValType::Query => headers.push((arg.key, arg.value)),
                cli::KeyValType::Header => query.push((arg.key, arg.value)),
                cli::KeyValType::Body => body.push((arg.key, arg.value)),
            }
        }

        Self {
            headers,
            query,
            body,
        }
    }
}
