use std::cmp::min;
use std::io;
use std::io::Write;
use std::process;
use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct Input {
    files: Vec<File>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
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
    let num_files = parsed_json.files.len();
    let mut handles = vec![];
    let num_threads = min(num_files, 4);
    let files_per_thread = (num_files + num_threads - 1).div_ceil(num_threads); // Ceiling division

    let subject = Arc::new(subject.to_string());
    let files = Arc::new(parsed_json.files);

    for i in 0..num_threads {
        let subject = Arc::clone(&subject);
        let files = Arc::clone(&files);

        let handle = thread::spawn(move || {
            let start = i * files_per_thread;
            let end = min((i + 1) * files_per_thread, num_files);

            for j in start..end {
                if let Some(file) = files.get(j) {
                    let file_name = if std::env::consts::OS == "windows" {
                        format!("{}\\{}", subject, file.name)
                    } else {
                        format!("{}/{}", subject, file.name)
                    };

                    curl(&file.url, &file_name);
                }
            }
        });

        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
}

fn curl(url: &str, file_name: &str) {
    println!("\nDownloading {}", file_name);
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
    println!("\n")
}
