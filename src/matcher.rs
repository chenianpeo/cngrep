use std::{
    fmt::Debug,
    fs::File,
    io::{self, BufRead, BufReader},
    path::PathBuf,
    vec,
};

use crate::{common::Range, error::Error, reader::Input};

#[derive(Debug)]
pub struct MatchMode {
    pub count: bool,
    pub ignore_case: bool,
}

#[derive(Debug)]
pub enum MatchResult {
    Normal(NormalResult),
    Count(CountResult),
}

#[derive(Debug)]
pub enum SearchResult {
    Normal(Vec<NormalResult>),
    Count(Vec<CountResult>),
}

#[derive(Debug)]
pub struct NormalResult {
    pub path: Option<PathBuf>,
    pub matches: Vec<Match>,
}

#[derive(Debug)]
pub struct Match {
    pub line_num: usize,
    pub content: String,
    pub range: Range,
}

#[derive(Debug)]
pub struct CountResult {
    pub path: Option<PathBuf>,
    pub number: usize,
}

pub fn search(pattern: &str, read_result: &Input, mode: &MatchMode) -> Result<SearchResult, Error> {
    match read_result {
        Input::Stdin => {
            let result = matcher(pattern, io::stdin().lock(), None, mode)?;

            match result {
                MatchResult::Normal(normal) => Ok(SearchResult::Normal(vec![normal])),
                MatchResult::Count(count) => Ok(SearchResult::Count(vec![count])),
            }
        }
        Input::MultiFile(files) => {
            let result = matchers(pattern, files, mode)?;
            Ok(result)
        }
    }
}

fn matcher<W: BufRead>(
    pattern: &str,
    reader: W,
    path: Option<PathBuf>,
    mode: &MatchMode,
) -> Result<MatchResult, Error> {
    let pattern_len = pattern.len();
    let pattern_lower = pattern.to_lowercase();

    let mut matches: Vec<Match> = Vec::new();

    for (line_num, line) in reader.lines().enumerate() {
        let line = line?;

        let (matched, start) = if mode.ignore_case {
            let line_lower = line.to_lowercase();

            match line_lower.find(&pattern_lower) {
                Some(index) => (true, index),
                None => (false, 0),
            }
        } else {
            match line.find(pattern) {
                Some(index) => (true, index),
                None => (false, 0),
            }
        };

        if matched {
            let end = start + pattern_len;

            matches.push(Match {
                line_num,
                content: line,
                range: Range { start, end },
            });
        }
    }

    let number = matches.len();

    if mode.count {
        let count = CountResult { path, number };
        Ok(MatchResult::Count(count))
    } else {
        let normal = NormalResult {
            path: path.clone(),
            matches,
        };
        Ok(MatchResult::Normal(normal))
    }
}

