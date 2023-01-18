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
    /// 根据所给的 profile 比较两个 API 的回复
    Run(RunArgs),
    /// 解析 URLs 生成 profile.
    Parse,
}

#[derive(Parser, Debug, Clone)]
pub struct RunArgs {
    /// 使用的profile名称.
    #[clap(short, long, value_parser)]
    pub profile: String,
    /// 覆盖参数, 覆盖请求的 query.
    /// For query params. use `-q key=value`
    #[clap(short, long, value_parser = KeyValue::from_str, number_of_values = 1)]
    pub query: Vec<KeyValue>,
    /// 覆盖参数, 覆盖请求的 header.
    /// For headers. use `-d key=value`
    #[clap(short = 'd', long, value_parser = KeyValue::from_str, number_of_values = 1)]
    pub header: Vec<KeyValue>,
    /// 覆盖参数, 覆盖请求的 body.
    /// For body. use `-b @key=value`
    #[clap(short, long, value_parser = KeyValue::from_str, number_of_values = 1)]
    pub body: Vec<KeyValue>,
    // 使用的配置文件.
    #[clap(short, long, value_parser)]
    pub config: Option<String>,
}
