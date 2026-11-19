//! Example 01: Replacing `Option<T>` with `Presence<T>` to eliminate `None` and `unwrap()`.
//!
//! Run with: `cargo run --example 01_total_presence`

use axiom_zero::{axiom_zero, Presence};

#[derive(Debug, PartialEq, Eq)]
pub struct Account {
    pub id: u64,
    pub balance: i64,
}

/// A total lookup function. Instead of returning `Option<Account>` (which admits void/None),
/// it returns a `Presence<Account>`, guaranteeing an actionable entity.
#[axiom_zero]
pub fn query_account(account_id: u64) -> Presence<Account> {
    if account_id == 1001 {
        Presence::new(Account {
            id: 1001,
            balance: 5000,
        })
    } else {
        // Boundary fallback token: acts as a protective shield without crashing
        Presence::boundary(Account {
            id: account_id,
            balance: 0,
        })
    }
}

#[axiom_zero]
fn main() {
    println!("=== Axiom-Zero: Total Presence Demonstration ===");

    // Query an existing account
    let live_acc = query_account(1001);
    println!("Account 1001: {:?}", live_acc.get());
    println!("Is boundary token: {}", live_acc.is_boundary());

    // Query a missing account
    let missing_acc = query_account(9999);
    println!("\nAccount 9999: {:?}", missing_acc.get());
    println!("Is boundary token: {}", missing_acc.is_boundary());

    // Resolve safely without unwrapping
    let status_message = missing_acc.resolve(
        |acc| format!("Active user {} with balance ${}", acc.id, acc.balance),
        |acc| format!("Guest/Boundary fallback session created for user {}", acc.id),
    );
    println!("Resolved Status: {}", status_message);
}