fn matchers(pattern: &str, files: &[PathBuf], mode: &MatchMode) -> Result<SearchResult, Error> {
    let mut normals: Vec<NormalResult> = Vec::new();
    let mut counts: Vec<CountResult> = Vec::new();

    for file in files {
        let reader = BufReader::new(File::open(file)?);

        let matcher = matcher(pattern, reader, Some(file.to_path_buf()), mode)?;

        match matcher {
            MatchResult::Normal(normal) => {
                if normal.matches.is_empty() {
                    continue;
                }
                normals.push(normal)
            }
            MatchResult::Count(count) => {
                if count.number == 0 {
                    continue;
                }
                counts.push(count)
            }
        }
    }

    if !mode.count {
        Ok(SearchResult::Normal(normals))
    } else {
        Ok(SearchResult::Count(counts))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs, io::Cursor, path::PathBuf};

    fn normal_mode() -> MatchMode {
        MatchMode {
            count: false,
            ignore_case: false,
        }
    }

    fn count_mode() -> MatchMode {
        MatchMode {
            count: true,
            ignore_case: false,
        }
    }

    fn ignore_case_mode() -> MatchMode {
        MatchMode {
            count: false,
            ignore_case: true,
        }
    }

    #[test]
    fn find_match() {
        let input = "hello world\nfoo bar\nhello rust\n";
        let reader = Cursor::new(input);

        let result = matcher("hello", reader, None, &normal_mode()).unwrap();

        match result {
            MatchResult::Normal(result) => {
                assert_eq!(result.path, None);
                assert_eq!(result.matches.len(), 2);

                assert_eq!(result.matches[0].line_num, 0);
                assert_eq!(result.matches[0].content, "hello world");
                assert_eq!(result.matches[0].range.start, 0);
                assert_eq!(result.matches[0].range.end, 5);

                assert_eq!(result.matches[1].line_num, 2);
                assert_eq!(result.matches[1].content, "hello rust");
                assert_eq!(result.matches[1].range.start, 0);
                assert_eq!(result.matches[1].range.end, 5);
            }

            MatchResult::Count(_) => panic!("expected Normal result"),
        }
    }

    #[test]
    fn find_nothing() {
        let input = "hello\nworld\nrust\n";
        let reader = Cursor::new(input);

        let result = matcher("python", reader, None, &normal_mode()).unwrap();

        match result {
            MatchResult::Normal(result) => {
                assert!(result.matches.is_empty());
                assert_eq!(result.path, None);
            }

            MatchResult::Count(_) => panic!("expected Normal result"),
        }
    }

    #[test]
    fn find_middle() {
        let input = "hello rust world\n";
        let reader = Cursor::new(input);

        let result = matcher("rust", reader, None, &normal_mode()).unwrap();

        match result {
            MatchResult::Normal(result) => {
                assert_eq!(result.matches.len(), 1);

                let m = &result.matches[0];

                assert_eq!(m.line_num, 0);
                assert_eq!(m.content, "hello rust world");
                assert_eq!(m.range.start, 6);
                assert_eq!(m.range.end, 10);
            }

            MatchResult::Count(_) => panic!("expected Normal result"),
        }
    }

    #[test]
    fn find_multiple() {
        let input = "rust\nrust\nhello\nrust\n";
        let reader = Cursor::new(input);

        let result = matcher("rust", reader, None, &normal_mode()).unwrap();

        match result {
            MatchResult::Normal(result) => {
                assert_eq!(result.matches.len(), 3);

                assert_eq!(result.matches[0].line_num, 0);
                assert_eq!(result.matches[1].line_num, 1);
                assert_eq!(result.matches[2].line_num, 3);
            }

            MatchResult::Count(_) => panic!("expected Normal result"),
        }
    }

    #[test]
    fn count_matches() {
        let input = "rust\nhello\nrust\nrust\n";
        let reader = Cursor::new(input);

        let result = matcher("rust", reader, None, &count_mode()).unwrap();

        match result {
            MatchResult::Count(result) => {
                assert_eq!(result.number, 3);
                assert_eq!(result.path, None);
            }

            MatchResult::Normal(_) => panic!("expected Count result"),
        }
    }

    #[test]
    fn count_zero() {
        let input = "hello\nworld\n";
        let reader = Cursor::new(input);

        let result = matcher("rust", reader, None, &count_mode()).unwrap();

        match result {
            MatchResult::Count(result) => {
                assert_eq!(result.number, 0);
            }

            MatchResult::Normal(_) => panic!("expected Count result"),
        }
    }

    #[test]
    fn match_ignore_case() {
        let input = "Hello\nHELLO\nhello\nHeLlO\n";
        let reader = Cursor::new(input);

        let result = matcher("hello", reader, None, &ignore_case_mode()).unwrap();

        match result {
            MatchResult::Normal(result) => {
                assert_eq!(result.matches.len(), 4);

                assert_eq!(result.matches[0].line_num, 0);
                assert_eq!(result.matches[1].line_num, 1);
                assert_eq!(result.matches[2].line_num, 2);
                assert_eq!(result.matches[3].line_num, 3);
            }

            MatchResult::Count(_) => panic!("expected Normal result"),
        }
    }

    #[test]
    fn match_preserve_path() {
        let input = "hello world\n";
        let reader = Cursor::new(input);
        let path = PathBuf::from("test.txt");

        let result = matcher("hello", reader, Some(path.clone()), &normal_mode()).unwrap();

        match result {
            MatchResult::Normal(result) => {
                assert_eq!(result.path, Some(path));
            }

            MatchResult::Count(_) => panic!("expected Normal result"),
        }
    }

    #[test]
    fn count_preserve_path() {
        let input = "hello\nhello\n";
        let reader = Cursor::new(input);
        let path = PathBuf::from("test.txt");

        let result = matcher("hello", reader, Some(path.clone()), &count_mode()).unwrap();

        match result {
            MatchResult::Count(result) => {
                assert_eq!(result.path, Some(path));
                assert_eq!(result.number, 2);
            }

            MatchResult::Normal(_) => panic!("expected Count result"),
        }
    }

    #[test]
    fn search_multiple_files() {
        let dir = tempfile::tempdir().unwrap();

        let file1 = dir.path().join("one.txt");
        let file2 = dir.path().join("two.txt");

        fs::write(&file1, "hello\nworld\n").unwrap();
        fs::write(&file2, "rust\nhello rust\n").unwrap();

        let files = vec![file1.clone(), file2.clone()];

        let result = matchers("hello", &files, &normal_mode()).unwrap();

        match result {
            SearchResult::Normal(results) => {
                assert_eq!(results.len(), 2);

                assert_eq!(results[0].path, Some(file1));
                assert_eq!(results[0].matches.len(), 1);

                assert_eq!(results[1].path, Some(file2));
                assert_eq!(results[1].matches.len(), 1);
            }

            SearchResult::Count(_) => panic!("expected Normal result"),
        }
    }

    #[test]
    fn skip_unmatched_files() {
        let dir = tempfile::tempdir().unwrap();

        let file1 = dir.path().join("match.txt");
        let file2 = dir.path().join("no_match.txt");

        fs::write(&file1, "hello\n").unwrap();
        fs::write(&file2, "rust\n").unwrap();

        let files = vec![file1.clone(), file2];

        let result = matchers("hello", &files, &normal_mode()).unwrap();

        match result {
            SearchResult::Normal(results) => {
                assert_eq!(results.len(), 1);
                assert_eq!(results[0].path, Some(file1));
            }

            SearchResult::Count(_) => panic!("expected Normal result"),
        }
    }

    #[test]
    fn count_multiple_files() {
        let dir = tempfile::tempdir().unwrap();

        let file1 = dir.path().join("one.txt");
        let file2 = dir.path().join("two.txt");
        let file3 = dir.path().join("three.txt");

        fs::write(&file1, "hello\nhello\n").unwrap();
        fs::write(&file2, "hello\nrust\nhello\n").unwrap();
        fs::write(&file3, "rust\n").unwrap();

        let files = vec![file1.clone(), file2.clone(), file3];

        let result = matchers("hello", &files, &count_mode()).unwrap();

        match result {
            SearchResult::Count(results) => {
                assert_eq!(results.len(), 2);

                assert_eq!(results[0].path, Some(file1));
                assert_eq!(results[0].number, 2);

                assert_eq!(results[1].path, Some(file2));
                assert_eq!(results[1].number, 2);
            }

            SearchResult::Normal(_) => panic!("expected Count result"),
        }
    }

    #[test]
    fn search_no_matches() {
        let dir = tempfile::tempdir().unwrap();

        let file1 = dir.path().join("one.txt");
        let file2 = dir.path().join("two.txt");

        fs::write(&file1, "hello\n").unwrap();
        fs::write(&file2, "world\n").unwrap();

        let files = vec![file1, file2];

        let result = matchers("rust", &files, &normal_mode()).unwrap();

        match result {
            SearchResult::Normal(results) => {
                assert!(results.is_empty());
            }

            SearchResult::Count(_) => panic!("expected Normal result"),
        }
    }

    #[test]
    fn search_normal_result() {
        let dir = tempfile::tempdir().unwrap();

        let file = dir.path().join("test.txt");
        fs::write(&file, "hello\nrust\n").unwrap();

        let input = Input::MultiFile(vec![file.clone()]);

        let result = search("hello", &input, &normal_mode()).unwrap();

        match result {
            SearchResult::Normal(results) => {
                assert_eq!(results.len(), 1);
                assert_eq!(results[0].path, Some(file));
                assert_eq!(results[0].matches.len(), 1);
            }

            SearchResult::Count(_) => panic!("expected Normal result"),
        }
    }

    #[test]
    fn search_count_result() {
        let dir = tempfile::tempdir().unwrap();

        let file = dir.path().join("test.txt");
        fs::write(&file, "hello\nhello\n").unwrap();

        let input = Input::MultiFile(vec![file.clone()]);

        let result = search("hello", &input, &count_mode()).unwrap();

        match result {
            SearchResult::Count(results) => {
                assert_eq!(results.len(), 1);
                assert_eq!(results[0].path, Some(file));
                assert_eq!(results[0].number, 2);
            }

            SearchResult::Normal(_) => panic!("expected Count result"),
        }
    }

    #[test]
    fn search_missing_file() {
        let input = Input::MultiFile(vec![PathBuf::from("this_file_should_not_exist.txt")]);

        let result = search("hello", &input, &normal_mode());

        assert!(result.is_err());
    }
}
