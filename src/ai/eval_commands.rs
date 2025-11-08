/// Evaluation command parsing for TUI
///
/// This module provides command parsing for running evaluations from within the TUI.
/// Commands are similar to CLI but designed for interactive use.
use crate::ai::evaluation::DatasetSource;
use std::path::PathBuf;

/// Error type for command parsing
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseError {
    /// Unknown command
    UnknownCommand(String),
    /// Missing required argument
    MissingArgument(String),
    /// Invalid argument value
    InvalidArgument {
        arg: String,
        value: String,
        reason: String,
    },
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::UnknownCommand(cmd) => write!(f, "Unknown command: {}", cmd),
            ParseError::MissingArgument(arg) => write!(f, "Missing required argument: {}", arg),
            ParseError::InvalidArgument { arg, value, reason } => {
                write!(
                    f,
                    "Invalid value '{}' for argument '{}': {}",
                    value, arg, reason
                )
            }
        }
    }
}

impl std::error::Error for ParseError {}

/// Arguments for the `eval` command
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EvalArgs {
    /// Dataset source (local file or SWE-bench variant)
    pub dataset: DatasetSource,
    /// Number of tasks to evaluate
    pub count: Option<usize>,
    /// Milestone/configuration to use (1, 2, or 3)
    pub milestone: usize,
    /// Output directory for results (optional)
    pub output: Option<PathBuf>,
}

impl Default for EvalArgs {
    fn default() -> Self {
        Self {
            dataset: DatasetSource::Verified,
            count: Some(10),
            milestone: 1,
            output: None,
        }
    }
}

/// Arguments for the `compare` command
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompareArgs {
    /// Dataset source
    pub dataset: DatasetSource,
    /// Number of tasks to evaluate
    pub count: Option<usize>,
    /// Baseline milestone configuration
    pub baseline: usize,
    /// Test milestone configuration
    pub test: usize,
    /// Output directory for results (optional)
    pub output: Option<PathBuf>,
}

impl Default for CompareArgs {
    fn default() -> Self {
        Self {
            dataset: DatasetSource::Verified,
            count: Some(20),
            baseline: 1,
            test: 2,
            output: None,
        }
    }
}

/// Arguments for the `show-config` command
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShowConfigArgs {
    /// Milestone to show configuration for
    pub milestone: usize,
}

impl Default for ShowConfigArgs {
    fn default() -> Self {
        Self { milestone: 1 }
    }
}

/// Parsed evaluation command
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EvalCommand {
    /// Run evaluation on a dataset
    Eval(EvalArgs),
    /// Compare two configurations
    Compare(CompareArgs),
    /// Show configuration for a milestone
    ShowConfig(ShowConfigArgs),
}

/// Parse an eval command from a string
///
/// # Examples
///
/// ```
/// use toad::eval_commands::parse_eval_command;
///
/// let cmd = parse_eval_command("eval --swebench verified --count 10 --milestone 1");
/// assert!(cmd.is_ok());
///
/// let cmd = parse_eval_command("compare --baseline 1 --test 2 --count 20");
/// assert!(cmd.is_ok());
///
/// let cmd = parse_eval_command("show-config --milestone 2");
/// assert!(cmd.is_ok());
/// ```
pub fn parse_eval_command(input: &str) -> Result<EvalCommand, ParseError> {
    let parts: Vec<&str> = input.split_whitespace().collect();

    if parts.is_empty() {
        return Err(ParseError::UnknownCommand(String::new()));
    }

    let cmd_name = parts[0];
    let args = &parts[1..];

    match cmd_name {
        "eval" => parse_eval_args(args).map(EvalCommand::Eval),
        "compare" => parse_compare_args(args).map(EvalCommand::Compare),
        "show-config" | "showconfig" => parse_show_config_args(args).map(EvalCommand::ShowConfig),
        _ => Err(ParseError::UnknownCommand(cmd_name.to_string())),
    }
}

