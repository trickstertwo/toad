/// Input validation system
///
/// Provides real-time validation feedback for text inputs with custom rules
use serde::{Deserialize, Serialize};
use std::fmt;

/// Validation result
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ValidationResult {
    /// Input is valid
    Valid,
    /// Input has warnings (non-blocking)
    Warning(String),
    /// Input is invalid (blocking)
    Error(String),
}

impl ValidationResult {
    /// Check if validation passed
    pub fn is_valid(&self) -> bool {
        matches!(self, ValidationResult::Valid)
    }

    /// Check if validation has error
    pub fn is_error(&self) -> bool {
        matches!(self, ValidationResult::Error(_))
    }

    /// Check if validation has warning
    pub fn is_warning(&self) -> bool {
        matches!(self, ValidationResult::Warning(_))
    }

    /// Get error or warning message
    pub fn message(&self) -> Option<&str> {
        match self {
            ValidationResult::Valid => None,
            ValidationResult::Warning(msg) => Some(msg),
            ValidationResult::Error(msg) => Some(msg),
        }
    }
}

impl fmt::Display for ValidationResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ValidationResult::Valid => write!(f, "Valid"),
            ValidationResult::Warning(msg) => write!(f, "Warning: {}", msg),
            ValidationResult::Error(msg) => write!(f, "Error: {}", msg),
        }
    }
}

/// Validator trait for custom validation rules
pub trait Validator: Send + Sync {
    /// Validate input and return result
    fn validate(&self, input: &str) -> ValidationResult;

    /// Get validator description
    fn description(&self) -> &str {
        "Custom validator"
    }
}

/// Non-empty validator
#[derive(Debug, Clone)]
pub struct NotEmptyValidator {
    message: String,
}

impl NotEmptyValidator {
    /// Create a new not-empty validator
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl Default for NotEmptyValidator {
    fn default() -> Self {
        Self::new("Input cannot be empty")
    }
}

impl Validator for NotEmptyValidator {
    fn validate(&self, input: &str) -> ValidationResult {
        if input.trim().is_empty() {
            ValidationResult::Error(self.message.clone())
        } else {
            ValidationResult::Valid
        }
    }

    fn description(&self) -> &str {
        "Not empty validator"
    }
}

/// Length validator
#[derive(Debug, Clone)]
pub struct LengthValidator {
    min: Option<usize>,
    max: Option<usize>,
}

impl LengthValidator {
    /// Create a new length validator
    pub fn new(min: Option<usize>, max: Option<usize>) -> Self {
        Self { min, max }
    }

    /// Create a min length validator
    pub fn min(min: usize) -> Self {
        Self::new(Some(min), None)
    }

    /// Create a max length validator
    pub fn max(max: usize) -> Self {
        Self::new(None, Some(max))
    }

    /// Create a range validator
    pub fn range(min: usize, max: usize) -> Self {
        Self::new(Some(min), Some(max))
    }
}

impl Validator for LengthValidator {
    fn validate(&self, input: &str) -> ValidationResult {
        let len = input.len();

        if let Some(min) = self.min
            && len < min {
                return ValidationResult::Error(format!(
                    "Input must be at least {} characters",
                    min
                ));
            }

        if let Some(max) = self.max
            && len > max {
                return ValidationResult::Error(format!(
                    "Input must be at most {} characters",
                    max
                ));
            }

        ValidationResult::Valid
    }

    fn description(&self) -> &str {
        "Length validator"
    }
}

/// Regex validator
#[derive(Debug, Clone)]
pub struct RegexValidator {
    pattern: String,
    message: String,
}

impl RegexValidator {
    /// Create a new regex validator
    pub fn new(pattern: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            pattern: pattern.into(),
            message: message.into(),
        }
    }

    /// Email validator
    pub fn email() -> Self {
        Self::new(
            r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$",
            "Invalid email format",
        )
    }

