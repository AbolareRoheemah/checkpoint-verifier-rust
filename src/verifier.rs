use crate::checkpoint::{Checkpoint, SecurityReport, RiskLevel};
use std::time::{SystemTime, Duration};

pub struct CheckpointVerifier {}

impl CheckpointVerifier {
    pub fn new() -> Self {
        Self {}
    }

    pub fn verify(&self, checkpoints: Vec<Checkpoint>) -> SecurityReport {
        if checkpoints.len() < 2 {
            return SecurityReport {
                checkpoints,
                risk_level: RiskLevel::Critical,
                issues: vec![String::from("Only one checkpoint source available")],
                recommendations: vec![String::from("Potential eclipse attack - verify network connectivity")]
            };
        }
        let block_roots_match = checkpoints.iter().all(|checkpoint| checkpoint.block_root == checkpoints[0].block_root);
        if !block_roots_match {
            return SecurityReport {
                checkpoints,
                risk_level: RiskLevel::Critical,
                issues: vec![String::from("Block root mismatch between sources!")],
                recommendations: vec![String::from("Potential attack - verify network connectivity")]
            };
        }
        let state_roots_match = checkpoints.iter().all(|checkpoint| checkpoint.state_root == checkpoints[0].state_root);
        if !state_roots_match {
            return SecurityReport {
                checkpoints,
                risk_level: RiskLevel::Danger,
                issues: vec![String::from("State root mismatch between sources!")],
                recommendations: vec![String::from("Potential attack - verify network connectivity")]
            };
        }
        let current_epoch = self.estimate_current_epoch();
if current_epoch - checkpoints[0].epoch > 5 {
            return SecurityReport {
                checkpoints,
                risk_level: RiskLevel::Warning,
                issues: vec![String::from("Checkpoint is more than 5 epochs old")],
                recommendations: vec![String::from("Potential attack - verify network connectivity")]
            };
        }
        SecurityReport {
            checkpoints,
            risk_level: RiskLevel::Safe,
            issues: vec![],
            recommendations: vec![String::from("Well done!")]
        }
    }

    pub fn estimate_current_epoch(&self) -> u64 {
        let genesis = SystemTime::UNIX_EPOCH + Duration::from_secs(1606824023);
        let current_timestamp = SystemTime::now();

        // Calculate seconds elapsed since genesis
        let time_elapsed_since_genesis = current_timestamp.duration_since(genesis).unwrap().as_secs();

        let time_elapsed_in_slot = time_elapsed_since_genesis / 12;
        let time_elapsed_in_epoch = time_elapsed_in_slot / 32;
        println!("time_elapsed_in_epoch: {}", time_elapsed_in_epoch);

        time_elapsed_in_epoch
    }

    fn detect_suspicious_patterns(
        &self,
        checkpoints: &[Checkpoint],
        issues: &mut Vec<String>,
        risk_level: &mut RiskLevel,
    ) {
        let n = checkpoints.len();
        if n < 3 {
            return;
        }

        for (i, checkpoint) in checkpoints.iter().enumerate() {
            let mut disagreements = 0;
            for (j, other) in checkpoints.iter().enumerate() {
                if i == j {
                    continue;
                }
                if checkpoint.block_root != other.block_root || checkpoint.state_root != other.state_root {
                    disagreements += 1;
                }
            }
            // If this checkpoint disagrees with all others, it's an outlier
            if disagreements == n - 1 {
                issues.push(format!(
                    "Source '{}' disagrees with all others - potentially compromised",
                    checkpoint.source
                ));
                *risk_level = RiskLevel::Critical;
                break;
            }
        }
    }
}
