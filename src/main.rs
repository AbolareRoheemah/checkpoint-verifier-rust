mod checkpoint;
mod verifier;
use std::env;
fn main() {
    let checkpoint_verify = verifier::CheckpointVerifier::new();
    let honest_checkpoints = checkpoint::Checkpoint::create_honest_scenario();
    let attack_checkpoints = checkpoint::Checkpoint::create_attack_scenario();
    let eclipse_checkpoints = checkpoint::Checkpoint::create_eclipse_scenario();

    let cli_arg = env::args().collect::<Vec<String>>();
    println!("cli_arg: {:?}", cli_arg);
    // let verify = checkpoint_verify.verify(honest_checkpoints);
    // verify.print_report();
    // let verify_attack = checkpoint_verify.verify(attack_checkpoints);
    // verify_attack.print_report();
    // let verify_eclipse = checkpoint_verify.verify(eclipse_checkpoints);
    // verify_eclipse.print_report();
}
