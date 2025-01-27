pub mod cmd;
pub mod config;
pub mod renderer;
pub mod utils;

use fluent_templates::static_loader;
use once_cell::sync::Lazy;
use unic_langid::LanguageIdentifier;

static_loader! {
    static LOCALES = {
        locales: "./locales",
        fallback_language: "en-US",
    };
}

pub static LANG_ID: Lazy<LanguageIdentifier> = Lazy::new(|| {
    let locale = sys_locale::get_locale().unwrap_or_else(|| {
        eprintln!("Failed to get active locale for the system, use `en-US`");
        String::from("en-US")
    });

    match locale.parse::<LanguageIdentifier>() {
        Ok(lang_id) => lang_id,
        Err(error) => {
            eprintln!("Failed to parse LanguageIdentifier: {error}, use `en-US`");
            "en-US".parse::<LanguageIdentifier>().unwrap()
        }
    }
});