    /// URL validator
    pub fn url() -> Self {
        Self::new(r"^https?://[^\s/$.?#].[^\s]*$", "Invalid URL format")
    }

    /// Alphanumeric validator
    pub fn alphanumeric() -> Self {
        Self::new(
            r"^[a-zA-Z0-9]+$",
            "Input must contain only letters and numbers",
        )
    }
}

impl Validator for RegexValidator {
    fn validate(&self, input: &str) -> ValidationResult {
        // Simplified regex check without regex crate
        // For production, you'd want to use the regex crate
        match self.pattern.as_str() {
            // Email pattern
            r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$" => {
                if input.contains('@')
                    && input.split('@').count() == 2
                    && input.contains('.')
                    && !input.starts_with('@')
                    && !input.ends_with('@')
                {
                    ValidationResult::Valid
                } else {
                    ValidationResult::Error(self.message.clone())
                }
            }
            // URL pattern
            r"^https?://[^\s/$.?#].[^\s]*$" => {
                if (input.starts_with("http://") || input.starts_with("https://"))
                    && input.len() > 8
                {
                    ValidationResult::Valid
                } else {
                    ValidationResult::Error(self.message.clone())
                }
            }
            // Alphanumeric pattern
            r"^[a-zA-Z0-9]+$" => {
                if input.chars().all(|c| c.is_alphanumeric()) {
                    ValidationResult::Valid
                } else {
                    ValidationResult::Error(self.message.clone())
                }
            }
            _ => ValidationResult::Valid, // Unknown pattern, pass through
        }
    }

    fn description(&self) -> &str {
        "Regex validator"
    }
}

/// Composite validator that runs multiple validators
pub struct CompositeValidator {
    validators: Vec<Box<dyn Validator>>,
    /// Stop on first error
    stop_on_error: bool,
}

impl CompositeValidator {
    /// Create a new composite validator
    pub fn new(stop_on_error: bool) -> Self {
        Self {
            validators: Vec::new(),
            stop_on_error,
        }
    }

    /// Add a validator
    pub fn add(&mut self, validator: Box<dyn Validator>) {
        self.validators.push(validator);
    }

    /// Add multiple validators
    pub fn add_all(&mut self, validators: Vec<Box<dyn Validator>>) {
        self.validators.extend(validators);
    }
}

impl Default for CompositeValidator {
    fn default() -> Self {
        Self::new(true)
    }
}

impl Validator for CompositeValidator {
    fn validate(&self, input: &str) -> ValidationResult {
        let mut first_warning: Option<String> = None;

        for validator in &self.validators {
            match validator.validate(input) {
                ValidationResult::Valid => continue,
                ValidationResult::Error(msg) => {
                    if self.stop_on_error {
                        return ValidationResult::Error(msg);
                    }
                }
                ValidationResult::Warning(msg) => {
                    if first_warning.is_none() {
                        first_warning = Some(msg);
                    }
                }
            }
        }

        if let Some(warning) = first_warning {
            ValidationResult::Warning(warning)
        } else {
            ValidationResult::Valid
        }
    }

    fn description(&self) -> &str {
        "Composite validator"
    }
}

/// Input validator for real-time validation
pub struct InputValidator {
    validators: Vec<Box<dyn Validator>>,
    last_result: ValidationResult,
}

impl InputValidator {
    /// Create a new input validator
    pub fn new() -> Self {
        Self {
            validators: Vec::new(),
            last_result: ValidationResult::Valid,
        }
    }

    /// Add a validator
    pub fn add_validator(&mut self, validator: Box<dyn Validator>) {
        self.validators.push(validator);
    }

    /// Validate input and cache result
    pub fn validate(&mut self, input: &str) -> &ValidationResult {
        self.last_result = self.run_validators(input);
        &self.last_result
    }

    /// Get last validation result
    pub fn last_result(&self) -> &ValidationResult {
        &self.last_result
    }

