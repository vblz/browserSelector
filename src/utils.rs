fn file_in_exe_dir(file: &str) -> Result<PathBuf, Error> {
    let mut dir = env::current_exe()?;
    dir.pop();
    dir.push(file);
    Ok(dir)
}