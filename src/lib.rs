// src/lib.rs
/*
 * Core library for BulletProof
 */

use log::{info, error, debug};
use serde::{Serialize, Deserialize};
use std::fs;
use std::path::Path;

// Define a custom result type for clarity and convenience
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

/// Represents the result of processing data
#[derive(Debug, Serialize, Deserialize)]
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
pub struct BulletProofProcessor {
    /// Whether to log verbose output
    pub verbose: bool,
    /// The number of items processed so far
    pub processed_count: usize,
}

impl BulletProofProcessor {
    /// Creates a new processor instance
    pub fn new(verbose: bool) -> Self {
        Self {
            verbose,
            processed_count: 0,
        }
    }

    /// Processes a given piece of data
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
    pub fn get_stats(&self) -> serde_json::Value {
        serde_json::json!({
            "processed_count": self.processed_count,
            "verbose": self.verbose
        })
    }
}

/// Main processing function
pub fn run(verbose: bool, input: Option<String>, output: Option<String>) -> Result<()> {
    // Initialize logging
    if verbose {
        env_logger::Builder::from_default_env()
            .filter_level(log::LevelFilter::Debug)
            .init();
    } else {
        env_logger::init();
    }
    
    info!("Starting BulletProof processing");
    
    let mut processor = BulletProofProcessor::new(verbose);
    
    // Read input
    let input_data = match input {
        Some(path) => {
            info!("Reading input from file: {}", path);
            fs::read_to_string(path)?
        }
        None => {
            info!("Using default input");
            String::new()
        }
    };
    
    // Process the input data
    for line in input_data.lines() {
        let result = processor.process(line)?;
        info!("Processing result: {}", serde_json::to_string(&result)?);
    }
    
    // Return the final stats
    let stats = processor.get_stats();
    info!("Final stats: {}", serde_json::to_string(&stats)?);
    
    Ok(())
}