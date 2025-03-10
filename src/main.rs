use std::io;
use std::io::Write;
use std::process;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct Input {
    files: Vec<File>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct File {
    name: String,
    url: String,
}

fn main() {
    let mut file_path = String::new();
    print!("Enter the path to the JSON file:");
    io::stdout().flush().expect("Unable to flush stdout");
    let _ = io::stdin().read_line(&mut file_path);
    if file_path.trim().is_empty() {
        println!("No file path provided, using default paths.json");
        file_path = String::from("paths.json");
    }

    let mut subject = String::new();
    print!("Subject: ");
    io::stdout().flush().expect("Unable to flush stdout");
    let _ = io::stdin().read_line(&mut subject);
    let mut subject = subject.trim();
    if subject.is_empty() {
        println!("No subject provided, using ./");
        subject = ".";
    }

    let raw_json_result = std::fs::read_to_string(file_path.trim());
    let mut raw_json = String::new();

    if let Ok(file) = raw_json_result {
        raw_json.push_str(&file);
    } else {
        eprintln!("Unable to read {file_path} file. Check if the file exists, and check if the filename is correct");
        return;
    }
    let parsed_json: Input = serde_json::from_str(&raw_json).expect("Unable to parse JSON");
    let _ = std::fs::create_dir(subject.trim());
    for file in parsed_json.files {
        let mut file_name = String::new();
        if std::env::consts::OS == "windows" {
            file_name.push_str(&format!("{}\\{}", subject, file.name));
        } else {
            file_name.push_str(&format!("{}/{}", subject, file.name));
        }
        curl(&file.url, &file_name);
    }
}

fn curl(url: &str, file_name: &str) {
    let mut child = process::Command::new("curl")
        .arg("-o")
        .arg(file_name)
        .arg(url)
        .spawn()
        .expect("Unable to spawn child process");
    let status = child.wait().expect("Unable to wait for child process");
    if !status.success() {
        println!("Failed to download file");
    }
}
