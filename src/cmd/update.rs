use std::env;

use anyhow::{Ok, Result};
use clap::Args;
use fluent_templates::Loader;
use novel_api::Timing;
use self_update::{backends::github, cargo_crate_version};
use tokio::task;
use tracing::info;
use url::Url;

use crate::{LANG_ID, LOCALES};

#[must_use]
#[derive(Args)]
#[command(about = LOCALES.lookup(&LANG_ID, "update_command").unwrap())]
pub struct Update {
    #[arg(long, num_args = 0..=1, default_missing_value = super::PROXY,
        help = LOCALES.lookup(&LANG_ID, "proxy").unwrap())]
    pub proxy: Option<Url>,
}

pub async fn execute(config: Update) -> Result<()> {
    let mut timing = Timing::new();

    if let Some(proxy) = config.proxy {
        env::set_var("HTTP_PROXY", proxy.to_string());
        env::set_var("HTTPS_PROXY", proxy.to_string());
    }

    task::spawn_blocking(move || {
        github::Update::configure()
            .repo_owner("novel-rs")
            .repo_name("cli")
            .bin_name(env!("CARGO_PKG_NAME"))
            .show_download_progress(true)
            .current_version(cargo_crate_version!())
            .build()?
            .update()?;

        Ok(())
    })
    .await??;

    info!("Time spent on `check`: {}", timing.elapsed()?);

    Ok(())
}
