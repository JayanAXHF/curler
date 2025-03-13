use clap::{Parser, ValueEnum};
use std::cmp::min;
use std::io::{BufRead, BufReader, Write};
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
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// The subject, or directory, to download the files to
    #[clap(value_name = "SUBJECT")]
    subject: Option<String>,

    /// The path to the JSON file containing the URLs to download
    #[arg(short, long)]
    file_path: Option<String>,

    /// Maximun number of threads to use
    #[arg(short, long, default_value_t = 4)]
    max_threads: usize,

    #[arg(value_enum, long, default_value_t = Mode::Json)]
    mode: Mode,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum Mode {
    /// Parse JSON file
    Json,
    /// Parse text file
    Text,
}

fn main() {
    let args = Args::parse();
    let stdout_mutex = Arc::new(Mutex::new(()));
    let subject = match args.subject {
        Some(subject) => subject,
        None => ".".to_string(),
    };
    let file_path = if let Some(file_path) = args.file_path {
        file_path
    } else {
        match args.mode {
            Mode::Json => "paths.json".to_string(),
            Mode::Text => "paths.txt".to_string(),
        }
    };

    let max_threads = args.max_threads;

    let raw_content_result = std::fs::read_to_string(file_path.trim());
    let mut raw_content = String::new();

    if let Ok(file) = raw_content_result {
        raw_content.push_str(&file);
    } else {
        eprintln!("Unable to read {file_path} file. Check if the file exists, and check if the filename is correct");
        print!("Press any key to exit...");
        std::io::stdout().flush().expect("Unable to flush stdout");
        std::io::stdin()
            .read_line(&mut String::new())
            .expect("Unable to read line");
        return;
    }

    // Parses the file into a Vec<File>
    let files: Vec<File> = match args.mode {
        Mode::Json => json_parser(&raw_content),
        Mode::Text => text_parser(&raw_content),
    };
    // Creates the subject directory if it doesn't exist
    let _ = std::fs::create_dir(subject.trim());
    let num_files = files.len();
    let mut handles = vec![];
    let num_threads = min(num_files, max_threads);

    // Divides the files into equal parts for each thread.
    let files_per_thread = (num_files + num_threads - 1).div_ceil(num_threads); // Ceiling division

    let subject = Arc::new(subject.to_string());
    let files = Arc::new(files);

    for i in 0..num_threads {
        let subject = Arc::clone(&subject);
        let files = Arc::clone(&files);
        let stdout_mutex = Arc::clone(&stdout_mutex);

        let handle = thread::spawn(move || {
            let start = i * files_per_thread;
            let end = min((i + 1) * files_per_thread, num_files);

            for j in start..end {
                if let Some(file) = files.get(j) {
                    // Compatibity with the wierd windows path syntax
                    let file_name = if std::env::consts::OS == "windows" {
                        format!("{}\\{}", subject, file.name)
                    } else {
                        format!("{}/{}", subject, file.name)
                    };

                    curl(&file.url, &file_name, &stdout_mutex, i);
                }
            }
        });

        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    print!("Finished downloading files. Press any key to exit...");
    std::io::stdout().flush().expect("Unable to flush stdout");
    std::io::stdin()
        .read_line(&mut String::new())
        .expect("Unable to read line");
}

use std::process::{Command, Stdio};

fn curl(url: &str, file_name: &str, stdout_mutex: &Arc<Mutex<()>>, thread_id: usize) {
    // Get lock before initial message
    let _lock = stdout_mutex.lock().unwrap();
    // Release lock by dropping _lock
    drop(_lock);

    // Create command with piped stderr to capture progress output
    let mut child = Command::new("curl")
        .arg("-o")
        .arg(file_name)
        .arg(url)
        .stderr(Stdio::piped()) // Progress info goes to stderr
        .stdout(Stdio::piped()) // Just in case there's stdout
        .spawn()
        .expect("Unable to spawn child process");

    // Process stderr for progress information
    if let Some(stderr) = child.stderr.take() {
        let reader = BufReader::new(stderr);
        for line in reader.lines().map_while(Result::ok) {
            let _lock = stdout_mutex.lock().unwrap();
            println!("{:-^50}", "");
            println!("Thread {}", thread_id);
            println!("{}", line);
            println!("{:-^50}", "");
        }
    }

    // Process any stdout (likely very little or none)
    if let Some(stdout) = child.stdout.take() {
        let reader = BufReader::new(stdout);
        for line in reader.lines().map_while(Result::ok) {
            let _lock = stdout_mutex.lock().unwrap();
            println!("{:-^50}", "");
            println!("Thread {}", thread_id);
            println!("{}", line);
            println!("{:-^50}", "");
        }
    }

    let status = child.wait().expect("Unable to wait for child process");

    // Final status message
    let _lock = stdout_mutex.lock().unwrap();
    if !status.success() {
        println!("Thread {}: Failed to download {}", thread_id, file_name);
    } else {
        println!(
            "Thread {}: Successfully downloaded {}",
            thread_id, file_name
        );
    }
}

fn json_parser(json: &str) -> Vec<File> {
    let parsed_json: Input = serde_json::from_str(json).expect("Unable to parse JSON");
    parsed_json.files
}

fn text_parser(text: &str) -> Vec<File> {
    let mut files = vec![];
    for line in text.lines() {
        let split: Vec<&str> = line.split(",").collect();
        if split.len() >= 2 {
            let file = File {
                name: split[0].to_string(),
                url: split[1].trim().to_string(),
            };
            files.push(file);
        }
    }
    dbg!(&files);
    files
}
