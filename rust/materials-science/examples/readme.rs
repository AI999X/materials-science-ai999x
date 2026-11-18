//! Materials Science: 15 calculations through the Rust library (the same engine as the desktop app, the
//! command line and the browser). Run from the rust/ folder:
//!     cargo run --release --example readme -p materials-science-ai999x
//! Inputs are text, read like a form field ("2 m", "0.5", "x^3 - 3x"); a JSON list stays a list.

use serde_json::{json, Value};

/// One calculation: the request in, its outcome and every value with its unit out.
fn show(request: Value) {
    let answer: Value = serde_json::from_str(&materials_science_ai999x::solve_json(&request.to_string())).unwrap();
    println!("{}", answer["outcome"].as_str().unwrap_or(""));
    for v in answer["values"].as_array().into_iter().flatten() {
        let unit = v[2].as_str().unwrap_or("");
        let unit = if unit.is_empty() || unit == "1" { String::new() } else { format!(" {unit}") };
        println!("{} = {}{}", v[0].as_str().unwrap_or(""), v[1], unit);
    }
}

fn main() {
    show(json!({"key": "radius_ratio_coordination", "inputs": {"rc": "0.102 nm", "ra": "0.181 nm"}}));
    println!();
    show(json!({"key": "critical_fiber_length", "inputs": {"sigma_f": "3.5 GPa", "d": "10 um", "tau_c": "20 MPa"}}));
    println!();
    show(json!({"key": "angle_between_directions_cubic", "inputs": {"u1": "1", "v1": "0", "w1": "0", "u2": "1", "v2": "1", "w2": "1"}}));
    println!();
    show(json!({"key": "burgers_vector", "inputs": {"structure": "FCC", "a": "0.3615 nm"}}));
    println!();
    show(json!({"key": "bragg_angle", "inputs": {"d": "0.2087 nm", "lam": "0.1542 nm"}}));
    println!();
    show(json!({"key": "diffusion_arrhenius_fit", "inputs": {"T": [773, 873], "D": [5.8e-13, 5.3e-12]}}));
    println!();
    show(json!({"key": "band_gap_from_conductivity", "inputs": {"T": [300, 400], "sigma": [2.2, 63]}}));
    println!();
    show(json!({"key": "critical_crack_length", "inputs": {"K_IC": "55 MPa*m^0.5", "sigma": "1000 MPa"}}));
    println!();
    show(json!({"key": "offset_yield_strength", "inputs": {"strain": [0, 0.002, 0.004, 0.006, 0.008], "stress": [0, 400, 520, 560, 580], "E": "200 GPa"}}));
    println!();
    show(json!({"key": "atomic_to_weight_percent", "inputs": {"element1": "Al", "element2": "Cu", "at1": "90"}}));
    println!();
    show(json!({"key": "avrami_kinetics", "inputs": {"t": [50, 100], "y": [0.2, 0.8]}}));
    println!();
    show(json!({"key": "copolymer_degree_of_polymerization", "inputs": {"formula1": "C4H6", "formula2": "C8H8", "f1": "0.75", "Mn": "350000 g/mol"}}));
    println!();
    show(json!({"key": "wiedemann_franz", "inputs": {"T": "300 K", "sigma": "59600000 S/m"}}));
    println!();
    show(json!({"key": "ceramic_density", "inputs": {"prototype": "rocksalt", "rc": "0.102 nm", "ra": "0.181 nm", "Ac": "22.99 g/mol", "Aa": "35.45 g/mol"}}));
    println!();
    show(json!({"key": "composite_rule_of_mixtures", "inputs": {"Ef": "69 GPa", "Em": "3.4 GPa", "Vf": "0.4"}}));
}
