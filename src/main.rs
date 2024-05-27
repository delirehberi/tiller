use std::fs::{self,File};
use std::io::{self,Write};
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
const APP_VERSION: &str = "v0.0.1";

fn main() -> io::Result<()> {
    
    let args: Vec<String> = std::env::args().collect();
    check_args(&args)?;

    let config = config()?;

    let Config { editor, til_folder, repo_path } = config;
    let full_til_path = format!("{}/{}",repo_path,til_folder);

    let input_content = get_content(editor)?;
    if input_content.len() < 1 {
        println!("You can't save an empty til!");
        std::process::exit(1)
    }

    let new_file_name = get_next_file_name(&full_til_path)?;

    let new_file_path = format!("{}/{}", full_til_path, new_file_name);

    let prepend_path = dirs::home_dir()
        .expect("Failed to find home directory")
        .join(format!(".config/{}/prepend.md",APP_NAME));

    let prepend_template = read_to_string(prepend_path)?;
    let prepend_content = update_prepend_content(&prepend_template,&new_file_name)?;

    let mut new_file = File::create(&new_file_path)?;
    write!(new_file,"{} \n {}", prepend_content, input_content)?;

    publish(&new_file_path,&repo_path,&new_file_name)
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

fn publish(new_file_path: &str,repo_path: &str,new_file_name: &str) -> io::Result<()>{
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
fn config()->io::Result<Config>{

    let config_path = dirs::home_dir()
        .expect("failed to find home dir")
        .join(format!(".config/{}/config.json",APP_NAME))
        ;
    let config_data = read_to_string(config_path)?;
    let config:Config = from_str(&config_data).expect("Invalid config format");

    Ok(config)
}

fn get_content(editor: String) -> io::Result<String>{
    let temp_file = NamedTempFile::new()?;
    let temp_path = temp_file.path().to_str().unwrap().to_string();

    Command::new(editor)
        .arg(&temp_path)
        .status()
        .expect("Failed to open editor");

    read_to_string(temp_path)
}

fn check_args(args: &[String]) ->io::Result<()> {
        match args.get(1) {
            Some(t) => run(&t, &args[2..]),
            _ => Ok(())
        }
}

fn run(command: &str, args: &[String])-> io::Result<()>{
    match command {
        "--version" => println!("{} Version {}",APP_NAME, APP_VERSION),
        _ => println!("Command is not registered {}",command)
    }

    std::process::exit(1)
}
