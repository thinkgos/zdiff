use anyhow::{anyhow, Result};
use clap::Parser;
use dialoguer::{theme::ColorfulTheme, Input, MultiSelect};
use std::io::Write;

use zdiff::cli::{Action, Args, RunArgs};
use zdiff::config::{DiffConfig, ExtraArgs, Profile, RequestProfile, ResponseProfile};
use zdiff::{diff, highlight_text};

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    match args.action {
        Action::Run(args) => run(args).await?,
        Action::Parse => parse()?,
        _ => panic!("Not implemented"),
    }

    Ok(())
}

async fn run(args: RunArgs) -> Result<()> {
    let config_file = args.config.unwrap_or_else(|| "./zdiff.yml".to_string());
    let config = DiffConfig::load_yaml(&config_file).await?;

    let profile = config.get_profile(&args.profile).ok_or_else(|| {
        anyhow!(
            "Profile {} not found in config file {}",
            args.profile,
            config_file
        )
    })?;

    let extra_args = ExtraArgs {
        headers: args.header.into(),
        query: args.query.into(),
        body: args.body.into(),
    };
    let diff_str = diff(profile, extra_args).await?;

    println!("{}", diff_str);
    Ok(())
}

fn parse() -> Result<()> {
    let url1: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("url1")
        .interact_text()?;
    let url2: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("url2")
        .interact_text()?;
    let profile: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("profile")
        .interact_text()?;

    let headers = [
        "date",
        "x-ratelimit-limit",
        "x-ratelimit-remaining",
        "x-ratelimit-reset",
        "vary",
        "cache-control",
        "expires",
        "etag",
        "via",
        "cf-cache-status",
        "expect-ct",
        "report-to",
        "cf-ray",
    ];
    let chosen = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select headers to skip")
        .items(&headers)
        .interact()?;

    let skip_headers = chosen.iter().map(|v| headers[*v].to_string()).collect();

    let req1: RequestProfile = url1.parse()?;
    let req2: RequestProfile = url2.parse()?;
    let res = ResponseProfile::new(skip_headers, vec![]);
    let diff_profile = Profile::new(req1, req2, Some(res));
    let config = DiffConfig::new(&profile, diff_profile);

    let result = serde_yaml::to_string(&config)?;
    let result = highlight_text(&result, "yaml")?;

    let stdout = std::io::stdout();
    let mut stdout = stdout.lock();
    write!(stdout, "---\n{}", result)?;
    Ok(())
}
