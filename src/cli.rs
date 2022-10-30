use std::str::FromStr;

use clap::{Parser, Subcommand};

use crate::config::KeyValue;

/// Diff two http requests and compare the difference of the responses
#[derive(Parser, Debug, Clone)]
#[clap(version,author,about,long_about = None)]
pub struct Args {
    #[clap(subcommand)]
    pub action: Action,
}

#[derive(Subcommand, Debug, Clone)]
#[non_exhaustive] // 表明未来还有其它元素添加
pub enum Action {
    /// Diff tow API responses base on given profile
    Run(RunArgs),
    /// Parse URLs to generate profile.
    Parse,
}

#[derive(Parser, Debug, Clone)]
pub struct RunArgs {
    // profile name want to use.
    #[clap(short, long, value_parser)]
    pub profile: String,
    /// Overrides args. override the query of the request.
    /// For query params. use `-q key=value`
    #[clap(short, long, value_parser = KeyValue::from_str, number_of_values = 1)]
    pub query: Vec<KeyValue>,
    /// Overrides args. override the headers of the request.
    /// For headers. use `-d key=value`
    #[clap(short = 'd', long, value_parser = KeyValue::from_str, number_of_values = 1)]
    pub header: Vec<KeyValue>,
    /// Overrides args. override the body of the request.
    /// For body. use `-b @key=value`
    #[clap(short, long, value_parser = KeyValue::from_str, number_of_values = 1)]
    pub body: Vec<KeyValue>,
    // COnfiguration file to use.
    #[clap(short, long, value_parser)]
    pub config: Option<String>,
}
