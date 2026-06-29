// This module does the following tasks
// 1. Read the file(s) from the path(s).
// 2. Reads file content one by one and finds pattern
// 3. Return data with matched line along with line no

use std::{
    error::Error,
    fs::{self},
    path::PathBuf,
    sync::Arc,
};

use walkdir::{DirEntry, WalkDir};

pub struct FileMatch {
    pub path: Arc<PathBuf>,
    pub line_no: usize,
    pub line: String,
}

// flag options
pub struct SearchOptions {
    pub ignore_case: bool,
    pub count_mode: bool,
}

pub struct FileCount {
    pub path: Arc<PathBuf>,
    pub count: usize,
}

pub enum SearchResult {
    Matches(Vec<FileMatch>),
    Counts(Vec<FileCount>),
}

fn is_ignored(entry: &DirEntry) -> bool {
    let name = entry.file_name().to_string_lossy();
    name.starts_with('.') || name == "target"
}

fn read_files(paths: &[PathBuf]) -> Vec<(Arc<PathBuf>, String)> {
    paths
        .iter()
        .flat_map(|path| {
            WalkDir::new(path)
                .into_iter()
                .filter_entry(|entry| !is_ignored(entry))
        })
        .filter_map(Result::ok)
        .filter(|entry| entry.file_type().is_file())
        .filter_map(|entry| {
            let path = entry.path().to_path_buf();

            fs::read_to_string(&path)
                .inspect_err(|e| eprintln!("Unable to read {path:?} cause ->{e}"))
                .ok()
                // Arc is used so further cloning is in-expensive.
                .map(|content| (Arc::new(path), content))
        })
        .collect()
}

fn match_lines(buffer: &str, pattern: &str, ignore_case: bool) -> Vec<(usize, String)> {
    if ignore_case {
        let pattern = pattern.to_lowercase();

        buffer
            .lines()
            .enumerate()
            .filter(|(_, line)| line.to_lowercase().contains(&pattern))
            .map(|(i, line)| (i + 1, line.to_string()))
            .collect()
    } else {
        buffer
            .lines()
            .enumerate()
            .filter(|(_, line)| line.contains(pattern))
            .map(|(i, line)| (i + 1, line.to_string()))
            .collect()
    }
}

fn count_matches(buffer: &str, pattern: &str, ignore_case: bool) -> usize {
    match_lines(buffer, pattern, ignore_case).len()
}

pub fn search_pattern(
    paths: &[PathBuf],
    pattern: &str,
    search_options: SearchOptions,
) -> Result<SearchResult, Box<dyn Error>> {
    let files = read_files(paths);

    if search_options.count_mode {
        let counts = files
            .into_iter()
            .map(|(path, content)| {
                let count = count_matches(&content, pattern, search_options.ignore_case);
                FileCount { path, count }
            })
            .collect();
        Ok(SearchResult::Counts(counts))
    } else {
        let matches = files
            .into_iter()
            .flat_map(|(path, content)| {
                match_lines(&content, pattern, search_options.ignore_case)
                    .into_iter()
                    .map(move |(line_no, line)| FileMatch {
                        path: path.clone(),
                        line_no,
                        line,
                    })
            })
            .collect();

        Ok(SearchResult::Matches(matches))
    }
}
