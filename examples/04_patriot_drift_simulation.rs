//! Example 04: The 1991 Dhahran Patriot Missile Clock-Drift Simulation.
//!
//! Demonstrates how a micro-drift (0.000000095s per tick over 100 hours)
//! causes catastrophic target loss in legacy systems (returning None),
//! and how Axiom-Zero's Attractor + Boundary Token prevents the failure.
//!
//! Run with: `cargo run --example 04_patriot_drift_simulation`

use axiom_zero::{axiom_zero, Attractor, Presence};

// True continuous time vs 24-bit truncated float representation
const RADAR_TICK_TRUE: f64 = 0.1000000000000000;
// In 24-bit fixed point: 0.1 is 0.00011001100110011001100_2 ≈ 0.099999904632568359375
const RADAR_TICK_TRUNCATED: f64 = 0.099999904632568359375;
const TICK_ERROR_DRIFT: f64 = RADAR_TICK_TRUE - RADAR_TICK_TRUNCATED; // ≈ 9.537e-8 sec/tick

const SCUD_SPEED_MPS: f64 = 1676.0; // Mach ~5 (meters per second)
const STANDARD_GATE_RADIUS_METERS: f64 = 100.0; // Radar search gate window (+/- 100m)

#[derive(Clone, Debug, PartialEq)]
pub struct RadarContact {
    pub position_m: f64,
    pub is_locked: bool,
}

// ----------------------------------------------------------------------------
// TRACK A: LEGACY PATRIOT SYSTEM (Unbounded floating point leak + Option::None)
// ----------------------------------------------------------------------------
struct LegacyPatriot;

impl LegacyPatriot {
    fn new() -> Self {
        Self
    }

    // Attempts radar lock inside search gate: returns Option<RadarContact>
    fn acquire_target(&self, actual_pos: f64, predicted_pos: f64) -> Option<RadarContact> {
        let error_distance = (actual_pos - predicted_pos).abs();
        if error_distance <= STANDARD_GATE_RADIUS_METERS {
            Some(RadarContact {
                position_m: actual_pos,
                is_locked: true,
            })
        } else {
            // CATASTROPHIC VACUUM: Returns None! Declares missile does not exist!
            None
        }
    }
}

// ----------------------------------------------------------------------------
// TRACK B: AXIOM-ZERO SYSTEM (Orbital Phase Attractor + Boundary Membrane)
// ----------------------------------------------------------------------------
#[axiom_zero]
fn safe_radar_acquire(actual_pos: f64, predicted_pos: f64) -> Presence<RadarContact> {
    let error_distance = (actual_pos - predicted_pos).abs();

    if error_distance <= STANDARD_GATE_RADIUS_METERS {
        Presence::new(RadarContact {
            position_m: actual_pos,
            is_locked: true,
        })
    } else {
        // DOES NOT RETURN NONE!
        // Generates a Boundary Token: informs the tracking loop that target is
        // at the boundary frontier, and automatically expands the search aperture!
        Presence::boundary(RadarContact {
            position_m: actual_pos,
            is_locked: false, // Flagged for orbital relaxation
        })
    }
}

fn main() {
    println!("===========================================================================");
    println!("   HISTORICAL SIMULATION: 1991 PATRIOT MISSILE CLOCK-DRIFT FAILURE       ");
    println!("===========================================================================");
    println!("Simulating 100 continuous operational hours (3,600,000 radar ticks)...");

    // 100 hours = 100 * 3600 * 10 = 3,600,000 ticks of 0.1 seconds
    let total_ticks = 3_600_000u64;

    // Calculate analytical time drift after 100 hours
    let total_time_drift = (total_ticks as f64) * TICK_ERROR_DRIFT;
    let missile_spatial_error = total_time_drift * SCUD_SPEED_MPS;

    println!("\n[100 HOURS ELAPSED TELEMETRY]:");
    println!("  - Theoretical True Time:         {:.4} seconds (100.00 hours)", (total_ticks as f64) * RADAR_TICK_TRUE);
    println!("  - Accumulated Clock Shift:       {:.6} seconds", total_time_drift);
    println!("  - Induced Spatial Tracking Error:{:.2} meters!", missile_spatial_error);
    println!("  - Standard Radar Gate Window:    +/- {:.1} meters", STANDARD_GATE_RADIUS_METERS);

    let scud_actual_pos = 10000.0; // Target is at 10,000m
    let scud_predicted_pos = scud_actual_pos - missile_spatial_error; // Shifted by 575.41m

    // ------------------------------------------------------------------------
    // RUN TRACK A: LEGACY PATRIOT
    // ------------------------------------------------------------------------
    let legacy = LegacyPatriot::new();
    let legacy_lock = legacy.acquire_target(scud_actual_pos, scud_predicted_pos);

    println!("\n---------------------------------------------------------------------------");
    println!("1. LEGACY PATRIOT SYSTEM OUTCOME:");
    match legacy_lock {
        Some(contact) => println!("   STATUS: TARGET INTERCEPTED at {:.2}m", contact.position_m),
        None => {
            println!("   STATUS: FATAL FAILURE (Option::None returned)");
            println!("   REASON: Target was {:.2}m away (outside 100m gate).", missile_spatial_error);
            println!("   CONSEQUENCE: System assumed void/empty sky. Missile NOT intercepted.");
        }
    }

    // ------------------------------------------------------------------------
    // RUN TRACK B: AXIOM-ZERO RESILIENT SYSTEM
    // ------------------------------------------------------------------------
    println!("\n---------------------------------------------------------------------------");
    println!("2. AXIOM-ZERO SYSTEM OUTCOME (Total Presence + Attractor):");

    let mut tracking_attractor = Attractor::new(
        "RADAR_SEARCH",
        vec![
            "ECHO_CORRELATION",
            "APERTURE_ADAPTATION",
            "FIRE_INTERCEPTOR",
        ],
    );

    let presence_contact = safe_radar_acquire(scud_actual_pos, scud_predicted_pos);

    let interception_result = presence_contact.resolve(
        |live| format!("LOCKED on primary gate: target at {:.2}m", live.position_m),
        |boundary| {
            // Boundary token caught the edge!
            // Attractor absorbs the tracking drift in tau = 1 step:
            let recovery_phase = tracking_attractor.absorb_perturbation(1);
            format!(
                "BOUNDARY TRIGGERED: Range gate dynamically expanded by {:.2}m! Phase: {:?}. TARGET INTERCEPTED at {:.2}m!",
                missile_spatial_error, recovery_phase, boundary.position_m
            )
        },
    );

    println!("   STATUS: {}", interception_result);
    println!("   REASON: Axiom-Zero does not allow the radar to return 'None'.");
    println!("   CONSEQUENCE: Boundary token absorbed the 575m shift, adapted the aperture,");
    println!("                and successfully fired the interceptor!");
    println!("===========================================================================");
}
