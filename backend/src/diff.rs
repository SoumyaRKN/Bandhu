pub fn generate(old_content: &str, new_content: &str, path: &str) -> String {
    let old_lines: Vec<&str> = old_content.lines().collect();
    let new_lines: Vec<&str> = new_content.lines().collect();

    let mut result = String::new();
    result.push_str(&format!("--- a/{}\n", path));
    result.push_str(&format!("+++ b/{}\n", path));

    if old_lines.is_empty() && new_lines.is_empty() {
        return result;
    }

    let old_len = old_lines.len();
    let new_len = new_lines.len();

    if old_len == 0 {
        for line in new_lines.iter() {
            result.push_str(&format!("+{}\n", line));
        }
        return result;
    }

    if new_len == 0 {
        for line in old_lines.iter() {
            result.push_str(&format!("-{}\n", line));
        }
        return result;
    }

    let diffs = compute_diff(&old_lines, &new_lines);

    for diff in diffs {
        match diff {
            Diff::Remove(line, _) => result.push_str(&format!("-{}\n", line)),
            Diff::Add(line, _) => result.push_str(&format!("+{}\n", line)),
            Diff::Equal(_, _) => {}
        }
    }

    result
}

#[allow(dead_code)]
enum Diff {
    Remove(String, usize),
    Add(String, usize),
    Equal(String, usize),
}

fn compute_diff(old: &[&str], new: &[&str]) -> Vec<Diff> {
    let mut result = Vec::new();
    let mut o_idx = 0;
    let mut n_idx = 0;

    while o_idx < old.len() || n_idx < new.len() {
        if o_idx < old.len() && n_idx < new.len() && old[o_idx] == new[n_idx] {
            result.push(Diff::Equal(old[o_idx].to_string(), o_idx));
            o_idx += 1;
            n_idx += 1;
        } else if n_idx < new.len()
            && (o_idx >= old.len()
                || (n_idx > 0 && new[n_idx] != *old.get(o_idx.saturating_sub(1)).unwrap_or(&"")))
        {
            if let Some(next_old) = old.get(o_idx) {
                if next_old == new.get(n_idx).unwrap_or(&"") {
                    o_idx += 1;
                    n_idx += 1;
                    continue;
                }
            }
            result.push(Diff::Add(new[n_idx].to_string(), n_idx));
            n_idx += 1;
        } else if o_idx < old.len() {
            result.push(Diff::Remove(old[o_idx].to_string(), o_idx));
            o_idx += 1;
        } else if n_idx < new.len() {
            result.push(Diff::Add(new[n_idx].to_string(), n_idx));
            n_idx += 1;
        }
    }

    result
}

pub fn apply(patch: &str, original: &str) -> Result<String, String> {
    let lines: Vec<&str> = original.lines().collect();
    let mut new_lines = Vec::new();

    let mut old_idx = 0;
    for line in patch.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("---") || trimmed.starts_with("+++") {
            continue;
        }
        if trimmed.starts_with('-') && trimmed.len() > 1 {
            old_idx += 1;
        } else if trimmed.starts_with('+') && trimmed.len() > 1 {
            new_lines.push(trimmed[1..].to_string());
            old_idx += 1;
        } else if !trimmed.is_empty() {
            if old_idx < lines.len() {
                new_lines.push(lines[old_idx].to_string());
            }
            old_idx += 1;
        }
    }

    Ok(new_lines.join("\n"))
}

#[allow(dead_code)]
pub fn hunks(patch: &str) -> Vec<Hunk> {
    let mut hunks = Vec::new();
    let mut current_hunk: Option<Hunk> = None;

    for line in patch.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("---") || trimmed.starts_with("+++") {
            continue;
        }

        if trimmed.starts_with("@@") {
            if let Some(hunk) = current_hunk.take() {
                hunks.push(hunk);
            }
            current_hunk = Some(Hunk {
                old_start: 0,
                new_start: 0,
                lines: Vec::new(),
            });
        }

        if let Some(ref mut hunk) = current_hunk {
            if trimmed.starts_with('-') {
                hunk.lines.push(HunkLine::Remove(trimmed[1..].to_string()));
            } else if trimmed.starts_with('+') {
                hunk.lines.push(HunkLine::Add(trimmed[1..].to_string()));
            } else if !trimmed.is_empty() {
                hunk.lines.push(HunkLine::Context(trimmed.to_string()));
            }
        }
    }

    if let Some(hunk) = current_hunk {
        hunks.push(hunk);
    }

    hunks
}

#[allow(dead_code)]
pub struct Hunk {
    pub old_start: usize,
    pub new_start: usize,
    pub lines: Vec<HunkLine>,
}

#[allow(dead_code)]
pub enum HunkLine {
    Remove(String),
    Add(String),
    Context(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_simple_diff() {
        let old = "line1\nline2\nline3";
        let new = "line1\nline2 modified\nline3";
        let patch = generate(old, new, "test.txt");

        assert!(patch.contains("--- a/test.txt"));
        assert!(patch.contains("+++ b/test.txt"));
    }

    #[test]
    fn handles_empty_old() {
        let old = "";
        let new = "new content";
        let patch = generate(old, new, "test.txt");

        assert!(patch.contains("+new content"));
    }

    #[test]
    fn handles_empty_new() {
        let old = "old content";
        let new = "";
        let patch = generate(old, new, "test.txt");

        assert!(patch.contains("-old content"));
    }
}
