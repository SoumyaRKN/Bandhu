use crate::config::Config;
use serde::Serialize;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize)]
pub struct ContextItem {
    pub path: String,
    pub content: String,
}

pub struct ContextBuilder {
    config: Config,
}

impl ContextBuilder {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub fn build(&self, task: &str) -> Result<Vec<ContextItem>, String> {
        let candidates = self.search_stage(task)?;
        let selected = self.select_stage(candidates)?;
        let contents = self.read_stage(selected)?;
        self.pack_stage(contents)
    }

    fn search_stage(&self, task: &str) -> Result<Vec<Candidate>, String> {
        let keywords = self.extract_keywords(task);
        let mut candidates = Vec::new();
        for keyword in keywords {
            match crate::search::Search::execute_search(
                &keyword,
                &std::env::current_dir()
                    .map_err(|e| e.to_string())?
                    .to_string_lossy()
                    .to_string(),
                &self.config,
            ) {
                Ok(result) => {
                    if let Some(matches) = result.get("matches").and_then(|m| m.as_array()) {
                        for m in matches {
                            let path = m
                                .get("path")
                                .and_then(|p| p.as_str())
                                .unwrap_or("")
                                .to_string();
                            let score = self.score_match(
                                task,
                                m.get("text").and_then(|t| t.as_str()).unwrap_or(""),
                            );
                            candidates.push(Candidate { path, score });
                        }
                    }
                }
                Err(_) => continue,
            }
        }
        Ok(candidates)
    }

    fn select_stage(&self, candidates: Vec<Candidate>) -> Result<Vec<Candidate>, String> {
        let mut sorted = candidates;
        sorted.sort_by(|a, b| b.score.cmp(&a.score).then(a.path.cmp(&b.path)));
        sorted.truncate(self.config.context_top_n);
        Ok(sorted)
    }

    fn read_stage(&self, selected: Vec<Candidate>) -> Result<Vec<Contents>, String> {
        let mut contents = Vec::new();
        for candidate in selected {
            let path_buf = PathBuf::from(&candidate.path);
            let metadata = std::fs::metadata(&path_buf).map_err(|e| e.to_string())?;
            let file_size = metadata.len() as usize;
            if file_size > self.config.context_max_file_bytes {
                contents.push(Contents {
                    path: candidate.path,
                    content: format!(
                        "[file truncated: {} bytes exceeds limit {}]",
                        file_size, self.config.context_max_file_bytes
                    ),
                });
            } else {
                let content = std::fs::read_to_string(&path_buf).map_err(|e| e.to_string())?;
                contents.push(Contents {
                    path: candidate.path,
                    content,
                });
            }
        }
        Ok(contents)
    }

    fn pack_stage(&self, contents: Vec<Contents>) -> Result<Vec<ContextItem>, String> {
        let mut items = Vec::new();
        for item in contents {
            items.push(ContextItem {
                path: item.path,
                content: item.content,
            });
        }
        Ok(items)
    }

    fn extract_keywords(&self, task: &str) -> Vec<String> {
        task.split_whitespace().map(|s| s.to_string()).collect()
    }

    fn score_match(&self, task: &str, text: &str) -> u32 {
        let lower_task = task.to_lowercase();
        let lower_text = text.to_lowercase();
        let mut score = 0;
        for word in lower_task.split_whitespace() {
            if lower_text.contains(word) {
                score += 1;
            }
        }
        score
    }
}

struct Candidate {
    path: String,
    score: u32,
}

struct Contents {
    path: String,
    content: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;

    #[test]
    fn extracts_keywords() {
        let config = Config::from_env();
        let builder = ContextBuilder::new(config);
        let keywords = builder.extract_keywords("find the main function");
        assert!(keywords.contains(&"find".to_string()));
        assert!(keywords.contains(&"main".to_string()));
    }

    #[test]
    fn scores_match() {
        let config = Config::from_env();
        let builder = ContextBuilder::new(config);
        let score = builder.score_match("main function", "fn main() { }");
        assert!(score > 0);
    }

    #[test]
    fn build_returns_items() {
        let config = Config::from_env();
        let builder = ContextBuilder::new(config);
        let result = builder.build("TODO");
        assert!(result.is_ok());
    }
}
