

#[derive(Deserialize)]
struct Browser {
    path: String,
    words: Vec<String>,
}

#[derive(Deserialize)]
struct Settings {
    browsers: HashMap<String, Browser>,
    default: String,
}

fn load_settings() -> Result<Settings, Error> {
    let file_path = file_in_exe_dir("settings.toml")?;
    let settings_content = fs::read_to_string(file_path)?;

    toml::from_str(settings_content.into())
}