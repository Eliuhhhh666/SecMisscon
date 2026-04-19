use serde::{Serialize, Deserialize};
use colored::*;

// Represents the impact level of a discovered Misconfiguration.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Severity {
    Critical,
    High,
    Medium,
    Low,
    Info,
}

impl Severity {
    /// Returns a colored String representation of the severity for terminal output 
    pub fn to_colored_string(&self) -> String {
        match self {
            Severity::Critical => "CRITICAL".bright_red().bold().to_string(),
            Severity::High => "HIGH".red().bold().to_string(),
            Severity::Medium => "MEDIUM".yellow().bold().to_string(),
            Severity::Low => "LOW".cyan().to_string(),
            Severity::Info => "INFO".blue().to_string(),
        }
    }
}

/// The primary data structure representing a verified security misconfiguration 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Finding {
    pub url: String,
    pub name: String,
    pub description: String,
    pub severity: Severity,
    pub poc_data: Option<String>,
}

impl Finding {
    pub fn new(url: String, name: String, description: String, severity: Severity) -> Self {
        Self {
            url,
            name,
            description,
            severity,
            poc_data: None,
        }
    }

    // Appends POC data to the finding.
    pub fn with_poc(mut self, data: String) -> Self {
        self.poc_data = Some(data);
        self
    }
}