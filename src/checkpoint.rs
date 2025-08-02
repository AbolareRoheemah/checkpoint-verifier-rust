use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Checkpoint {
    pub slot: u64,
    pub epoch: u64,
    pub block_root: String,
    pub state_root: String,
    pub source: String
}

#[derive(PartialEq)]
pub enum RiskLevel {
    Safe,
    Warning,
    Danger,
    Critical
}

pub struct SecurityReport {
    pub checkpoints: Vec<Checkpoint>,
    pub risk_level: RiskLevel,
    pub issues: Vec<String>,
    pub recommendations: Vec<String>
}

impl SecurityReport {
    pub fn print_report(&self) {
        println!("=== CHECKPOINT SECURITY ANALYSIS ===");
        if self.risk_level == RiskLevel::Safe {
            println!("🟢 Safe");
        } else if self.risk_level == RiskLevel::Warning {
            println!("🟡 Warning");
        } else if self.risk_level == RiskLevel::Danger {
            println!("🟠 Danger");
        } else {
            println!("🔴 Critical")
        }
        println!("{} sources analyzed", self.checkpoints.len());
        for checkpoint in self.checkpoints.iter() {
            println!("Checkpoint: {} {} {}", checkpoint.source, checkpoint.epoch, &checkpoint.block_root[..16])
        }
        if !self.issues.is_empty() {
            println!("Issues found:");
            for (i, issue) in self.issues.iter().enumerate() {
                println!("  {}. {}", i + 1, issue);
            }
            println!("Recommendations:");
            for (i, reco) in self.recommendations.iter().enumerate() {
                println!(" {}. {}", i + 1, reco)
            }
        }
    }
}

impl Checkpoint {
    pub fn create_honest_scenario() -> Vec<Checkpoint> {
        let mut checkpoint_array = vec![];
        let checkpoint1 = Checkpoint {
            slot: 8234567,
            epoch: 383607,
            block_root: String::from("0xabc123def4567890abc123def4567890abc123def4567890abc123def4567890"),
            state_root: String::from("0xdef456abc1237890def456abc1237890def456abc1237890def456abc1237890"),
            source: String::from("Infura")
        };
        let checkpoint2 = Checkpoint {
            slot: 8234567,
            epoch: 383607,
            block_root: String::from("0xabc123def4567890abc123def4567890abc123def4567890abc123def4567890"),
            state_root: String::from("0xdef456abc1237890def456abc1237890def456abc1237890def456abc1237890"),
            source: String::from("Quicknode")
        };
        let checkpoint3 = Checkpoint {
            slot: 8234567,
            epoch: 383607,
            block_root: String::from("0xabc123def4567890abc123def4567890abc123def4567890abc123def4567890"),
            state_root: String::from("0xdef456abc1237890def456abc1237890def456abc1237890def456abc1237890"),
            source: String::from("ethereum_org")
        };

        checkpoint_array.push(checkpoint1);
        checkpoint_array.push(checkpoint2);
        checkpoint_array.push(checkpoint3);

        checkpoint_array
    }

    pub fn create_attack_scenario() -> Vec<Checkpoint> {
        let checkpoint1 = Checkpoint {
            slot: 8234567,
            epoch: 383607,
            block_root: String::from("0xabc123def4567890abc123def4567890abc123def4567890abc123def4567890"),
            state_root: String::from("0xdef456abc1237890def456abc1237890def456abc1237890def456abc1237890"),
            source: String::from("Infura")
        };
        let malicious_source = Checkpoint {
            slot: 8234567,
            epoch: 383607,
            block_root: String::from("0xdef456abc1237890def456abc1237890def456abc1237890def456abc1237890"),
            state_root: String::from("0xdef456abc1237890def456abc1237890def456abc1237890def456abc1237890"),
            source: String::from("Quicknode")
        };
        let checkpoint3 = Checkpoint {
            slot: 8234567,
            epoch: 383607,
            block_root: String::from("0xabc123def4567890abc123def4567890abc123def4567890abc123def4567890"),
            state_root: String::from("0xdef456abc1237890def456abc1237890def456abc1237890def456abc1237890"),
            source: String::from("ethereum_org")
        };

        vec![checkpoint1, malicious_source, checkpoint3]
    }

    pub fn create_eclipse_scenario() -> Vec<Checkpoint> {
        let isolated_source = Checkpoint {
            slot: 8234567,
            epoch: 383607,
            block_root: String::from("0xabc123def4567890abc123def4567890abc123def4567890abc123def4567890"),
            state_root: String::from("0xdef456abc1237890def456abc1237890def456abc1237890def456abc1237890"),
            source: String::from("ethereum_org")
        };

        vec![isolated_source]
    }
}