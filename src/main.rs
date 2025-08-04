mod checkpoint;
mod verifier;
use std::env;
fn main() {
    let checkpoint_verify = verifier::CheckpointVerifier::new();
    let honest_checkpoints = checkpoint::Checkpoint::create_honest_scenario();
    let attack_checkpoints = checkpoint::Checkpoint::create_attack_scenario();
    let eclipse_checkpoints = checkpoint::Checkpoint::create_eclipse_scenario();

    let cli_arg = env::args().collect::<Vec<String>>();
    if cli_arg.len() == 1 {
        println!("No arguments provided");
        return;
    } else if cli_arg[1]  == "honest" {
        let verify = checkpoint_verify.verify(honest_checkpoints);
        verify.print_report();
    } else if cli_arg[1] == "attack" {
        let verify = checkpoint_verify.verify(attack_checkpoints);
        verify.print_report();
    } else if cli_arg[1] == "eclipse" {
        let verify = checkpoint_verify.verify(eclipse_checkpoints);
        verify.print_report();
    } else {
        println!("Invalid argument");
        return;
    }
}
