use std::process::Command;
use std::error::Error;

mod settings;

fn open(url: &str) -> Result<(), Error> {
    let browser = choose_browser(url)?;

    open_in_browser(browser, url)
}

fn choose_browser(url: &str) -> Result<String, Error> {
    let settings = load_settings()?;

    for (_, value) in settings.browsers {
        for substr in value.words {
            if url.contains(&substr) {
                return Ok(value.path);
            }
        }
    }

    return Ok(settings.default);
}

fn open_in_browser(browser: String, url: &str) -> Result<(), Error> {
    Command::new(browser.into())
        .args(&[url])
        .status()
}