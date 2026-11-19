//! Example 02: Resilient state machine using Homeostatic Attractors.
//!
//! Run with: `cargo run --example 02_homeostatic_service`

use axiom_zero::{axiom_zero, Attractor};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum WorkerState {
    Idle,
    Processing,
    Validating,
    Synchronizing,
}

#[axiom_zero]
fn main() {
    println!("=== Axiom-Zero: Homeostatic Attractor Service ===");

    // Define an invariant 4-phase limit cycle
    let mut service = Attractor::new(
        WorkerState::Idle,
        vec![
            WorkerState::Processing,
            WorkerState::Validating,
            WorkerState::Synchronizing,
        ],
    );

    println!("Initial Phase (0): {:?}", service.current_state());
    println!("Advance Phase (1): {:?}", service.step());
    println!("Advance Phase (2): {:?}", service.step());

    // SIMULATE CORRUPTED HARDWARE INPUT OR PACKET DROP:
    println!("\n[EXTERNAL PERTURBATION INJECTED: Signal Corrupted]");
    let recovered_state = service.absorb_perturbation(3);

    println!(
        "-> HOMEOSTASIS (tau = 1 step): Service absorbed perturbation without panicking!"
    );
    println!("-> Recovered Phase: {:?}", recovered_state);

    // Continue normal cyclic operation
    println!("\nNext normal cycle step: {:?}", service.step());
}
