use std::io;

use anyhow::Result;
use clap::{Args, CommandFactory};
use clap_complete::Shell;
use fluent_templates::Loader;

use crate::{config::Config, LANG_ID, LOCALES};

#[must_use]
#[derive(Debug, Args)]
#[command(arg_required_else_help = true,
    about = LOCALES.lookup(&LANG_ID, "completions_command").expect("`completions_command` does not exists"))]
pub struct Completions {
    #[arg(value_enum,
        help = LOCALES.lookup(&LANG_ID, "shell").expect("`shell` does not exists"))]
    pub shell: Shell,
}

pub fn execute(config: Completions) -> Result<()> {
    let mut cmd = Config::command();
    let bin_name = cmd.get_name().to_string();

    clap_complete::generate(config.shell, &mut cmd, bin_name, &mut io::stdout());

    Ok(())
}
