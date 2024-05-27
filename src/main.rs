use std::env;
use std::fs::{self,File};
use std::io::{self,Write};
use std::path::Path;
use std::process::Command;
use tempfile::NamedTempFile;
use chrono::Local;
use serde::{Deserialize};
use serde_json::from_str;
use std::fs::read_to_string;

#[derive(Deserialize)]
struct Config{
    editor: String,
    til_folder: String,
    repo_path: String,
}
const APP_NAME: &str = "tiller";

fn main() -> io::Result<()> {
    let config_path = dirs::home_dir()
        .expect("failed to find home dir")
        .join(format!(".config/{}/config.json",APP_NAME))
        ;
    let config_data = read_to_string(config_path)?;
    let config:Config = from_str(&config_data).expect("Invalid config format");

    let Config { editor, til_folder, repo_path } = config;
    let full_til_path = format!("{}/{}",repo_path,til_folder);

    let mut temp_file = NamedTempFile::new()?;
    let temp_path = temp_file.path().to_str().unwrap().to_string();

    Command::new(editor)
        .arg(&temp_path)
        .status()
        .expect("Failed to open editor");

    let input_content = read_to_string(temp_path)?;
    println!("Content is {}",&input_content);

    let new_file_name = get_next_file_name(&full_til_path)?;
    println!("New file name is {}", &new_file_name);

    let new_file_path = format!("{}/{}", full_til_path, new_file_name);
    println!("New file path is {}",&new_file_path);

    let prepend_path = dirs::home_dir()
        .expect("Failed to find home directory")
        .join(format!(".config/{}/prepend.md",APP_NAME));

    let prepend_template = read_to_string(prepend_path)?;
    let prepend_content = update_prepend_content(&prepend_template,&new_file_name)?;
    println!("Prepend content is {}",&prepend_content);
    println!("File is creating: {}",&new_file_path);

    let mut new_file = File::create(&new_file_path)?;
    write!(new_file,"{} \n {}", prepend_content, input_content)?;

    Command::new("git")
        .args(&["add", &new_file_path])
        .current_dir(&repo_path)
        .status()
        .expect("Failed to add git");

    Command::new("git")
        .args(&["commit","-m", &format!("{} created",new_file_name)])
        .current_dir(&repo_path)
        .status()
        .expect("Failed to commit");

    Command::new("git")
        .args(&["push","origin","master"])
        .current_dir(&repo_path)
        .status()
        .expect("Failed to push");

    Ok(())

}

fn get_next_file_name(folder: &str) -> io::Result<String> {
    let paths = fs::read_dir(folder)?
        .filter_map(Result::ok)
        .filter(|e| e.path().extension().and_then(|s| s.to_str())==Some("md"))
        .collect::<Vec<_>>();
    let max_num = paths.iter()
        .filter_map(|e| e.path().file_stem().and_then(|s| s.to_str()).and_then(|s| s.parse::<u8>().ok()))
        .max()
        .unwrap_or(0);
    Ok(format!("{:02}.md",max_num + 1))
}

fn update_prepend_content(template: &str, filename: &str) -> io::Result<String> {
    let date = Local::now().format("%Y-%m-%dT%H:%M:%S").to_string();
    let filename_wo_ext = filename.replace(".md","");
    let content = template
        .replace("$TITLE",&filename_wo_ext)
        .replace("$DATE",&date);
    Ok(content)
}