/// Parse arguments for the `eval` command
fn parse_eval_args(args: &[&str]) -> Result<EvalArgs, ParseError> {
    let mut result = EvalArgs::default();
    let mut i = 0;

    while i < args.len() {
        let arg = args[i];

        match arg {
            "--dataset" => {
                i += 1;
                if i >= args.len() {
                    return Err(ParseError::MissingArgument("--dataset".to_string()));
                }
                result.dataset = DatasetSource::Local(PathBuf::from(args[i]));
            }
            "--swebench" => {
                i += 1;
                if i >= args.len() {
                    return Err(ParseError::MissingArgument("--swebench".to_string()));
                }
                result.dataset = match args[i] {
                    "verified" => DatasetSource::Verified,
                    "lite" => DatasetSource::Lite,
                    "full" => DatasetSource::Full,
                    other => {
                        return Err(ParseError::InvalidArgument {
                            arg: "--swebench".to_string(),
                            value: other.to_string(),
                            reason: "Expected: verified, lite, or full".to_string(),
                        });
                    }
                };
            }
            "--count" => {
                i += 1;
                if i >= args.len() {
                    return Err(ParseError::MissingArgument("--count".to_string()));
                }
                result.count = Some(args[i].parse().map_err(|_| ParseError::InvalidArgument {
                    arg: "--count".to_string(),
                    value: args[i].to_string(),
                    reason: "Expected a positive integer".to_string(),
                })?);
            }
            "--milestone" | "-m" => {
                i += 1;
                if i >= args.len() {
                    return Err(ParseError::MissingArgument("--milestone".to_string()));
                }
                let milestone: usize =
                    args[i].parse().map_err(|_| ParseError::InvalidArgument {
                        arg: "--milestone".to_string(),
                        value: args[i].to_string(),
                        reason: "Expected 1, 2, or 3".to_string(),
                    })?;

                if !(1..=3).contains(&milestone) {
                    return Err(ParseError::InvalidArgument {
                        arg: "--milestone".to_string(),
                        value: args[i].to_string(),
                        reason: "Must be 1, 2, or 3".to_string(),
                    });
                }
                result.milestone = milestone;
            }
            "--output" | "-o" => {
                i += 1;
                if i >= args.len() {
                    return Err(ParseError::MissingArgument("--output".to_string()));
                }
                result.output = Some(PathBuf::from(args[i]));
            }
            unknown => {
                return Err(ParseError::InvalidArgument {
                    arg: unknown.to_string(),
                    value: String::new(),
                    reason: "Unknown argument".to_string(),
                });
            }
        }

        i += 1;
    }

    Ok(result)
}

/// Parse arguments for the `compare` command
fn parse_compare_args(args: &[&str]) -> Result<CompareArgs, ParseError> {
    let mut result = CompareArgs::default();
    let mut i = 0;

    while i < args.len() {
        let arg = args[i];

        match arg {
            "--dataset" => {
                i += 1;
                if i >= args.len() {
                    return Err(ParseError::MissingArgument("--dataset".to_string()));
                }
                result.dataset = DatasetSource::Local(PathBuf::from(args[i]));
            }
            "--swebench" => {
                i += 1;
                if i >= args.len() {
                    return Err(ParseError::MissingArgument("--swebench".to_string()));
                }
                result.dataset = match args[i] {
                    "verified" => DatasetSource::Verified,
                    "lite" => DatasetSource::Lite,
                    "full" => DatasetSource::Full,
                    other => {
                        return Err(ParseError::InvalidArgument {
                            arg: "--swebench".to_string(),
                            value: other.to_string(),
                            reason: "Expected: verified, lite, or full".to_string(),
                        });
                    }
                };
            }
            "--count" => {
                i += 1;
                if i >= args.len() {
                    return Err(ParseError::MissingArgument("--count".to_string()));
                }
                result.count = Some(args[i].parse().map_err(|_| ParseError::InvalidArgument {
                    arg: "--count".to_string(),
                    value: args[i].to_string(),
                    reason: "Expected a positive integer".to_string(),
                })?);
            }
            "--baseline" => {
                i += 1;
                if i >= args.len() {
                    return Err(ParseError::MissingArgument("--baseline".to_string()));
                }
                let milestone: usize =
                    args[i].parse().map_err(|_| ParseError::InvalidArgument {
                        arg: "--baseline".to_string(),
                        value: args[i].to_string(),
                        reason: "Expected 1, 2, or 3".to_string(),
                    })?;

                if !(1..=3).contains(&milestone) {
                    return Err(ParseError::InvalidArgument {
                        arg: "--baseline".to_string(),
                        value: args[i].to_string(),
                        reason: "Must be 1, 2, or 3".to_string(),
                    });
                }
                result.baseline = milestone;
            }
            "--test" => {
                i += 1;
                if i >= args.len() {
                    return Err(ParseError::MissingArgument("--test".to_string()));
                }
                let milestone: usize =
                    args[i].parse().map_err(|_| ParseError::InvalidArgument {
                        arg: "--test".to_string(),
                        value: args[i].to_string(),
                        reason: "Expected 1, 2, or 3".to_string(),
                    })?;

                if !(1..=3).contains(&milestone) {
                    return Err(ParseError::InvalidArgument {
                        arg: "--test".to_string(),
                        value: args[i].to_string(),
                        reason: "Must be 1, 2, or 3".to_string(),
                    });
                }
                result.test = milestone;
            }
            "--output" | "-o" => {
                i += 1;
                if i >= args.len() {
                    return Err(ParseError::MissingArgument("--output".to_string()));
                }
                result.output = Some(PathBuf::from(args[i]));
            }
            unknown => {
                return Err(ParseError::InvalidArgument {
                    arg: unknown.to_string(),
                    value: String::new(),
                    reason: "Unknown argument".to_string(),
                });
            }
        }

        i += 1;
    }

    Ok(result)
}

