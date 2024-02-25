use std::error::Error;
use std::env;
use std::fs;
use std::fmt;

#[derive(Debug)]
pub struct Config {
    query: String,
    file_path: String,
    case_insensitive: bool,
    invert_search: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 { 
            return Err("Invalid Number Of Arguments.");
        }

        let ignore_case = env::var("IGNORE_CASE").is_ok();
        let invert = env::var("INVERT_SEARCH").is_ok();

        let config = Config {
            query: args[1].clone(),
            file_path: args[2].clone(),
            case_insensitive: ignore_case,
            invert_search: invert,
        };

        Ok(config)
    }
}

// Struct with fields, line_content and line_number to store search results.
#[derive(Debug, PartialEq)]
struct SearchResult<'a> {
    line_content: &'a str,
    line_number: usize,
}

impl<'a> fmt::Display for SearchResult<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}. {}", self.line_number, self.line_content)
    }
}

#[cfg(test)]    // At the moment, only used by tests module.
impl<'a> SearchResult<'a> {
    fn new(line_content: &'a str, line_number: usize) -> Self {
        Self {
            line_content,
            line_number,
        }
    }
}

pub fn run(conf: &Config) -> Result<(), Box<dyn Error>> {
    let content: String = fs::read_to_string(&conf.file_path)?;

    let results: Vec<SearchResult> = if conf.invert_search && conf.case_insensitive {
        search_inverted_insensitive(&content, &conf.query)
    } else if conf.invert_search && !conf.case_insensitive {
        search_inverted(&content, &conf.query)
    } else if !conf.invert_search && conf.case_insensitive {
        search_insensitive(&content, &conf.query)
    } else {
        search(&content, &conf.query)
    };

    // let results: Vec<SearchResult> = if conf.invert_search {
    //     if conf.case_insensitive {
    //         search_inverted_insensitive(&content, &conf.query)
    //     } else {
    //         search_inverted(&content, &conf.query)
    //     }
    // } else {
    //     if conf.case_insensitive {
    //         search_insensitive(&content, &conf.query)
    //     } else {
    //         search(&content, &conf.query)
    //     }
    // };

    if results.is_empty() {
        println!("No Search Results.");
    } else {
        for i in results {
            println!("{}", i);
        }
    }

    Ok(())
}

fn search<'a>(content: &'a str, query: &'a str) -> Vec<SearchResult<'a>> {
    let mut result: Vec<SearchResult<'a>> = Vec::new();

    let mut line_count = 1;
    for line in content.lines() {
        if line.contains(query) {
            result.push(SearchResult {
                line_content: line,
                line_number: line_count,
            });
        } 
        line_count += 1;
    }

    result
}

fn search_insensitive<'a>(content: &'a str, query: &'a str) -> Vec<SearchResult<'a>> {
    let mut result: Vec<SearchResult<'a>> = Vec::new();

    let search_query = &query.to_lowercase();

    let mut line_count = 1;
    for line in content.lines() {
        if line.to_lowercase().contains(search_query) {
            result.push(SearchResult {
                line_content: line,
                line_number: line_count,
            });
        } 
        line_count += 1;
    }

    result
}

fn search_inverted<'a>(content: &'a str, query: &'a str) -> Vec<SearchResult<'a>> {
    let mut result: Vec<SearchResult<'a>> = Vec::new();

    let mut line_count = 1;
    for line in content.lines() {
        if !line.contains(query) {
            result.push(SearchResult {
                line_content: line,
                line_number: line_count,
            });
        } 
        line_count += 1;
    }

    result
}

fn search_inverted_insensitive<'a>(content: &'a str, query: &'a str) -> Vec<SearchResult<'a>> {
    let mut result: Vec<SearchResult<'a>> = Vec::new();

    let search_query = &query.to_lowercase();

    let mut line_count = 1;
    for line in content.lines() {
        if !line.to_lowercase().contains(search_query) {
            result.push(SearchResult {
                line_content: line,
                line_number: line_count,
            });
        } 
        line_count += 1;
    }

    result
}

#[cfg(test)]
pub mod tests {

    use super::*;

    #[test]
    fn test_search_exact_match() {
        let content = "This is a sample content for testing.";
        let query = "sample";
        let result = search(content, query);
        assert_eq!(result, vec![SearchResult::new("This is a sample content for testing.", 1)]);
    }

    #[test]
    fn test_search_insensitive_match() {
        let content = "Case INseNSitive Match.";
        let query = "insensitive";
        let result = search_insensitive(content, query);
        assert_eq!(result, vec![SearchResult::new("Case INseNSitive Match.", 1)]);
    }

    #[test]
    fn test_search_inverted_no_match() {
        let content = "No matches here.";
        let query = "notfound";
        let result = search_inverted(content, query);
        assert_eq!(result, vec![SearchResult::new("No matches here.", 1)]);
    }

    #[test]
    fn test_search_inverted_insensitive_no_match() {
        let content = "No matches here.";
        let query = "NOTFOUND";
        let result = search_inverted_insensitive(content, query);
        assert_eq!(result, vec![SearchResult::new("No matches here.", 1)]);
    }

    #[test]
    fn test_search_multiple_matches() {
        let content = "Multiple matches in this content. Matches can occur multiple times.";
        let query = "matches";
        let result = search(content, query);
        assert_eq!(result, vec![
            SearchResult::new("Multiple matches in this content. Matches can occur multiple times.", 1)
        ]);
    }

    #[test]
    fn test_search_inverted_multiple_matches() {
        let content = "Multiple matches in this content. Matches can occur multiple times.";
        let query = "notfound";
        let result = search_inverted(content, query);
        assert_eq!(result, vec![SearchResult::new("Multiple matches in this content. Matches can occur multiple times.", 1)]);
    }

    #[test]
    #[ignore]
    fn test_search_empty_query() {
        let content = "Content with an empty query.";
        let query = "";
        let result = search(content, query);
        assert_eq!(result, Vec::<SearchResult>::new());
    }

    #[test]
    fn test_search_empty_content() {
        let content = "";
        let query = "somequery";
        let result = search(content, query);
        assert_eq!(result, Vec::<SearchResult>::new());
    }
}