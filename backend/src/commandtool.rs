use crate::error::{BackendError, BackendResult};
use std::env;
use std::io::Read;
use std::process::{Command, Stdio};
use std::thread;
use std::time::{Duration, Instant};

pub struct Output {
    pub stdout: String,
    pub stderr: String,
    pub status: i32,
}

pub fn command(name: &str, fallback: &str) -> String {
    env::var(name)
        .ok()
        .filter(|value| !value.trim().is_empty())
        .unwrap_or_else(|| fallback.to_string())
}

pub fn directory(name: &str, fallback: &str) -> String {
    env::var(name)
        .ok()
        .filter(|value| !value.trim().is_empty())
        .unwrap_or_else(|| fallback.to_string())
}

pub fn timeout() -> u64 {
    env::var("BANDHU_TOOL_TIMEOUT_SECS")
        .ok()
        .and_then(|value| value.parse().ok())
        .filter(|value| *value > 0)
        .unwrap_or(120)
}

pub fn run(command: &str, directory: &str, timeout_secs: u64) -> BackendResult<Output> {
    let mut child = shell(command, directory)?;
    let stdout = child.stdout.take();
    let stderr = child.stderr.take();
    let outhandle = stdout.map(|pipe| thread::spawn(move || readall(pipe)));
    let errhandle = stderr.map(|pipe| thread::spawn(move || readall(pipe)));
    let timeout = Duration::from_secs(timeout_secs);
    let start = Instant::now();

    loop {
        if let Some(status) = child
            .try_wait()
            .map_err(|err| BackendError::Io(err.to_string()))?
        {
            let stdout = String::from_utf8_lossy(&join(outhandle)).to_string();
            let stderr = String::from_utf8_lossy(&join(errhandle)).to_string();
            return Ok(Output {
                stdout,
                stderr,
                status: status.code().unwrap_or(-1),
            });
        }

        if start.elapsed() >= timeout {
            let _ = child.kill();
            let _ = child.wait();
            let _ = join(outhandle);
            let _ = join(errhandle);
            return Err(BackendError::Timeout(format!(
                "command timed out after {} seconds",
                timeout_secs
            )));
        }

        thread::sleep(Duration::from_millis(100));
    }
}

pub fn failures(stdout: &str, stderr: &str) -> Vec<String> {
    let mut lines = Vec::new();
    for line in stdout.lines().chain(stderr.lines()) {
        let lower = line.to_lowercase();
        if lower.contains("failed") || lower.contains("error:") || lower.contains("panic") {
            lines.push(line.to_string());
            if lines.len() >= 20 {
                break;
            }
        }
    }
    lines
}

fn shell(command: &str, directory: &str) -> BackendResult<std::process::Child> {
    let directory = if directory.trim().is_empty() {
        "."
    } else {
        directory
    };

    if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/C", command])
            .current_dir(directory)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|err| BackendError::Io(format!("failed to run command: {}", err)))
    } else {
        Command::new("sh")
            .args(["-c", command])
            .current_dir(directory)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|err| BackendError::Io(format!("failed to run command: {}", err)))
    }
}

fn readall(mut pipe: impl Read) -> Vec<u8> {
    let mut data = Vec::new();
    let _ = pipe.read_to_end(&mut data);
    data
}

fn join(handle: Option<thread::JoinHandle<Vec<u8>>>) -> Vec<u8> {
    match handle {
        Some(handle) => handle.join().unwrap_or_default(),
        None => Vec::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn findsfailures() {
        let found = failures("one\nFAILED test_a\n", "error: missing value\n");

        assert_eq!(found.len(), 2);
        assert!(found[0].contains("FAILED"));
        assert!(found[1].contains("error:"));
    }
}