/// Parse arguments for the `show-config` command
fn parse_show_config_args(args: &[&str]) -> Result<ShowConfigArgs, ParseError> {
    let mut result = ShowConfigArgs::default();
    let mut i = 0;

    while i < args.len() {
        let arg = args[i];

        match arg {
            "--milestone" | "-m" => {
                i += 1;
                if i >= args.len() {
                    return Err(ParseError::MissingArgument("--milestone".to_string()));
                }
                let milestone: usize =
                    args[i].parse().map_err(|_| ParseError::InvalidArgument {
                        arg: "--milestone".to_string(),
                        value: args[i].to_string(),
                        reason: "Expected 1, 2, or 3".to_string(),
                    })?;

                if !(1..=3).contains(&milestone) {
                    return Err(ParseError::InvalidArgument {
                        arg: "--milestone".to_string(),
                        value: args[i].to_string(),
                        reason: "Must be 1, 2, or 3".to_string(),
                    });
                }
                result.milestone = milestone;
            }
            unknown => {
                return Err(ParseError::InvalidArgument {
                    arg: unknown.to_string(),
                    value: String::new(),
                    reason: "Unknown argument".to_string(),
                });
            }
        }

        i += 1;
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_eval_basic() {
        let cmd = parse_eval_command("eval").unwrap();
        if let EvalCommand::Eval(args) = cmd {
            assert_eq!(args.dataset, DatasetSource::Verified);
            assert_eq!(args.count, Some(10));
            assert_eq!(args.milestone, 1);
        } else {
            panic!("Expected Eval command");
        }
    }

    #[test]
    fn test_parse_eval_with_args() {
        let cmd = parse_eval_command("eval --swebench verified --count 25 --milestone 2").unwrap();
        if let EvalCommand::Eval(args) = cmd {
            assert_eq!(args.dataset, DatasetSource::Verified);
            assert_eq!(args.count, Some(25));
            assert_eq!(args.milestone, 2);
        } else {
            panic!("Expected Eval command");
        }
    }

    #[test]
    fn test_parse_eval_lite() {
        let cmd = parse_eval_command("eval --swebench lite").unwrap();
        if let EvalCommand::Eval(args) = cmd {
            assert_eq!(args.dataset, DatasetSource::Lite);
        } else {
            panic!("Expected Eval command");
        }
    }

    #[test]
    fn test_parse_compare() {
        let cmd = parse_eval_command("compare --baseline 1 --test 2 --count 30").unwrap();
        if let EvalCommand::Compare(args) = cmd {
            assert_eq!(args.baseline, 1);
            assert_eq!(args.test, 2);
            assert_eq!(args.count, Some(30));
        } else {
            panic!("Expected Compare command");
        }
    }

    #[test]
    fn test_parse_show_config() {
        let cmd = parse_eval_command("show-config --milestone 3").unwrap();
        if let EvalCommand::ShowConfig(args) = cmd {
            assert_eq!(args.milestone, 3);
        } else {
            panic!("Expected ShowConfig command");
        }
    }

    #[test]
    fn test_parse_unknown_command() {
        let result = parse_eval_command("unknown");
        assert!(matches!(result, Err(ParseError::UnknownCommand(_))));
    }

    #[test]
    fn test_parse_invalid_milestone() {
        let result = parse_eval_command("eval --milestone 5");
        assert!(matches!(result, Err(ParseError::InvalidArgument { .. })));
    }

    #[test]
    fn test_parse_missing_argument() {
        let result = parse_eval_command("eval --count");
        assert!(matches!(result, Err(ParseError::MissingArgument(_))));
    }
}