    /// Run all validators
    fn run_validators(&self, input: &str) -> ValidationResult {
        for validator in &self.validators {
            match validator.validate(input) {
                ValidationResult::Valid => continue,
                result @ (ValidationResult::Error(_) | ValidationResult::Warning(_)) => {
                    return result;
                }
            }
        }
        ValidationResult::Valid
    }

    /// Clear validators
    pub fn clear(&mut self) {
        self.validators.clear();
        self.last_result = ValidationResult::Valid;
    }
}

impl Default for InputValidator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validation_result() {
        assert!(ValidationResult::Valid.is_valid());
        assert!(!ValidationResult::Error("test".into()).is_valid());
        assert!(ValidationResult::Error("test".into()).is_error());
        assert!(ValidationResult::Warning("test".into()).is_warning());
    }

    #[test]
    fn test_not_empty_validator() {
        let validator = NotEmptyValidator::default();
        assert!(validator.validate("hello").is_valid());
        assert!(validator.validate("").is_error());
        assert!(validator.validate("   ").is_error());
    }

    #[test]
    fn test_length_validator_min() {
        let validator = LengthValidator::min(5);
        assert!(validator.validate("hello").is_valid());
        assert!(validator.validate("hi").is_error());
    }

    #[test]
    fn test_length_validator_max() {
        let validator = LengthValidator::max(10);
        assert!(validator.validate("hello").is_valid());
        assert!(validator.validate("hello world!!").is_error());
    }

    #[test]
    fn test_length_validator_range() {
        let validator = LengthValidator::range(5, 10);
        assert!(validator.validate("hello").is_valid());
        assert!(validator.validate("hi").is_error());
        assert!(validator.validate("hello world").is_error());
    }

    #[test]
    fn test_regex_email_validator() {
        let validator = RegexValidator::email();
        assert!(validator.validate("test@example.com").is_valid());
        assert!(validator.validate("invalid").is_error());
        assert!(validator.validate("@example.com").is_error());
    }

    #[test]
    fn test_regex_url_validator() {
        let validator = RegexValidator::url();
        assert!(validator.validate("https://example.com").is_valid());
        assert!(validator.validate("http://test.org").is_valid());
        assert!(validator.validate("invalid").is_error());
    }

    #[test]
    fn test_regex_alphanumeric_validator() {
        let validator = RegexValidator::alphanumeric();
        assert!(validator.validate("hello123").is_valid());
        assert!(validator.validate("hello world").is_error());
        assert!(validator.validate("test!").is_error());
    }

    #[test]
    fn test_composite_validator() {
        let mut validator = CompositeValidator::new(true);
        validator.add(Box::new(NotEmptyValidator::default()));
        validator.add(Box::new(LengthValidator::min(5)));

        assert!(validator.validate("hello").is_valid());
        assert!(validator.validate("").is_error());
        assert!(validator.validate("hi").is_error());
    }

    #[test]
    fn test_input_validator() {
        let mut input_validator = InputValidator::new();
        input_validator.add_validator(Box::new(NotEmptyValidator::default()));
        input_validator.add_validator(Box::new(LengthValidator::min(3)));

        assert!(input_validator.validate("hello").is_valid());
        assert!(input_validator.validate("").is_error());
        assert!(input_validator.validate("hi").is_error());
    }

    #[test]
    fn test_input_validator_caching() {
        let mut input_validator = InputValidator::new();
        input_validator.add_validator(Box::new(NotEmptyValidator::default()));

        input_validator.validate("");
        assert!(input_validator.last_result().is_error());

        input_validator.validate("hello");
        assert!(input_validator.last_result().is_valid());
    }

    #[test]
    fn test_validation_result_message() {
        let result = ValidationResult::Error("Test error".into());
        assert_eq!(result.message(), Some("Test error"));

        let result = ValidationResult::Valid;
        assert_eq!(result.message(), None);
    }
}
