use std::str::FromStr;

use clap::{Parser, Subcommand};

use crate::config::KeyVal;

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
    /// Diff  tow API responses base on given profile
    Run(RunArgs),
    /// Parse URLs to generate profile.
    Parse,
}

#[derive(Parser, Debug, Clone)]
pub struct RunArgs {
    #[clap(short, long, value_parser)]
    pub profile: String,

    /// Overrides args. override the query, headers and b ody of the request.
    /// For query params. use `-e key=value`
    /// For headers. use `-e %key=value`
    /// For body. use `-e @key=value`
    #[clap(short, long, value_parser = KeyVal::from_str, number_of_values = 1)]
    pub query: Vec<KeyVal>,
    #[clap(short = 'u', long, value_parser = KeyVal::from_str, number_of_values = 1)]
    pub header: Vec<KeyVal>,
    #[clap(short, long, value_parser = KeyVal::from_str, number_of_values = 1)]
    pub body: Vec<KeyVal>,
    // COnfiguration to use.
    #[clap(short, long, value_parser)]
    pub config: Option<String>,
}
