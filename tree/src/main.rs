mod types;
use std::path::PathBuf;
use clap::Parser;
use types::Config;
use types::Dir;
use types::File;

#[derive(Parser)]
#[command(name = "Disk Usage")]
#[command(author = "Alexander Schenkel")]
#[command(about = "Shows file and directory usage")]
struct Args {
    #[arg(help = "paths / files to analyze")]
    path: Vec<PathBuf>,

    #[arg(short, long, help = "Include files in output (default: only dirs)")]
    files: bool,

    #[arg(short, long, help = "Only print a summary for each given path")]
    summary: bool,
}

fn main() {
    let mut cli = Args::parse();

    let config = Config {
        print_files: cli.files,
        summary: cli.summary,
    };

    if cli.path.len() == 0 {
        let act = std::env::current_dir().unwrap();
        cli.path.push(act);
    }

    for path_buf in cli.path {
        let path_buf = match std::fs::canonicalize(path_buf.as_path()) {
            Ok(p) => p,
            Err(e) => {
                eprintln!("Cannot absolutize path {:?}: {}", path_buf, e);
                continue;
            }
        };

        let start_dir = path_buf.as_path();
        if start_dir.is_dir() {
            let dir =
                Dir::examine(start_dir).expect(format!("Cannot read dir {:?}", start_dir).as_str());
            dir.print(&config);
        } else if start_dir.is_file() {
            let file = File::examine(start_dir)
                .expect(format!("Cannot read file {:?}", start_dir).as_str());
            file.print();
        }
    }
}
