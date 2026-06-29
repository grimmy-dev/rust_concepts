use std::{env, path::PathBuf};

use clap::Parser;

use crate::search::{SearchOptions, SearchResult};
mod search;

#[derive(Parser, Debug)]
#[command(name = "mgrep")]
struct Args {
    #[arg(short = 'i', long)]
    ignore_case: bool,

    #[arg(short = 'c', long)]
    count: bool,

    pattern: String,
    paths: Vec<PathBuf>,
}

fn main() {
    let cli = Args::parse();

    let cwd = if cli.paths.is_empty() {
        Some(env::current_dir().expect("failed to get current directory"))
    } else {
        None
    };

    // if no paths are given then current directory is choose as search area.
    let paths = if let Some(cwd) = &cwd {
        vec![cwd.clone()]
    } else {
        cli.paths
    };

    match search::search_pattern(
        &paths,
        &cli.pattern,
        SearchOptions {
            ignore_case: cli.ignore_case,
            count_mode: cli.count,
        },
    ) {
        Ok(SearchResult::Matches(matches)) => {
            for m in matches {
                // TODO: The paths when cwd is chooses shows absolute path, make it display relative paths.
                println!("{}:{} -> {}", m.path.display(), m.line_no, m.line);
            }
        }

        Ok(SearchResult::Counts(counts)) => {
            for c in counts {
                println!("{}: {}", c.path.display(), c.count);
            }
        }

        Err(e) => {
            eprintln!("Error caused {}", e);
        }
    }
}
