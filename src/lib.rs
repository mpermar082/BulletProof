// src/lib.rs
/*
 * Core library for BulletProof
 */

use log::{info, error, debug};
use serde::{Serialize, Deserialize};
use std::fs;
use std::path::Path;

// Define a custom result type for clarity and convenience
/// Custom result type with error handling
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

/// Represents the result of processing data
#[derive(Debug, Serialize, Deserialize)]
/// Processing result with success status, message, and optional data
pub struct ProcessResult {
    /// Whether the processing was successful
    pub success: bool,
    /// A message describing the result
    pub message: String,
    /// Optional data related to the processing result
    pub data: Option<serde_json::Value>,
}

/// The main BulletProof processor
#[derive(Debug)]
/// BulletProof processor with verbosity and processed item count
pub struct BulletProofProcessor {
    /// Whether to log verbose output
    pub verbose: bool,
    /// The number of items processed so far
    pub processed_count: usize,
}

impl BulletProofProcessor {
    /// Creates a new processor instance
    /// # Arguments
    /// * `verbose` - Whether to log verbose output
    pub fn new(verbose: bool) -> Self {
        Self {
            verbose,
            processed_count: 0,
        }
    }

    /// Processes a given piece of data
    /// # Arguments
    /// * `data` - The data to process
    /// # Returns
    /// A `Result` containing the processing result
    pub fn process(&mut self, data: &str) -> Result<ProcessResult> {
        if self.verbose {
            debug!("Processing data of length: {}", data.len());
        }

        // Simulate processing
        self.processed_count += 1;
        
        let result = ProcessResult {
            success: true,
            message: format!("Successfully processed item #{}", self.processed_count),
            data: Some(serde_json::json!({
                "length": data.len(),
                "processed_at": chrono::Utc::now().to_rfc3339(),
                "item_number": self.processed_count
            })),
        };

        Ok(result)
    }

    /// Returns statistics about the processor's state
    /// # Returns
    /// A JSON value containing processor statistics
    pub fn get_stats(&self) -> serde_json::Value {
        serde_json::json!({
            "processed_count": self.processed_count,
            "verbose": self.verbose
        })
    }
}
