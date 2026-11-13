//! materials_science: procedures translated from the package's Python by tools/py2rust.py (do not edit; regenerate).
//! Source: models/materials_science/capabilities/procedures.py. Each keeps Python's number rules through
//! ax_engine::py and is checked against the package by tools/rust_accuracy.py before the build is listed.
#![allow(clippy::all, non_snake_case, unused_parens, unused_braces, unused_mut, unused_variables, unreachable_code)]

use ax_engine::py::{self, Py, PyErr, R};
use ax_engine::Val;
use serde_json::Value;
use std::collections::HashMap;

/// The procedures this crate implements (tools/export_rust_pack.py reads this list).
pub const NATIVE: &[&str] = &[
    "allowed_reflections_cubic",
    "angle_between_directions",
    "angle_between_directions_cubic",
    "angle_between_planes",
    "arrhenius_diffusivity",
    "atomic_packing_factor",
    "atomic_to_weight_percent",
    "avrami_kinetics",
    "band_gap_from_conductivity",
    "bragg_angle",
    "bragg_spacing",
    "bragg_wavelength",
    "burgers_vector",
    "ceramic_density",
    "composite_rule_of_mixtures",
    "copolymer_degree_of_polymerization",
    "creep_parameters_fit",
    "critical_crack_length",
    "critical_fiber_length",
    "cubic_lattice_parameter",
    "cubic_reflection_angle",
    "d_spacing_cubic",
    "d_spacing_hexagonal",
    "d_spacing_orthorhombic",
    "d_spacing_tetragonal",
    "d_spacing_triclinic",
    "debye_heat_capacity",
    "degree_of_polymerization",
    "diffusion_arrhenius_fit",
    "diffusion_depth_of_concentration",
    "diffusion_time_to_concentration",
    "einstein_heat_capacity",
    "elastic_deformation_of_a_rod",
    "eutectic_microconstituents",
    "extrinsic_carrier_concentration",
    "fermi_energy_free_electron",
    "fick_first_law_flux",
    "fick_second_law_concentration",
    "fracture_critical_stress",
    "gibbs_phase_rule",
    "griffith_critical_stress",
    "hall_petch",
    "hcp_cell_density",
    "homogeneous_nucleation",
    "interstitial_site_size",
    "intrinsic_carrier_concentration",
    "iron_carbon_microconstituents",
    "isotropic_elastic_constants",
    "kelvin_voigt_creep",
    "larson_miller_parameter",
    "lattice_parameter_from_density",
    "lever_rule",
    "linear_density",
    "linear_thermal_expansion",
    "max_schmid_factor",
    "maxwell_stress_relaxation",
    "miller_bravais_direction",
    "miller_bravais_to_three_index",
    "miller_indices_from_intercepts",
    "modulus_of_resilience",
    "molecular_weight_averages",
    "offset_yield_strength",
    "paris_law_fatigue_life",
    "planar_density",
    "pn_junction_built_in_potential",
    "polymer_chain_dimensions",
    "polymer_crystallinity",
    "polymorphic_volume_change",
    "radius_ratio_coordination",
    "rubber_elasticity",
    "schmid_factor",
    "space_group",
    "strain_hardening_exponent",
    "structure_factor_cubic",
    "tensile_toughness",
    "theoretical_density",
    "thermal_stress",
    "true_stress_and_strain",
    "unit_cell_volume",
    "vacancy_concentration",
    "vacancy_formation_energy_fit",
    "volumetric_thermal_expansion",
    "weibull_design_stress",
    "weibull_failure_probability",
    "weibull_modulus_fit",
    "weight_to_atomic_percent",
    "wiedemann_franz",
    "youngs_modulus_from_data",
];

type Out = Result<(Vec<(String, Value)>, f64), String>;

/// The run's result as the engine's answer: a dict of outputs and the residual of the package's own check; a
/// ValueError is the refusal's reason; any other error is an internal error (None).
fn finish(r: R, residual: impl Fn(&Py, &Py) -> f64, v: &HashMap<String, Val>) -> Option<Out> {
    let o = match r {
        Ok(o) => o,
        Err(PyErr::Value(m)) => return Some(Err(m)),
        Err(e) => {
            if std::env::var_os("AX_DEBUG").is_some() {        // the error behind an internal error, for diagnosis
                eprintln!("internal error: {e:?}");
            }
            return None;
        }
    };
    let Py::Dict(d) = &o else { return None };
    let outs: Vec<(String, Value)> = d.borrow().iter().map(|(k, x)| (py::to_str(k), py::to_json(x))).collect();
    let i = Py::dict(v.iter().map(|(k, x)| (Py::str(k), py::from_val(x))).collect());
    let res = residual(&i, &o);
    if std::env::var_os("AX_DEBUG").is_some() && !(res <= 1e-9) {
        eprintln!("check failed: residual {res}");
    }
    Some(Ok((outs, res)))
}

/// The engine's entry point (ax_engine::pack::NativeFn): None for a key that is not one of these procedures.
pub fn native(key: &str, v: &HashMap<String, Val>) -> Option<Out> {
    match key {
        "allowed_reflections_cubic" => {
            let r = f_allowed_reflections(match v.get("structure") { Some(x) => py::from_val(x), None => { return None } }, match v.get("count") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Int(6i64)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_31(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "angle_between_directions" => {
            let r = f_angle_directions_general(match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c") { Some(x) => py::from_val(x), None => { return None } }, match v.get("alpha") { Some(x) => py::from_val(x), None => { return None } }, match v.get("beta") { Some(x) => py::from_val(x), None => { return None } }, match v.get("gamma") { Some(x) => py::from_val(x), None => { return None } }, match v.get("u1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("v1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("w1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("u2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("v2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("w2") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_94(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "angle_between_directions_cubic" => {
            let r = f_angle_directions_cubic(match v.get("u1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("v1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("w1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("u2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("v2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("w2") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_142(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "angle_between_planes" => {
            let r = f_angle_planes_general(match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c") { Some(x) => py::from_val(x), None => { return None } }, match v.get("alpha") { Some(x) => py::from_val(x), None => { return None } }, match v.get("beta") { Some(x) => py::from_val(x), None => { return None } }, match v.get("gamma") { Some(x) => py::from_val(x), None => { return None } }, match v.get("h1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("l1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("h2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("l2") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_209(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "arrhenius_diffusivity" => {
            let r = f_arrhenius_diffusivity(match v.get("D0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Q") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_246(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "atomic_packing_factor" => {
            let r = f_packing_factor(match v.get("structure") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_280(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "atomic_to_weight_percent" => {
            let r = f_at_to_wt(match v.get("element1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("element2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("at1") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_321(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "avrami_kinetics" => {
            let r = f_avrami_fit(match v.get("t") { Some(x) => py::from_val(x), None => { return None } }, match v.get("y") { Some(x) => py::from_val(x), None => { return None } }, match v.get("y_target") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_391(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "band_gap_from_conductivity" => {
            let r = f_band_gap_fit(match v.get("T") { Some(x) => py::from_val(x), None => { return None } }, match v.get("sigma") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_450(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bragg_angle" => {
            let r = f_bragg_angle(match v.get("d") { Some(x) => py::from_val(x), None => { return None } }, match v.get("lam") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Int(1i64)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_491(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bragg_spacing" => {
            let r = f_bragg_spacing(match v.get("lam") { Some(x) => py::from_val(x), None => { return None } }, match v.get("theta") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Int(1i64)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_533(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bragg_wavelength" => {
            let r = f_bragg_wavelength(match v.get("d") { Some(x) => py::from_val(x), None => { return None } }, match v.get("theta") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Int(1i64)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_575(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "burgers_vector" => {
            let r = f_burgers_vector(match v.get("structure") { Some(x) => py::from_val(x), None => { return None } }, match v.get("a") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_610(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ceramic_density" => {
            let r = f_ceramic_density(match v.get("prototype") { Some(x) => py::from_val(x), None => { return None } }, match v.get("rc") { Some(x) => py::from_val(x), None => { return None } }, match v.get("ra") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Ac") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Aa") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_648(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "composite_rule_of_mixtures" => {
            let r = f_rule_of_mixtures(match v.get("Ef") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Em") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Vf") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_692(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "copolymer_degree_of_polymerization" => {
            let r = f_copolymer_dp(match v.get("formula1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("formula2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("f1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Mn") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_760(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "creep_parameters_fit" => {
            let r = f_creep_fit(match v.get("stress_MPa") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T") { Some(x) => py::from_val(x), None => { return None } }, match v.get("rate") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_824(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "critical_crack_length" => {
            let r = f_critical_crack(match v.get("K_IC") { Some(x) => py::from_val(x), None => { return None } }, match v.get("sigma") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Y") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(1.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_861(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "critical_fiber_length" => {
            let r = f_critical_fiber_length(match v.get("sigma_f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("d") { Some(x) => py::from_val(x), None => { return None } }, match v.get("tau_c") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_898(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cubic_lattice_parameter" => {
            let r = f_cubic_lattice_parameter(match v.get("structure") { Some(x) => py::from_val(x), None => { return None } }, match v.get("R") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_935(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cubic_reflection_angle" => {
            let r = f_cubic_reflection(match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("h") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k") { Some(x) => py::from_val(x), None => { return None } }, match v.get("l") { Some(x) => py::from_val(x), None => { return None } }, match v.get("lam") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_984(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "d_spacing_cubic" => {
            let r = f_d_cubic(match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("h") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k") { Some(x) => py::from_val(x), None => { return None } }, match v.get("l") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1031(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "d_spacing_hexagonal" => {
            let r = f_d_hexagonal(match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c") { Some(x) => py::from_val(x), None => { return None } }, match v.get("h") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k") { Some(x) => py::from_val(x), None => { return None } }, match v.get("l") { Some(x) => py::from_val(x), None => { return None } }, match v.get("i") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1103(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "d_spacing_orthorhombic" => {
            let r = f_d_orthorhombic(match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c") { Some(x) => py::from_val(x), None => { return None } }, match v.get("h") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k") { Some(x) => py::from_val(x), None => { return None } }, match v.get("l") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1170(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "d_spacing_tetragonal" => {
            let r = f_d_tetragonal(match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c") { Some(x) => py::from_val(x), None => { return None } }, match v.get("h") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k") { Some(x) => py::from_val(x), None => { return None } }, match v.get("l") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1237(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "d_spacing_triclinic" => {
            let r = f_d_triclinic(match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c") { Some(x) => py::from_val(x), None => { return None } }, match v.get("alpha") { Some(x) => py::from_val(x), None => { return None } }, match v.get("beta") { Some(x) => py::from_val(x), None => { return None } }, match v.get("gamma") { Some(x) => py::from_val(x), None => { return None } }, match v.get("h") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k") { Some(x) => py::from_val(x), None => { return None } }, match v.get("l") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1320(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "debye_heat_capacity" => {
            let r = f_debye_heat_capacity(match v.get("theta_D") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1347(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "degree_of_polymerization" => {
            let r = f_degree_of_polymerization(match v.get("formula") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Mn") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1407(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "diffusion_arrhenius_fit" => {
            let r = f_arrhenius_fit(match v.get("T") { Some(x) => py::from_val(x), None => { return None } }, match v.get("D") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T_target") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1470(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "diffusion_depth_of_concentration" => {
            let r = f_diffusion_depth(match v.get("C0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Cs") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Cx") { Some(x) => py::from_val(x), None => { return None } }, match v.get("D") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1521(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "diffusion_time_to_concentration" => {
            let r = f_diffusion_time(match v.get("C0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Cs") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Cx") { Some(x) => py::from_val(x), None => { return None } }, match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("D") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1572(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "einstein_heat_capacity" => {
            let r = f_einstein_heat_capacity(match v.get("theta_E") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1611(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "elastic_deformation_of_a_rod" => {
            let r = f_elastic_deformation(match v.get("F") { Some(x) => py::from_val(x), None => { return None } }, match v.get("d0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("l0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("E") { Some(x) => py::from_val(x), None => { return None } }, match v.get("nu") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1651(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "eutectic_microconstituents" => {
            let r = f_eutectic_constituents(match v.get("C0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("C_alpha") { Some(x) => py::from_val(x), None => { return None } }, match v.get("C_E") { Some(x) => py::from_val(x), None => { return None } }, match v.get("C_beta") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1701(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "extrinsic_carrier_concentration" => {
            let r = f_extrinsic_carriers(match v.get("Nd") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Na") { Some(x) => py::from_val(x), None => { return None } }, match v.get("ni") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1755(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "fermi_energy_free_electron" => {
            let r = f_fermi_energy(match v.get("rho") { Some(x) => py::from_val(x), None => { return None } }, match v.get("A") { Some(x) => py::from_val(x), None => { return None } }, match v.get("valence") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1792(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "fick_first_law_flux" => {
            let r = f_fick_first(match v.get("D") { Some(x) => py::from_val(x), None => { return None } }, match v.get("C1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("C2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("x1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("x2") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1831(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "fick_second_law_concentration" => {
            let r = f_fick_second(match v.get("C0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Cs") { Some(x) => py::from_val(x), None => { return None } }, match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("D") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1870(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "fracture_critical_stress" => {
            let r = f_fracture_stress(match v.get("K_IC") { Some(x) => py::from_val(x), None => { return None } }, match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Y") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(1.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1907(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "gibbs_phase_rule" => {
            let r = f_gibbs_phase_rule(match v.get("C") { Some(x) => py::from_val(x), None => { return None } }, match v.get("P") { Some(x) => py::from_val(x), None => { return None } }, match v.get("pressure") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::str("variable")) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1941(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "griffith_critical_stress" => {
            let r = f_griffith_stress(match v.get("E") { Some(x) => py::from_val(x), None => { return None } }, match v.get("gamma_s") { Some(x) => py::from_val(x), None => { return None } }, match v.get("a") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1978(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hall_petch" => {
            let r = f_hall_petch_fit(match v.get("d_um") { Some(x) => py::from_val(x), None => { return None } }, match v.get("yield_MPa") { Some(x) => py::from_val(x), None => { return None } }, match v.get("d_target_um") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2037(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hcp_cell_density" => {
            let r = f_hcp_cell(match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c") { Some(x) => py::from_val(x), None => { return None } }, match v.get("A") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2074(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "homogeneous_nucleation" => {
            let r = f_nucleation(match v.get("gamma") { Some(x) => py::from_val(x), None => { return None } }, match v.get("dHf") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Tm") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2114(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "interstitial_site_size" => {
            let r = f_interstitial_site(match v.get("structure") { Some(x) => py::from_val(x), None => { return None } }, match v.get("site") { Some(x) => py::from_val(x), None => { return None } }, match v.get("R") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2151(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "intrinsic_carrier_concentration" => {
            let r = f_intrinsic_carriers(match v.get("Eg") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T") { Some(x) => py::from_val(x), None => { return None } }, match v.get("me") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(1.0)) })().ok()? } }, match v.get("mh") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(1.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2189(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "iron_carbon_microconstituents" => {
            let r = f_iron_carbon(match v.get("C0") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2229(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "isotropic_elastic_constants" => {
            let r = f_elastic_constants(match v.get("E") { Some(x) => py::from_val(x), None => { return None } }, match v.get("nu") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2269(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "kelvin_voigt_creep" => {
            let r = f_kelvin_voigt_creep(match v.get("sigma0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("E") { Some(x) => py::from_val(x), None => { return None } }, match v.get("eta") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2308(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "larson_miller_parameter" => {
            let r = f_larson_miller(match v.get("T") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t_r") { Some(x) => py::from_val(x), None => { return None } }, match v.get("C") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(20.0)) })().ok()? } }, match v.get("T2") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2355(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "lattice_parameter_from_density" => {
            let r = f_lattice_from_density(match v.get("structure") { Some(x) => py::from_val(x), None => { return None } }, match v.get("rho") { Some(x) => py::from_val(x), None => { return None } }, match v.get("A") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2394(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "lever_rule" => {
            let r = f_lever_rule(match v.get("C0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("C1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("C2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("mass") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2447(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "linear_density" => {
            let r = f_linear_density(match v.get("structure") { Some(x) => py::from_val(x), None => { return None } }, match v.get("u") { Some(x) => py::from_val(x), None => { return None } }, match v.get("v") { Some(x) => py::from_val(x), None => { return None } }, match v.get("w") { Some(x) => py::from_val(x), None => { return None } }, match v.get("a") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2508(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "linear_thermal_expansion" => {
            let r = f_linear_expansion(match v.get("alpha") { Some(x) => py::from_val(x), None => { return None } }, match v.get("L0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("dT") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2547(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "max_schmid_factor" => {
            let r = f_max_schmid(match v.get("structure") { Some(x) => py::from_val(x), None => { return None } }, match v.get("load") { Some(x) => py::from_val(x), None => { return None } }, match v.get("tau_crss") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2608(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "maxwell_stress_relaxation" => {
            let r = f_maxwell_relaxation(match v.get("sigma0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("E") { Some(x) => py::from_val(x), None => { return None } }, match v.get("eta") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2647(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "miller_bravais_direction" => {
            let r = f_three_to_four(match v.get("U") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V") { Some(x) => py::from_val(x), None => { return None } }, match v.get("W") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2684(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "miller_bravais_to_three_index" => {
            let r = f_four_to_three(match v.get("u") { Some(x) => py::from_val(x), None => { return None } }, match v.get("v") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } }, match v.get("w") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2711(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "miller_indices_from_intercepts" => {
            let r = f_miller_from_intercepts(match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("y") { Some(x) => py::from_val(x), None => { return None } }, match v.get("z") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f_verify_intercepts(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "modulus_of_resilience" => {
            let r = f_resilience(match v.get("sy") { Some(x) => py::from_val(x), None => { return None } }, match v.get("E") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2774(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "molecular_weight_averages" => {
            let r = f_mw_averages(match v.get("M") { Some(x) => py::from_val(x), None => { return None } }, match v.get("fraction") { Some(x) => py::from_val(x), None => { return None } }, match v.get("basis") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::str("number")) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2831(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "offset_yield_strength" => {
            let r = f_offset_yield(match v.get("strain") { Some(x) => py::from_val(x), None => { return None } }, match v.get("stress") { Some(x) => py::from_val(x), None => { return None } }, match v.get("E") { Some(x) => py::from_val(x), None => { return None } }, match v.get("offset") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.002)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2888(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "paris_law_fatigue_life" => {
            let r = f_paris_life(match v.get("C") { Some(x) => py::from_val(x), None => { return None } }, match v.get("m") { Some(x) => py::from_val(x), None => { return None } }, match v.get("dsigma") { Some(x) => py::from_val(x), None => { return None } }, match v.get("a0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Y") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(1.0)) })().ok()? } }, match v.get("af") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("K_IC") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("sigma_max") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2947(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "planar_density" => {
            let r = f_planar_density(match v.get("structure") { Some(x) => py::from_val(x), None => { return None } }, match v.get("h") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k") { Some(x) => py::from_val(x), None => { return None } }, match v.get("l") { Some(x) => py::from_val(x), None => { return None } }, match v.get("a") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3005(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "pn_junction_built_in_potential" => {
            let r = f_built_in_potential(match v.get("Na") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Nd") { Some(x) => py::from_val(x), None => { return None } }, match v.get("ni") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T") { Some(x) => py::from_val(x), None => { return None } }, match v.get("eps_r") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3052(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "polymer_chain_dimensions" => {
            let r = f_chain_dimensions(match v.get("n") { Some(x) => py::from_val(x), None => { return None } }, match v.get("l") { Some(x) => py::from_val(x), None => { return None } }, match v.get("theta") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(109.47)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3098(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "polymer_crystallinity" => {
            let r = f_crystallinity(match v.get("rho_s") { Some(x) => py::from_val(x), None => { return None } }, match v.get("rho_c") { Some(x) => py::from_val(x), None => { return None } }, match v.get("rho_a") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3138(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "polymorphic_volume_change" => {
            let r = f_polymorph_volume_change(match v.get("R_bcc") { Some(x) => py::from_val(x), None => { return None } }, match v.get("R_fcc") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3175(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "radius_ratio_coordination" => {
            let r = f_radius_ratio(match v.get("rc") { Some(x) => py::from_val(x), None => { return None } }, match v.get("ra") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3218(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "rubber_elasticity" => {
            let r = f_rubber_elasticity(match v.get("rho") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Mc") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T") { Some(x) => py::from_val(x), None => { return None } }, match v.get("stretch") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3255(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "schmid_factor" => {
            let r = f_schmid(match v.get("load") { Some(x) => py::from_val(x), None => { return None } }, match v.get("plane") { Some(x) => py::from_val(x), None => { return None } }, match v.get("direction") { Some(x) => py::from_val(x), None => { return None } }, match v.get("sigma") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("tau_crss") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3313(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "space_group" => {
            let r = f_space_group(match v.get("structure") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3342(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "strain_hardening_exponent" => {
            let r = f_hollomon_fit(match v.get("true_strain") { Some(x) => py::from_val(x), None => { return None } }, match v.get("true_stress") { Some(x) => py::from_val(x), None => { return None } }, match v.get("strain_target") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3402(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "structure_factor_cubic" => {
            let r = f_structure_factor(match v.get("structure") { Some(x) => py::from_val(x), None => { return None } }, match v.get("h") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k") { Some(x) => py::from_val(x), None => { return None } }, match v.get("l") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f_verify_structure_factor(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "tensile_toughness" => {
            let r = f_tensile_toughness(match v.get("strain") { Some(x) => py::from_val(x), None => { return None } }, match v.get("stress") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3488(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "theoretical_density" => {
            let r = f_theoretical_density(match v.get("structure") { Some(x) => py::from_val(x), None => { return None } }, match v.get("A") { Some(x) => py::from_val(x), None => { return None } }, match v.get("R") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3527(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "thermal_stress" => {
            let r = f_thermal_stress(match v.get("E") { Some(x) => py::from_val(x), None => { return None } }, match v.get("alpha") { Some(x) => py::from_val(x), None => { return None } }, match v.get("dT") { Some(x) => py::from_val(x), None => { return None } }, match v.get("constraint") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::str("uniaxial")) })().ok()? } }, match v.get("nu") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3573(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "true_stress_and_strain" => {
            let r = f_true_stress_strain(match v.get("stress") { Some(x) => py::from_val(x), None => { return None } }, match v.get("strain") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3607(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "unit_cell_volume" => {
            let r = f_cell_volume(match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c") { Some(x) => py::from_val(x), None => { return None } }, match v.get("alpha") { Some(x) => py::from_val(x), None => { return None } }, match v.get("beta") { Some(x) => py::from_val(x), None => { return None } }, match v.get("gamma") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3653(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "vacancy_concentration" => {
            let r = f_vacancy_concentration(match v.get("rho") { Some(x) => py::from_val(x), None => { return None } }, match v.get("A") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Qv") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3690(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "vacancy_formation_energy_fit" => {
            let r = f_vacancy_energy_fit(match v.get("T") { Some(x) => py::from_val(x), None => { return None } }, match v.get("fraction") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3757(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "volumetric_thermal_expansion" => {
            let r = f_volumetric_expansion(match v.get("alpha") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("dT") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3796(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "weibull_design_stress" => {
            let r = f_weibull_stress(match v.get("P_failure") { Some(x) => py::from_val(x), None => { return None } }, match v.get("sigma0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("m") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V_ratio") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(1.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3840(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "weibull_failure_probability" => {
            let r = f_weibull_failure(match v.get("sigma") { Some(x) => py::from_val(x), None => { return None } }, match v.get("sigma0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("m") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V_ratio") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(1.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3877(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "weibull_modulus_fit" => {
            let r = f_weibull_fit(match v.get("strengths") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3937(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "weight_to_atomic_percent" => {
            let r = f_wt_to_at(match v.get("element1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("element2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("wt1") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3978(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "wiedemann_franz" => {
            let r = f_wiedemann_franz(match v.get("T") { Some(x) => py::from_val(x), None => { return None } }, match v.get("sigma") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("rho") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4025(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "youngs_modulus_from_data" => {
            let r = f_youngs_modulus_fit(match v.get("strain") { Some(x) => py::from_val(x), None => { return None } }, match v.get("stress") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4079(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        _ => None,
    }
}

#[allow(non_snake_case)]
fn c_ATOMIC_WEIGHT() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::dict(vec![(Py::str("H"), Py::Float(1.00794)), (Py::str("He"), Py::Float(4.002602)), (Py::str("Li"), Py::Float(6.941)), (Py::str("Be"), Py::Float(9.012182)), (Py::str("B"), Py::Float(10.811)), (Py::str("C"), Py::Float(12.0107)), (Py::str("N"), Py::Float(14.0067)), (Py::str("O"), Py::Float(15.9994)), (Py::str("F"), Py::Float(18.9984032)), (Py::str("Ne"), Py::Float(20.1797)), (Py::str("Na"), Py::Float(22.98976928)), (Py::str("Mg"), Py::Float(24.305)), (Py::str("Al"), Py::Float(26.9815386)), (Py::str("Si"), Py::Float(28.0855)), (Py::str("P"), Py::Float(30.973762)), (Py::str("S"), Py::Float(32.065)), (Py::str("Cl"), Py::Float(35.453)), (Py::str("Ar"), Py::Float(39.948)), (Py::str("K"), Py::Float(39.0983)), (Py::str("Ca"), Py::Float(40.078)), (Py::str("Sc"), Py::Float(44.955912)), (Py::str("Ti"), Py::Float(47.867)), (Py::str("V"), Py::Float(50.9415)), (Py::str("Cr"), Py::Float(51.9961)), (Py::str("Mn"), Py::Float(54.938045)), (Py::str("Fe"), Py::Float(55.845)), (Py::str("Co"), Py::Float(58.933195)), (Py::str("Ni"), Py::Float(58.6934)), (Py::str("Cu"), Py::Float(63.546)), (Py::str("Zn"), Py::Float(65.409)), (Py::str("Ga"), Py::Float(69.723)), (Py::str("Ge"), Py::Float(72.64)), (Py::str("As"), Py::Float(74.9216)), (Py::str("Se"), Py::Float(78.96)), (Py::str("Br"), Py::Float(79.904)), (Py::str("Kr"), Py::Float(83.798)), (Py::str("Rb"), Py::Float(85.4678)), (Py::str("Sr"), Py::Float(87.62)), (Py::str("Y"), Py::Float(88.90585)), (Py::str("Zr"), Py::Float(91.224)), (Py::str("Nb"), Py::Float(92.90638)), (Py::str("Mo"), Py::Float(95.94)), (Py::str("Tc"), Py::Float(98.0)), (Py::str("Ru"), Py::Float(101.07)), (Py::str("Rh"), Py::Float(102.9055)), (Py::str("Pd"), Py::Float(106.42)), (Py::str("Ag"), Py::Float(107.8682)), (Py::str("Cd"), Py::Float(112.411)), (Py::str("In"), Py::Float(114.818)), (Py::str("Sn"), Py::Float(118.71)), (Py::str("Sb"), Py::Float(121.76)), (Py::str("Te"), Py::Float(127.6)), (Py::str("I"), Py::Float(126.90447)), (Py::str("Xe"), Py::Float(131.293)), (Py::str("Cs"), Py::Float(132.9054519)), (Py::str("Ba"), Py::Float(137.327)), (Py::str("La"), Py::Float(138.90547)), (Py::str("Ce"), Py::Float(140.116)), (Py::str("Pr"), Py::Float(140.90765)), (Py::str("Nd"), Py::Float(144.242)), (Py::str("Pm"), Py::Float(145.0)), (Py::str("Sm"), Py::Float(150.36)), (Py::str("Eu"), Py::Float(151.964)), (Py::str("Gd"), Py::Float(157.25)), (Py::str("Tb"), Py::Float(158.92535)), (Py::str("Dy"), Py::Float(162.5)), (Py::str("Ho"), Py::Float(164.93032)), (Py::str("Er"), Py::Float(167.259)), (Py::str("Tm"), Py::Float(168.93421)), (Py::str("Yb"), Py::Float(173.04)), (Py::str("Lu"), Py::Float(174.967)), (Py::str("Hf"), Py::Float(178.49)), (Py::str("Ta"), Py::Float(180.94788)), (Py::str("W"), Py::Float(183.84)), (Py::str("Re"), Py::Float(186.207)), (Py::str("Os"), Py::Float(190.23)), (Py::str("Ir"), Py::Float(192.217)), (Py::str("Pt"), Py::Float(195.084)), (Py::str("Au"), Py::Float(196.966569)), (Py::str("Hg"), Py::Float(200.59)), (Py::str("Tl"), Py::Float(204.3833)), (Py::str("Pb"), Py::Float(207.2)), (Py::str("Bi"), Py::Float(208.9804)), (Py::str("Po"), Py::Float(210.0)), (Py::str("At"), Py::Float(210.0)), (Py::str("Rn"), Py::Float(220.0)), (Py::str("Fr"), Py::Float(223.0)), (Py::str("Ra"), Py::Float(226.0)), (Py::str("Ac"), Py::Float(227.0)), (Py::str("Th"), Py::Float(232.03806)), (Py::str("Pa"), Py::Float(231.03588)), (Py::str("U"), Py::Float(238.02891)), (Py::str("Np"), Py::Float(237.0)), (Py::str("Pu"), Py::Float(244.0))])) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_EPS0() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(8.8541878188e-12)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_HBAR() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(1.0545718176461565e-34)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_HP() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(6.62607015e-34)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_KB() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(1.380649e-23)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_ME() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(9.1093837139e-31)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_NA() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(6.02214076e+23)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_QE() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(1.602176634e-19)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_RGAS() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(8.31446261815324)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_SQ2() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(1.4142135623730951)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_SQ3() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(1.7320508075688772)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__A_PER_R() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::dict(vec![(Py::str("SC"), Py::Float(2.0)), (Py::str("BCC"), Py::Float(2.3094010767585034)), (Py::str("FCC"), Py::Float(2.8284271247461903)), (Py::str("diamond"), Py::Float(4.618802153517007)), (Py::str("HCP"), Py::Float(2.0))])) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__BASIS() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::dict(vec![(Py::str("SC"), Py::list(vec![Py::tuple(vec![Py::Int(0i64), Py::Int(0i64), Py::Int(0i64)])])), (Py::str("BCC"), Py::list(vec![Py::tuple(vec![Py::Int(0i64), Py::Int(0i64), Py::Int(0i64)]), Py::tuple(vec![Py::Float(0.5), Py::Float(0.5), Py::Float(0.5)])])), (Py::str("FCC"), Py::list(vec![Py::tuple(vec![Py::Int(0i64), Py::Int(0i64), Py::Int(0i64)]), Py::tuple(vec![Py::Float(0.5), Py::Float(0.5), Py::Int(0i64)]), Py::tuple(vec![Py::Float(0.5), Py::Int(0i64), Py::Float(0.5)]), Py::tuple(vec![Py::Int(0i64), Py::Float(0.5), Py::Float(0.5)])])), (Py::str("diamond"), Py::list(vec![Py::tuple(vec![Py::Int(0i64), Py::Int(0i64), Py::Int(0i64)]), Py::tuple(vec![Py::Float(0.5), Py::Float(0.5), Py::Int(0i64)]), Py::tuple(vec![Py::Float(0.5), Py::Int(0i64), Py::Float(0.5)]), Py::tuple(vec![Py::Int(0i64), Py::Float(0.5), Py::Float(0.5)]), Py::tuple(vec![Py::Float(0.25), Py::Float(0.25), Py::Float(0.25)]), Py::tuple(vec![Py::Float(0.75), Py::Float(0.75), Py::Float(0.25)]), Py::tuple(vec![Py::Float(0.75), Py::Float(0.25), Py::Float(0.75)]), Py::tuple(vec![Py::Float(0.25), Py::Float(0.75), Py::Float(0.75)])]))])) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__CERAMIC() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::dict(vec![(Py::str("rocksalt"), Py::tuple(vec![Py::Int(4i64), Py::Int(1i64), Py::Float(2.0)])), (Py::str("CsCl"), Py::tuple(vec![Py::Int(1i64), Py::Int(1i64), Py::Float(1.1547005383792517)])), (Py::str("zincblende"), Py::tuple(vec![Py::Int(4i64), Py::Int(1i64), Py::Float(2.3094010767585034)])), (Py::str("fluorite"), Py::tuple(vec![Py::Int(4i64), Py::Int(2i64), Py::Float(2.3094010767585034)]))])) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__CN() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::dict(vec![(Py::str("SC"), Py::Int(6i64)), (Py::str("BCC"), Py::Int(8i64)), (Py::str("FCC"), Py::Int(12i64)), (Py::str("diamond"), Py::Int(4i64)), (Py::str("HCP"), Py::Int(12i64))])) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__COA() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(1.632993161855452)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__CRIT() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::list(vec![Py::tuple(vec![Py::Float(0.15470053837925168), Py::Int(3i64), Py::str("triangular")]), Py::tuple(vec![Py::Float(0.22474487139158894), Py::Int(4i64), Py::str("tetrahedral")]), Py::tuple(vec![Py::Float(0.41421356237309515), Py::Int(6i64), Py::str("octahedral")]), Py::tuple(vec![Py::Float(0.7320508075688772), Py::Int(8i64), Py::str("cubic")])])) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__EL() -> R {
    thread_local!(static V: R = (|| -> R { Ok(ax_engine::pyre::module_call("compile", &Py::str("([A-Z][a-z]?)(\\d*\\.?\\d*)"), &[], 0)?) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__FE_A() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(0.022)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__FE_C() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(6.7)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__FE_E() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(0.76)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__LORENZ() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(2.443004509073667e-08)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__NN_PER_A() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::dict(vec![(Py::str("SC"), Py::Float(1.0)), (Py::str("BCC"), Py::Float(0.8660254037844386)), (Py::str("FCC"), Py::Float(0.7071067811865475))])) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__N_ATOMS() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::dict(vec![(Py::str("SC"), Py::Int(1i64)), (Py::str("BCC"), Py::Int(2i64)), (Py::str("FCC"), Py::Int(4i64)), (Py::str("diamond"), Py::Int(8i64)), (Py::str("HCP"), Py::Int(6i64))])) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__SG() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::dict(vec![(Py::str("SC"), Py::tuple(vec![Py::Int(221i64), Py::str("Pm-3m")])), (Py::str("BCC"), Py::tuple(vec![Py::Int(229i64), Py::str("Im-3m")])), (Py::str("FCC"), Py::tuple(vec![Py::Int(225i64), Py::str("Fm-3m")])), (Py::str("HCP"), Py::tuple(vec![Py::Int(194i64), Py::str("P6_3/mmc")])), (Py::str("diamond"), Py::tuple(vec![Py::Int(227i64), Py::str("Fd-3m")])), (Py::str("rocksalt"), Py::tuple(vec![Py::Int(225i64), Py::str("Fm-3m")])), (Py::str("CsCl"), Py::tuple(vec![Py::Int(221i64), Py::str("Pm-3m")])), (Py::str("zincblende"), Py::tuple(vec![Py::Int(216i64), Py::str("F-43m")])), (Py::str("fluorite"), Py::tuple(vec![Py::Int(225i64), Py::str("Fm-3m")]))])) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__SITE_RATIO() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::dict(vec![(Py::tuple(vec![Py::str("FCC"), Py::str("octahedral")]), Py::Float(0.41421356237309515)), (Py::tuple(vec![Py::str("FCC"), Py::str("tetrahedral")]), Py::Float(0.22474487139158894)), (Py::tuple(vec![Py::str("BCC"), Py::str("octahedral")]), Py::Float(0.15470053837925168)), (Py::tuple(vec![Py::str("BCC"), Py::str("tetrahedral")]), Py::Float(0.2909944487358056))])) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__SLIP() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::dict(vec![(Py::str("SC"), Py::str("<100>")), (Py::str("BCC"), Py::str("a/2<111>")), (Py::str("FCC"), Py::str("a/2<110>"))])) })());
    V.with(|v| v.clone())
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__angle_from(mut v_G: Py, mut v_p: Py, mut v_q: Py) -> R {
    let mut v_cross2: Py = Py::None;
    let mut v_pp: Py = Py::None;
    let mut v_pq: Py = Py::None;
    let mut v_qq: Py = Py::None;
    let t204 = (Py::tuple(vec![f__quad(v_G.clone(), v_p.clone(), v_p.clone())?, f__quad(v_G.clone(), v_q.clone(), v_q.clone())?, f__quad(v_G.clone(), v_p.clone(), v_q.clone())?])).items()?;
    if t204.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_pp = t204[0].clone();
    v_qq = t204[1].clone();
    v_pq = t204[2].clone();
    v_cross2 = py::sub(&py::mul(&v_pp.clone(), &v_qq.clone())?, &py::mul(&v_pq.clone(), &v_pq.clone())?)?;
    if ({ let t205 = v_cross2.clone(); Py::Bool({ let t206 = py::mul(&py::mul(&Py::Float(1e-12), &v_pp.clone())?, &v_qq.clone())?; if py::le(&t205, &t206)? { true } else { false } }) }).truthy() {
        return Ok((if ({ let t207 = v_pq.clone(); Py::Bool({ let t208 = Py::Int(0i64); if py::gt(&t207, &t208)? { true } else { false } }) }).truthy() { Py::Float(0.0) } else { Py::Float(180.0) }));
    }
    return Ok(py::math1("degrees", &py::atan2(&py::math1("sqrt", &v_cross2.clone())?, &v_pq.clone())?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__between(mut v_name: Py, mut v_c: Py, mut v_lo: Py, mut v_hi: Py, mut v_strict: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let t2420 = (Py::tuple(vec![py::minmax(&[v_lo.clone(), v_hi.clone()], false)?, py::minmax(&[v_lo.clone(), v_hi.clone()], true)?])).items()?;
    if t2420.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_a = t2420[0].clone();
    v_b = t2420[1].clone();
    if ({ let t2429 = { let t2424 = v_strict.clone(); if !t2424.truthy() { t2424 } else { py::not(&{ let t2421 = v_a.clone(); Py::Bool({ let t2422 = v_c.clone(); if py::lt(&t2421, &t2422)? { { let t2423 = v_b.clone(); if py::lt(&t2422, &t2423)? { true } else { false } } } else { false } }) }) } }; if t2429.truthy() { t2429 } else { { let t2428 = py::not(&v_strict.clone()); if !t2428.truthy() { t2428 } else { py::not(&{ let t2425 = v_a.clone(); Py::Bool({ let t2426 = v_c.clone(); if py::le(&t2425, &t2426)? { { let t2427 = v_b.clone(); if py::le(&t2426, &t2427)? { true } else { false } } } else { false } }) }) } } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_name.clone(), "")?, " must lie between the two phase compositions".to_string()].concat()))));
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__cell_check(mut v_a: Py, mut v_b: Py, mut v_c: Py, mut v_al: Py, mut v_be: Py, mut v_ga: Py) -> R {
    let mut v_ca: Py = Py::None;
    let mut v_cb: Py = Py::None;
    let mut v_cg: Py = Py::None;
    let mut v_rad: Py = Py::None;
    let mut v_x: Py = Py::None;
    let _ = f__pos(Py::str("the cell lengths"), Py::tuple(vec![v_a.clone(), v_b.clone(), v_c.clone()]))?;
    for t3636 in py::iter(&(Py::tuple(vec![v_al.clone(), v_be.clone(), v_ga.clone()])))? {
        v_x = t3636;
        if (py::not(&{ let t3637 = Py::Int(0i64); Py::Bool({ let t3638 = v_x.clone(); if py::lt(&t3637, &t3638)? { { let t3639 = Py::Int(180i64); if py::lt(&t3638, &t3639)? { true } else { false } } } else { false } }) })).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("cell angles must lie between 0 and 180 degrees"))));
        }
    }
    let t3642 = (Py::tuple(vec![f__cos_deg(v_al.clone())?, f__cos_deg(v_be.clone())?, f__cos_deg(v_ga.clone())?])).items()?;
    if t3642.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_ca = t3642[0].clone();
    v_cb = t3642[1].clone();
    v_cg = t3642[2].clone();
    v_rad = py::add(&py::sub(&py::sub(&py::sub(&Py::Int(1i64), &py::mul(&v_ca.clone(), &v_ca.clone())?)?, &py::mul(&v_cb.clone(), &v_cb.clone())?)?, &py::mul(&v_cg.clone(), &v_cg.clone())?)?, &py::mul(&py::mul(&py::mul(&Py::Int(2i64), &v_ca.clone())?, &v_cb.clone())?, &v_cg.clone())?)?;
    if ({ let t3643 = v_rad.clone(); Py::Bool({ let t3644 = Py::Float(1e-12); if py::le(&t3643, &t3644)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the cell angles are incompatible (no cell has them)"))));
    }
    return Ok(v_rad.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__cell_volume(mut v_struct: Py, mut v_a: Py) -> R {
    if ({ let t3517 = v_struct.clone(); Py::Bool({ let t3518 = Py::str("HCP"); if py::eq(&t3517, &t3518) { true } else { false } }) }).truthy() {
        return Ok(py::mul(&py::mul(&py::mul(&py::mul(&Py::Float(1.5), &c_SQ3()?)?, &v_a.clone())?, &v_a.clone())?, &py::mul(&c__COA()?, &v_a.clone())?)?);
    }
    return Ok(py::pow(&v_a.clone(), &Py::Int(3i64))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__check_uvtw(mut v_inputs: Py, mut v_outputs: Py, mut v_three: Py) -> R {
    let mut v_want: Py = Py::None;
    let t2716: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            if (v_three.clone()).truthy() {
                v_want = { let t2714 = v_inputs.clone(); f_three_to_four(py::getitem(&t2714, &Py::str("U"))?, py::getitem(&t2714, &Py::str("V"))?, py::getitem(&t2714, &Py::str("W"))?)? };
            } else {
                v_want = { let t2715 = v_inputs.clone(); f_four_to_three(py::getitem(&t2715, &Py::str("u"))?, py::getitem(&t2715, &Py::str("v"))?, py::getitem(&t2715, &Py::str("t"))?, py::getitem(&t2715, &Py::str("w"))?)? };
            }
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2716 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    return Ok((if (Py::Bool(({ let mut out = vec![]; for t2717 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t2718 = (t2717).items()?;
if t2718.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t2718[0].clone();
v_v = t2718[1].clone();
out.push({ let t2719 = py::method(&v_outputs.clone(), "get", &[v_k.clone()])?; Py::Bool({ let t2720 = v_v.clone(); if py::eq(&t2719, &t2720) { true } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy()))).truthy() { Py::Float(0.0) } else { Py::Float(f64::INFINITY) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__cos_deg(mut v_x: Py) -> R {
    if ({ let t3640 = py::modulo(&v_x.clone(), &Py::Int(90i64))?; Py::Bool({ let t3641 = Py::Int(0i64); if py::eq(&t3640, &t3641) { true } else { false } }) }).truthy() {
        return Ok(py::getitem(&Py::list(vec![Py::Float(1.0), Py::Float(0.0), py::neg(&Py::Float(1.0))?, Py::Float(0.0)]), &py::modulo(&py::int(&py::floordiv(&v_x.clone(), &Py::Int(90i64))?)?, &Py::Int(4i64))?)?);
    }
    return Ok(py::math1("cos", &py::math1("radians", &v_x.clone())?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__d_general(mut v_a: Py, mut v_b: Py, mut v_c: Py, mut v_al: Py, mut v_be: Py, mut v_ga: Py, mut v_h: Py, mut v_k: Py, mut v_l: Py) -> R {
    let mut v_Gs: Py = Py::None;
    let _ = f__cell_check(v_a.clone(), v_b.clone(), v_c.clone(), v_al.clone(), v_be.clone(), v_ga.clone())?;
    let t1304 = (f__ints(Py::str("the Miller indices"), Py::tuple(vec![v_h.clone(), v_k.clone(), v_l.clone()]))?).items()?;
    if t1304.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_h = t1304[0].clone();
    v_k = t1304[1].clone();
    v_l = t1304[2].clone();
    v_Gs = f__inv3(f__metric(v_a.clone(), v_b.clone(), v_c.clone(), v_al.clone(), v_be.clone(), v_ga.clone())?)?;
    return Ok(py::truediv(&Py::Int(1i64), &py::math1("sqrt", &f__quad(v_Gs.clone(), Py::list(vec![v_h.clone(), v_k.clone(), v_l.clone()]), Py::list(vec![v_h.clone(), v_k.clone(), v_l.clone()]))?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__det3(mut v_m: Py) -> R {
    return Ok(py::add(&py::sub(&py::mul(&py::getitem(&py::getitem(&v_m.clone(), &Py::Int(0i64))?, &Py::Int(0i64))?, &py::sub(&py::mul(&py::getitem(&py::getitem(&v_m.clone(), &Py::Int(1i64))?, &Py::Int(1i64))?, &py::getitem(&py::getitem(&v_m.clone(), &Py::Int(2i64))?, &Py::Int(2i64))?)?, &py::mul(&py::getitem(&py::getitem(&v_m.clone(), &Py::Int(1i64))?, &Py::Int(2i64))?, &py::getitem(&py::getitem(&v_m.clone(), &Py::Int(2i64))?, &Py::Int(1i64))?)?)?)?, &py::mul(&py::getitem(&py::getitem(&v_m.clone(), &Py::Int(0i64))?, &Py::Int(1i64))?, &py::sub(&py::mul(&py::getitem(&py::getitem(&v_m.clone(), &Py::Int(1i64))?, &Py::Int(0i64))?, &py::getitem(&py::getitem(&v_m.clone(), &Py::Int(2i64))?, &Py::Int(2i64))?)?, &py::mul(&py::getitem(&py::getitem(&v_m.clone(), &Py::Int(1i64))?, &Py::Int(2i64))?, &py::getitem(&py::getitem(&v_m.clone(), &Py::Int(2i64))?, &Py::Int(0i64))?)?)?)?)?, &py::mul(&py::getitem(&py::getitem(&v_m.clone(), &Py::Int(0i64))?, &Py::Int(2i64))?, &py::sub(&py::mul(&py::getitem(&py::getitem(&v_m.clone(), &Py::Int(1i64))?, &Py::Int(0i64))?, &py::getitem(&py::getitem(&v_m.clone(), &Py::Int(2i64))?, &Py::Int(1i64))?)?, &py::mul(&py::getitem(&py::getitem(&v_m.clone(), &Py::Int(1i64))?, &Py::Int(1i64))?, &py::getitem(&py::getitem(&v_m.clone(), &Py::Int(2i64))?, &Py::Int(0i64))?)?)?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__dot3(mut v_a: Py, mut v_b: Py) -> R {
    return Ok(py::sum(&{ let mut out = vec![]; for t3289 in py::iter(&(py::zip(&[v_a.clone(), v_b.clone()])?))? { let mut v_x: Py; let mut v_y: Py; 
let t3290 = (t3289).items()?;
if t3290.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_x = t3290[0].clone();
v_y = t3290[1].clone();
out.push(py::mul(&v_x.clone(), &v_y.clone())?); } Py::list(out) }, None)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__erf_solution(mut v_C0: Py, mut v_Cs: Py, mut v_x: Py, mut v_D: Py, mut v_t: Py) -> R {
    return Ok(py::sub(&v_Cs.clone(), &py::mul(&py::sub(&v_Cs.clone(), &v_C0.clone())?, &py::math1("erf", &py::truediv(&v_x.clone(), &py::mul(&Py::Int(2i64), &py::math1("sqrt", &py::mul(&v_D.clone(), &v_t.clone())?)?)?)?)?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__f_rule(mut v_structure: Py, mut v_h: Py, mut v_k: Py, mut v_l: Py) -> R {
    let mut v_same: Py = Py::None;
    if ({ let t3436 = v_structure.clone(); Py::Bool({ let t3437 = Py::str("SC"); if py::eq(&t3436, &t3437) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(1.0));
    }
    if ({ let t3438 = v_structure.clone(); Py::Bool({ let t3439 = Py::str("BCC"); if py::eq(&t3438, &t3439) { true } else { false } }) }).truthy() {
        return Ok((if ({ let t3440 = py::modulo(&py::add(&py::add(&v_h.clone(), &v_k.clone())?, &v_l.clone())?, &Py::Int(2i64))?; Py::Bool({ let t3441 = Py::Int(0i64); if py::eq(&t3440, &t3441) { true } else { false } }) }).truthy() { Py::Float(2.0) } else { Py::Float(0.0) }));
    }
    v_same = { let t3442 = py::modulo(&v_h.clone(), &Py::Int(2i64))?; Py::Bool({ let t3443 = py::modulo(&v_k.clone(), &Py::Int(2i64))?; if py::eq(&t3442, &t3443) { { let t3444 = py::modulo(&v_l.clone(), &Py::Int(2i64))?; if py::eq(&t3443, &t3444) { true } else { false } } } else { false } }) };
    if ({ let t3445 = v_structure.clone(); Py::Bool({ let t3446 = Py::str("FCC"); if py::eq(&t3445, &t3446) { true } else { false } }) }).truthy() {
        return Ok((if (v_same.clone()).truthy() { Py::Float(4.0) } else { Py::Float(0.0) }));
    }
    if (py::not(&v_same.clone())).truthy() {
        return Ok(Py::Float(0.0));
    }
    if (py::modulo(&v_h.clone(), &Py::Int(2i64))?).truthy() {
        return Ok(py::mul(&Py::Int(4i64), &c_SQ2()?)?);
    }
    return Ok((if ({ let t3447 = py::modulo(&py::add(&py::add(&v_h.clone(), &v_k.clone())?, &v_l.clone())?, &Py::Int(4i64))?; Py::Bool({ let t3448 = Py::Int(0i64); if py::eq(&t3447, &t3448) { true } else { false } }) }).truthy() { Py::Float(8.0) } else { Py::Float(0.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__f_sum(mut v_structure: Py, mut v_h: Py, mut v_k: Py, mut v_l: Py) -> R {
    let mut v_im_: Py = Py::None;
    let mut v_re_: Py = Py::None;
    let mut v_v: Py = Py::None;
    v_re_ = py::sum(&{ let mut out = vec![]; for t3459 in py::iter(&(py::getitem(&c__BASIS()?, &v_structure.clone())?))? { let mut v_x: Py; let mut v_y: Py; let mut v_z: Py; 
let t3460 = (t3459).items()?;
if t3460.len() != 3 { return Err(PyErr::Value("unpack".into())); }
v_x = t3460[0].clone();
v_y = t3460[1].clone();
v_z = t3460[2].clone();
out.push(py::math1("cos", &py::mul(&py::mul(&Py::Int(2i64), &Py::Float(std::f64::consts::PI))?, &py::add(&py::add(&py::mul(&v_h.clone(), &v_x.clone())?, &py::mul(&v_k.clone(), &v_y.clone())?)?, &py::mul(&v_l.clone(), &v_z.clone())?)?)?)?); } Py::list(out) }, None)?;
    v_im_ = py::sum(&{ let mut out = vec![]; for t3461 in py::iter(&(py::getitem(&c__BASIS()?, &v_structure.clone())?))? { let mut v_x: Py; let mut v_y: Py; let mut v_z: Py; 
let t3462 = (t3461).items()?;
if t3462.len() != 3 { return Err(PyErr::Value("unpack".into())); }
v_x = t3462[0].clone();
v_y = t3462[1].clone();
v_z = t3462[2].clone();
out.push(py::math1("sin", &py::mul(&py::mul(&Py::Int(2i64), &Py::Float(std::f64::consts::PI))?, &py::add(&py::add(&py::mul(&v_h.clone(), &v_x.clone())?, &py::mul(&v_k.clone(), &v_y.clone())?)?, &py::mul(&v_l.clone(), &v_z.clone())?)?)?)?); } Py::list(out) }, None)?;
    v_v = py::hypot(&[v_re_.clone(), v_im_.clone()])?;
    return Ok((if ({ let t3463 = v_v.clone(); Py::Bool({ let t3464 = Py::Float(1e-09); if py::lt(&t3463, &t3464)? { true } else { false } }) }).truthy() { Py::Float(0.0) } else { v_v.clone() }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__formula_counts(mut v_formula: Py) -> R {
    let mut v_counts: Py = Py::None;
    let mut v_f: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_parse: Py = Py::None;
    v_f = py::method(&Py::str(&py::to_str(&v_formula.clone())), "replace", &[Py::str(" "), Py::str("")])?;
    if ({ let t1376 = py::not(&v_f.clone()); if t1376.truthy() { t1376 } else { py::not(&ax_engine::pyre::module_call("fullmatch", &Py::str("[A-Za-z0-9().]+"), &[v_f.clone()], 0)?) } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("write the formula with element symbols, counts and parentheses only (e.g. C2H3Cl)"))));
    }
    let rec1377 = py::rec_cell();
    v_parse = { let v_f = v_f.clone(); let v_formula = v_formula.clone(); let rec1377 = rec1377.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_s: Py = a.get(0).cloned().unwrap_or(Py::None);
            let mut v_i: Py = a.get(1).cloned().unwrap_or(Py::None);
            let v_parse: Py = py::rec_get(&rec1377)?;
            let mut v_counts: Py = Py::None;
            let mut v_el: Py = Py::None;
            let mut v_inner: Py = Py::None;
            let mut v_k: Py = Py::None;
            let mut v_m: Py = Py::None;
            let mut v_mult: Py = Py::None;
            let mut v_num: Py = Py::None;
            let mut v_v: Py = Py::None;
            v_counts = Py::dict(vec![]);
            while ({ let t1391 = v_i.clone(); Py::Bool({ let t1392 = py::len(&v_s.clone())?; if py::lt(&t1391, &t1392)? { true } else { false } }) }).truthy() {
                if ({ let t1378 = py::getitem(&v_s.clone(), &v_i.clone())?; Py::Bool({ let t1379 = Py::str("("); if py::eq(&t1378, &t1379) { true } else { false } }) }).truthy() {
                    let t1380 = (py::call(&v_parse.clone(), &[v_s.clone(), py::add(&v_i.clone(), &Py::Int(1i64))?])?).items()?;
                    if t1380.len() != 2 { return Err(PyErr::Value("unpack".into())); }
                    v_inner = t1380[0].clone();
                    v_i = t1380[1].clone();
                    v_m = ax_engine::pyre::module_call("match", &Py::str("\\d+"), &[py::slice(&v_s.clone(), Some((v_i.clone()).i()?), None, None)?], 0)?;
                    v_mult = (if (v_m.clone()).truthy() { py::int(&py::method(&v_m.clone(), "group", &[])?)? } else { Py::Int(1i64) });
                    v_i = py::iadd(&v_i, &(if (v_m.clone()).truthy() { py::len(&py::method(&v_m.clone(), "group", &[])?)? } else { Py::Int(0i64) }))?;
                    for t1381 in py::iter(&(py::method(&v_inner.clone(), "items", &[])?))? {
                        let t1382 = (t1381).items()?;
                        if t1382.len() != 2 { return Err(PyErr::Value("unpack".into())); }
                        v_k = t1382[0].clone();
                        v_v = t1382[1].clone();
                        py::setitem(&v_counts.clone(), &v_k.clone(), py::add(&py::method(&v_counts.clone(), "get", &[v_k.clone(), Py::Int(0i64)])?, &py::mul(&v_v.clone(), &v_mult.clone())?)?)?;
                    }
                } else {
                    if ({ let t1383 = py::getitem(&v_s.clone(), &v_i.clone())?; Py::Bool({ let t1384 = Py::str(")"); if py::eq(&t1383, &t1384) { true } else { false } }) }).truthy() {
                        return Ok(Py::tuple(vec![v_counts.clone(), py::add(&v_i.clone(), &Py::Int(1i64))?]));
                    } else {
                        v_m = py::method(&c__EL()?, "match", &[v_s.clone(), v_i.clone()])?;
                        if ({ let t1387 = py::not(&v_m.clone()); if t1387.truthy() { t1387 } else { { let t1385 = py::method(&v_m.clone(), "end", &[])?; Py::Bool({ let t1386 = v_i.clone(); if py::eq(&t1385, &t1386) { true } else { false } }) } } }).truthy() {
                            return Err(py::value_error(py::to_str(&Py::str(&["cannot read the formula at '".to_string(), py::format(&py::slice(&v_s.clone(), Some((v_i.clone()).i()?), None, None)?, "")?, "'".to_string()].concat()))));
                        }
                        let t1388 = (Py::tuple(vec![py::method(&v_m.clone(), "group", &[Py::Int(1i64)])?, py::method(&v_m.clone(), "group", &[Py::Int(2i64)])?])).items()?;
                        if t1388.len() != 2 { return Err(PyErr::Value("unpack".into())); }
                        v_el = t1388[0].clone();
                        v_num = t1388[1].clone();
                        if ({ let t1389 = v_el.clone(); Py::Bool({ let t1390 = c_ATOMIC_WEIGHT()?; if !py::contains(&t1390, &t1389)? { true } else { false } }) }).truthy() {
                            return Err(py::value_error(py::to_str(&Py::str(&["unknown element ".to_string(), py::format(&v_el.clone(), "")?].concat()))));
                        }
                        py::setitem(&v_counts.clone(), &v_el.clone(), py::add(&py::method(&v_counts.clone(), "get", &[v_el.clone(), Py::Int(0i64)])?, &(if (v_num.clone()).truthy() { py::float(&v_num.clone())? } else { Py::Int(1i64) }))?)?;
                        v_i = py::method(&v_m.clone(), "end", &[])?;
                    }
                }
            }
            return Ok(Py::tuple(vec![v_counts.clone(), v_i.clone()]));
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    py::rec_set(&rec1377, &v_parse);
    let t1393 = (py::call(&v_parse.clone(), &[v_f.clone(), Py::Int(0i64)])?).items()?;
    if t1393.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_counts = t1393[0].clone();
    v_i = t1393[1].clone();
    if ({ let t1396 = { let t1394 = v_i.clone(); Py::Bool({ let t1395 = py::len(&v_f.clone())?; if py::ne(&t1394, &t1395) { true } else { false } }) }; if t1396.truthy() { t1396 } else { py::not(&v_counts.clone()) } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("unbalanced parentheses in the formula"))));
    }
    return Ok(v_counts.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__formula_mass(mut v_formula: Py) -> R {
    return Ok(py::sum(&{ let mut out = vec![]; for t1397 in py::iter(&(py::method(&f__formula_counts(v_formula.clone())?, "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t1398 = (t1397).items()?;
if t1398.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t1398[0].clone();
v_v = t1398[1].clone();
out.push(py::mul(&py::getitem(&c_ATOMIC_WEIGHT()?, &v_k.clone())?, &v_v.clone())?); } Py::list(out) }, None)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__frac01(mut v_name: Py, mut v_v: Py, mut v_closed: Py) -> R {
    let mut v_ok: Py = Py::None;
    v_ok = (if (v_closed.clone()).truthy() { { let t3825 = Py::Int(0i64); Py::Bool({ let t3826 = v_v.clone(); if py::le(&t3825, &t3826)? { { let t3827 = Py::Int(1i64); if py::le(&t3826, &t3827)? { true } else { false } } } else { false } }) } } else { { let t3828 = Py::Int(0i64); Py::Bool({ let t3829 = v_v.clone(); if py::lt(&t3828, &t3829)? { { let t3830 = Py::Int(1i64); if py::lt(&t3829, &t3830)? { true } else { false } } } else { false } }) } });
    if (py::not(&{ let t3831 = py::math1("isfinite", &v_v.clone())?; if !t3831.truthy() { t3831 } else { v_ok.clone() } })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_name.clone(), "")?, " must lie between 0 and 1".to_string()].concat()))));
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__gcd3(mut v_v: Py) -> R {
    let mut v_g: Py = Py::None;
    let mut v_x: Py = Py::None;
    v_g = Py::Int(0i64);
    for t2986 in py::iter(&(v_v.clone()))? {
        v_x = t2986;
        v_g = py::gcd(&v_g.clone(), &py::abs(&py::int(&v_x.clone())?)?)?;
    }
    return Ok(v_g.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__intercept(mut v_text: Py, mut v_name: Py) -> R {
    let mut v_t: Py = Py::None;
    let mut v_v: Py = Py::None;
    v_t = py::method(&py::method(&py::method(&Py::str(&py::to_str(&v_text.clone())), "strip", &[])?, "lower", &[])?, "replace", &[Py::str(" "), Py::str("")])?;
    if ({ let t2721 = v_t.clone(); Py::Bool({ let t2722 = Py::tuple(vec![Py::str("inf"), Py::str("infinity"), Py::str("∞"), Py::str("+inf"), Py::str("-inf"), Py::str("parallel")]); if py::contains(&t2722, &t2721)? { true } else { false } }) }).truthy() {
        return Ok(Py::None);
    }
    let t2723: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_v = py::fraction(&[v_t.clone()])?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2723 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Value(_))) || (matches!(e, PyErr::ZeroDivision)) => {
            return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_name.clone(), "")?, ": write a number, a fraction such as 1/2, or inf".to_string()].concat()))));
        }
        Err(e) => return Err(e),
    }
    if ({ let t2724 = v_v.clone(); Py::Bool({ let t2725 = Py::Int(0i64); if py::eq(&t2724, &t2725) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("an intercept of 0 means the plane passes through the origin: move the origin first"))));
    }
    return Ok(v_v.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__ints(mut v_name: Py, mut v_v: Py) -> R {
    let mut v_x: Py = Py::None;
    for t3426 in py::iter(&(v_v.clone()))? {
        v_x = t3426;
        if (py::not(&{ let t3430 = Py::Bool(matches!(v_x.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_))); if t3430.truthy() { t3430 } else { { let t3429 = Py::Bool(matches!(v_x.clone(), Py::Float(_))); if !t3429.truthy() { t3429 } else { { let t3427 = v_x.clone(); Py::Bool({ let t3428 = py::int(&v_x.clone())?; if py::eq(&t3427, &t3428) { true } else { false } }) } } } } })).truthy() {
            return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_name.clone(), "")?, " must be whole numbers".to_string()].concat()))));
        }
    }
    if (Py::Bool(({ let mut out = vec![]; for t3431 in py::iter(&(v_v.clone()))? { let mut v_x: Py; 
v_x = t3431;
out.push({ let t3432 = py::int(&v_x.clone())?; Py::Bool({ let t3433 = Py::Int(0i64); if py::eq(&t3432, &t3433) { true } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy()))).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_name.clone(), "")?, " must not all be zero".to_string()].concat()))));
    }
    return Ok({ let mut out = vec![]; for t3434 in py::iter(&(v_v.clone()))? { let mut v_x: Py; 
v_x = t3434;
out.push(py::int(&v_x.clone())?); } Py::list(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__inv3(mut v_m: Py) -> R {
    let mut v_c: Py = Py::None;
    let mut v_d: Py = Py::None;
    v_d = f__det3(v_m.clone())?;
    if ({ let t1305 = v_d.clone(); Py::Bool({ let t1306 = Py::Int(0i64); if py::le(&t1305, &t1306)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the cell parameters do not form a cell (the angles are incompatible)"))));
    }
    v_c = Py::list(vec![Py::list(vec![py::sub(&py::mul(&py::getitem(&py::getitem(&v_m.clone(), &Py::Int(1i64))?, &Py::Int(1i64))?, &py::getitem(&py::getitem(&v_m.clone(), &Py::Int(2i64))?, &Py::Int(2i64))?)?, &py::mul(&py::getitem(&py::getitem(&v_m.clone(), &Py::Int(1i64))?, &Py::Int(2i64))?, &py::getitem(&py::getitem(&v_m.clone(), &Py::Int(2i64))?, &Py::Int(1i64))?)?)?, py::sub(&py::mul(&py::getitem(&py::getitem(&v_m.clone(), &Py::Int(0i64))?, &Py::Int(2i64))?, &py::getitem(&py::getitem(&v_m.clone(), &Py::Int(2i64))?, &Py::Int(1i64))?)?, &py::mul(&py::getitem(&py::getitem(&v_m.clone(), &Py::Int(0i64))?, &Py::Int(1i64))?, &py::getitem(&py::getitem(&v_m.clone(), &Py::Int(2i64))?, &Py::Int(2i64))?)?)?, py::sub(&py::mul(&py::getitem(&py::getitem(&v_m.clone(), &Py::Int(0i64))?, &Py::Int(1i64))?, &py::getitem(&py::getitem(&v_m.clone(), &Py::Int(1i64))?, &Py::Int(2i64))?)?, &py::mul(&py::getitem(&py::getitem(&v_m.clone(), &Py::Int(0i64))?, &Py::Int(2i64))?, &py::getitem(&py::getitem(&v_m.clone(), &Py::Int(1i64))?, &Py::Int(1i64))?)?)?]), Py::list(vec![py::sub(&py::mul(&py::getitem(&py::getitem(&v_m.clone(), &Py::Int(1i64))?, &Py::Int(2i64))?, &py::getitem(&py::getitem(&v_m.clone(), &Py::Int(2i64))?, &Py::Int(0i64))?)?, &py::mul(&py::getitem(&py::getitem(&v_m.clone(), &Py::Int(1i64))?, &Py::Int(0i64))?, &py::getitem(&py::getitem(&v_m.clone(), &Py::Int(2i64))?, &Py::Int(2i64))?)?)?, py::sub(&py::mul(&py::getitem(&py::getitem(&v_m.clone(), &Py::Int(0i64))?, &Py::Int(0i64))?, &py::getitem(&py::getitem(&v_m.clone(), &Py::Int(2i64))?, &Py::Int(2i64))?)?, &py::mul(&py::getitem(&py::getitem(&v_m.clone(), &Py::Int(0i64))?, &Py::Int(2i64))?, &py::getitem(&py::getitem(&v_m.clone(), &Py::Int(2i64))?, &Py::Int(0i64))?)?)?, py::sub(&py::mul(&py::getitem(&py::getitem(&v_m.clone(), &Py::Int(0i64))?, &Py::Int(2i64))?, &py::getitem(&py::getitem(&v_m.clone(), &Py::Int(1i64))?, &Py::Int(0i64))?)?, &py::mul(&py::getitem(&py::getitem(&v_m.clone(), &Py::Int(0i64))?, &Py::Int(0i64))?, &py::getitem(&py::getitem(&v_m.clone(), &Py::Int(1i64))?, &Py::Int(2i64))?)?)?]), Py::list(vec![py::sub(&py::mul(&py::getitem(&py::getitem(&v_m.clone(), &Py::Int(1i64))?, &Py::Int(0i64))?, &py::getitem(&py::getitem(&v_m.clone(), &Py::Int(2i64))?, &Py::Int(1i64))?)?, &py::mul(&py::getitem(&py::getitem(&v_m.clone(), &Py::Int(1i64))?, &Py::Int(1i64))?, &py::getitem(&py::getitem(&v_m.clone(), &Py::Int(2i64))?, &Py::Int(0i64))?)?)?, py::sub(&py::mul(&py::getitem(&py::getitem(&v_m.clone(), &Py::Int(0i64))?, &Py::Int(1i64))?, &py::getitem(&py::getitem(&v_m.clone(), &Py::Int(2i64))?, &Py::Int(0i64))?)?, &py::mul(&py::getitem(&py::getitem(&v_m.clone(), &Py::Int(0i64))?, &Py::Int(0i64))?, &py::getitem(&py::getitem(&v_m.clone(), &Py::Int(2i64))?, &Py::Int(1i64))?)?)?, py::sub(&py::mul(&py::getitem(&py::getitem(&v_m.clone(), &Py::Int(0i64))?, &Py::Int(0i64))?, &py::getitem(&py::getitem(&v_m.clone(), &Py::Int(1i64))?, &Py::Int(1i64))?)?, &py::mul(&py::getitem(&py::getitem(&v_m.clone(), &Py::Int(0i64))?, &Py::Int(1i64))?, &py::getitem(&py::getitem(&v_m.clone(), &Py::Int(1i64))?, &Py::Int(0i64))?)?)?])]);
    return Ok({ let mut out = vec![]; for t1307 in py::iter(&(v_c.clone()))? { let mut v_row: Py; 
v_row = t1307;
out.push({ let mut out = vec![]; for t1308 in py::iter(&(v_row.clone()))? { let mut v_x: Py; 
v_x = t1308;
out.push(py::truediv(&v_x.clone(), &v_d.clone())?); } Py::list(out) }); } Py::list(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__inverse_erf(mut v_y: Py) -> R {
    let mut v__: Py = Py::None;
    let mut v_hi: Py = Py::None;
    let mut v_lo: Py = Py::None;
    let mut v_mid: Py = Py::None;
    if (py::not(&{ let t1555 = Py::Int(0i64); Py::Bool({ let t1556 = v_y.clone(); if py::lt(&t1555, &t1556)? { { let t1557 = Py::Int(1i64); if py::lt(&t1556, &t1557)? { true } else { false } } } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the concentration must lie strictly between the initial and the surface values"))));
    }
    let t1558 = (Py::tuple(vec![Py::Float(0.0), Py::Float(6.0)])).items()?;
    if t1558.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_lo = t1558[0].clone();
    v_hi = t1558[1].clone();
    for t1559 in py::range_iter(&[(Py::Int(200i64)).i()?]) {
        v__ = t1559;
        v_mid = py::truediv(&py::add(&v_lo.clone(), &v_hi.clone())?, &Py::Int(2i64))?;
        if ({ let t1560 = v_mid.clone(); Py::Bool({ let t1561 = Py::tuple(vec![v_lo.clone(), v_hi.clone()]); if py::contains(&t1561, &t1560)? { true } else { false } }) }).truthy() {
            break;
        }
        if ({ let t1562 = py::math1("erf", &v_mid.clone())?; Py::Bool({ let t1563 = v_y.clone(); if py::lt(&t1562, &t1563)? { true } else { false } }) }).truthy() {
            v_lo = v_mid.clone();
        } else {
            v_hi = v_mid.clone();
        }
    }
    return Ok(py::truediv(&py::add(&v_lo.clone(), &v_hi.clone())?, &Py::Int(2i64))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__linfit(mut v_x: Py, mut v_y: Py) -> R {
    let mut v_b1: Py = Py::None;
    let mut v_mx: Py = Py::None;
    let mut v_my: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_r2: Py = Py::None;
    let mut v_sxx: Py = Py::None;
    let mut v_sxy: Py = Py::None;
    let mut v_syy: Py = Py::None;
    v_n = py::len(&v_x.clone())?;
    if ({ let t4051 = v_n.clone(); Py::Bool({ let t4052 = py::len(&v_y.clone())?; if py::ne(&t4051, &t4052) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the two lists must have the same length"))));
    }
    if ({ let t4053 = v_n.clone(); Py::Bool({ let t4054 = Py::Int(2i64); if py::lt(&t4053, &t4054)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("give at least two points"))));
    }
    let t4055 = (Py::tuple(vec![py::truediv(&py::sum(&v_x.clone(), None)?, &v_n.clone())?, py::truediv(&py::sum(&v_y.clone(), None)?, &v_n.clone())?])).items()?;
    if t4055.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_mx = t4055[0].clone();
    v_my = t4055[1].clone();
    v_sxx = py::sum(&{ let mut out = vec![]; for t4056 in py::iter(&(v_x.clone()))? { let mut v_a: Py; 
v_a = t4056;
out.push(py::pow(&py::sub(&v_a.clone(), &v_mx.clone())?, &Py::Int(2i64))?); } Py::list(out) }, None)?;
    if ({ let t4057 = v_sxx.clone(); Py::Bool({ let t4058 = py::mul(&Py::Float(1e-24), &py::pow(&py::minmax(&[Py::Float(1.0), py::minmax(&[{ let mut out = vec![]; for t4059 in py::iter(&(v_x.clone()))? { let mut v_a: Py; 
v_a = t4059;
out.push(py::abs(&v_a.clone())?); } Py::list(out) }], true)?], true)?, &Py::Int(2i64))?)?; if py::le(&t4057, &t4058)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the x values must not all be equal"))));
    }
    v_sxy = py::sum(&{ let mut out = vec![]; for t4060 in py::iter(&(py::zip(&[v_x.clone(), v_y.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t4061 = (t4060).items()?;
if t4061.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t4061[0].clone();
v_b = t4061[1].clone();
out.push(py::mul(&py::sub(&v_a.clone(), &v_mx.clone())?, &py::sub(&v_b.clone(), &v_my.clone())?)?); } Py::list(out) }, None)?;
    v_syy = py::sum(&{ let mut out = vec![]; for t4062 in py::iter(&(v_y.clone()))? { let mut v_b: Py; 
v_b = t4062;
out.push(py::pow(&py::sub(&v_b.clone(), &v_my.clone())?, &Py::Int(2i64))?); } Py::list(out) }, None)?;
    v_b1 = py::truediv(&v_sxy.clone(), &v_sxx.clone())?;
    v_r2 = (if ({ let t4063 = v_syy.clone(); Py::Bool({ let t4064 = Py::Int(0i64); if py::eq(&t4063, &t4064) { true } else { false } }) }).truthy() { Py::Float(1.0) } else { py::truediv(&py::mul(&v_sxy.clone(), &v_sxy.clone())?, &py::mul(&v_sxx.clone(), &v_syy.clone())?)? });
    return Ok(Py::tuple(vec![py::sub(&v_my.clone(), &py::mul(&v_b1.clone(), &v_mx.clone())?)?, v_b1.clone(), v_r2.clone()]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__majority(mut v_Nd: Py, mut v_Na: Py, mut v_ni: Py) -> R {
    let mut v_D: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_s: Py = Py::None;
    v_D = py::sub(&v_Nd.clone(), &v_Na.clone())?;
    v_s = py::math1("sqrt", &py::add(&py::mul(&py::mul(&Py::Float(0.25), &v_D.clone())?, &v_D.clone())?, &py::mul(&v_ni.clone(), &v_ni.clone())?)?)?;
    if ({ let t3034 = v_D.clone(); Py::Bool({ let t3035 = Py::Int(0i64); if py::ge(&t3034, &t3035)? { true } else { false } }) }).truthy() {
        v_n = py::add(&py::mul(&Py::Float(0.5), &v_D.clone())?, &v_s.clone())?;
        return Ok(Py::tuple(vec![v_n.clone(), py::truediv(&py::mul(&v_ni.clone(), &v_ni.clone())?, &v_n.clone())?]));
    }
    v_p = py::add(&py::mul(&py::neg(&Py::Float(0.5))?, &v_D.clone())?, &v_s.clone())?;
    return Ok(Py::tuple(vec![py::truediv(&py::mul(&v_ni.clone(), &v_ni.clone())?, &v_p.clone())?, v_p.clone()]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__metric(mut v_a: Py, mut v_b: Py, mut v_c: Py, mut v_al: Py, mut v_be: Py, mut v_ga: Py) -> R {
    let mut v_ca: Py = Py::None;
    let mut v_cb: Py = Py::None;
    let mut v_cg: Py = Py::None;
    let t1309 = (Py::tuple(vec![f__cos_deg(v_al.clone())?, f__cos_deg(v_be.clone())?, f__cos_deg(v_ga.clone())?])).items()?;
    if t1309.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_ca = t1309[0].clone();
    v_cb = t1309[1].clone();
    v_cg = t1309[2].clone();
    return Ok(Py::list(vec![Py::list(vec![py::mul(&v_a.clone(), &v_a.clone())?, py::mul(&py::mul(&v_a.clone(), &v_b.clone())?, &v_cg.clone())?, py::mul(&py::mul(&v_a.clone(), &v_c.clone())?, &v_cb.clone())?]), Py::list(vec![py::mul(&py::mul(&v_a.clone(), &v_b.clone())?, &v_cg.clone())?, py::mul(&v_b.clone(), &v_b.clone())?, py::mul(&py::mul(&v_b.clone(), &v_c.clone())?, &v_ca.clone())?]), Py::list(vec![py::mul(&py::mul(&v_a.clone(), &v_c.clone())?, &v_cb.clone())?, py::mul(&py::mul(&v_b.clone(), &v_c.clone())?, &v_ca.clone())?, py::mul(&v_c.clone(), &v_c.clone())?])]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__nc(mut v_m_ratio: Py, mut v_T: Py) -> R {
    return Ok(py::mul(&Py::Int(2i64), &py::pow(&py::truediv(&py::mul(&py::mul(&py::mul(&py::mul(&py::mul(&Py::Int(2i64), &Py::Float(std::f64::consts::PI))?, &v_m_ratio.clone())?, &c_ME()?)?, &c_KB()?)?, &v_T.clone())?, &py::mul(&c_HP()?, &c_HP()?)?)?, &Py::Float(1.5))?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__norm3(mut v_v: Py) -> R {
    return Ok(py::math1("sqrt", &py::sum(&{ let mut out = vec![]; for t3284 in py::iter(&(v_v.clone()))? { let mut v_x: Py; 
v_x = t3284;
out.push(py::mul(&v_x.clone(), &v_x.clone())?); } Py::list(out) }, None)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__out(mut v_kw: Py) -> R {
    return Ok({ let mut out = vec![]; for t4071 in py::iter(&(py::method(&v_kw.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t4072 = (t4071).items()?;
if t4072.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t4072[0].clone();
v_v = t4072[1].clone();
out.push((v_k.clone(), f__r(v_v.clone())?)); } Py::dict(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__pos(mut v_name: Py, mut v_vals: Py) -> R {
    let mut v_v: Py = Py::None;
    for t4002 in py::iter(&(v_vals.clone()))? {
        v_v = t4002;
        if (py::not(&{ let t4006 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t4006.truthy() { t4006 } else { { let t4005 = py::math1("isfinite", &v_v.clone())?; if !t4005.truthy() { t4005 } else { { let t4003 = v_v.clone(); Py::Bool({ let t4004 = Py::Int(0i64); if py::gt(&t4003, &t4004)? { true } else { false } }) } } } } })).truthy() {
            return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_name.clone(), "")?, " must be a positive number".to_string()].concat()))));
        }
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__quad(mut v_G: Py, mut v_u: Py, mut v_v: Py) -> R {
    return Ok(py::sum(&{ let mut out = vec![]; for t1310 in py::range_iter(&[(Py::Int(3i64)).i()?]) { let mut v_i: Py; 
v_i = t1310;
for t1311 in py::range_iter(&[(Py::Int(3i64)).i()?]) { let mut v_j: Py; 
v_j = t1311;
out.push(py::mul(&py::mul(&py::getitem(&v_u.clone(), &v_i.clone())?, &py::getitem(&py::getitem(&v_G.clone(), &v_i.clone())?, &v_j.clone())?)?, &py::getitem(&v_v.clone(), &v_j.clone())?)?); }} Py::list(out) }, None)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__r(mut v_x: Py) -> R {
    if (Py::Bool(matches!(v_x.clone(), Py::Bool(_) | Py::Str(_) | Py::Int(_) | Py::Big(_) | Py::Bool(_)))).truthy() {
        return Ok(v_x.clone());
    }
    if (Py::Bool(matches!(v_x.clone(), Py::List(_)))).truthy() {
        return Ok({ let mut out = vec![]; for t4073 in py::iter(&(v_x.clone()))? { let mut v_v: Py; 
v_v = t4073;
out.push(f__r(v_v.clone())?); } Py::list(out) });
    }
    if ({ let t4076 = { let t4074 = v_x.clone(); Py::Bool({ let t4075 = Py::Int(0i64); if py::eq(&t4074, &t4075) { true } else { false } }) }; if t4076.truthy() { t4076 } else { py::not(&py::math1("isfinite", &v_x.clone())?) } }).truthy() {
        return Ok((if ({ let t4077 = v_x.clone(); Py::Bool({ let t4078 = Py::Int(0i64); if py::eq(&t4077, &t4078) { true } else { false } }) }).truthy() { Py::Float(0.0) } else { v_x.clone() }));
    }
    return Ok(py::add(&py::float(&Py::str(&[py::format(&v_x.clone(), ".12g")?].concat()))?, &Py::Float(0.0))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__reciprocal_ok(mut v_structure: Py, mut v_h: Py, mut v_k: Py, mut v_l: Py) -> R {
    if ({ let t2988 = v_structure.clone(); Py::Bool({ let t2989 = Py::str("SC"); if py::eq(&t2988, &t2989) { true } else { false } }) }).truthy() {
        return Ok(Py::Bool(true));
    }
    if ({ let t2990 = v_structure.clone(); Py::Bool({ let t2991 = Py::str("BCC"); if py::eq(&t2990, &t2991) { true } else { false } }) }).truthy() {
        return Ok({ let t2992 = py::modulo(&py::add(&py::add(&v_h.clone(), &v_k.clone())?, &v_l.clone())?, &Py::Int(2i64))?; Py::Bool({ let t2993 = Py::Int(0i64); if py::eq(&t2992, &t2993) { true } else { false } }) });
    }
    return Ok({ let t2994 = py::modulo(&v_h.clone(), &Py::Int(2i64))?; Py::Bool({ let t2995 = py::modulo(&v_k.clone(), &Py::Int(2i64))?; if py::eq(&t2994, &t2995) { { let t2996 = py::modulo(&v_l.clone(), &Py::Int(2i64))?; if py::eq(&t2995, &t2996) { true } else { false } } } else { false } }) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__reduce_ints(mut v_vals: Py) -> R {
    let mut v_g: Py = Py::None;
    let mut v_v: Py = Py::None;
    v_g = Py::Int(0i64);
    for t2708 in py::iter(&(v_vals.clone()))? {
        v_v = t2708;
        v_g = py::gcd(&v_g.clone(), &py::abs(&v_v.clone())?)?;
    }
    return Ok({ let mut out = vec![]; for t2709 in py::iter(&(v_vals.clone()))? { let mut v_v: Py; 
v_v = t2709;
out.push(py::floordiv(&v_v.clone(), &v_g.clone())?); } Py::list(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__rel(mut v_a: Py, mut v_b: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_s: Py = Py::None;
    if ({ let t4085 = Py::Bool(matches!(v_a.clone(), Py::Str(_) | Py::Bool(_))); if t4085.truthy() { t4085 } else { Py::Bool(matches!(v_b.clone(), Py::Str(_) | Py::Bool(_))) } }).truthy() {
        return Ok((if ({ let t4086 = v_a.clone(); Py::Bool({ let t4087 = v_b.clone(); if py::eq(&t4086, &t4087) { true } else { false } }) }).truthy() { Py::Float(0.0) } else { Py::Float(f64::INFINITY) }));
    }
    if ({ let t4088 = Py::Bool(matches!(v_a.clone(), Py::List(_))); if t4088.truthy() { t4088 } else { Py::Bool(matches!(v_b.clone(), Py::List(_))) } }).truthy() {
        if ({ let t4092 = py::not(&{ let t4089 = Py::Bool(matches!(v_a.clone(), Py::List(_))); if !t4089.truthy() { t4089 } else { Py::Bool(matches!(v_b.clone(), Py::List(_))) } }); if t4092.truthy() { t4092 } else { { let t4090 = py::len(&v_a.clone())?; Py::Bool({ let t4091 = py::len(&v_b.clone())?; if py::ne(&t4090, &t4091) { true } else { false } }) } } }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        return Ok(py::minmax(&[{ let t4095 = { let mut out = vec![]; for t4093 in py::iter(&(py::zip(&[v_a.clone(), v_b.clone()])?))? { let mut v_x: Py; let mut v_y: Py; 
let t4094 = (t4093).items()?;
if t4094.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_x = t4094[0].clone();
v_y = t4094[1].clone();
out.push(f__rel(v_x.clone(), v_y.clone())?); } Py::list(out) }; if t4095.truthy() { t4095 } else { Py::list(vec![Py::Float(0.0)]) } }], true)?);
    }
    let t4097: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            let t4096 = (Py::tuple(vec![py::float(&v_a.clone())?, py::float(&v_b.clone())?])).items()?;
            if t4096.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_a = t4096[0].clone();
            v_b = t4096[1].clone();
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4097 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if ((matches!(e, PyErr::Type(_)) || matches!(&e, PyErr::Other(c) if c.as_str() == "TypeError"))) || (matches!(e, PyErr::Value(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_d = py::abs(&py::sub(&v_a.clone(), &v_b.clone())?)?;
    if ({ let t4098 = v_d.clone(); Py::Bool({ let t4099 = Py::Int(0i64); if py::eq(&t4098, &t4099) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    v_s = py::minmax(&[py::abs(&v_a.clone())?, py::abs(&v_b.clone())?], true)?;
    return Ok((if ({ let t4100 = v_s.clone(); Py::Bool({ let t4101 = Py::Int(0i64); if py::gt(&t4100, &t4101)? { true } else { false } }) }).truthy() { py::truediv(&v_d.clone(), &v_s.clone())? } else { Py::Float(f64::INFINITY) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__slip_systems(mut v_structure: Py) -> R {
    let mut v_dirs: Py = Py::None;
    let mut v_planes: Py = Py::None;
    v_planes = (if ({ let t2577 = v_structure.clone(); Py::Bool({ let t2578 = Py::str("FCC"); if py::eq(&t2577, &t2578) { true } else { false } }) }).truthy() { Py::list(vec![Py::tuple(vec![Py::Int(1i64), Py::Int(1i64), Py::Int(1i64)]), Py::tuple(vec![py::neg(&Py::Int(1i64))?, Py::Int(1i64), Py::Int(1i64)]), Py::tuple(vec![Py::Int(1i64), py::neg(&Py::Int(1i64))?, Py::Int(1i64)]), Py::tuple(vec![Py::Int(1i64), Py::Int(1i64), py::neg(&Py::Int(1i64))?])]) } else { Py::list(vec![Py::tuple(vec![Py::Int(1i64), Py::Int(1i64), Py::Int(0i64)]), Py::tuple(vec![Py::Int(1i64), py::neg(&Py::Int(1i64))?, Py::Int(0i64)]), Py::tuple(vec![Py::Int(1i64), Py::Int(0i64), Py::Int(1i64)]), Py::tuple(vec![Py::Int(1i64), Py::Int(0i64), py::neg(&Py::Int(1i64))?]), Py::tuple(vec![Py::Int(0i64), Py::Int(1i64), Py::Int(1i64)]), Py::tuple(vec![Py::Int(0i64), Py::Int(1i64), py::neg(&Py::Int(1i64))?])]) });
    v_dirs = (if ({ let t2579 = v_structure.clone(); Py::Bool({ let t2580 = Py::str("FCC"); if py::eq(&t2579, &t2580) { true } else { false } }) }).truthy() { Py::list(vec![Py::tuple(vec![Py::Int(1i64), py::neg(&Py::Int(1i64))?, Py::Int(0i64)]), Py::tuple(vec![Py::Int(1i64), Py::Int(0i64), py::neg(&Py::Int(1i64))?]), Py::tuple(vec![Py::Int(0i64), Py::Int(1i64), py::neg(&Py::Int(1i64))?]), Py::tuple(vec![Py::Int(1i64), Py::Int(1i64), Py::Int(0i64)]), Py::tuple(vec![Py::Int(1i64), Py::Int(0i64), Py::Int(1i64)]), Py::tuple(vec![Py::Int(0i64), Py::Int(1i64), Py::Int(1i64)])]) } else { Py::list(vec![Py::tuple(vec![Py::Int(1i64), Py::Int(1i64), Py::Int(1i64)]), Py::tuple(vec![py::neg(&Py::Int(1i64))?, Py::Int(1i64), Py::Int(1i64)]), Py::tuple(vec![Py::Int(1i64), py::neg(&Py::Int(1i64))?, Py::Int(1i64)]), Py::tuple(vec![Py::Int(1i64), Py::Int(1i64), py::neg(&Py::Int(1i64))?])]) });
    return Ok({ let mut out = vec![]; for t2581 in py::iter(&(v_planes.clone()))? { let mut v_p: Py; 
v_p = t2581;
for t2582 in py::iter(&(v_dirs.clone()))? { let mut v_d: Py; 
v_d = t2582;
if ({ let t2583 = f__dot3(v_p.clone(), v_d.clone())?; Py::Bool({ let t2584 = Py::Int(0i64); if py::eq(&t2583, &t2584) { true } else { false } }) }).truthy() { out.push(Py::tuple(vec![v_p.clone(), v_d.clone()])); }}} Py::list(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__system(mut v_n: Py) -> R {
    let mut v_name: Py = Py::None;
    let mut v_top: Py = Py::None;
    for t3338 in py::iter(&(Py::tuple(vec![Py::tuple(vec![Py::Int(2i64), Py::str("triclinic")]), Py::tuple(vec![Py::Int(15i64), Py::str("monoclinic")]), Py::tuple(vec![Py::Int(74i64), Py::str("orthorhombic")]), Py::tuple(vec![Py::Int(142i64), Py::str("tetragonal")]), Py::tuple(vec![Py::Int(167i64), Py::str("trigonal")]), Py::tuple(vec![Py::Int(194i64), Py::str("hexagonal")]), Py::tuple(vec![Py::Int(230i64), Py::str("cubic")])])))? {
        let t3341 = (t3338).items()?;
        if t3341.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_top = t3341[0].clone();
        v_name = t3341[1].clone();
        if ({ let t3339 = v_n.clone(); Py::Bool({ let t3340 = v_top.clone(); if py::le(&t3339, &t3340)? { true } else { false } }) }).truthy() {
            return Ok(v_name.clone());
        }
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_allowed_reflections(mut v_structure: Py, mut v_count: Py) -> R {
    let mut v_N: Py = Py::None;
    let mut v_Ns: Py = Py::None;
    let mut v_h: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_l: Py = Py::None;
    let mut v_l2: Py = Py::None;
    let mut v_ok: Py = Py::None;
    let mut v_r: Py = Py::None;
    v_count = py::int(&v_count.clone())?;
    if (py::not(&{ let t1 = Py::Int(1i64); Py::Bool({ let t2 = v_count.clone(); if py::le(&t1, &t2)? { { let t3 = Py::Int(15i64); if py::le(&t2, &t3)? { true } else { false } } } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("count must be between 1 and 15"))));
    }
    v_Ns = Py::list(vec![]);
    v_N = Py::Int(1i64);
    while ({ let t29 = py::len(&v_Ns.clone())?; Py::Bool({ let t30 = v_count.clone(); if py::lt(&t29, &t30)? { true } else { false } }) }).truthy() {
        v_ok = Py::Bool(false);
        v_r = py::int(&py::isqrt(&v_N.clone())?)?;
        for t4 in py::range_iter(&[(py::add(&v_r.clone(), &Py::Int(1i64))?).i()?]) {
            v_h = t4;
            for t5 in py::range_iter(&[(py::add(&v_h.clone(), &Py::Int(1i64))?).i()?]) {
                v_k = t5;
                v_l2 = py::sub(&py::sub(&v_N.clone(), &py::mul(&v_h.clone(), &v_h.clone())?)?, &py::mul(&v_k.clone(), &v_k.clone())?)?;
                if ({ let t6 = v_l2.clone(); Py::Bool({ let t7 = Py::Int(0i64); if py::lt(&t6, &t7)? { true } else { false } }) }).truthy() {
                    continue;
                }
                v_l = py::isqrt(&v_l2.clone())?;
                if ({ let t28 = { let t8 = py::mul(&v_l.clone(), &v_l.clone())?; Py::Bool({ let t9 = v_l2.clone(); if py::eq(&t8, &t9) { true } else { false } }) }; if !t28.truthy() { t28 } else { { let t27 = { let t10 = v_l.clone(); Py::Bool({ let t11 = v_k.clone(); if py::le(&t10, &t11)? { true } else { false } }) }; if !t27.truthy() { t27 } else { { let t12 = f__f_rule(v_structure.clone(), v_h.clone(), v_k.clone(), v_l.clone())?; Py::Bool({ let t13 = Py::Int(0i64); if py::gt(&t12, &t13)? { true } else { false } }) } } } } }).truthy() {
                    v_ok = Py::Bool(true);
                }
            }
        }
        if (v_ok.clone()).truthy() {
            let _ = py::method(&v_Ns.clone(), "append", &[v_N.clone()])?;
        }
        v_N = py::iadd(&v_N, &Py::Int(1i64))?;
    }
    return Ok(Py::dict(vec![(Py::str("N_values"), v_Ns.clone()), (Py::str("N_last"), py::getitem(&v_Ns.clone(), &py::neg(&Py::Int(1i64))?)?), (Py::str("N_first"), py::getitem(&v_Ns.clone(), &Py::Int(0i64))?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_angle_directions_cubic(mut v_u1: Py, mut v_v1: Py, mut v_w1: Py, mut v_u2: Py, mut v_v2: Py, mut v_w2: Py) -> R {
    let mut v_I: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_q: Py = Py::None;
    v_p = f__ints(Py::str("the first direction"), Py::tuple(vec![v_u1.clone(), v_v1.clone(), v_w1.clone()]))?;
    v_q = f__ints(Py::str("the second direction"), Py::tuple(vec![v_u2.clone(), v_v2.clone(), v_w2.clone()]))?;
    v_I = Py::list(vec![Py::list(vec![Py::Int(1i64), Py::Int(0i64), Py::Int(0i64)]), Py::list(vec![Py::Int(0i64), Py::Int(1i64), Py::Int(0i64)]), Py::list(vec![Py::Int(0i64), Py::Int(0i64), Py::Int(1i64)])]);
    return Ok(f__out(Py::dict(vec![(Py::str("angle_deg"), f__angle_from(v_I.clone(), v_p.clone(), v_q.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_angle_directions_general(mut v_a: Py, mut v_b: Py, mut v_c: Py, mut v_alpha: Py, mut v_beta: Py, mut v_gamma: Py, mut v_u1: Py, mut v_v1: Py, mut v_w1: Py, mut v_u2: Py, mut v_v2: Py, mut v_w2: Py) -> R {
    let mut v_G: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_q: Py = Py::None;
    let _ = f__cell_check(v_a.clone(), v_b.clone(), v_c.clone(), v_alpha.clone(), v_beta.clone(), v_gamma.clone())?;
    v_p = f__ints(Py::str("the first direction"), Py::tuple(vec![v_u1.clone(), v_v1.clone(), v_w1.clone()]))?;
    v_q = f__ints(Py::str("the second direction"), Py::tuple(vec![v_u2.clone(), v_v2.clone(), v_w2.clone()]))?;
    v_G = f__metric(v_a.clone(), v_b.clone(), v_c.clone(), v_alpha.clone(), v_beta.clone(), v_gamma.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("angle_deg"), f__angle_from(v_G.clone(), v_p.clone(), v_q.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_angle_planes_general(mut v_a: Py, mut v_b: Py, mut v_c: Py, mut v_alpha: Py, mut v_beta: Py, mut v_gamma: Py, mut v_h1: Py, mut v_k1: Py, mut v_l1: Py, mut v_h2: Py, mut v_k2: Py, mut v_l2: Py) -> R {
    let mut v_Gs: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_q: Py = Py::None;
    let _ = f__cell_check(v_a.clone(), v_b.clone(), v_c.clone(), v_alpha.clone(), v_beta.clone(), v_gamma.clone())?;
    v_p = f__ints(Py::str("the first plane"), Py::tuple(vec![v_h1.clone(), v_k1.clone(), v_l1.clone()]))?;
    v_q = f__ints(Py::str("the second plane"), Py::tuple(vec![v_h2.clone(), v_k2.clone(), v_l2.clone()]))?;
    v_Gs = f__inv3(f__metric(v_a.clone(), v_b.clone(), v_c.clone(), v_alpha.clone(), v_beta.clone(), v_gamma.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("angle_deg"), f__angle_from(v_Gs.clone(), v_p.clone(), v_q.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_arrhenius_diffusivity(mut v_D0: Py, mut v_Q: Py, mut v_T: Py) -> R {
    let mut v_x: Py = Py::None;
    let _ = f__pos(Py::str("D0, Q and T"), Py::tuple(vec![v_D0.clone(), v_Q.clone(), v_T.clone()]))?;
    v_x = py::truediv(&v_Q.clone(), &py::mul(&c_RGAS()?, &v_T.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("D"), py::mul(&v_D0.clone(), &py::math1("exp", &py::neg(&v_x.clone())?)?)?), (Py::str("log10_D"), py::sub(&py::math1("log10", &v_D0.clone())?, &py::truediv(&v_x.clone(), &py::math1("log", &Py::Int(10i64))?)?)?), (Py::str("Q_over_RT"), v_x.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_arrhenius_fit(mut v_T: Py, mut v_D: Py, mut v_T_target: Py) -> R {
    let mut v_Q: Py = Py::None;
    let mut v_b0: Py = Py::None;
    let mut v_b1: Py = Py::None;
    let mut v_lnd: Py = Py::None;
    let mut v_out: Py = Py::None;
    let mut v_r2: Py = Py::None;
    let mut v_v: Py = Py::None;
    if ({ let t1431 = py::len(&v_T.clone())?; Py::Bool({ let t1432 = py::len(&v_D.clone())?; if py::ne(&t1431, &t1432) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("T and D must have the same length"))));
    }
    for t1433 in py::iter(&(py::add(&Py::list((v_T.clone()).items()?), &Py::list((v_D.clone()).items()?))?))? {
        v_v = t1433;
        let _ = f__pos(Py::str("every temperature and diffusion coefficient"), Py::tuple(vec![v_v.clone()]))?;
    }
    let t1455 = (f__linfit({ let mut out = vec![]; for t1453 in py::iter(&(v_T.clone()))? { let mut v_t: Py; 
v_t = t1453;
out.push(py::truediv(&Py::Int(1i64), &v_t.clone())?); } Py::list(out) }, { let mut out = vec![]; for t1454 in py::iter(&(v_D.clone()))? { let mut v_d: Py; 
v_d = t1454;
out.push(py::math1("log", &v_d.clone())?); } Py::list(out) })?).items()?;
    if t1455.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_b0 = t1455[0].clone();
    v_b1 = t1455[1].clone();
    v_r2 = t1455[2].clone();
    if ({ let t1456 = v_b1.clone(); Py::Bool({ let t1457 = Py::Int(0i64); if py::ge(&t1456, &t1457)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("D must increase with temperature (a positive activation energy)"))));
    }
    v_Q = py::mul(&py::neg(&v_b1.clone())?, &c_RGAS()?)?;
    v_out = py::dict_from(&Py::dict(vec![]), vec![(Py::str("Q"), v_Q.clone()), (Py::str("Q_kJmol"), py::truediv(&v_Q.clone(), &Py::Int(1000i64))?), (Py::str("D0"), py::math1("exp", &v_b0.clone())?), (Py::str("log10_D0"), py::truediv(&v_b0.clone(), &py::math1("log", &Py::Int(10i64))?)?), (Py::str("r_squared"), v_r2.clone())])?;
    if ({ let t1458 = v_T_target.clone(); Py::Bool({ let t1459 = Py::None; if !py::is(&t1458, &t1459) { true } else { false } }) }).truthy() {
        let _ = f__pos(Py::str("the target temperature"), Py::tuple(vec![v_T_target.clone()]))?;
        v_lnd = py::add(&v_b0.clone(), &py::truediv(&v_b1.clone(), &v_T_target.clone())?)?;
        let _ = { let t1460 = v_out.clone(); py::setitem(&t1460, &Py::str("D_target"), py::math1("exp", &v_lnd.clone())?)?; py::setitem(&t1460, &Py::str("log10_D_target"), py::truediv(&v_lnd.clone(), &py::math1("log", &Py::Int(10i64))?)?)?; Py::None };
    }
    return Ok(f__out(v_out.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_at_to_wt(mut v_element1: Py, mut v_element2: Py, mut v_at1: Py) -> R {
    let mut v_A1: Py = Py::None;
    let mut v_A2: Py = Py::None;
    let mut v_e: Py = Py::None;
    let mut v_wt1: Py = Py::None;
    for t304 in py::iter(&(Py::tuple(vec![v_element1.clone(), v_element2.clone()])))? {
        v_e = t304;
        if ({ let t305 = v_e.clone(); Py::Bool({ let t306 = c_ATOMIC_WEIGHT()?; if !py::contains(&t306, &t305)? { true } else { false } }) }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str(&["unknown element ".to_string(), py::format(&v_e.clone(), "")?].concat()))));
        }
    }
    if ({ let t307 = v_element1.clone(); Py::Bool({ let t308 = v_element2.clone(); if py::eq(&t307, &t308) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the two elements must differ"))));
    }
    if (py::not(&{ let t309 = Py::Int(0i64); Py::Bool({ let t310 = v_at1.clone(); if py::le(&t309, &t310)? { { let t311 = Py::Int(100i64); if py::le(&t310, &t311)? { true } else { false } } } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the atomic percent must lie between 0 and 100"))));
    }
    let t312 = (Py::tuple(vec![py::getitem(&c_ATOMIC_WEIGHT()?, &v_element1.clone())?, py::getitem(&c_ATOMIC_WEIGHT()?, &v_element2.clone())?])).items()?;
    if t312.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_A1 = t312[0].clone();
    v_A2 = t312[1].clone();
    v_wt1 = py::mul(&py::truediv(&py::mul(&v_at1.clone(), &v_A1.clone())?, &py::add(&py::mul(&v_at1.clone(), &v_A1.clone())?, &py::mul(&py::sub(&Py::Int(100i64), &v_at1.clone())?, &v_A2.clone())?)?)?, &Py::Int(100i64))?;
    return Ok(f__out(Py::dict(vec![(Py::str("wt1"), v_wt1.clone()), (Py::str("wt2"), py::sub(&Py::Int(100i64), &v_wt1.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_avrami_fit(mut v_t: Py, mut v_y: Py, mut v_y_target: Py) -> R {
    let mut v_b0: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_out: Py = Py::None;
    let mut v_r2: Py = Py::None;
    let mut v_v: Py = Py::None;
    if ({ let t345 = py::len(&v_t.clone())?; Py::Bool({ let t346 = py::len(&v_y.clone())?; if py::ne(&t345, &t346) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("t and y must have the same length"))));
    }
    for t347 in py::iter(&(v_t.clone()))? {
        v_v = t347;
        let _ = f__pos(Py::str("every time"), Py::tuple(vec![v_v.clone()]))?;
    }
    for t353 in py::iter(&(v_y.clone()))? {
        v_v = t353;
        let _ = f__frac01(Py::str("every transformed fraction"), v_v.clone(), Py::Bool(false))?;
    }
    let t377 = (f__linfit({ let mut out = vec![]; for t375 in py::iter(&(v_t.clone()))? { let mut v_v: Py; 
v_v = t375;
out.push(py::math1("log", &v_v.clone())?); } Py::list(out) }, { let mut out = vec![]; for t376 in py::iter(&(v_y.clone()))? { let mut v_v: Py; 
v_v = t376;
out.push(py::math1("log", &py::neg(&py::math1("log1p", &py::neg(&v_v.clone())?)?)?)?); } Py::list(out) })?).items()?;
    if t377.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_b0 = t377[0].clone();
    v_n = t377[1].clone();
    v_r2 = t377[2].clone();
    if ({ let t378 = v_n.clone(); Py::Bool({ let t379 = Py::Int(0i64); if py::le(&t378, &t379)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the transformed fraction must increase with time"))));
    }
    v_k = py::math1("exp", &v_b0.clone())?;
    v_out = py::dict_from(&Py::dict(vec![]), vec![(Py::str("n"), v_n.clone()), (Py::str("k"), v_k.clone()), (Py::str("log10_k"), py::truediv(&v_b0.clone(), &py::math1("log", &Py::Int(10i64))?)?), (Py::str("t_half"), py::pow(&py::truediv(&py::math1("log", &Py::Int(2i64))?, &v_k.clone())?, &py::truediv(&Py::Int(1i64), &v_n.clone())?)?), (Py::str("r_squared"), v_r2.clone())])?;
    if ({ let t380 = v_y_target.clone(); Py::Bool({ let t381 = Py::None; if !py::is(&t380, &t381) { true } else { false } }) }).truthy() {
        let _ = f__frac01(Py::str("the target fraction"), v_y_target.clone(), Py::Bool(false))?;
        py::setitem(&v_out.clone(), &Py::str("t_target"), py::pow(&py::truediv(&py::neg(&py::math1("log1p", &py::neg(&v_y_target.clone())?)?)?, &v_k.clone())?, &py::truediv(&Py::Int(1i64), &v_n.clone())?)?)?;
    }
    return Ok(f__out(v_out.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_band_gap_fit(mut v_T: Py, mut v_sigma: Py) -> R {
    let mut v_b0: Py = Py::None;
    let mut v_b1: Py = Py::None;
    let mut v_r2: Py = Py::None;
    let mut v_v: Py = Py::None;
    if ({ let t415 = py::len(&v_T.clone())?; Py::Bool({ let t416 = py::len(&v_sigma.clone())?; if py::ne(&t415, &t416) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("T and sigma must have the same length"))));
    }
    for t417 in py::iter(&(py::add(&Py::list((v_T.clone()).items()?), &Py::list((v_sigma.clone()).items()?))?))? {
        v_v = t417;
        let _ = f__pos(Py::str("every temperature and conductivity"), Py::tuple(vec![v_v.clone()]))?;
    }
    let t439 = (f__linfit({ let mut out = vec![]; for t437 in py::iter(&(v_T.clone()))? { let mut v_t: Py; 
v_t = t437;
out.push(py::truediv(&Py::Int(1i64), &v_t.clone())?); } Py::list(out) }, { let mut out = vec![]; for t438 in py::iter(&(v_sigma.clone()))? { let mut v_s: Py; 
v_s = t438;
out.push(py::math1("log", &v_s.clone())?); } Py::list(out) })?).items()?;
    if t439.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_b0 = t439[0].clone();
    v_b1 = t439[1].clone();
    v_r2 = t439[2].clone();
    if ({ let t440 = v_b1.clone(); Py::Bool({ let t441 = Py::Int(0i64); if py::ge(&t440, &t441)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("an intrinsic conductivity rises with temperature"))));
    }
    return Ok(f__out(Py::dict(vec![(Py::str("Eg_eV"), py::truediv(&py::mul(&py::mul(&py::neg(&Py::Int(2i64))?, &v_b1.clone())?, &c_KB()?)?, &c_QE()?)?), (Py::str("prefactor"), py::math1("exp", &v_b0.clone())?), (Py::str("r_squared"), v_r2.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_bragg_angle(mut v_d: Py, mut v_lam: Py, mut v_n: Py) -> R {
    let mut v_s: Py = Py::None;
    let mut v_th: Py = Py::None;
    let _ = f__pos(Py::str("the spacing and wavelength"), Py::tuple(vec![v_d.clone(), v_lam.clone()]))?;
    v_n = py::int(&v_n.clone())?;
    if ({ let t479 = v_n.clone(); Py::Bool({ let t480 = Py::Int(1i64); if py::lt(&t479, &t480)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the order n must be a positive whole number"))));
    }
    v_s = py::truediv(&py::mul(&v_n.clone(), &v_lam.clone())?, &py::mul(&Py::Int(2i64), &v_d.clone())?)?;
    if (py::not(&{ let t481 = v_s.clone(); Py::Bool({ let t482 = Py::Int(1i64); if py::lt(&t481, &t482)? { true } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("n lambda must be smaller than 2d (no reflection otherwise)"))));
    }
    v_th = py::math1("degrees", &py::math1("asin", &v_s.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("theta_deg"), v_th.clone()), (Py::str("two_theta_deg"), py::mul(&Py::Int(2i64), &v_th.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_bragg_spacing(mut v_lam: Py, mut v_theta: Py, mut v_n: Py) -> R {
    let mut v_d: Py = Py::None;
    let _ = f__pos(Py::str("the wavelength"), Py::tuple(vec![v_lam.clone()]))?;
    v_n = py::int(&v_n.clone())?;
    if ({ let t520 = v_n.clone(); Py::Bool({ let t521 = Py::Int(1i64); if py::lt(&t520, &t521)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the order n must be a positive whole number"))));
    }
    if (py::not(&{ let t522 = Py::Int(0i64); Py::Bool({ let t523 = v_theta.clone(); if py::lt(&t522, &t523)? { { let t524 = Py::Int(90i64); if py::lt(&t523, &t524)? { true } else { false } } } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the Bragg angle theta must lie between 0 and 90 degrees"))));
    }
    v_d = py::truediv(&py::mul(&v_n.clone(), &v_lam.clone())?, &py::mul(&Py::Int(2i64), &py::math1("sin", &py::math1("radians", &v_theta.clone())?)?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("d"), v_d.clone()), (Py::str("d_nm"), py::mul(&v_d.clone(), &Py::Float(1000000000.0))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_bragg_wavelength(mut v_d: Py, mut v_theta: Py, mut v_n: Py) -> R {
    let mut v_lam: Py = Py::None;
    let _ = f__pos(Py::str("the spacing"), Py::tuple(vec![v_d.clone()]))?;
    v_n = py::int(&v_n.clone())?;
    if ({ let t562 = v_n.clone(); Py::Bool({ let t563 = Py::Int(1i64); if py::lt(&t562, &t563)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the order n must be a positive whole number"))));
    }
    if (py::not(&{ let t564 = Py::Int(0i64); Py::Bool({ let t565 = v_theta.clone(); if py::lt(&t564, &t565)? { { let t566 = Py::Int(90i64); if py::lt(&t565, &t566)? { true } else { false } } } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the Bragg angle theta must lie between 0 and 90 degrees"))));
    }
    v_lam = py::truediv(&py::mul(&py::mul(&Py::Int(2i64), &v_d.clone())?, &py::math1("sin", &py::math1("radians", &v_theta.clone())?)?)?, &v_n.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("lam"), v_lam.clone()), (Py::str("lam_nm"), py::mul(&v_lam.clone(), &Py::Float(1000000000.0))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_built_in_potential(mut v_Na: Py, mut v_Nd: Py, mut v_ni: Py, mut v_T: Py, mut v_eps_r: Py) -> R {
    let mut v_Vbi: Py = Py::None;
    let mut v_W: Py = Py::None;
    let mut v__np: Py = Py::None;
    let mut v__pn: Py = Py::None;
    let mut v_nn: Py = Py::None;
    let mut v_out: Py = Py::None;
    let mut v_pp: Py = Py::None;
    let mut v_vt: Py = Py::None;
    let _ = f__pos(Py::str("Na, Nd, ni and T"), Py::tuple(vec![v_Na.clone(), v_Nd.clone(), v_ni.clone(), v_T.clone()]))?;
    let t3036 = (f__majority(v_Nd.clone(), Py::Float(0.0), v_ni.clone())?).items()?;
    if t3036.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_nn = t3036[0].clone();
    v__pn = t3036[1].clone();
    let t3037 = (f__majority(Py::Float(0.0), v_Na.clone(), v_ni.clone())?).items()?;
    if t3037.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v__np = t3037[0].clone();
    v_pp = t3037[1].clone();
    v_vt = py::truediv(&py::mul(&c_KB()?, &v_T.clone())?, &c_QE()?)?;
    v_Vbi = py::mul(&v_vt.clone(), &py::math1("log", &py::truediv(&py::mul(&v_nn.clone(), &v_pp.clone())?, &py::mul(&v_ni.clone(), &v_ni.clone())?)?)?)?;
    v_out = py::dict_from(&Py::dict(vec![]), vec![(Py::str("V_bi"), v_Vbi.clone()), (Py::str("V_bi_approx"), py::mul(&v_vt.clone(), &py::add(&py::math1("log", &py::truediv(&v_Na.clone(), &v_ni.clone())?)?, &py::math1("log", &py::truediv(&v_Nd.clone(), &v_ni.clone())?)?)?)?), (Py::str("thermal_voltage"), v_vt.clone())])?;
    if ({ let t3038 = v_eps_r.clone(); Py::Bool({ let t3039 = Py::None; if !py::is(&t3038, &t3039) { true } else { false } }) }).truthy() {
        let _ = f__pos(Py::str("the relative permittivity"), Py::tuple(vec![v_eps_r.clone()]))?;
        if ({ let t3040 = v_Vbi.clone(); Py::Bool({ let t3041 = Py::Int(0i64); if py::le(&t3040, &t3041)? { true } else { false } }) }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("no depletion region for a non-positive built-in potential"))));
        }
        v_W = py::math1("sqrt", &py::mul(&py::truediv(&py::mul(&py::mul(&py::mul(&Py::Int(2i64), &v_eps_r.clone())?, &c_EPS0()?)?, &v_Vbi.clone())?, &c_QE()?)?, &py::add(&py::truediv(&Py::Int(1i64), &v_Na.clone())?, &py::truediv(&Py::Int(1i64), &v_Nd.clone())?)?)?)?;
        let _ = { let t3042 = v_out.clone(); py::setitem(&t3042, &Py::str("W"), v_W.clone())?; py::setitem(&t3042, &Py::str("W_um"), py::mul(&v_W.clone(), &Py::Float(1000000.0))?)?; py::setitem(&t3042, &Py::str("x_n_um"), py::mul(&py::truediv(&py::mul(&v_W.clone(), &v_Na.clone())?, &py::add(&v_Na.clone(), &v_Nd.clone())?)?, &Py::Float(1000000.0))?)?; py::setitem(&t3042, &Py::str("x_p_um"), py::mul(&py::truediv(&py::mul(&v_W.clone(), &v_Nd.clone())?, &py::add(&v_Na.clone(), &v_Nd.clone())?)?, &Py::Float(1000000.0))?)?; Py::None };
    }
    return Ok(f__out(v_out.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_burgers_vector(mut v_structure: Py, mut v_a: Py) -> R {
    let mut v_b: Py = Py::None;
    let _ = f__pos(Py::str("the lattice parameter"), Py::tuple(vec![v_a.clone()]))?;
    v_b = py::mul(&py::getitem(&c__NN_PER_A()?, &v_structure.clone())?, &v_a.clone())?;
    return Ok(Py::dict(vec![(Py::str("b"), f__r(v_b.clone())?), (Py::str("b_nm"), f__r(py::mul(&v_b.clone(), &Py::Float(1000000000.0))?)?), (Py::str("direction"), py::getitem(&c__SLIP()?, &v_structure.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_cell_volume(mut v_a: Py, mut v_b: Py, mut v_c: Py, mut v_alpha: Py, mut v_beta: Py, mut v_gamma: Py) -> R {
    let mut v_V: Py = Py::None;
    let mut v_rad: Py = Py::None;
    v_rad = f__cell_check(v_a.clone(), v_b.clone(), v_c.clone(), v_alpha.clone(), v_beta.clone(), v_gamma.clone())?;
    v_V = py::mul(&py::mul(&py::mul(&v_a.clone(), &v_b.clone())?, &v_c.clone())?, &py::math1("sqrt", &v_rad.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("V"), v_V.clone()), (Py::str("V_nm3"), py::mul(&v_V.clone(), &Py::Float(1e+27))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_ceramic_density(mut v_prototype: Py, mut v_rc: Py, mut v_ra: Py, mut v_Ac: Py, mut v_Aa: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_f: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_rho: Py = Py::None;
    let _ = f__pos(Py::str("the ionic radii"), Py::tuple(vec![v_rc.clone(), v_ra.clone()]))?;
    let _ = f__pos(Py::str("the molar masses"), Py::tuple(vec![v_Ac.clone(), v_Aa.clone()]))?;
    let t639 = (py::getitem(&c__CERAMIC()?, &v_prototype.clone())?).items()?;
    if t639.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_n = t639[0].clone();
    v_k = t639[1].clone();
    v_f = t639[2].clone();
    v_a = py::mul(&v_f.clone(), &py::add(&v_rc.clone(), &v_ra.clone())?)?;
    v_rho = py::truediv(&py::mul(&v_n.clone(), &py::add(&v_Ac.clone(), &py::mul(&v_k.clone(), &v_Aa.clone())?)?)?, &py::mul(&py::pow(&v_a.clone(), &Py::Int(3i64))?, &c_NA()?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("a"), v_a.clone()), (Py::str("a_nm"), py::mul(&v_a.clone(), &Py::Float(1000000000.0))?), (Py::str("rho"), v_rho.clone()), (Py::str("rho_gcc"), py::truediv(&v_rho.clone(), &Py::Int(1000i64))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_chain_dimensions(mut v_n: Py, mut v_l: Py, mut v_theta: Py) -> R {
    let mut v_c: Py = Py::None;
    let mut v_r2: Py = Py::None;
    v_n = py::int(&v_n.clone())?;
    if ({ let t3076 = v_n.clone(); Py::Bool({ let t3077 = Py::Int(1i64); if py::lt(&t3076, &t3077)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the number of bonds must be at least 1"))));
    }
    let _ = f__pos(Py::str("the bond length"), Py::tuple(vec![v_l.clone()]))?;
    if (py::not(&{ let t3083 = Py::Int(90i64); Py::Bool({ let t3084 = v_theta.clone(); if py::lt(&t3083, &t3084)? { { let t3085 = Py::Int(180i64); if py::lt(&t3084, &t3085)? { true } else { false } } } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the bond angle must lie between 90 and 180 degrees"))));
    }
    v_c = py::neg(&f__cos_deg(v_theta.clone())?)?;
    if ({ let t3088 = py::abs(&v_c.clone())?; Py::Bool({ let t3089 = Py::Float(1e-15); if py::lt(&t3088, &t3089)? { true } else { false } }) }).truthy() {
        v_r2 = py::mul(&py::mul(&v_n.clone(), &v_l.clone())?, &v_l.clone())?;
    } else {
        v_r2 = py::mul(&py::mul(&py::mul(&v_n.clone(), &v_l.clone())?, &v_l.clone())?, &py::sub(&py::truediv(&py::add(&Py::Int(1i64), &v_c.clone())?, &py::sub(&Py::Int(1i64), &v_c.clone())?)?, &py::truediv(&py::mul(&py::mul(&Py::Int(2i64), &v_c.clone())?, &py::sub(&Py::Int(1i64), &py::pow(&v_c.clone(), &v_n.clone())?)?)?, &py::mul(&v_n.clone(), &py::pow(&py::sub(&Py::Int(1i64), &v_c.clone())?, &Py::Int(2i64))?)?)?)?)?;
    }
    return Ok(f__out(Py::dict(vec![(Py::str("r_rms"), py::math1("sqrt", &v_r2.clone())?), (Py::str("r_rms_nm"), py::mul(&py::math1("sqrt", &v_r2.clone())?, &Py::Float(1000000000.0))?), (Py::str("C_n"), py::truediv(&v_r2.clone(), &py::mul(&py::mul(&v_n.clone(), &v_l.clone())?, &v_l.clone())?)?), (Py::str("contour_nm"), py::mul(&py::mul(&py::mul(&v_n.clone(), &v_l.clone())?, &py::math1("sin", &py::truediv(&py::math1("radians", &v_theta.clone())?, &Py::Int(2i64))?)?)?, &Py::Float(1000000000.0))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_copolymer_dp(mut v_formula1: Py, mut v_formula2: Py, mut v_f1: Py, mut v_Mn: Py) -> R {
    let mut v_m: Py = Py::None;
    let mut v_m1: Py = Py::None;
    let mut v_m2: Py = Py::None;
    let _ = f__frac01(Py::str("the mole fraction of repeat unit 1"), v_f1.clone(), Py::Bool(true))?;
    let _ = f__pos(Py::str("Mn"), Py::tuple(vec![v_Mn.clone()]))?;
    let t751 = (Py::tuple(vec![f__formula_mass(v_formula1.clone())?, f__formula_mass(v_formula2.clone())?])).items()?;
    if t751.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_m1 = t751[0].clone();
    v_m2 = t751[1].clone();
    v_m = py::add(&py::mul(&v_f1.clone(), &v_m1.clone())?, &py::mul(&py::sub(&Py::Int(1i64), &v_f1.clone())?, &v_m2.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("m_avg_g_mol"), v_m.clone()), (Py::str("DP"), py::truediv(&py::mul(&v_Mn.clone(), &Py::Int(1000i64))?, &v_m.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_creep_fit(mut v_stress_MPa: Py, mut v_T: Py, mut v_rate: Py) -> R {
    let mut v_Q: Py = Py::None;
    let mut v_X: Py = Py::None;
    let mut v_a11: Py = Py::None;
    let mut v_a12: Py = Py::None;
    let mut v_a22: Py = Py::None;
    let mut v_b1: Py = Py::None;
    let mut v_b2: Py = Py::None;
    let mut v_det: Py = Py::None;
    let mut v_m1: Py = Py::None;
    let mut v_m2: Py = Py::None;
    let mut v_my: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_nexp: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_y: Py = Py::None;
    v_n = py::len(&v_stress_MPa.clone())?;
    if ({ let t789 = py::not(&{ let t784 = v_n.clone(); Py::Bool({ let t785 = py::len(&v_T.clone())?; if py::eq(&t784, &t785) { { let t786 = py::len(&v_rate.clone())?; if py::eq(&t785, &t786) { true } else { false } } } else { false } }) }); if t789.truthy() { t789 } else { { let t787 = v_n.clone(); Py::Bool({ let t788 = Py::Int(3i64); if py::lt(&t787, &t788)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("stress, T and rate must be lists of the same length (at least 3 points)"))));
    }
    for t790 in py::iter(&(py::add(&py::add(&Py::list((v_stress_MPa.clone()).items()?), &Py::list((v_T.clone()).items()?))?, &Py::list((v_rate.clone()).items()?))?))? {
        v_v = t790;
        let _ = f__pos(Py::str("every stress, temperature and creep rate"), Py::tuple(vec![v_v.clone()]))?;
    }
    v_X = { let mut out = vec![]; for t796 in py::iter(&(py::zip(&[v_stress_MPa.clone(), v_T.clone()])?))? { let mut v_s: Py; let mut v_t: Py; 
let t797 = (t796).items()?;
if t797.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_s = t797[0].clone();
v_t = t797[1].clone();
out.push(Py::list(vec![Py::Float(1.0), py::math1("log", &v_s.clone())?, py::truediv(&py::neg(&Py::Int(1i64))?, &py::mul(&c_RGAS()?, &v_t.clone())?)?])); } Py::list(out) };
    v_y = { let mut out = vec![]; for t798 in py::iter(&(v_rate.clone()))? { let mut v_r_: Py; 
v_r_ = t798;
out.push(py::math1("log", &v_r_.clone())?); } Py::list(out) };
    v_m1 = py::truediv(&py::sum(&{ let mut out = vec![]; for t799 in py::iter(&(v_X.clone()))? { let mut v_r_: Py; 
v_r_ = t799;
out.push(py::getitem(&v_r_.clone(), &Py::Int(1i64))?); } Py::list(out) }, None)?, &v_n.clone())?;
    v_m2 = py::truediv(&py::sum(&{ let mut out = vec![]; for t800 in py::iter(&(v_X.clone()))? { let mut v_r_: Py; 
v_r_ = t800;
out.push(py::getitem(&v_r_.clone(), &Py::Int(2i64))?); } Py::list(out) }, None)?, &v_n.clone())?;
    v_my = py::truediv(&py::sum(&v_y.clone(), None)?, &v_n.clone())?;
    v_a11 = py::sum(&{ let mut out = vec![]; for t801 in py::iter(&(v_X.clone()))? { let mut v_r_: Py; 
v_r_ = t801;
out.push(py::pow(&py::sub(&py::getitem(&v_r_.clone(), &Py::Int(1i64))?, &v_m1.clone())?, &Py::Int(2i64))?); } Py::list(out) }, None)?;
    v_a22 = py::sum(&{ let mut out = vec![]; for t802 in py::iter(&(v_X.clone()))? { let mut v_r_: Py; 
v_r_ = t802;
out.push(py::pow(&py::sub(&py::getitem(&v_r_.clone(), &Py::Int(2i64))?, &v_m2.clone())?, &Py::Int(2i64))?); } Py::list(out) }, None)?;
    v_a12 = py::sum(&{ let mut out = vec![]; for t803 in py::iter(&(v_X.clone()))? { let mut v_r_: Py; 
v_r_ = t803;
out.push(py::mul(&py::sub(&py::getitem(&v_r_.clone(), &Py::Int(1i64))?, &v_m1.clone())?, &py::sub(&py::getitem(&v_r_.clone(), &Py::Int(2i64))?, &v_m2.clone())?)?); } Py::list(out) }, None)?;
    v_det = py::sub(&py::mul(&v_a11.clone(), &v_a22.clone())?, &py::mul(&v_a12.clone(), &v_a12.clone())?)?;
    if ({ let t811 = { let t804 = v_a11.clone(); Py::Bool({ let t805 = Py::Int(0i64); if py::le(&t804, &t805)? { true } else { false } }) }; if t811.truthy() { t811 } else { { let t810 = { let t806 = v_a22.clone(); Py::Bool({ let t807 = Py::Int(0i64); if py::le(&t806, &t807)? { true } else { false } }) }; if t810.truthy() { t810 } else { { let t808 = v_det.clone(); Py::Bool({ let t809 = py::mul(&py::mul(&Py::Float(1e-08), &v_a11.clone())?, &v_a22.clone())?; if py::le(&t808, &t809)? { true } else { false } }) } } } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the stresses and temperatures must both vary, and not in proportion"))));
    }
    v_b1 = py::sum(&{ let mut out = vec![]; for t812 in py::iter(&(py::zip(&[v_X.clone(), v_y.clone()])?))? { let mut v_r_: Py; let mut v_v: Py; 
let t813 = (t812).items()?;
if t813.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_r_ = t813[0].clone();
v_v = t813[1].clone();
out.push(py::mul(&py::sub(&py::getitem(&v_r_.clone(), &Py::Int(1i64))?, &v_m1.clone())?, &py::sub(&v_v.clone(), &v_my.clone())?)?); } Py::list(out) }, None)?;
    v_b2 = py::sum(&{ let mut out = vec![]; for t814 in py::iter(&(py::zip(&[v_X.clone(), v_y.clone()])?))? { let mut v_r_: Py; let mut v_v: Py; 
let t815 = (t814).items()?;
if t815.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_r_ = t815[0].clone();
v_v = t815[1].clone();
out.push(py::mul(&py::sub(&py::getitem(&v_r_.clone(), &Py::Int(2i64))?, &v_m2.clone())?, &py::sub(&v_v.clone(), &v_my.clone())?)?); } Py::list(out) }, None)?;
    v_nexp = py::truediv(&py::sub(&py::mul(&v_b1.clone(), &v_a22.clone())?, &py::mul(&v_b2.clone(), &v_a12.clone())?)?, &v_det.clone())?;
    v_Q = py::truediv(&py::sub(&py::mul(&v_a11.clone(), &v_b2.clone())?, &py::mul(&v_a12.clone(), &v_b1.clone())?)?, &v_det.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("n"), v_nexp.clone()), (Py::str("Q_kJmol"), py::truediv(&v_Q.clone(), &Py::Int(1000i64))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_critical_crack(mut v_K_IC: Py, mut v_sigma: Py, mut v_Y: Py) -> R {
    let mut v_a: Py = Py::None;
    let _ = f__pos(Py::str("K_IC, the stress and Y"), Py::tuple(vec![v_K_IC.clone(), v_sigma.clone(), v_Y.clone()]))?;
    v_a = py::truediv(&py::pow(&py::truediv(&v_K_IC.clone(), &py::mul(&v_Y.clone(), &v_sigma.clone())?)?, &Py::Int(2i64))?, &Py::Float(std::f64::consts::PI))?;
    return Ok(f__out(Py::dict(vec![(Py::str("a_c"), v_a.clone()), (Py::str("a_c_mm"), py::mul(&v_a.clone(), &Py::Float(1000.0))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_critical_fiber_length(mut v_sigma_f: Py, mut v_d: Py, mut v_tau_c: Py) -> R {
    let mut v_lc: Py = Py::None;
    let _ = f__pos(Py::str("the fibre strength, diameter and bond strength"), Py::tuple(vec![v_sigma_f.clone(), v_d.clone(), v_tau_c.clone()]))?;
    v_lc = py::truediv(&py::mul(&v_sigma_f.clone(), &v_d.clone())?, &py::mul(&Py::Int(2i64), &v_tau_c.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("lc"), v_lc.clone()), (Py::str("lc_mm"), py::mul(&v_lc.clone(), &Py::Float(1000.0))?), (Py::str("aspect_ratio"), py::truediv(&v_lc.clone(), &v_d.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_crystallinity(mut v_rho_s: Py, mut v_rho_c: Py, mut v_rho_a: Py) -> R {
    let mut v_pc: Py = Py::None;
    let _ = f__pos(Py::str("the densities"), Py::tuple(vec![v_rho_s.clone(), v_rho_c.clone(), v_rho_a.clone()]))?;
    if (py::not(&{ let t3127 = v_rho_a.clone(); Py::Bool({ let t3128 = v_rho_s.clone(); if py::lt(&t3127, &t3128)? { { let t3129 = v_rho_c.clone(); if py::lt(&t3128, &t3129)? { true } else { false } } } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the sample density must lie between the amorphous and the crystalline densities"))));
    }
    v_pc = py::mul(&py::truediv(&py::mul(&v_rho_c.clone(), &py::sub(&v_rho_s.clone(), &v_rho_a.clone())?)?, &py::mul(&v_rho_s.clone(), &py::sub(&v_rho_c.clone(), &v_rho_a.clone())?)?)?, &Py::Int(100i64))?;
    return Ok(f__out(Py::dict(vec![(Py::str("crystallinity_percent"), v_pc.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_cubic_lattice_parameter(mut v_structure: Py, mut v_R: Py) -> R {
    let mut v_a: Py = Py::None;
    let _ = f__pos(Py::str("the atomic radius"), Py::tuple(vec![v_R.clone()]))?;
    v_a = py::mul(&py::getitem(&c__A_PER_R()?, &v_structure.clone())?, &v_R.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("a"), v_a.clone()), (Py::str("a_nm"), py::mul(&v_a.clone(), &Py::Float(1000000000.0))?), (Py::str("nearest_neighbour_nm"), py::mul(&py::mul(&Py::Int(2i64), &v_R.clone())?, &Py::Float(1000000000.0))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_cubic_reflection(mut v_a: Py, mut v_h: Py, mut v_k: Py, mut v_l: Py, mut v_lam: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_th: Py = Py::None;
    let _ = f__pos(Py::str("the lattice parameter and wavelength"), Py::tuple(vec![v_a.clone(), v_lam.clone()]))?;
    let t973 = (f__ints(Py::str("the Miller indices"), Py::tuple(vec![v_h.clone(), v_k.clone(), v_l.clone()]))?).items()?;
    if t973.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_h = t973[0].clone();
    v_k = t973[1].clone();
    v_l = t973[2].clone();
    v_d = py::truediv(&v_a.clone(), &py::math1("sqrt", &py::add(&py::add(&py::mul(&v_h.clone(), &v_h.clone())?, &py::mul(&v_k.clone(), &v_k.clone())?)?, &py::mul(&v_l.clone(), &v_l.clone())?)?)?)?;
    v_s = py::truediv(&v_lam.clone(), &py::mul(&Py::Int(2i64), &v_d.clone())?)?;
    if (py::not(&{ let t974 = v_s.clone(); Py::Bool({ let t975 = Py::Int(1i64); if py::lt(&t974, &t975)? { true } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("this reflection does not occur at this wavelength (lambda >= 2d)"))));
    }
    v_th = py::math1("degrees", &py::math1("asin", &v_s.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("d_nm"), py::mul(&v_d.clone(), &Py::Float(1000000000.0))?), (Py::str("theta_deg"), v_th.clone()), (Py::str("two_theta_deg"), py::mul(&Py::Int(2i64), &v_th.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_d_cubic(mut v_a: Py, mut v_h: Py, mut v_k: Py, mut v_l: Py) -> R {
    let mut v_d: Py = Py::None;
    let _ = f__pos(Py::str("the lattice parameter"), Py::tuple(vec![v_a.clone()]))?;
    let t1022 = (f__ints(Py::str("the Miller indices"), Py::tuple(vec![v_h.clone(), v_k.clone(), v_l.clone()]))?).items()?;
    if t1022.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_h = t1022[0].clone();
    v_k = t1022[1].clone();
    v_l = t1022[2].clone();
    v_d = py::truediv(&v_a.clone(), &py::math1("sqrt", &py::add(&py::add(&py::mul(&v_h.clone(), &v_h.clone())?, &py::mul(&v_k.clone(), &v_k.clone())?)?, &py::mul(&v_l.clone(), &v_l.clone())?)?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("d"), v_d.clone()), (Py::str("d_nm"), py::mul(&v_d.clone(), &Py::Float(1000000000.0))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_d_hexagonal(mut v_a: Py, mut v_c: Py, mut v_h: Py, mut v_k: Py, mut v_l: Py, mut v_i: Py) -> R {
    let mut v_d: Py = Py::None;
    let _ = f__pos(Py::str("the lattice parameters"), Py::tuple(vec![v_a.clone(), v_c.clone()]))?;
    let t1089 = (f__ints(Py::str("the Miller indices"), Py::tuple(vec![v_h.clone(), v_k.clone(), v_l.clone()]))?).items()?;
    if t1089.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_h = t1089[0].clone();
    v_k = t1089[1].clone();
    v_l = t1089[2].clone();
    if ({ let t1094 = { let t1090 = v_i.clone(); Py::Bool({ let t1091 = Py::None; if !py::is(&t1090, &t1091) { true } else { false } }) }; if !t1094.truthy() { t1094 } else { { let t1092 = py::int(&v_i.clone())?; Py::Bool({ let t1093 = py::neg(&py::add(&v_h.clone(), &v_k.clone())?)?; if py::ne(&t1092, &t1093) { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("in (h k i l) the index i must equal -(h + k)"))));
    }
    v_d = py::truediv(&Py::Int(1i64), &py::math1("sqrt", &py::add(&py::truediv(&py::mul(&py::truediv(&Py::Int(4i64), &Py::Int(3i64))?, &py::add(&py::add(&py::mul(&v_h.clone(), &v_h.clone())?, &py::mul(&v_h.clone(), &v_k.clone())?)?, &py::mul(&v_k.clone(), &v_k.clone())?)?)?, &py::mul(&v_a.clone(), &v_a.clone())?)?, &py::truediv(&py::mul(&v_l.clone(), &v_l.clone())?, &py::mul(&v_c.clone(), &v_c.clone())?)?)?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("d"), v_d.clone()), (Py::str("d_nm"), py::mul(&v_d.clone(), &Py::Float(1000000000.0))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_d_orthorhombic(mut v_a: Py, mut v_b: Py, mut v_c: Py, mut v_h: Py, mut v_k: Py, mut v_l: Py) -> R {
    let mut v_d: Py = Py::None;
    let _ = f__pos(Py::str("the lattice parameters"), Py::tuple(vec![v_a.clone(), v_b.clone(), v_c.clone()]))?;
    let t1161 = (f__ints(Py::str("the Miller indices"), Py::tuple(vec![v_h.clone(), v_k.clone(), v_l.clone()]))?).items()?;
    if t1161.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_h = t1161[0].clone();
    v_k = t1161[1].clone();
    v_l = t1161[2].clone();
    v_d = py::truediv(&Py::Int(1i64), &py::math1("sqrt", &py::add(&py::add(&py::truediv(&py::mul(&v_h.clone(), &v_h.clone())?, &py::mul(&v_a.clone(), &v_a.clone())?)?, &py::truediv(&py::mul(&v_k.clone(), &v_k.clone())?, &py::mul(&v_b.clone(), &v_b.clone())?)?)?, &py::truediv(&py::mul(&v_l.clone(), &v_l.clone())?, &py::mul(&v_c.clone(), &v_c.clone())?)?)?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("d"), v_d.clone()), (Py::str("d_nm"), py::mul(&v_d.clone(), &Py::Float(1000000000.0))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_d_tetragonal(mut v_a: Py, mut v_c: Py, mut v_h: Py, mut v_k: Py, mut v_l: Py) -> R {
    let mut v_d: Py = Py::None;
    let _ = f__pos(Py::str("the lattice parameters"), Py::tuple(vec![v_a.clone(), v_c.clone()]))?;
    let t1228 = (f__ints(Py::str("the Miller indices"), Py::tuple(vec![v_h.clone(), v_k.clone(), v_l.clone()]))?).items()?;
    if t1228.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_h = t1228[0].clone();
    v_k = t1228[1].clone();
    v_l = t1228[2].clone();
    v_d = py::truediv(&Py::Int(1i64), &py::math1("sqrt", &py::add(&py::truediv(&py::add(&py::mul(&v_h.clone(), &v_h.clone())?, &py::mul(&v_k.clone(), &v_k.clone())?)?, &py::mul(&v_a.clone(), &v_a.clone())?)?, &py::truediv(&py::mul(&v_l.clone(), &v_l.clone())?, &py::mul(&v_c.clone(), &v_c.clone())?)?)?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("d"), v_d.clone()), (Py::str("d_nm"), py::mul(&v_d.clone(), &Py::Float(1000000000.0))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_d_triclinic(mut v_a: Py, mut v_b: Py, mut v_c: Py, mut v_alpha: Py, mut v_beta: Py, mut v_gamma: Py, mut v_h: Py, mut v_k: Py, mut v_l: Py) -> R {
    let mut v_d: Py = Py::None;
    v_d = f__d_general(v_a.clone(), v_b.clone(), v_c.clone(), v_alpha.clone(), v_beta.clone(), v_gamma.clone(), v_h.clone(), v_k.clone(), v_l.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("d"), v_d.clone()), (Py::str("d_nm"), py::mul(&v_d.clone(), &Py::Float(1000000000.0))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(non_snake_case)]
fn f_debye_heat_capacity(v_theta_D: Py, v_T: Py) -> R {
    ax_engine::pysym::helper("materials_science:debye_heat_capacity", &[v_theta_D, v_T], &[])
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_degree_of_polymerization(mut v_formula: Py, mut v_Mn: Py) -> R {
    let mut v_m: Py = Py::None;
    let _ = f__pos(Py::str("the number-average molecular weight"), Py::tuple(vec![v_Mn.clone()]))?;
    v_m = f__formula_mass(v_formula.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("repeat_unit_g_mol"), v_m.clone()), (Py::str("DP"), py::truediv(&py::mul(&v_Mn.clone(), &Py::Int(1000i64))?, &v_m.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_diffusion_depth(mut v_C0: Py, mut v_Cs: Py, mut v_Cx: Py, mut v_D: Py, mut v_t: Py) -> R {
    let mut v_x: Py = Py::None;
    let mut v_y: Py = Py::None;
    let mut v_z: Py = Py::None;
    let _ = f__pos(Py::str("D and t"), Py::tuple(vec![v_D.clone(), v_t.clone()]))?;
    if ({ let t1499 = v_Cs.clone(); Py::Bool({ let t1500 = v_C0.clone(); if py::eq(&t1499, &t1500) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the surface and initial concentrations must differ"))));
    }
    v_y = py::truediv(&py::sub(&v_Cs.clone(), &v_Cx.clone())?, &py::sub(&v_Cs.clone(), &v_C0.clone())?)?;
    if (py::not(&{ let t1501 = Py::Float(0.002); Py::Bool({ let t1502 = v_y.clone(); if py::lt(&t1501, &t1502)? { { let t1503 = Py::Float(0.998); if py::lt(&t1502, &t1503)? { true } else { false } } } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the target concentration must lie strictly between C0 and Cs (and not within 0.2 % of either)"))));
    }
    v_z = f__inverse_erf(v_y.clone())?;
    v_x = py::mul(&py::mul(&Py::Int(2i64), &v_z.clone())?, &py::math1("sqrt", &py::mul(&v_D.clone(), &v_t.clone())?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("x"), v_x.clone()), (Py::str("x_mm"), py::mul(&v_x.clone(), &Py::Float(1000.0))?), (Py::str("z"), v_z.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_diffusion_time(mut v_C0: Py, mut v_Cs: Py, mut v_Cx: Py, mut v_x: Py, mut v_D: Py) -> R {
    let mut v_t: Py = Py::None;
    let mut v_y: Py = Py::None;
    let mut v_z: Py = Py::None;
    let _ = f__pos(Py::str("x and D"), Py::tuple(vec![v_x.clone(), v_D.clone()]))?;
    if ({ let t1550 = v_Cs.clone(); Py::Bool({ let t1551 = v_C0.clone(); if py::eq(&t1550, &t1551) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the surface and initial concentrations must differ"))));
    }
    v_y = py::truediv(&py::sub(&v_Cs.clone(), &v_Cx.clone())?, &py::sub(&v_Cs.clone(), &v_C0.clone())?)?;
    if (py::not(&{ let t1552 = Py::Float(0.002); Py::Bool({ let t1553 = v_y.clone(); if py::lt(&t1552, &t1553)? { { let t1554 = Py::Float(0.998); if py::lt(&t1553, &t1554)? { true } else { false } } } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the target concentration must lie strictly between C0 and Cs (and not within 0.2 % of either)"))));
    }
    v_z = f__inverse_erf(v_y.clone())?;
    v_t = py::truediv(&py::mul(&v_x.clone(), &v_x.clone())?, &py::mul(&py::mul(&py::mul(&Py::Int(4i64), &v_D.clone())?, &v_z.clone())?, &v_z.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("t"), v_t.clone()), (Py::str("t_hours"), py::truediv(&v_t.clone(), &Py::Int(3600i64))?), (Py::str("z"), v_z.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_einstein_heat_capacity(mut v_theta_E: Py, mut v_T: Py) -> R {
    let mut v_Cv: Py = Py::None;
    let mut v_x: Py = Py::None;
    let _ = f__pos(Py::str("the Einstein temperature and the temperature"), Py::tuple(vec![v_theta_E.clone(), v_T.clone()]))?;
    v_x = py::truediv(&v_theta_E.clone(), &v_T.clone())?;
    if ({ let t1601 = v_x.clone(); Py::Bool({ let t1602 = Py::Int(700i64); if py::gt(&t1601, &t1602)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("T is far below theta_E: the heat capacity underflows to 0"))));
    }
    v_Cv = py::truediv(&py::mul(&py::mul(&py::mul(&py::mul(&Py::Int(3i64), &c_RGAS()?)?, &v_x.clone())?, &v_x.clone())?, &py::math1("exp", &py::neg(&v_x.clone())?)?)?, &py::pow(&py::neg(&py::math1("expm1", &py::neg(&v_x.clone())?)?)?, &Py::Int(2i64))?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("Cv"), v_Cv.clone()), (Py::str("Cv_over_3R"), py::truediv(&v_Cv.clone(), &py::mul(&Py::Int(3i64), &c_RGAS()?)?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_elastic_constants(mut v_E: Py, mut v_nu: Py) -> R {
    let mut v_G: Py = Py::None;
    let mut v_K: Py = Py::None;
    let mut v_lam: Py = Py::None;
    let _ = f__pos(Py::str("Young's modulus"), Py::tuple(vec![v_E.clone()]))?;
    if (py::not(&{ let t2258 = py::neg(&Py::Int(1i64))?; Py::Bool({ let t2259 = v_nu.clone(); if py::lt(&t2258, &t2259)? { { let t2260 = Py::Float(0.5); if py::lt(&t2259, &t2260)? { true } else { false } } } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("Poisson's ratio must lie between -1 and 0.5"))));
    }
    v_G = py::truediv(&v_E.clone(), &py::mul(&Py::Int(2i64), &py::add(&Py::Int(1i64), &v_nu.clone())?)?)?;
    v_K = py::truediv(&v_E.clone(), &py::mul(&Py::Int(3i64), &py::sub(&Py::Int(1i64), &py::mul(&Py::Int(2i64), &v_nu.clone())?)?)?)?;
    v_lam = py::truediv(&py::mul(&v_E.clone(), &v_nu.clone())?, &py::mul(&py::add(&Py::Int(1i64), &v_nu.clone())?, &py::sub(&Py::Int(1i64), &py::mul(&Py::Int(2i64), &v_nu.clone())?)?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("G"), v_G.clone()), (Py::str("G_GPa"), py::truediv(&v_G.clone(), &Py::Float(1000000000.0))?), (Py::str("K"), v_K.clone()), (Py::str("K_GPa"), py::truediv(&v_K.clone(), &Py::Float(1000000000.0))?), (Py::str("lame_GPa"), py::truediv(&v_lam.clone(), &Py::Float(1000000000.0))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_elastic_deformation(mut v_F: Py, mut v_d0: Py, mut v_l0: Py, mut v_E: Py, mut v_nu: Py) -> R {
    let mut v_A0: Py = Py::None;
    let mut v_ex: Py = Py::None;
    let mut v_ez: Py = Py::None;
    let mut v_s: Py = Py::None;
    let _ = f__pos(Py::str("F, d0, l0 and E"), Py::tuple(vec![v_F.clone(), v_d0.clone(), v_l0.clone(), v_E.clone()]))?;
    if (py::not(&{ let t1640 = py::neg(&Py::Int(1i64))?; Py::Bool({ let t1641 = v_nu.clone(); if py::lt(&t1640, &t1641)? { { let t1642 = Py::Float(0.5); if py::lt(&t1641, &t1642)? { true } else { false } } } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("Poisson's ratio must lie between -1 and 0.5"))));
    }
    v_A0 = py::truediv(&py::mul(&py::mul(&Py::Float(std::f64::consts::PI), &v_d0.clone())?, &v_d0.clone())?, &Py::Int(4i64))?;
    v_s = py::truediv(&v_F.clone(), &v_A0.clone())?;
    v_ez = py::truediv(&v_s.clone(), &v_E.clone())?;
    v_ex = py::mul(&py::neg(&v_nu.clone())?, &v_ez.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("stress_MPa"), py::truediv(&v_s.clone(), &Py::Float(1000000.0))?), (Py::str("axial_strain"), v_ez.clone()), (Py::str("elongation_mm"), py::mul(&py::mul(&v_ez.clone(), &v_l0.clone())?, &Py::Float(1000.0))?), (Py::str("lateral_strain"), v_ex.clone()), (Py::str("diameter_change_um"), py::mul(&py::mul(&v_ex.clone(), &v_d0.clone())?, &Py::Float(1000000.0))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_eutectic_constituents(mut v_C0: Py, mut v_C_alpha: Py, mut v_C_E: Py, mut v_C_beta: Py) -> R {
    let mut v_Wa: Py = Py::None;
    let mut v_Wp: Py = Py::None;
    let mut v_primary: Py = Py::None;
    if (py::not(&{ let t1675 = v_C_alpha.clone(); Py::Bool({ let t1676 = v_C_E.clone(); if py::lt(&t1675, &t1676)? { { let t1677 = v_C_beta.clone(); if py::lt(&t1676, &t1677)? { true } else { false } } } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("compositions must satisfy C_alpha < C_E < C_beta"))));
    }
    let _ = f__between(Py::str("C0"), v_C0.clone(), v_C_alpha.clone(), v_C_beta.clone(), Py::Bool(true))?;
    if ({ let t1688 = v_C0.clone(); Py::Bool({ let t1689 = v_C_E.clone(); if py::le(&t1688, &t1689)? { true } else { false } }) }).truthy() {
        let t1690 = (Py::tuple(vec![Py::str("alpha"), py::truediv(&py::sub(&v_C_E.clone(), &v_C0.clone())?, &py::sub(&v_C_E.clone(), &v_C_alpha.clone())?)?])).items()?;
        if t1690.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_primary = t1690[0].clone();
        v_Wp = t1690[1].clone();
    } else {
        let t1691 = (Py::tuple(vec![Py::str("beta"), py::truediv(&py::sub(&v_C0.clone(), &v_C_E.clone())?, &py::sub(&v_C_beta.clone(), &v_C_E.clone())?)?])).items()?;
        if t1691.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_primary = t1691[0].clone();
        v_Wp = t1691[1].clone();
    }
    v_Wa = py::truediv(&py::sub(&v_C_beta.clone(), &v_C0.clone())?, &py::sub(&v_C_beta.clone(), &v_C_alpha.clone())?)?;
    return Ok({ let t1692 = Py::dict(vec![]); py::kw_merge(&t1692, &f__out(Py::dict(vec![(Py::str("W_primary"), v_Wp.clone()), (Py::str("W_eutectic"), py::sub(&Py::Int(1i64), &v_Wp.clone())?), (Py::str("W_alpha_total"), v_Wa.clone()), (Py::str("W_beta_total"), py::sub(&Py::Int(1i64), &v_Wa.clone())?)]))?)?; py::setitem(&t1692, &Py::str("primary_phase"), v_primary.clone())?; t1692 });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_extrinsic_carriers(mut v_Nd: Py, mut v_Na: Py, mut v_ni: Py, mut v_T: Py) -> R {
    let mut v_n: Py = Py::None;
    let mut v_out: Py = Py::None;
    let mut v_p: Py = Py::None;
    if (py::not(&{ let t1731 = { let t1725 = v_Nd.clone(); Py::Bool({ let t1726 = Py::Int(0i64); if py::ge(&t1725, &t1726)? { true } else { false } }) }; if !t1731.truthy() { t1731 } else { { let t1730 = { let t1727 = v_Na.clone(); Py::Bool({ let t1728 = Py::Int(0i64); if py::ge(&t1727, &t1728)? { true } else { false } }) }; if !t1730.truthy() { t1730 } else { { let t1729 = py::math1("isfinite", &v_Nd.clone())?; if !t1729.truthy() { t1729 } else { py::math1("isfinite", &v_Na.clone())? } } } } } })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the donor and acceptor concentrations must be zero or positive"))));
    }
    let _ = f__pos(Py::str("the intrinsic concentration"), Py::tuple(vec![v_ni.clone()]))?;
    let t1739 = (f__majority(v_Nd.clone(), v_Na.clone(), v_ni.clone())?).items()?;
    if t1739.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_n = t1739[0].clone();
    v_p = t1739[1].clone();
    v_out = py::dict_from(&Py::dict(vec![]), vec![(Py::str("n"), v_n.clone()), (Py::str("p"), v_p.clone()), (Py::str("log10_n"), py::math1("log10", &v_n.clone())?), (Py::str("log10_p"), py::math1("log10", &v_p.clone())?), (Py::str("carrier_type"), (if ({ let t1740 = v_n.clone(); Py::Bool({ let t1741 = v_p.clone(); if py::gt(&t1740, &t1741)? { true } else { false } }) }).truthy() { Py::str("n-type") } else { (if ({ let t1742 = v_p.clone(); Py::Bool({ let t1743 = v_n.clone(); if py::gt(&t1742, &t1743)? { true } else { false } }) }).truthy() { Py::str("p-type") } else { Py::str("intrinsic") }) }))])?;
    if ({ let t1744 = v_T.clone(); Py::Bool({ let t1745 = Py::None; if !py::is(&t1744, &t1745) { true } else { false } }) }).truthy() {
        let _ = f__pos(Py::str("the temperature"), Py::tuple(vec![v_T.clone()]))?;
        py::setitem(&v_out.clone(), &Py::str("EF_minus_Ei_eV"), py::mul(&py::truediv(&py::mul(&c_KB()?, &v_T.clone())?, &c_QE()?)?, &py::math1("log", &py::truediv(&v_n.clone(), &v_ni.clone())?)?)?)?;
    }
    return Ok(f__out(v_out.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_fermi_energy(mut v_rho: Py, mut v_A: Py, mut v_valence: Py) -> R {
    let mut v_EF: Py = Py::None;
    let mut v_n: Py = Py::None;
    let _ = f__pos(Py::str("the density, molar mass and valence"), Py::tuple(vec![v_rho.clone(), v_A.clone(), v_valence.clone()]))?;
    v_n = py::truediv(&py::mul(&py::mul(&v_valence.clone(), &c_NA()?)?, &v_rho.clone())?, &v_A.clone())?;
    v_EF = py::mul(&py::truediv(&py::mul(&c_HBAR()?, &c_HBAR()?)?, &py::mul(&Py::Int(2i64), &c_ME()?)?)?, &py::pow(&py::mul(&py::mul(&py::mul(&Py::Int(3i64), &Py::Float(std::f64::consts::PI))?, &Py::Float(std::f64::consts::PI))?, &v_n.clone())?, &py::truediv(&Py::Int(2i64), &Py::Int(3i64))?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("n"), v_n.clone()), (Py::str("log10_n"), py::math1("log10", &v_n.clone())?), (Py::str("EF"), v_EF.clone()), (Py::str("EF_eV"), py::truediv(&v_EF.clone(), &c_QE()?)?), (Py::str("TF"), py::truediv(&v_EF.clone(), &c_KB()?)?), (Py::str("vF"), py::math1("sqrt", &py::truediv(&py::mul(&Py::Int(2i64), &v_EF.clone())?, &c_ME()?)?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_fick_first(mut v_D: Py, mut v_C1: Py, mut v_C2: Py, mut v_x1: Py, mut v_x2: Py) -> R {
    let mut v_J: Py = Py::None;
    let _ = f__pos(Py::str("the diffusion coefficient"), Py::tuple(vec![v_D.clone()]))?;
    if ({ let t1821 = v_x1.clone(); Py::Bool({ let t1822 = v_x2.clone(); if py::eq(&t1821, &t1822) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the two positions must differ"))));
    }
    v_J = py::truediv(&py::mul(&py::neg(&v_D.clone())?, &py::sub(&v_C2.clone(), &v_C1.clone())?)?, &py::sub(&v_x2.clone(), &v_x1.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("J"), v_J.clone()), (Py::str("J_per_hour"), py::mul(&v_J.clone(), &Py::Int(3600i64))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_fick_second(mut v_C0: Py, mut v_Cs: Py, mut v_x: Py, mut v_D: Py, mut v_t: Py) -> R {
    let mut v_Cx: Py = Py::None;
    let mut v_z: Py = Py::None;
    let _ = f__pos(Py::str("x, D and t"), Py::tuple(vec![v_x.clone(), v_D.clone(), v_t.clone()]))?;
    if ({ let t1860 = v_Cs.clone(); Py::Bool({ let t1861 = v_C0.clone(); if py::eq(&t1860, &t1861) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the surface and initial concentrations must differ"))));
    }
    v_z = py::truediv(&v_x.clone(), &py::mul(&Py::Int(2i64), &py::math1("sqrt", &py::mul(&v_D.clone(), &v_t.clone())?)?)?)?;
    v_Cx = f__erf_solution(v_C0.clone(), v_Cs.clone(), v_x.clone(), v_D.clone(), v_t.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("Cx"), v_Cx.clone()), (Py::str("z"), v_z.clone()), (Py::str("ratio"), py::truediv(&py::sub(&v_Cx.clone(), &v_C0.clone())?, &py::sub(&v_Cs.clone(), &v_C0.clone())?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_four_to_three(mut v_u: Py, mut v_v: Py, mut v_t: Py, mut v_w: Py) -> R {
    let mut v_U: Py = Py::None;
    let mut v_V: Py = Py::None;
    let mut v_W: Py = Py::None;
    let t2705 = (f__ints(Py::str("the direction indices"), Py::tuple(vec![v_u.clone(), v_v.clone(), v_t.clone(), v_w.clone()]))?).items()?;
    if t2705.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_u = t2705[0].clone();
    v_v = t2705[1].clone();
    v_t = t2705[2].clone();
    v_w = t2705[3].clone();
    if ({ let t2706 = v_t.clone(); Py::Bool({ let t2707 = py::neg(&py::add(&v_u.clone(), &v_v.clone())?)?; if py::ne(&t2706, &t2707) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("in [u v t w] the index t must equal -(u + v)"))));
    }
    let t2710 = (f__reduce_ints(Py::list(vec![py::sub(&v_u.clone(), &v_t.clone())?, py::sub(&v_v.clone(), &v_t.clone())?, v_w.clone()]))?).items()?;
    if t2710.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_U = t2710[0].clone();
    v_V = t2710[1].clone();
    v_W = t2710[2].clone();
    return Ok(Py::dict(vec![(Py::str("U"), v_U.clone()), (Py::str("V"), v_V.clone()), (Py::str("W"), v_W.clone()), (Py::str("indices"), Py::str(&["[".to_string(), py::format(&v_U.clone(), "")?, " ".to_string(), py::format(&v_V.clone(), "")?, " ".to_string(), py::format(&v_W.clone(), "")?, "]".to_string()].concat()))]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_fracture_stress(mut v_K_IC: Py, mut v_a: Py, mut v_Y: Py) -> R {
    let mut v_s: Py = Py::None;
    let _ = f__pos(Py::str("K_IC, the crack length and Y"), Py::tuple(vec![v_K_IC.clone(), v_a.clone(), v_Y.clone()]))?;
    v_s = py::truediv(&v_K_IC.clone(), &py::mul(&v_Y.clone(), &py::math1("sqrt", &py::mul(&Py::Float(std::f64::consts::PI), &v_a.clone())?)?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("sigma_c"), v_s.clone()), (Py::str("sigma_c_MPa"), py::truediv(&v_s.clone(), &Py::Float(1000000.0))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_gibbs_phase_rule(mut v_C: Py, mut v_P: Py, mut v_pressure: Py) -> R {
    let mut v_F: Py = Py::None;
    let t1931 = (Py::tuple(vec![py::int(&v_C.clone())?, py::int(&v_P.clone())?])).items()?;
    if t1931.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_C = t1931[0].clone();
    v_P = t1931[1].clone();
    if ({ let t1936 = { let t1932 = v_C.clone(); Py::Bool({ let t1933 = Py::Int(1i64); if py::lt(&t1932, &t1933)? { true } else { false } }) }; if t1936.truthy() { t1936 } else { { let t1934 = v_P.clone(); Py::Bool({ let t1935 = Py::Int(1i64); if py::lt(&t1934, &t1935)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the numbers of components and phases must be at least 1"))));
    }
    v_F = py::add(&py::sub(&v_C.clone(), &v_P.clone())?, &(if ({ let t1937 = v_pressure.clone(); Py::Bool({ let t1938 = Py::str("variable"); if py::eq(&t1937, &t1938) { true } else { false } }) }).truthy() { Py::Int(2i64) } else { Py::Int(1i64) }))?;
    if ({ let t1939 = v_F.clone(); Py::Bool({ let t1940 = Py::Int(0i64); if py::lt(&t1939, &t1940)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("more phases than the phase rule allows in equilibrium"))));
    }
    return Ok(Py::dict(vec![(Py::str("F"), v_F.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_griffith_stress(mut v_E: Py, mut v_gamma_s: Py, mut v_a: Py) -> R {
    let mut v_s: Py = Py::None;
    let _ = f__pos(Py::str("E, the surface energy and the crack length"), Py::tuple(vec![v_E.clone(), v_gamma_s.clone(), v_a.clone()]))?;
    v_s = py::math1("sqrt", &py::truediv(&py::mul(&py::mul(&Py::Int(2i64), &v_E.clone())?, &v_gamma_s.clone())?, &py::mul(&Py::Float(std::f64::consts::PI), &v_a.clone())?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("sigma_c"), v_s.clone()), (Py::str("sigma_c_MPa"), py::truediv(&v_s.clone(), &Py::Float(1000000.0))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_hall_petch_fit(mut v_d_um: Py, mut v_yield_MPa: Py, mut v_d_target_um: Py) -> R {
    let mut v_b0: Py = Py::None;
    let mut v_b1: Py = Py::None;
    let mut v_out: Py = Py::None;
    let mut v_r2: Py = Py::None;
    let mut v_v: Py = Py::None;
    if ({ let t2002 = py::len(&v_d_um.clone())?; Py::Bool({ let t2003 = py::len(&v_yield_MPa.clone())?; if py::ne(&t2002, &t2003) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the two lists must have the same length"))));
    }
    for t2004 in py::iter(&(v_d_um.clone()))? {
        v_v = t2004;
        let _ = f__pos(Py::str("every grain diameter"), Py::tuple(vec![v_v.clone()]))?;
    }
    let t2025 = (f__linfit({ let mut out = vec![]; for t2024 in py::iter(&(v_d_um.clone()))? { let mut v_v: Py; 
v_v = t2024;
out.push(py::pow(&v_v.clone(), &py::neg(&Py::Float(0.5))?)?); } Py::list(out) }, Py::list((v_yield_MPa.clone()).items()?))?).items()?;
    if t2025.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_b0 = t2025[0].clone();
    v_b1 = t2025[1].clone();
    v_r2 = t2025[2].clone();
    v_out = py::dict_from(&Py::dict(vec![]), vec![(Py::str("sigma0_MPa"), v_b0.clone()), (Py::str("k_MPa_sqrt_um"), v_b1.clone()), (Py::str("k_MPa_sqrt_m"), py::mul(&v_b1.clone(), &Py::Float(0.001))?), (Py::str("r_squared"), v_r2.clone())])?;
    if ({ let t2026 = v_d_target_um.clone(); Py::Bool({ let t2027 = Py::None; if !py::is(&t2026, &t2027) { true } else { false } }) }).truthy() {
        let _ = f__pos(Py::str("the target grain diameter"), Py::tuple(vec![v_d_target_um.clone()]))?;
        py::setitem(&v_out.clone(), &Py::str("yield_target_MPa"), py::add(&v_b0.clone(), &py::mul(&v_b1.clone(), &py::pow(&v_d_target_um.clone(), &py::neg(&Py::Float(0.5))?)?)?)?)?;
    }
    return Ok(f__out(v_out.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_hcp_cell(mut v_a: Py, mut v_c: Py, mut v_A: Py) -> R {
    let mut v_V: Py = Py::None;
    let mut v_rho: Py = Py::None;
    let _ = f__pos(Py::str("a"), Py::tuple(vec![v_a.clone()]))?;
    let _ = f__pos(Py::str("c"), Py::tuple(vec![v_c.clone()]))?;
    let _ = f__pos(Py::str("the molar mass"), Py::tuple(vec![v_A.clone()]))?;
    v_V = py::mul(&py::mul(&py::mul(&py::mul(&Py::Float(1.5), &c_SQ3()?)?, &v_a.clone())?, &v_a.clone())?, &v_c.clone())?;
    v_rho = py::truediv(&py::mul(&Py::Int(6i64), &v_A.clone())?, &py::mul(&v_V.clone(), &c_NA()?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("V"), v_V.clone()), (Py::str("V_nm3"), py::mul(&v_V.clone(), &Py::Float(1e+27))?), (Py::str("rho"), v_rho.clone()), (Py::str("rho_gcc"), py::truediv(&v_rho.clone(), &Py::Int(1000i64))?), (Py::str("c_over_a"), py::truediv(&v_c.clone(), &v_a.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_hollomon_fit(mut v_true_strain: Py, mut v_true_stress: Py, mut v_strain_target: Py) -> R {
    let mut v_b0: Py = Py::None;
    let mut v_b1: Py = Py::None;
    let mut v_out: Py = Py::None;
    let mut v_r2: Py = Py::None;
    let mut v_v: Py = Py::None;
    if ({ let t3366 = py::len(&v_true_strain.clone())?; Py::Bool({ let t3367 = py::len(&v_true_stress.clone())?; if py::ne(&t3366, &t3367) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the two lists must have the same length"))));
    }
    for t3368 in py::iter(&(py::add(&Py::list((v_true_strain.clone()).items()?), &Py::list((v_true_stress.clone()).items()?))?))? {
        v_v = t3368;
        let _ = f__pos(Py::str("every true strain and stress (plastic region)"), Py::tuple(vec![v_v.clone()]))?;
    }
    let t3390 = (f__linfit({ let mut out = vec![]; for t3388 in py::iter(&(v_true_strain.clone()))? { let mut v_e: Py; 
v_e = t3388;
out.push(py::math1("log", &v_e.clone())?); } Py::list(out) }, { let mut out = vec![]; for t3389 in py::iter(&(v_true_stress.clone()))? { let mut v_s: Py; 
v_s = t3389;
out.push(py::math1("log", &v_s.clone())?); } Py::list(out) })?).items()?;
    if t3390.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_b0 = t3390[0].clone();
    v_b1 = t3390[1].clone();
    v_r2 = t3390[2].clone();
    v_out = py::dict_from(&Py::dict(vec![]), vec![(Py::str("n"), v_b1.clone()), (Py::str("K_MPa"), py::math1("exp", &v_b0.clone())?), (Py::str("r_squared"), v_r2.clone())])?;
    if ({ let t3391 = v_strain_target.clone(); Py::Bool({ let t3392 = Py::None; if !py::is(&t3391, &t3392) { true } else { false } }) }).truthy() {
        let _ = f__pos(Py::str("the target strain"), Py::tuple(vec![v_strain_target.clone()]))?;
        py::setitem(&v_out.clone(), &Py::str("stress_target_MPa"), py::mul(&py::math1("exp", &v_b0.clone())?, &py::pow(&v_strain_target.clone(), &v_b1.clone())?)?)?;
    }
    return Ok(f__out(v_out.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_interstitial_site(mut v_structure: Py, mut v_site: Py, mut v_R: Py) -> R {
    let mut v_q: Py = Py::None;
    let _ = f__pos(Py::str("the atomic radius"), Py::tuple(vec![v_R.clone()]))?;
    v_q = py::getitem(&c__SITE_RATIO()?, &Py::tuple(vec![v_structure.clone(), v_site.clone()]))?;
    return Ok(f__out(Py::dict(vec![(Py::str("r"), py::mul(&v_q.clone(), &v_R.clone())?), (Py::str("r_nm"), py::mul(&py::mul(&v_q.clone(), &v_R.clone())?, &Py::Float(1000000000.0))?), (Py::str("ratio"), v_q.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_intrinsic_carriers(mut v_Eg: Py, mut v_T: Py, mut v_me: Py, mut v_mh: Py) -> R {
    let mut v_Nc: Py = Py::None;
    let mut v_Nv: Py = Py::None;
    let mut v_lnni: Py = Py::None;
    let _ = f__pos(Py::str("the band gap, temperature and effective masses"), Py::tuple(vec![v_Eg.clone(), v_T.clone(), v_me.clone(), v_mh.clone()]))?;
    let t2180 = (Py::tuple(vec![f__nc(v_me.clone(), v_T.clone())?, f__nc(v_mh.clone(), v_T.clone())?])).items()?;
    if t2180.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_Nc = t2180[0].clone();
    v_Nv = t2180[1].clone();
    v_lnni = py::sub(&py::mul(&Py::Float(0.5), &py::math1("log", &py::mul(&v_Nc.clone(), &v_Nv.clone())?)?)?, &py::truediv(&v_Eg.clone(), &py::mul(&py::mul(&Py::Int(2i64), &c_KB()?)?, &v_T.clone())?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("Nc"), v_Nc.clone()), (Py::str("Nv"), v_Nv.clone()), (Py::str("ni"), py::math1("exp", &v_lnni.clone())?), (Py::str("log10_ni"), py::truediv(&v_lnni.clone(), &py::math1("log", &Py::Int(10i64))?)?), (Py::str("log10_Nc"), py::math1("log10", &v_Nc.clone())?), (Py::str("Ei_offset_meV"), py::mul(&py::truediv(&py::mul(&py::mul(&py::mul(&Py::Float(0.75), &c_KB()?)?, &v_T.clone())?, &py::math1("log", &py::truediv(&v_mh.clone(), &v_me.clone())?)?)?, &c_QE()?)?, &Py::Int(1000i64))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_iron_carbon(mut v_C0: Py) -> R {
    let mut v_Wf: Py = Py::None;
    let mut v_Wp: Py = Py::None;
    let mut v_pro: Py = Py::None;
    if (py::not(&{ let t2213 = c__FE_A()?; Py::Bool({ let t2214 = v_C0.clone(); if py::lt(&t2213, &t2214)? { { let t2215 = Py::Float(2.14); if py::lt(&t2214, &t2215)? { true } else { false } } } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("a steel: the carbon content must lie between 0.022 and 2.14 wt%"))));
    }
    if ({ let t2216 = v_C0.clone(); Py::Bool({ let t2217 = c__FE_E()?; if py::le(&t2216, &t2217)? { true } else { false } }) }).truthy() {
        let t2218 = (Py::tuple(vec![Py::str("ferrite"), py::truediv(&py::sub(&c__FE_E()?, &v_C0.clone())?, &py::sub(&c__FE_E()?, &c__FE_A()?)?)?])).items()?;
        if t2218.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_pro = t2218[0].clone();
        v_Wp = t2218[1].clone();
    } else {
        let t2219 = (Py::tuple(vec![Py::str("cementite"), py::truediv(&py::sub(&v_C0.clone(), &c__FE_E()?)?, &py::sub(&c__FE_C()?, &c__FE_E()?)?)?])).items()?;
        if t2219.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_pro = t2219[0].clone();
        v_Wp = t2219[1].clone();
    }
    v_Wf = py::truediv(&py::sub(&c__FE_C()?, &v_C0.clone())?, &py::sub(&c__FE_C()?, &c__FE_A()?)?)?;
    return Ok({ let t2220 = Py::dict(vec![]); py::kw_merge(&t2220, &f__out(Py::dict(vec![(Py::str("W_proeutectoid"), v_Wp.clone()), (Py::str("W_pearlite"), py::sub(&Py::Int(1i64), &v_Wp.clone())?), (Py::str("W_ferrite_total"), v_Wf.clone()), (Py::str("W_cementite_total"), py::sub(&Py::Int(1i64), &v_Wf.clone())?)]))?)?; py::setitem(&t2220, &Py::str("proeutectoid_phase"), v_pro.clone())?; t2220 });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_kelvin_voigt_creep(mut v_sigma0: Py, mut v_E: Py, mut v_eta: Py, mut v_t: Py) -> R {
    let mut v_frac: Py = Py::None;
    let mut v_tau: Py = Py::None;
    let _ = f__pos(Py::str("the stress, modulus and viscosity"), Py::tuple(vec![v_sigma0.clone(), v_E.clone(), v_eta.clone()]))?;
    if ({ let t2298 = v_t.clone(); Py::Bool({ let t2299 = Py::Int(0i64); if py::lt(&t2298, &t2299)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the time must not be negative"))));
    }
    v_tau = py::truediv(&v_eta.clone(), &v_E.clone())?;
    v_frac = py::neg(&py::math1("expm1", &py::truediv(&py::neg(&v_t.clone())?, &v_tau.clone())?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("strain"), py::mul(&py::truediv(&v_sigma0.clone(), &v_E.clone())?, &v_frac.clone())?), (Py::str("fraction_of_elastic"), v_frac.clone()), (Py::str("tau"), v_tau.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_larson_miller(mut v_T: Py, mut v_t_r: Py, mut v_C: Py, mut v_T2: Py) -> R {
    let mut v_h: Py = Py::None;
    let mut v_l2: Py = Py::None;
    let mut v_lmp: Py = Py::None;
    let mut v_out: Py = Py::None;
    let _ = f__pos(Py::str("the temperature and the rupture time"), Py::tuple(vec![v_T.clone(), v_t_r.clone()]))?;
    if (py::not(&{ let t2340 = py::math1("isfinite", &v_C.clone())?; if !t2340.truthy() { t2340 } else { { let t2337 = Py::Int(5i64); Py::Bool({ let t2338 = v_C.clone(); if py::le(&t2337, &t2338)? { { let t2339 = Py::Int(50i64); if py::le(&t2338, &t2339)? { true } else { false } } } else { false } }) } } })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the Larson-Miller constant C must lie between 5 and 50 (typically 20)"))));
    }
    v_h = py::truediv(&v_t_r.clone(), &Py::Float(3600.0))?;
    v_lmp = py::mul(&v_T.clone(), &py::add(&v_C.clone(), &py::math1("log10", &v_h.clone())?)?)?;
    v_out = py::dict_from(&Py::dict(vec![]), vec![(Py::str("LMP"), v_lmp.clone()), (Py::str("LMP_thousands"), py::truediv(&v_lmp.clone(), &Py::Int(1000i64))?), (Py::str("log10_t_hours"), py::math1("log10", &v_h.clone())?)])?;
    if ({ let t2341 = v_T2.clone(); Py::Bool({ let t2342 = Py::None; if !py::is(&t2341, &t2342) { true } else { false } }) }).truthy() {
        let _ = f__pos(Py::str("the second temperature"), Py::tuple(vec![v_T2.clone()]))?;
        v_l2 = py::sub(&py::truediv(&v_lmp.clone(), &v_T2.clone())?, &v_C.clone())?;
        if ({ let t2343 = v_l2.clone(); Py::Bool({ let t2344 = Py::Int(12i64); if py::gt(&t2343, &t2344)? { true } else { false } }) }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("the rupture time at T2 exceeds 1e12 hours"))));
        }
        let _ = { let t2345 = v_out.clone(); py::setitem(&t2345, &Py::str("t_r2_hours"), py::pow(&Py::Int(10i64), &v_l2.clone())?)?; py::setitem(&t2345, &Py::str("log10_t2_hours"), v_l2.clone())?; Py::None };
    }
    return Ok(f__out(v_out.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_lattice_from_density(mut v_structure: Py, mut v_rho: Py, mut v_A: Py) -> R {
    let mut v_R: Py = Py::None;
    let mut v_a: Py = Py::None;
    let _ = f__pos(Py::str("the density"), Py::tuple(vec![v_rho.clone()]))?;
    let _ = f__pos(Py::str("the molar mass"), Py::tuple(vec![v_A.clone()]))?;
    if ({ let t2384 = v_structure.clone(); Py::Bool({ let t2385 = Py::str("HCP"); if py::eq(&t2384, &t2385) { true } else { false } }) }).truthy() {
        v_a = py::pow(&py::truediv(&py::mul(&py::getitem(&c__N_ATOMS()?, &Py::str("HCP"))?, &v_A.clone())?, &py::mul(&py::mul(&py::mul(&py::mul(&v_rho.clone(), &c_NA()?)?, &Py::Float(1.5))?, &c_SQ3()?)?, &c__COA()?)?)?, &py::truediv(&Py::Int(1i64), &Py::Int(3i64))?)?;
    } else {
        v_a = py::pow(&py::truediv(&py::mul(&py::getitem(&c__N_ATOMS()?, &v_structure.clone())?, &v_A.clone())?, &py::mul(&v_rho.clone(), &c_NA()?)?)?, &py::truediv(&Py::Int(1i64), &Py::Int(3i64))?)?;
    }
    v_R = py::truediv(&v_a.clone(), &py::getitem(&c__A_PER_R()?, &v_structure.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("a"), v_a.clone()), (Py::str("a_nm"), py::mul(&v_a.clone(), &Py::Float(1000000000.0))?), (Py::str("R_nm"), py::mul(&v_R.clone(), &Py::Float(1000000000.0))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_lever_rule(mut v_C0: Py, mut v_C1: Py, mut v_C2: Py, mut v_mass: Py) -> R {
    let mut v_W1: Py = Py::None;
    let mut v_W2: Py = Py::None;
    let mut v_out: Py = Py::None;
    if ({ let t2418 = v_C1.clone(); Py::Bool({ let t2419 = v_C2.clone(); if py::eq(&t2418, &t2419) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the two phase compositions must differ"))));
    }
    let _ = f__between(Py::str("the overall composition C0"), v_C0.clone(), v_C1.clone(), v_C2.clone(), Py::Bool(false))?;
    v_W1 = py::truediv(&py::sub(&v_C2.clone(), &v_C0.clone())?, &py::sub(&v_C2.clone(), &v_C1.clone())?)?;
    v_W2 = py::truediv(&py::sub(&v_C0.clone(), &v_C1.clone())?, &py::sub(&v_C2.clone(), &v_C1.clone())?)?;
    v_out = py::dict_from(&Py::dict(vec![]), vec![(Py::str("W1"), v_W1.clone()), (Py::str("W2"), v_W2.clone())])?;
    if ({ let t2430 = v_mass.clone(); Py::Bool({ let t2431 = Py::None; if !py::is(&t2430, &t2431) { true } else { false } }) }).truthy() {
        let _ = f__pos(Py::str("the mass"), Py::tuple(vec![v_mass.clone()]))?;
        let _ = { let t2437 = v_out.clone(); py::setitem(&t2437, &Py::str("m1"), py::mul(&v_W1.clone(), &v_mass.clone())?)?; py::setitem(&t2437, &Py::str("m2"), py::mul(&v_W2.clone(), &v_mass.clone())?)?; Py::None };
    }
    return Ok(f__out(v_out.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_linear_density(mut v_structure: Py, mut v_u: Py, mut v_v: Py, mut v_w: Py, mut v_a: Py) -> R {
    let mut v_L: Py = Py::None;
    let mut v_g: Py = Py::None;
    let mut v_half: Py = Py::None;
    let mut v_ld: Py = Py::None;
    let mut v_odd: Py = Py::None;
    let mut v_rep: Py = Py::None;
    let _ = f__pos(Py::str("the lattice parameter"), Py::tuple(vec![v_a.clone()]))?;
    let t2485 = (f__ints(Py::str("the direction indices"), Py::tuple(vec![v_u.clone(), v_v.clone(), v_w.clone()]))?).items()?;
    if t2485.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_u = t2485[0].clone();
    v_v = t2485[1].clone();
    v_w = t2485[2].clone();
    v_g = f__gcd3(Py::tuple(vec![v_u.clone(), v_v.clone(), v_w.clone()]))?;
    let t2487 = (Py::tuple(vec![py::floordiv(&v_u.clone(), &v_g.clone())?, py::floordiv(&v_v.clone(), &v_g.clone())?, py::floordiv(&v_w.clone(), &v_g.clone())?])).items()?;
    if t2487.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_u = t2487[0].clone();
    v_v = t2487[1].clone();
    v_w = t2487[2].clone();
    v_L = py::mul(&v_a.clone(), &py::math1("sqrt", &py::add(&py::add(&py::mul(&v_u.clone(), &v_u.clone())?, &py::mul(&v_v.clone(), &v_v.clone())?)?, &py::mul(&v_w.clone(), &v_w.clone())?)?)?)?;
    v_odd = py::sum(&{ let mut out = vec![]; for t2488 in py::iter(&(Py::tuple(vec![v_u.clone(), v_v.clone(), v_w.clone()])))? { let mut v_x: Py; 
v_x = t2488;
out.push(py::modulo(&v_x.clone(), &Py::Int(2i64))?); } Py::list(out) }, None)?;
    v_half = { let t2499 = { let t2493 = { let t2489 = v_structure.clone(); Py::Bool({ let t2490 = Py::str("BCC"); if py::eq(&t2489, &t2490) { true } else { false } }) }; if !t2493.truthy() { t2493 } else { { let t2491 = v_odd.clone(); Py::Bool({ let t2492 = Py::Int(3i64); if py::eq(&t2491, &t2492) { true } else { false } }) } } }; if t2499.truthy() { t2499 } else { { let t2498 = { let t2494 = v_structure.clone(); Py::Bool({ let t2495 = Py::str("FCC"); if py::eq(&t2494, &t2495) { true } else { false } }) }; if !t2498.truthy() { t2498 } else { { let t2496 = v_odd.clone(); Py::Bool({ let t2497 = Py::Int(2i64); if py::eq(&t2496, &t2497) { true } else { false } }) } } } } };
    v_rep = (if (v_half.clone()).truthy() { py::truediv(&v_L.clone(), &Py::Int(2i64))? } else { v_L.clone() });
    v_ld = py::truediv(&Py::Int(1i64), &v_rep.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("LD_per_nm"), py::mul(&v_ld.clone(), &Py::Float(1e-09))?), (Py::str("LD"), v_ld.clone()), (Py::str("repeat_nm"), py::mul(&v_rep.clone(), &Py::Float(1000000000.0))?), (Py::str("LPF"), py::mul(&py::mul(&py::getitem(&c__NN_PER_A()?, &v_structure.clone())?, &v_a.clone())?, &v_ld.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_linear_expansion(mut v_alpha: Py, mut v_L0: Py, mut v_dT: Py) -> R {
    let mut v_dL: Py = Py::None;
    let _ = f__pos(Py::str("alpha and L0"), Py::tuple(vec![v_alpha.clone(), v_L0.clone()]))?;
    if (py::not(&py::math1("isfinite", &v_dT.clone())?)).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the temperature change must be a number"))));
    }
    v_dL = py::mul(&py::mul(&v_alpha.clone(), &v_L0.clone())?, &v_dT.clone())?;
    if ({ let t2537 = py::add(&v_L0.clone(), &v_dL.clone())?; Py::Bool({ let t2538 = Py::Int(0i64); if py::le(&t2537, &t2538)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the length change exceeds the length (outside the linear range)"))));
    }
    return Ok(f__out(Py::dict(vec![(Py::str("dL"), v_dL.clone()), (Py::str("dL_mm"), py::mul(&v_dL.clone(), &Py::Float(1000.0))?), (Py::str("L"), py::add(&v_L0.clone(), &v_dL.clone())?), (Py::str("strain"), py::mul(&v_alpha.clone(), &v_dT.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_max_schmid(mut v_structure: Py, mut v_load: Py, mut v_tau_crss: Py) -> R {
    let mut v_count: Py = Py::None;
    let mut v_ms: Py = Py::None;
    let mut v_mx: Py = Py::None;
    let mut v_out: Py = Py::None;
    if ({ let t2576 = { let t2571 = py::len(&v_load.clone())?; Py::Bool({ let t2572 = Py::Int(3i64); if py::ne(&t2571, &t2572) { true } else { false } }) }; if t2576.truthy() { t2576 } else { { let t2573 = f__norm3(v_load.clone())?; Py::Bool({ let t2574 = Py::Int(0i64); if py::eq(&t2573, &t2574) { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the loading direction must be a nonzero 3-vector"))));
    }
    v_ms = { let mut out = vec![]; for t2587 in py::iter(&(f__slip_systems(v_structure.clone())?))? { let mut v_d: Py; let mut v_p: Py; 
let t2588 = (t2587).items()?;
if t2588.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_p = t2588[0].clone();
v_d = t2588[1].clone();
out.push(py::truediv(&py::abs(&py::mul(&f__dot3(v_load.clone(), v_p.clone())?, &f__dot3(v_load.clone(), v_d.clone())?)?)?, &py::mul(&py::mul(&py::pow(&f__norm3(v_load.clone())?, &Py::Int(2i64))?, &f__norm3(v_p.clone())?)?, &f__norm3(v_d.clone())?)?)?); } Py::list(out) };
    v_mx = py::minmax(&[v_ms.clone()], true)?;
    v_count = py::sum(&{ let mut out = vec![]; for t2589 in py::iter(&(v_ms.clone()))? { let mut v_v: Py; 
v_v = t2589;
if ({ let t2590 = v_v.clone(); Py::Bool({ let t2591 = py::mul(&v_mx.clone(), &py::sub(&Py::Int(1i64), &Py::Float(1e-09))?)?; if py::ge(&t2590, &t2591)? { true } else { false } }) }).truthy() { out.push(Py::Int(1i64)); }} Py::list(out) }, None)?;
    v_out = py::dict_from(&Py::dict(vec![]), vec![(Py::str("max_schmid_factor"), v_mx.clone()), (Py::str("active_systems"), v_count.clone()), (Py::str("systems"), py::len(&v_ms.clone())?)])?;
    if ({ let t2592 = v_tau_crss.clone(); Py::Bool({ let t2593 = Py::None; if !py::is(&t2592, &t2593) { true } else { false } }) }).truthy() {
        let _ = f__pos(Py::str("the critical resolved shear stress"), Py::tuple(vec![v_tau_crss.clone()]))?;
        py::setitem(&v_out.clone(), &Py::str("yield_MPa"), py::truediv(&py::truediv(&v_tau_crss.clone(), &v_mx.clone())?, &Py::Float(1000000.0))?)?;
    }
    return Ok(f__out(v_out.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_maxwell_relaxation(mut v_sigma0: Py, mut v_E: Py, mut v_eta: Py, mut v_t: Py) -> R {
    let mut v_r: Py = Py::None;
    let mut v_tau: Py = Py::None;
    let _ = f__pos(Py::str("the stress, modulus and viscosity"), Py::tuple(vec![v_sigma0.clone(), v_E.clone(), v_eta.clone()]))?;
    if ({ let t2637 = v_t.clone(); Py::Bool({ let t2638 = Py::Int(0i64); if py::lt(&t2637, &t2638)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the time must not be negative"))));
    }
    v_tau = py::truediv(&v_eta.clone(), &v_E.clone())?;
    v_r = py::math1("exp", &py::truediv(&py::neg(&v_t.clone())?, &v_tau.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("sigma_t"), py::mul(&v_sigma0.clone(), &v_r.clone())?), (Py::str("sigma_t_MPa"), py::truediv(&py::mul(&v_sigma0.clone(), &v_r.clone())?, &Py::Float(1000000.0))?), (Py::str("tau"), v_tau.clone()), (Py::str("ratio"), v_r.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_miller_from_intercepts(mut v_x: Py, mut v_y: Py, mut v_z: Py) -> R {
    let mut v_den: Py = Py::None;
    let mut v_g: Py = Py::None;
    let mut v_h: Py = Py::None;
    let mut v_hkl: Py = Py::None;
    let mut v_ints: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_l: Py = Py::None;
    let mut v_r_: Py = Py::None;
    let mut v_rec: Py = Py::None;
    v_ints = Py::list(vec![f__intercept(v_x.clone(), Py::str("x"))?, f__intercept(v_y.clone(), Py::str("y"))?, f__intercept(v_z.clone(), Py::str("z"))?]);
    if (Py::Bool(({ let mut out = vec![]; for t2726 in py::iter(&(v_ints.clone()))? { let mut v_v: Py; 
v_v = t2726;
out.push({ let t2727 = v_v.clone(); Py::Bool({ let t2728 = Py::None; if py::is(&t2727, &t2728) { true } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy()))).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("at least one intercept must be finite"))));
    }
    v_rec = { let mut out = vec![]; for t2729 in py::iter(&(v_ints.clone()))? { let mut v_v: Py; 
v_v = t2729;
out.push((if ({ let t2730 = v_v.clone(); Py::Bool({ let t2731 = Py::None; if py::is(&t2730, &t2731) { true } else { false } }) }).truthy() { py::fraction(&[Py::Int(0i64)])? } else { py::truediv(&Py::Int(1i64), &v_v.clone())? })); } Py::list(out) };
    v_den = Py::Int(1i64);
    for t2732 in py::iter(&(v_rec.clone()))? {
        v_r_ = t2732;
        v_den = py::floordiv(&py::mul(&v_den.clone(), &py::denominator(&v_r_.clone())?)?, &py::gcd(&v_den.clone(), &py::denominator(&v_r_.clone())?)?)?;
    }
    v_hkl = { let mut out = vec![]; for t2733 in py::iter(&(v_rec.clone()))? { let mut v_r_: Py; 
v_r_ = t2733;
out.push(py::int(&py::mul(&v_r_.clone(), &v_den.clone())?)?); } Py::list(out) };
    v_g = f__gcd3({ let mut v = vec![]; v.extend((v_hkl.clone()).items()?); Py::tuple(v) })?;
    let t2736 = ({ let mut out = vec![]; for t2735 in py::iter(&(v_hkl.clone()))? { let mut v_x_: Py; 
v_x_ = t2735;
out.push(py::floordiv(&v_x_.clone(), &v_g.clone())?); } Py::list(out) }).items()?;
    if t2736.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_h = t2736[0].clone();
    v_k = t2736[1].clone();
    v_l = t2736[2].clone();
    return Ok(Py::dict(vec![(Py::str("h"), v_h.clone()), (Py::str("k"), v_k.clone()), (Py::str("l"), v_l.clone()), (Py::str("indices"), Py::str(&["(".to_string(), py::format(&v_h.clone(), "")?, " ".to_string(), py::format(&v_k.clone(), "")?, " ".to_string(), py::format(&v_l.clone(), "")?, ")".to_string()].concat()))]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_mw_averages(mut v_M: Py, mut v_fraction: Py, mut v_basis: Py) -> R {
    let mut v_Mn: Py = Py::None;
    let mut v_Mw: Py = Py::None;
    let mut v_f: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_v: Py = Py::None;
    if ({ let t2798 = py::len(&v_M.clone())?; Py::Bool({ let t2799 = py::len(&v_fraction.clone())?; if py::ne(&t2798, &t2799) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("M and fraction must have the same length"))));
    }
    for t2800 in py::iter(&(v_M.clone()))? {
        v_v = t2800;
        let _ = f__pos(Py::str("every molecular weight"), Py::tuple(vec![v_v.clone()]))?;
    }
    if ({ let t2811 = Py::Bool(({ let mut out = vec![]; for t2806 in py::iter(&(v_fraction.clone()))? { let mut v_f: Py; 
v_f = t2806;
out.push({ let t2807 = v_f.clone(); Py::Bool({ let t2808 = Py::Int(0i64); if py::lt(&t2807, &t2808)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy())); if t2811.truthy() { t2811 } else { { let t2809 = py::sum(&v_fraction.clone(), None)?; Py::Bool({ let t2810 = Py::Int(0i64); if py::le(&t2809, &t2810)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("fractions must be non-negative and not all zero"))));
    }
    v_s = py::sum(&v_fraction.clone(), None)?;
    v_f = { let mut out = vec![]; for t2812 in py::iter(&(v_fraction.clone()))? { let mut v_v: Py; 
v_v = t2812;
out.push(py::truediv(&v_v.clone(), &v_s.clone())?); } Py::list(out) };
    if ({ let t2813 = v_basis.clone(); Py::Bool({ let t2814 = Py::str("number"); if py::eq(&t2813, &t2814) { true } else { false } }) }).truthy() {
        v_Mn = py::sum(&{ let mut out = vec![]; for t2815 in py::iter(&(py::zip(&[v_f.clone(), v_M.clone()])?))? { let mut v_m: Py; let mut v_x: Py; 
let t2816 = (t2815).items()?;
if t2816.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_x = t2816[0].clone();
v_m = t2816[1].clone();
out.push(py::mul(&v_x.clone(), &v_m.clone())?); } Py::list(out) }, None)?;
        v_Mw = py::truediv(&py::sum(&{ let mut out = vec![]; for t2817 in py::iter(&(py::zip(&[v_f.clone(), v_M.clone()])?))? { let mut v_m: Py; let mut v_x: Py; 
let t2818 = (t2817).items()?;
if t2818.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_x = t2818[0].clone();
v_m = t2818[1].clone();
out.push(py::mul(&py::mul(&v_x.clone(), &v_m.clone())?, &v_m.clone())?); } Py::list(out) }, None)?, &v_Mn.clone())?;
    } else {
        v_Mw = py::sum(&{ let mut out = vec![]; for t2819 in py::iter(&(py::zip(&[v_f.clone(), v_M.clone()])?))? { let mut v_m: Py; let mut v_w: Py; 
let t2820 = (t2819).items()?;
if t2820.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_w = t2820[0].clone();
v_m = t2820[1].clone();
out.push(py::mul(&v_w.clone(), &v_m.clone())?); } Py::list(out) }, None)?;
        v_Mn = py::truediv(&Py::Int(1i64), &py::sum(&{ let mut out = vec![]; for t2821 in py::iter(&(py::zip(&[v_f.clone(), v_M.clone()])?))? { let mut v_m: Py; let mut v_w: Py; 
let t2822 = (t2821).items()?;
if t2822.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_w = t2822[0].clone();
v_m = t2822[1].clone();
out.push(py::truediv(&v_w.clone(), &v_m.clone())?); } Py::list(out) }, None)?)?;
    }
    return Ok(f__out(Py::dict(vec![(Py::str("Mn"), v_Mn.clone()), (Py::str("Mw"), v_Mw.clone()), (Py::str("PDI"), py::truediv(&v_Mw.clone(), &v_Mn.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_nucleation(mut v_gamma: Py, mut v_dHf: Py, mut v_Tm: Py, mut v_T: Py) -> R {
    let mut v_G: Py = Py::None;
    let mut v_dGv: Py = Py::None;
    let mut v_r: Py = Py::None;
    let _ = f__pos(Py::str("gamma, the latent heat and the melting temperature"), Py::tuple(vec![v_gamma.clone(), v_dHf.clone(), v_Tm.clone()]))?;
    if (py::not(&{ let t2103 = Py::Int(0i64); Py::Bool({ let t2104 = v_T.clone(); if py::lt(&t2103, &t2104)? { { let t2105 = v_Tm.clone(); if py::lt(&t2104, &t2105)? { true } else { false } } } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the temperature must lie below the melting point (undercooled liquid)"))));
    }
    v_dGv = py::truediv(&py::mul(&py::neg(&v_dHf.clone())?, &py::sub(&v_Tm.clone(), &v_T.clone())?)?, &v_Tm.clone())?;
    v_r = py::truediv(&py::mul(&py::neg(&Py::Int(2i64))?, &v_gamma.clone())?, &v_dGv.clone())?;
    v_G = py::truediv(&py::mul(&py::mul(&Py::Int(16i64), &Py::Float(std::f64::consts::PI))?, &py::pow(&v_gamma.clone(), &Py::Int(3i64))?)?, &py::mul(&py::mul(&Py::Int(3i64), &v_dGv.clone())?, &v_dGv.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("undercooling"), py::sub(&v_Tm.clone(), &v_T.clone())?), (Py::str("dGv_MJ_m3"), py::truediv(&v_dGv.clone(), &Py::Float(1000000.0))?), (Py::str("r_star"), v_r.clone()), (Py::str("r_star_nm"), py::mul(&v_r.clone(), &Py::Float(1000000000.0))?), (Py::str("dG_star"), v_G.clone()), (Py::str("dG_star_eV"), py::truediv(&v_G.clone(), &c_QE()?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_offset_yield(mut v_strain: Py, mut v_stress: Py, mut v_E: Py, mut v_offset: Py) -> R {
    let mut v_Em: Py = Py::None;
    let mut v_ey: Py = Py::None;
    let mut v_g: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_sy: Py = Py::None;
    let mut v_t: Py = Py::None;
    if ({ let t2859 = { let t2855 = py::len(&v_strain.clone())?; Py::Bool({ let t2856 = py::len(&v_stress.clone())?; if py::ne(&t2855, &t2856) { true } else { false } }) }; if t2859.truthy() { t2859 } else { { let t2857 = py::len(&v_strain.clone())?; Py::Bool({ let t2858 = Py::Int(3i64); if py::lt(&t2857, &t2858)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("strain and stress must be lists of the same length (at least 3 points)"))));
    }
    if (Py::Bool(({ let mut out = vec![]; for t2860 in py::iter(&(py::zip(&[v_strain.clone(), py::slice(&v_strain.clone(), Some((Py::Int(1i64)).i()?), None, None)?])?))? { let mut v_a: Py; let mut v_b: Py; 
let t2861 = (t2860).items()?;
if t2861.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t2861[0].clone();
v_b = t2861[1].clone();
out.push({ let t2862 = v_b.clone(); Py::Bool({ let t2863 = v_a.clone(); if py::le(&t2862, &t2863)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy()))).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("strains must increase strictly"))));
    }
    let _ = f__pos(Py::str("Young's modulus"), Py::tuple(vec![v_E.clone()]))?;
    if (py::not(&{ let t2869 = Py::Int(0i64); Py::Bool({ let t2870 = v_offset.clone(); if py::lt(&t2869, &t2870)? { { let t2871 = Py::Float(0.05); if py::lt(&t2870, &t2871)? { true } else { false } } } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the offset must lie between 0 and 0.05"))));
    }
    v_Em = py::truediv(&v_E.clone(), &Py::Float(1000000.0))?;
    v_g = { let mut out = vec![]; for t2872 in py::iter(&(py::zip(&[v_strain.clone(), v_stress.clone()])?))? { let mut v_e: Py; let mut v_s: Py; 
let t2873 = (t2872).items()?;
if t2873.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_e = t2873[0].clone();
v_s = t2873[1].clone();
out.push(py::sub(&v_s.clone(), &py::mul(&v_Em.clone(), &py::sub(&v_e.clone(), &v_offset.clone())?)?)?); } Py::list(out) };
    if ({ let t2874 = py::getitem(&v_g.clone(), &Py::Int(0i64))?; Py::Bool({ let t2875 = Py::Int(0i64); if py::le(&t2874, &t2875)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the curve must start above the offset line"))));
    }
    for t2876 in py::range_iter(&[(py::sub(&py::len(&v_g.clone())?, &Py::Int(1i64))?).i()?]) {
        v_i = t2876;
        if ({ let t2877 = py::getitem(&v_g.clone(), &v_i.clone())?; Py::Bool({ let t2878 = Py::Int(0i64); if py::gt(&t2877, &t2878)? { { let t2879 = py::getitem(&v_g.clone(), &py::add(&v_i.clone(), &Py::Int(1i64))?)?; if py::ge(&t2878, &t2879)? { true } else { false } } } else { false } }) }).truthy() {
            v_t = py::truediv(&py::getitem(&v_g.clone(), &v_i.clone())?, &py::sub(&py::getitem(&v_g.clone(), &v_i.clone())?, &py::getitem(&v_g.clone(), &py::add(&v_i.clone(), &Py::Int(1i64))?)?)?)?;
            v_ey = py::add(&py::getitem(&v_strain.clone(), &v_i.clone())?, &py::mul(&v_t.clone(), &py::sub(&py::getitem(&v_strain.clone(), &py::add(&v_i.clone(), &Py::Int(1i64))?)?, &py::getitem(&v_strain.clone(), &v_i.clone())?)?)?)?;
            v_sy = py::add(&py::getitem(&v_stress.clone(), &v_i.clone())?, &py::mul(&v_t.clone(), &py::sub(&py::getitem(&v_stress.clone(), &py::add(&v_i.clone(), &Py::Int(1i64))?)?, &py::getitem(&v_stress.clone(), &v_i.clone())?)?)?)?;
            return Ok(f__out(Py::dict(vec![(Py::str("yield_MPa"), v_sy.clone()), (Py::str("yield_strain"), v_ey.clone())]))?);
        }
    }
    return Err(py::value_error(py::to_str(&Py::str("the curve never crosses the offset line: the data end before yielding"))));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_packing_factor(mut v_structure: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_apf: Py = Py::None;
    v_a = py::getitem(&c__A_PER_R()?, &v_structure.clone())?;
    v_apf = py::truediv(&py::mul(&py::mul(&py::getitem(&c__N_ATOMS()?, &v_structure.clone())?, &py::truediv(&Py::Int(4i64), &Py::Int(3i64))?)?, &Py::Float(std::f64::consts::PI))?, &f__cell_volume(v_structure.clone(), v_a.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("apf"), v_apf.clone()), (Py::str("coordination"), py::getitem(&c__CN()?, &v_structure.clone())?), (Py::str("atoms_per_cell"), py::getitem(&c__N_ATOMS()?, &v_structure.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_paris_life(mut v_C: Py, mut v_m: Py, mut v_dsigma: Py, mut v_a0: Py, mut v_Y: Py, mut v_af: Py, mut v_K_IC: Py, mut v_sigma_max: Py) -> R {
    let mut v_L: Py = Py::None;
    let mut v_N: Py = Py::None;
    let mut v_S: Py = Py::None;
    let mut v_a_end: Py = Py::None;
    let mut v_dK0: Py = Py::None;
    let mut v_out: Py = Py::None;
    let mut v_p: Py = Py::None;
    let _ = f__pos(Py::str("C, m, the stress range, the initial crack and Y"), Py::tuple(vec![v_C.clone(), v_m.clone(), v_dsigma.clone(), v_a0.clone(), v_Y.clone()]))?;
    if (py::not(&{ let t2917 = Py::Float(0.5); Py::Bool({ let t2918 = v_m.clone(); if py::le(&t2917, &t2918)? { { let t2919 = Py::Int(10i64); if py::le(&t2918, &t2919)? { true } else { false } } } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the Paris exponent m must lie between 0.5 and 10"))));
    }
    if ({ let t2920 = { let t2922 = v_af.clone(); Py::Bool({ let t2923 = Py::None; if py::is(&t2922, &t2923) { true } else { false } }) }; Py::Bool({ let t2921 = { let t2924 = v_K_IC.clone(); Py::Bool({ let t2925 = Py::None; if py::is(&t2924, &t2925) { true } else { false } }) }; if py::eq(&t2920, &t2921) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("give exactly one of the final crack length af and the fracture toughness K_IC"))));
    }
    if ({ let t2926 = v_K_IC.clone(); Py::Bool({ let t2927 = Py::None; if !py::is(&t2926, &t2927) { true } else { false } }) }).truthy() {
        let _ = f__pos(Py::str("K_IC"), Py::tuple(vec![v_K_IC.clone()]))?;
        if ({ let t2928 = v_sigma_max.clone(); Py::Bool({ let t2929 = Py::None; if py::is(&t2928, &t2929) { true } else { false } }) }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("the critical crack from K_IC needs the peak stress sigma_max of the cycle"))));
        }
        let _ = f__pos(Py::str("sigma_max"), Py::tuple(vec![v_sigma_max.clone()]))?;
        v_a_end = py::truediv(&py::pow(&py::truediv(&v_K_IC.clone(), &py::mul(&v_Y.clone(), &v_sigma_max.clone())?)?, &Py::Int(2i64))?, &Py::Float(std::f64::consts::PI))?;
    } else {
        if ({ let t2930 = v_sigma_max.clone(); Py::Bool({ let t2931 = Py::None; if !py::is(&t2930, &t2931) { true } else { false } }) }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("sigma_max is used only with K_IC (to find the critical crack)"))));
        }
        let _ = f__pos(Py::str("the final crack length"), Py::tuple(vec![v_af.clone()]))?;
        v_a_end = v_af.clone();
    }
    if (py::not(&{ let t2932 = v_a_end.clone(); Py::Bool({ let t2933 = v_a0.clone(); if py::gt(&t2932, &t2933)? { true } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the final (or critical) crack must be longer than the initial crack: no growth life remains"))));
    }
    v_S = py::mul(&py::mul(&v_Y.clone(), &py::truediv(&v_dsigma.clone(), &Py::Float(1000000.0))?)?, &py::math1("sqrt", &Py::Float(std::f64::consts::PI))?)?;
    v_L = py::math1("log", &py::truediv(&v_a_end.clone(), &v_a0.clone())?)?;
    v_p = py::sub(&Py::Float(1.0), &py::truediv(&v_m.clone(), &Py::Float(2.0))?)?;
    if ({ let t2934 = v_p.clone(); Py::Bool({ let t2935 = Py::Int(0i64); if py::eq(&t2934, &t2935) { true } else { false } }) }).truthy() {
        v_N = py::truediv(&v_L.clone(), &py::mul(&py::mul(&v_C.clone(), &v_S.clone())?, &v_S.clone())?)?;
    } else {
        v_N = py::truediv(&py::mul(&py::pow(&v_a0.clone(), &v_p.clone())?, &py::math1("expm1", &py::mul(&v_p.clone(), &v_L.clone())?)?)?, &py::mul(&py::mul(&v_C.clone(), &py::pow(&v_S.clone(), &v_m.clone())?)?, &v_p.clone())?)?;
    }
    v_dK0 = py::mul(&v_S.clone(), &py::math1("sqrt", &v_a0.clone())?)?;
    v_out = Py::dict(vec![(Py::str("N"), v_N.clone()), (Py::str("log10_N"), py::math1("log10", &v_N.clone())?), (Py::str("a_final"), v_a_end.clone()), (Py::str("a_final_mm"), py::mul(&v_a_end.clone(), &Py::Float(1000.0))?), (Py::str("dK_initial"), v_dK0.clone()), (Py::str("dK_final"), py::mul(&v_S.clone(), &py::math1("sqrt", &v_a_end.clone())?)?), (Py::str("dadN_initial"), py::mul(&v_C.clone(), &py::pow(&v_dK0.clone(), &v_m.clone())?)?)]);
    if ({ let t2936 = v_K_IC.clone(); Py::Bool({ let t2937 = Py::None; if !py::is(&t2936, &t2937) { true } else { false } }) }).truthy() {
        py::setitem(&v_out.clone(), &Py::str("a_c"), v_a_end.clone())?;
        py::setitem(&v_out.clone(), &Py::str("a_c_mm"), py::mul(&v_a_end.clone(), &Py::Float(1000.0))?)?;
    }
    return Ok(f__out(v_out.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_planar_density(mut v_structure: Py, mut v_h: Py, mut v_k: Py, mut v_l: Py, mut v_a: Py) -> R {
    let mut v_N: Py = Py::None;
    let mut v_R: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_g: Py = Py::None;
    let mut v_pd: Py = Py::None;
    let _ = f__pos(Py::str("the lattice parameter"), Py::tuple(vec![v_a.clone()]))?;
    let t2985 = (f__ints(Py::str("the Miller indices"), Py::tuple(vec![v_h.clone(), v_k.clone(), v_l.clone()]))?).items()?;
    if t2985.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_h = t2985[0].clone();
    v_k = t2985[1].clone();
    v_l = t2985[2].clone();
    v_g = f__gcd3(Py::tuple(vec![v_h.clone(), v_k.clone(), v_l.clone()]))?;
    let t2987 = (Py::tuple(vec![py::floordiv(&v_h.clone(), &v_g.clone())?, py::floordiv(&v_k.clone(), &v_g.clone())?, py::floordiv(&v_l.clone(), &v_g.clone())?])).items()?;
    if t2987.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_h = t2987[0].clone();
    v_k = t2987[1].clone();
    v_l = t2987[2].clone();
    v_N = py::add(&py::add(&py::mul(&v_h.clone(), &v_h.clone())?, &py::mul(&v_k.clone(), &v_k.clone())?)?, &py::mul(&v_l.clone(), &v_l.clone())?)?;
    v_d = py::truediv(&py::truediv(&v_a.clone(), &py::math1("sqrt", &v_N.clone())?)?, &(if (f__reciprocal_ok(v_structure.clone(), v_h.clone(), v_k.clone(), v_l.clone())?).truthy() { Py::Int(1i64) } else { Py::Int(2i64) }))?;
    v_pd = py::mul(&py::truediv(&py::getitem(&c__N_ATOMS()?, &v_structure.clone())?, &py::pow(&v_a.clone(), &Py::Int(3i64))?)?, &v_d.clone())?;
    v_R = py::truediv(&py::mul(&py::getitem(&c__NN_PER_A()?, &v_structure.clone())?, &v_a.clone())?, &Py::Int(2i64))?;
    return Ok(f__out(Py::dict(vec![(Py::str("PD_per_nm2"), py::mul(&v_pd.clone(), &Py::Float(1e-18))?), (Py::str("PD"), v_pd.clone()), (Py::str("PPF"), py::mul(&py::mul(&py::mul(&v_pd.clone(), &Py::Float(std::f64::consts::PI))?, &v_R.clone())?, &v_R.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_polymorph_volume_change(mut v_R_bcc: Py, mut v_R_fcc: Py) -> R {
    let mut v_v_b: Py = Py::None;
    let mut v_v_f: Py = Py::None;
    let _ = f__pos(Py::str("the atomic radii"), Py::tuple(vec![v_R_bcc.clone(), v_R_fcc.clone()]))?;
    v_v_b = py::truediv(&py::pow(&py::truediv(&py::mul(&Py::Int(4i64), &v_R_bcc.clone())?, &c_SQ3()?)?, &Py::Int(3i64))?, &Py::Int(2i64))?;
    v_v_f = py::truediv(&py::pow(&py::mul(&py::mul(&Py::Int(2i64), &c_SQ2()?)?, &v_R_fcc.clone())?, &Py::Int(3i64))?, &Py::Int(4i64))?;
    return Ok(f__out(Py::dict(vec![(Py::str("volume_change_percent"), py::mul(&py::truediv(&py::sub(&v_v_f.clone(), &v_v_b.clone())?, &v_v_b.clone())?, &Py::Int(100i64))?), (Py::str("V_atom_bcc_nm3"), py::mul(&v_v_b.clone(), &Py::Float(1e+27))?), (Py::str("V_atom_fcc_nm3"), py::mul(&v_v_f.clone(), &Py::Float(1e+27))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_radius_ratio(mut v_rc: Py, mut v_ra: Py) -> R {
    let mut v_cn: Py = Py::None;
    let mut v_g: Py = Py::None;
    let mut v_geo: Py = Py::None;
    let mut v_lim: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_q: Py = Py::None;
    let _ = f__pos(Py::str("the ionic radii"), Py::tuple(vec![v_rc.clone(), v_ra.clone()]))?;
    v_q = py::truediv(&v_rc.clone(), &v_ra.clone())?;
    if ({ let t3204 = v_q.clone(); Py::Bool({ let t3205 = Py::Int(1i64); if py::ge(&t3204, &t3205)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the cation must be smaller than the anion (rc/ra < 1) for the radius-ratio rules"))));
    }
    let t3206 = (Py::tuple(vec![Py::Int(2i64), Py::str("linear")])).items()?;
    if t3206.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_cn = t3206[0].clone();
    v_geo = t3206[1].clone();
    for t3207 in py::iter(&(c__CRIT()?))? {
        let t3211 = (t3207).items()?;
        if t3211.len() != 3 { return Err(PyErr::Value("unpack".into())); }
        v_lim = t3211[0].clone();
        v_n = t3211[1].clone();
        v_g = t3211[2].clone();
        if ({ let t3208 = v_q.clone(); Py::Bool({ let t3209 = v_lim.clone(); if py::ge(&t3208, &t3209)? { true } else { false } }) }).truthy() {
            let t3210 = (Py::tuple(vec![v_n.clone(), v_g.clone()])).items()?;
            if t3210.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_cn = t3210[0].clone();
            v_geo = t3210[1].clone();
        }
    }
    return Ok(Py::dict(vec![(Py::str("ratio"), f__r(v_q.clone())?), (Py::str("CN"), v_cn.clone()), (Py::str("geometry"), v_geo.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_resilience(mut v_sy: Py, mut v_E: Py) -> R {
    let mut v_U: Py = Py::None;
    let _ = f__pos(Py::str("the yield strength and Young's modulus"), Py::tuple(vec![v_sy.clone(), v_E.clone()]))?;
    v_U = py::truediv(&py::mul(&v_sy.clone(), &v_sy.clone())?, &py::mul(&Py::Int(2i64), &v_E.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("Ur"), v_U.clone()), (Py::str("Ur_MJ_m3"), py::truediv(&v_U.clone(), &Py::Float(1000000.0))?), (Py::str("yield_strain"), py::truediv(&v_sy.clone(), &v_E.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_rubber_elasticity(mut v_rho: Py, mut v_Mc: Py, mut v_T: Py, mut v_stretch: Py) -> R {
    let mut v_G: Py = Py::None;
    let mut v_s: Py = Py::None;
    let _ = f__pos(Py::str("the density, Mc, temperature and stretch"), Py::tuple(vec![v_rho.clone(), v_Mc.clone(), v_T.clone(), v_stretch.clone()]))?;
    v_G = py::truediv(&py::mul(&py::mul(&v_rho.clone(), &c_RGAS()?)?, &v_T.clone())?, &v_Mc.clone())?;
    v_s = py::mul(&v_G.clone(), &py::sub(&v_stretch.clone(), &py::pow(&v_stretch.clone(), &py::neg(&Py::Int(2i64))?)?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("G_MPa"), py::truediv(&v_G.clone(), &Py::Float(1000000.0))?), (Py::str("stress_MPa"), py::truediv(&v_s.clone(), &Py::Float(1000000.0))?), (Py::str("true_stress_MPa"), py::truediv(&py::mul(&v_s.clone(), &v_stretch.clone())?, &Py::Float(1000000.0))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_rule_of_mixtures(mut v_Ef: Py, mut v_Em: Py, mut v_Vf: Py) -> R {
    let mut v_El: Py = Py::None;
    let mut v_Et: Py = Py::None;
    let _ = f__pos(Py::str("the moduli"), Py::tuple(vec![v_Ef.clone(), v_Em.clone()]))?;
    let _ = f__frac01(Py::str("the fibre volume fraction"), v_Vf.clone(), Py::Bool(false))?;
    v_El = py::add(&py::mul(&v_Ef.clone(), &v_Vf.clone())?, &py::mul(&v_Em.clone(), &py::sub(&Py::Int(1i64), &v_Vf.clone())?)?)?;
    v_Et = py::truediv(&py::mul(&v_Ef.clone(), &v_Em.clone())?, &py::add(&py::mul(&v_Vf.clone(), &v_Em.clone())?, &py::mul(&py::sub(&Py::Int(1i64), &v_Vf.clone())?, &v_Ef.clone())?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("E_long_GPa"), py::truediv(&v_El.clone(), &Py::Float(1000000000.0))?), (Py::str("E_trans_GPa"), py::truediv(&v_Et.clone(), &Py::Float(1000000000.0))?), (Py::str("fiber_load_fraction"), py::truediv(&py::mul(&v_Ef.clone(), &v_Vf.clone())?, &v_El.clone())?), (Py::str("E_long"), v_El.clone()), (Py::str("E_trans"), v_Et.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_schmid(mut v_load: Py, mut v_plane: Py, mut v_direction: Py, mut v_sigma: Py, mut v_tau_crss: Py) -> R {
    let mut v_clam: Py = Py::None;
    let mut v_cphi: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_name: Py = Py::None;
    let mut v_out: Py = Py::None;
    let mut v_v: Py = Py::None;
    for t3279 in py::iter(&(Py::tuple(vec![Py::tuple(vec![v_load.clone(), Py::str("the loading direction")]), Py::tuple(vec![v_plane.clone(), Py::str("the slip-plane normal")]), Py::tuple(vec![v_direction.clone(), Py::str("the slip direction")])])))? {
        let t3286 = (t3279).items()?;
        if t3286.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_v = t3286[0].clone();
        v_name = t3286[1].clone();
        if ({ let t3285 = { let t3280 = py::len(&v_v.clone())?; Py::Bool({ let t3281 = Py::Int(3i64); if py::ne(&t3280, &t3281) { true } else { false } }) }; if t3285.truthy() { t3285 } else { { let t3282 = f__norm3(v_v.clone())?; Py::Bool({ let t3283 = Py::Int(0i64); if py::eq(&t3282, &t3283) { true } else { false } }) } } }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_name.clone(), "")?, " must be a nonzero 3-vector".to_string()].concat()))));
        }
    }
    if ({ let t3287 = py::abs(&f__dot3(v_plane.clone(), v_direction.clone())?)?; Py::Bool({ let t3288 = py::mul(&py::mul(&Py::Float(1e-09), &f__norm3(v_plane.clone())?)?, &f__norm3(v_direction.clone())?)?; if py::gt(&t3287, &t3288)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the slip direction must lie in the slip plane (h u + k v + l w = 0)"))));
    }
    v_cphi = py::truediv(&f__dot3(v_load.clone(), v_plane.clone())?, &py::mul(&f__norm3(v_load.clone())?, &f__norm3(v_plane.clone())?)?)?;
    v_clam = py::truediv(&f__dot3(v_load.clone(), v_direction.clone())?, &py::mul(&f__norm3(v_load.clone())?, &f__norm3(v_direction.clone())?)?)?;
    v_m = py::abs(&py::mul(&v_cphi.clone(), &v_clam.clone())?)?;
    v_m = (if ({ let t3291 = v_m.clone(); Py::Bool({ let t3292 = Py::Float(1e-12); if py::lt(&t3291, &t3292)? { true } else { false } }) }).truthy() { Py::Float(0.0) } else { v_m.clone() });
    v_out = py::dict_from(&Py::dict(vec![]), vec![(Py::str("schmid_factor"), v_m.clone()), (Py::str("phi_deg"), py::math1("degrees", &py::math1("acos", &py::minmax(&[py::neg(&Py::Float(1.0))?, py::minmax(&[Py::Float(1.0), py::abs(&v_cphi.clone())?], false)?], true)?)?)?), (Py::str("lambda_deg"), py::math1("degrees", &py::math1("acos", &py::minmax(&[py::neg(&Py::Float(1.0))?, py::minmax(&[Py::Float(1.0), py::abs(&v_clam.clone())?], false)?], true)?)?)?)])?;
    if ({ let t3293 = v_sigma.clone(); Py::Bool({ let t3294 = Py::None; if !py::is(&t3293, &t3294) { true } else { false } }) }).truthy() {
        py::setitem(&v_out.clone(), &Py::str("tau_R_MPa"), py::truediv(&py::mul(&v_sigma.clone(), &v_m.clone())?, &Py::Float(1000000.0))?)?;
    }
    if ({ let t3295 = v_tau_crss.clone(); Py::Bool({ let t3296 = Py::None; if !py::is(&t3295, &t3296) { true } else { false } }) }).truthy() {
        let _ = f__pos(Py::str("the critical resolved shear stress"), Py::tuple(vec![v_tau_crss.clone()]))?;
        if ({ let t3302 = v_m.clone(); Py::Bool({ let t3303 = Py::Int(0i64); if py::eq(&t3302, &t3303) { true } else { false } }) }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("the Schmid factor is 0: this system never yields under this load"))));
        }
        py::setitem(&v_out.clone(), &Py::str("yield_MPa"), py::truediv(&py::truediv(&v_tau_crss.clone(), &v_m.clone())?, &Py::Float(1000000.0))?)?;
    }
    return Ok(f__out(v_out.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_space_group(mut v_structure: Py) -> R {
    let mut v_n: Py = Py::None;
    let mut v_sym: Py = Py::None;
    let t3337 = (py::getitem(&c__SG()?, &v_structure.clone())?).items()?;
    if t3337.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_n = t3337[0].clone();
    v_sym = t3337[1].clone();
    return Ok(Py::dict(vec![(Py::str("number"), v_n.clone()), (Py::str("symbol"), v_sym.clone()), (Py::str("crystal_system"), f__system(v_n.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_structure_factor(mut v_structure: Py, mut v_h: Py, mut v_k: Py, mut v_l: Py) -> R {
    let mut v_F: Py = Py::None;
    let t3435 = (f__ints(Py::str("the Miller indices"), Py::tuple(vec![v_h.clone(), v_k.clone(), v_l.clone()]))?).items()?;
    if t3435.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_h = t3435[0].clone();
    v_k = t3435[1].clone();
    v_l = t3435[2].clone();
    v_F = f__f_rule(v_structure.clone(), v_h.clone(), v_k.clone(), v_l.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("F_over_f"), v_F.clone()), (Py::str("F2_over_f2"), py::mul(&v_F.clone(), &v_F.clone())?), (Py::str("allowed"), { let t3457 = v_F.clone(); Py::Bool({ let t3458 = Py::Int(0i64); if py::gt(&t3457, &t3458)? { true } else { false } }) })]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_tensile_toughness(mut v_strain: Py, mut v_stress: Py) -> R {
    let mut v_area: Py = Py::None;
    if ({ let t3474 = { let t3470 = py::len(&v_strain.clone())?; Py::Bool({ let t3471 = py::len(&v_stress.clone())?; if py::ne(&t3470, &t3471) { true } else { false } }) }; if t3474.truthy() { t3474 } else { { let t3472 = py::len(&v_strain.clone())?; Py::Bool({ let t3473 = Py::Int(2i64); if py::lt(&t3472, &t3473)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("strain and stress must be lists of the same length (at least 2 points)"))));
    }
    if (Py::Bool(({ let mut out = vec![]; for t3475 in py::iter(&(py::zip(&[v_strain.clone(), py::slice(&v_strain.clone(), Some((Py::Int(1i64)).i()?), None, None)?])?))? { let mut v_a: Py; let mut v_b: Py; 
let t3476 = (t3475).items()?;
if t3476.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t3476[0].clone();
v_b = t3476[1].clone();
out.push({ let t3477 = v_b.clone(); Py::Bool({ let t3478 = v_a.clone(); if py::lt(&t3477, &t3478)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy()))).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("strains must not decrease"))));
    }
    v_area = py::sum(&{ let mut out = vec![]; for t3479 in py::range_iter(&[(py::sub(&py::len(&v_strain.clone())?, &Py::Int(1i64))?).i()?]) { let mut v_i: Py; 
v_i = t3479;
out.push(py::truediv(&py::mul(&py::sub(&py::getitem(&v_strain.clone(), &py::add(&v_i.clone(), &Py::Int(1i64))?)?, &py::getitem(&v_strain.clone(), &v_i.clone())?)?, &py::add(&py::getitem(&v_stress.clone(), &py::add(&v_i.clone(), &Py::Int(1i64))?)?, &py::getitem(&v_stress.clone(), &v_i.clone())?)?)?, &Py::Int(2i64))?); } Py::list(out) }, None)?;
    return Ok(f__out(Py::dict(vec![(Py::str("toughness_MJ_m3"), v_area.clone()), (Py::str("toughness"), py::mul(&v_area.clone(), &Py::Float(1000000.0))?), (Py::str("uts_MPa"), py::minmax(&[v_stress.clone()], true)?), (Py::str("fracture_strain"), py::getitem(&v_strain.clone(), &py::neg(&Py::Int(1i64))?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_theoretical_density(mut v_structure: Py, mut v_A: Py, mut v_R: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_rho: Py = Py::None;
    let _ = f__pos(Py::str("the molar mass"), Py::tuple(vec![v_A.clone()]))?;
    let _ = f__pos(Py::str("the atomic radius"), Py::tuple(vec![v_R.clone()]))?;
    v_a = py::mul(&py::getitem(&c__A_PER_R()?, &v_structure.clone())?, &v_R.clone())?;
    v_rho = py::truediv(&py::mul(&py::getitem(&c__N_ATOMS()?, &v_structure.clone())?, &v_A.clone())?, &py::mul(&f__cell_volume(v_structure.clone(), v_a.clone())?, &c_NA()?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("rho"), v_rho.clone()), (Py::str("rho_gcc"), py::truediv(&v_rho.clone(), &Py::Int(1000i64))?), (Py::str("a_nm"), py::mul(&v_a.clone(), &Py::Float(1000000000.0))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_thermal_stress(mut v_E: Py, mut v_alpha: Py, mut v_dT: Py, mut v_constraint: Py, mut v_nu: Py) -> R {
    let mut v_f: Py = Py::None;
    let mut v_s: Py = Py::None;
    let _ = f__pos(Py::str("E and alpha"), Py::tuple(vec![v_E.clone(), v_alpha.clone()]))?;
    v_f = Py::Float(1.0);
    if ({ let t3556 = v_constraint.clone(); Py::Bool({ let t3557 = Py::str("uniaxial"); if py::ne(&t3556, &t3557) { true } else { false } }) }).truthy() {
        if ({ let t3558 = v_nu.clone(); Py::Bool({ let t3559 = Py::None; if py::is(&t3558, &t3559) { true } else { false } }) }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("Poisson's ratio is needed for a biaxial or triaxial constraint"))));
        }
        if (py::not(&{ let t3560 = py::neg(&Py::Int(1i64))?; Py::Bool({ let t3561 = v_nu.clone(); if py::lt(&t3560, &t3561)? { { let t3562 = Py::Float(0.5); if py::lt(&t3561, &t3562)? { true } else { false } } } else { false } }) })).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("Poisson's ratio must lie between -1 and 0.5"))));
        }
        v_f = (if ({ let t3563 = v_constraint.clone(); Py::Bool({ let t3564 = Py::str("biaxial"); if py::eq(&t3563, &t3564) { true } else { false } }) }).truthy() { py::truediv(&Py::Int(1i64), &py::sub(&Py::Int(1i64), &v_nu.clone())?)? } else { py::truediv(&Py::Int(1i64), &py::sub(&Py::Int(1i64), &py::mul(&Py::Int(2i64), &v_nu.clone())?)?)? });
    }
    v_s = py::mul(&py::mul(&py::mul(&py::neg(&v_E.clone())?, &v_alpha.clone())?, &v_dT.clone())?, &v_f.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("stress"), v_s.clone()), (Py::str("stress_MPa"), py::truediv(&v_s.clone(), &Py::Float(1000000.0))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_three_to_four(mut v_U: Py, mut v_V: Py, mut v_W: Py) -> R {
    let mut v_t: Py = Py::None;
    let mut v_u: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_w: Py = Py::None;
    let t2712 = (f__ints(Py::str("the direction indices"), Py::tuple(vec![v_U.clone(), v_V.clone(), v_W.clone()]))?).items()?;
    if t2712.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_U = t2712[0].clone();
    v_V = t2712[1].clone();
    v_W = t2712[2].clone();
    let t2713 = (f__reduce_ints(Py::list(vec![py::sub(&py::mul(&Py::Int(2i64), &v_U.clone())?, &v_V.clone())?, py::sub(&py::mul(&Py::Int(2i64), &v_V.clone())?, &v_U.clone())?, py::neg(&py::add(&v_U.clone(), &v_V.clone())?)?, py::mul(&Py::Int(3i64), &v_W.clone())?]))?).items()?;
    if t2713.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_u = t2713[0].clone();
    v_v = t2713[1].clone();
    v_t = t2713[2].clone();
    v_w = t2713[3].clone();
    return Ok(Py::dict(vec![(Py::str("u"), v_u.clone()), (Py::str("v"), v_v.clone()), (Py::str("t"), v_t.clone()), (Py::str("w"), v_w.clone()), (Py::str("indices"), Py::str(&["[".to_string(), py::format(&v_u.clone(), "")?, " ".to_string(), py::format(&v_v.clone(), "")?, " ".to_string(), py::format(&v_t.clone(), "")?, " ".to_string(), py::format(&v_w.clone(), "")?, "]".to_string()].concat()))]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_true_stress_strain(mut v_stress: Py, mut v_strain: Py) -> R {
    let mut v_eT: Py = Py::None;
    let mut v_sT: Py = Py::None;
    if (py::not(&{ let t3597 = v_strain.clone(); Py::Bool({ let t3598 = py::neg(&Py::Int(1i64))?; if py::gt(&t3597, &t3598)? { true } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the engineering strain must exceed -1"))));
    }
    v_sT = py::mul(&v_stress.clone(), &py::add(&Py::Int(1i64), &v_strain.clone())?)?;
    v_eT = py::math1("log1p", &v_strain.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("true_stress"), v_sT.clone()), (Py::str("true_stress_MPa"), py::truediv(&v_sT.clone(), &Py::Float(1000000.0))?), (Py::str("true_strain"), v_eT.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_vacancy_concentration(mut v_rho: Py, mut v_A: Py, mut v_Qv: Py, mut v_T: Py) -> R {
    let mut v_N: Py = Py::None;
    let mut v_f: Py = Py::None;
    let _ = f__pos(Py::str("the density, molar mass, formation energy and temperature"), Py::tuple(vec![v_rho.clone(), v_A.clone(), v_Qv.clone(), v_T.clone()]))?;
    v_N = py::truediv(&py::mul(&c_NA()?, &v_rho.clone())?, &v_A.clone())?;
    v_f = py::math1("exp", &py::truediv(&py::neg(&v_Qv.clone())?, &py::mul(&c_KB()?, &v_T.clone())?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("N"), v_N.clone()), (Py::str("Nv"), py::mul(&v_N.clone(), &v_f.clone())?), (Py::str("fraction"), v_f.clone()), (Py::str("log10_Nv"), py::sub(&py::math1("log10", &v_N.clone())?, &py::truediv(&py::truediv(&v_Qv.clone(), &py::mul(&c_KB()?, &v_T.clone())?)?, &py::math1("log", &Py::Int(10i64))?)?)?), (Py::str("N_per_cm3"), py::mul(&v_N.clone(), &Py::Float(1e-06))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_vacancy_energy_fit(mut v_T: Py, mut v_fraction: Py) -> R {
    let mut v_Q: Py = Py::None;
    let mut v_b0: Py = Py::None;
    let mut v_b1: Py = Py::None;
    let mut v_f: Py = Py::None;
    let mut v_r2: Py = Py::None;
    let mut v_t: Py = Py::None;
    if ({ let t3714 = py::len(&v_T.clone())?; Py::Bool({ let t3715 = py::len(&v_fraction.clone())?; if py::ne(&t3714, &t3715) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("T and fraction must have the same length"))));
    }
    for t3716 in py::iter(&(v_T.clone()))? {
        v_t = t3716;
        let _ = f__pos(Py::str("each temperature"), Py::tuple(vec![v_t.clone()]))?;
    }
    for t3722 in py::iter(&(v_fraction.clone()))? {
        v_f = t3722;
        let _ = f__frac01(Py::str("each vacancy fraction"), v_f.clone(), Py::Bool(false))?;
    }
    let t3746 = (f__linfit({ let mut out = vec![]; for t3744 in py::iter(&(v_T.clone()))? { let mut v_t: Py; 
v_t = t3744;
out.push(py::truediv(&Py::Int(1i64), &v_t.clone())?); } Py::list(out) }, { let mut out = vec![]; for t3745 in py::iter(&(v_fraction.clone()))? { let mut v_f: Py; 
v_f = t3745;
out.push(py::math1("log", &v_f.clone())?); } Py::list(out) })?).items()?;
    if t3746.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_b0 = t3746[0].clone();
    v_b1 = t3746[1].clone();
    v_r2 = t3746[2].clone();
    if ({ let t3747 = v_b1.clone(); Py::Bool({ let t3748 = Py::Int(0i64); if py::ge(&t3747, &t3748)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the vacancy fraction must increase with temperature"))));
    }
    v_Q = py::mul(&py::neg(&v_b1.clone())?, &c_KB()?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("Qv_eV"), py::truediv(&v_Q.clone(), &c_QE()?)?), (Py::str("Qv"), v_Q.clone()), (Py::str("prefactor"), py::math1("exp", &v_b0.clone())?), (Py::str("r_squared"), v_r2.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_verify_intercepts(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_h: Py = Py::None;
    let mut v_hk: Py = Py::None;
    let mut v_ints: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_l: Py = Py::None;
    let mut v_vals: Py = Py::None;
    let t2760: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_ints = { let mut out = vec![]; for t2737 in py::iter(&(Py::tuple(vec![Py::str("x"), Py::str("y"), Py::str("z")])))? { let mut v_s: Py; 
v_s = t2737;
out.push(f__intercept(py::getitem(&v_inputs.clone(), &v_s.clone())?, v_s.clone())?); } Py::list(out) };
            let t2739 = ({ let mut out = vec![]; for t2738 in py::iter(&(Py::tuple(vec![Py::str("h"), Py::str("k"), Py::str("l")])))? { let mut v_s: Py; 
v_s = t2738;
out.push(py::getitem(&v_outputs.clone(), &v_s.clone())?); } Py::list(out) }).items()?;
            if t2739.len() != 3 { return Err(PyErr::Value("unpack".into())); }
            v_h = t2739[0].clone();
            v_k = t2739[1].clone();
            v_l = t2739[2].clone();
            if ({ let t2743 = Py::Bool(({ let mut out = vec![]; for t2740 in py::iter(&(Py::tuple(vec![v_h.clone(), v_k.clone(), v_l.clone()])))? { let mut v_v: Py; 
v_v = t2740;
out.push(py::not(&py::method(&py::float(&v_v.clone())?, "is_integer", &[])?)); } Py::list(out) }).items()?.iter().any(|x| x.truthy())); if t2743.truthy() { t2743 } else { { let t2741 = f__gcd3(Py::tuple(vec![py::int(&v_h.clone())?, py::int(&v_k.clone())?, py::int(&v_l.clone())?]))?; Py::Bool({ let t2742 = Py::Int(1i64); if py::ne(&t2741, &t2742) { true } else { false } }) } } }).truthy() {
                return Ok(Some(Py::Float(f64::INFINITY)));
            }
            v_hk = Py::list(vec![py::int(&v_h.clone())?, py::int(&v_k.clone())?, py::int(&v_l.clone())?]);
            v_vals = { let mut out = vec![]; for t2744 in py::range_iter(&[(Py::Int(3i64)).i()?]) { let mut v_i: Py; 
v_i = t2744;
if ({ let t2745 = py::getitem(&v_ints.clone(), &v_i.clone())?; Py::Bool({ let t2746 = Py::None; if !py::is(&t2745, &t2746) { true } else { false } }) }).truthy() { out.push(py::mul(&py::getitem(&v_hk.clone(), &v_i.clone())?, &py::getitem(&v_ints.clone(), &v_i.clone())?)?); }} Py::list(out) };
            if ({ let t2757 = Py::Bool(({ let mut out = vec![]; for t2747 in py::range_iter(&[(Py::Int(3i64)).i()?]) { let mut v_i: Py; 
v_i = t2747;
if ({ let t2748 = py::getitem(&v_ints.clone(), &v_i.clone())?; Py::Bool({ let t2749 = Py::None; if py::is(&t2748, &t2749) { true } else { false } }) }).truthy() { out.push({ let t2750 = py::getitem(&v_hk.clone(), &v_i.clone())?; Py::Bool({ let t2751 = Py::Int(0i64); if py::ne(&t2750, &t2751) { true } else { false } }) }); }} Py::list(out) }).items()?.iter().any(|x| x.truthy())); if t2757.truthy() { t2757 } else { { let t2756 = { let t2752 = py::len(&py::set(&v_vals.clone())?)?; Py::Bool({ let t2753 = Py::Int(1i64); if py::ne(&t2752, &t2753) { true } else { false } }) }; if t2756.truthy() { t2756 } else { { let t2754 = py::getitem(&v_vals.clone(), &Py::Int(0i64))?; Py::Bool({ let t2755 = Py::Int(0i64); if py::le(&t2754, &t2755)? { true } else { false } }) } } } } }).truthy() {
                return Ok(Some(Py::Float(f64::INFINITY)));
            }
            return Ok(Some((if ({ let t2758 = py::method(&v_outputs.clone(), "get", &[Py::str("indices")])?; Py::Bool({ let t2759 = Py::str(&["(".to_string(), py::format(&py::getitem(&v_hk.clone(), &Py::Int(0i64))?, "")?, " ".to_string(), py::format(&py::getitem(&v_hk.clone(), &Py::Int(1i64))?, "")?, " ".to_string(), py::format(&py::getitem(&v_hk.clone(), &Py::Int(2i64))?, "")?, ")".to_string()].concat()); if py::eq(&t2758, &t2759) { true } else { false } }) }).truthy() { Py::Float(0.0) } else { Py::Float(f64::INFINITY) })));
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2760 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_verify_structure_factor(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_F: Py = Py::None;
    let t3465: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_F = f__f_sum(py::getitem(&v_inputs.clone(), &Py::str("structure"))?, py::int(&py::getitem(&v_inputs.clone(), &Py::str("h"))?)?, py::int(&py::getitem(&v_inputs.clone(), &Py::str("k"))?)?, py::int(&py::getitem(&v_inputs.clone(), &Py::str("l"))?)?)?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3465 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    return Ok(py::minmax(&[py::abs(&py::sub(&py::float(&py::getitem(&v_outputs.clone(), &Py::str("F_over_f"))?)?, &v_F.clone())?)?, py::truediv(&py::abs(&py::sub(&py::float(&py::getitem(&v_outputs.clone(), &Py::str("F2_over_f2"))?)?, &py::mul(&v_F.clone(), &v_F.clone())?)?)?, &py::minmax(&[Py::Float(1.0), py::mul(&v_F.clone(), &v_F.clone())?], true)?)?, (if ({ let t3466 = py::getitem(&v_outputs.clone(), &Py::str("allowed"))?; Py::Bool({ let t3467 = { let t3468 = v_F.clone(); Py::Bool({ let t3469 = Py::Int(0i64); if py::gt(&t3468, &t3469)? { true } else { false } }) }; if py::eq(&t3466, &t3467) { true } else { false } }) }).truthy() { Py::Float(0.0) } else { Py::Float(1.0) })], true)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_volumetric_expansion(mut v_alpha: Py, mut v_V0: Py, mut v_dT: Py) -> R {
    let mut v_beta: Py = Py::None;
    let mut v_dV: Py = Py::None;
    let mut v_dV_exact: Py = Py::None;
    let mut v_s: Py = Py::None;
    let _ = f__pos(Py::str("alpha and V0"), Py::tuple(vec![v_alpha.clone(), v_V0.clone()]))?;
    if (py::not(&py::math1("isfinite", &v_dT.clone())?)).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the temperature change must be a number"))));
    }
    v_s = py::mul(&v_alpha.clone(), &v_dT.clone())?;
    if ({ let t3786 = v_s.clone(); Py::Bool({ let t3787 = py::neg(&Py::Int(1i64))?; if py::le(&t3786, &t3787)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the strain alpha dT must exceed -1"))));
    }
    v_beta = py::mul(&Py::Int(3i64), &v_alpha.clone())?;
    v_dV = py::mul(&py::mul(&v_beta.clone(), &v_V0.clone())?, &v_dT.clone())?;
    v_dV_exact = py::mul(&py::mul(&v_V0.clone(), &v_s.clone())?, &py::add(&py::add(&Py::Int(3i64), &py::mul(&Py::Int(3i64), &v_s.clone())?)?, &py::mul(&v_s.clone(), &v_s.clone())?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("beta"), v_beta.clone()), (Py::str("dV"), v_dV.clone()), (Py::str("dV_cm3"), py::mul(&v_dV.clone(), &Py::Float(1000000.0))?), (Py::str("V"), py::add(&v_V0.clone(), &v_dV.clone())?), (Py::str("dV_exact"), v_dV_exact.clone()), (Py::str("dV_exact_cm3"), py::mul(&v_dV_exact.clone(), &Py::Float(1000000.0))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_weibull_failure(mut v_sigma: Py, mut v_sigma0: Py, mut v_m: Py, mut v_V_ratio: Py) -> R {
    let mut v_x: Py = Py::None;
    let _ = f__pos(Py::str("the stresses, the Weibull modulus and the volume ratio"), Py::tuple(vec![v_sigma.clone(), v_sigma0.clone(), v_m.clone(), v_V_ratio.clone()]))?;
    v_x = py::mul(&v_V_ratio.clone(), &py::pow(&py::truediv(&v_sigma.clone(), &v_sigma0.clone())?, &v_m.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("P_failure"), py::neg(&py::math1("expm1", &py::neg(&v_x.clone())?)?)?), (Py::str("P_survival"), py::math1("exp", &py::neg(&v_x.clone())?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_weibull_fit(mut v_strengths: Py) -> R {
    let mut v_F: Py = Py::None;
    let mut v_b0: Py = Py::None;
    let mut v_b1: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_r2: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_v: Py = Py::None;
    if ({ let t3901 = py::len(&v_strengths.clone())?; Py::Bool({ let t3902 = Py::Int(3i64); if py::lt(&t3901, &t3902)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("give at least three strengths"))));
    }
    for t3903 in py::iter(&(v_strengths.clone()))? {
        v_v = t3903;
        let _ = f__pos(Py::str("every strength"), Py::tuple(vec![v_v.clone()]))?;
    }
    v_s = py::sorted(&v_strengths.clone(), (Py::Bool(false)).truthy())?;
    v_n = py::len(&v_s.clone())?;
    if ({ let t3909 = py::getitem(&v_s.clone(), &Py::Int(0i64))?; Py::Bool({ let t3910 = py::getitem(&v_s.clone(), &py::neg(&Py::Int(1i64))?)?; if py::eq(&t3909, &t3910) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the strengths must not all be equal"))));
    }
    v_F = { let mut out = vec![]; for t3911 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t3911;
out.push(py::truediv(&py::sub(&py::add(&v_i.clone(), &Py::Int(1i64))?, &Py::Float(0.3))?, &py::add(&v_n.clone(), &Py::Float(0.4))?)?); } Py::list(out) };
    let t3928 = (f__linfit({ let mut out = vec![]; for t3926 in py::iter(&(v_s.clone()))? { let mut v_v: Py; 
v_v = t3926;
out.push(py::math1("log", &v_v.clone())?); } Py::list(out) }, { let mut out = vec![]; for t3927 in py::iter(&(v_F.clone()))? { let mut v_f: Py; 
v_f = t3927;
out.push(py::math1("log", &py::neg(&py::math1("log1p", &py::neg(&v_f.clone())?)?)?)?); } Py::list(out) })?).items()?;
    if t3928.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_b0 = t3928[0].clone();
    v_b1 = t3928[1].clone();
    v_r2 = t3928[2].clone();
    return Ok(f__out(Py::dict(vec![(Py::str("m"), v_b1.clone()), (Py::str("sigma0"), py::math1("exp", &py::truediv(&py::neg(&v_b0.clone())?, &v_b1.clone())?)?), (Py::str("r_squared"), v_r2.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_weibull_stress(mut v_P_failure: Py, mut v_sigma0: Py, mut v_m: Py, mut v_V_ratio: Py) -> R {
    let mut v_s: Py = Py::None;
    let _ = f__pos(Py::str("sigma0, m and the volume ratio"), Py::tuple(vec![v_sigma0.clone(), v_m.clone(), v_V_ratio.clone()]))?;
    let _ = f__frac01(Py::str("the failure probability"), v_P_failure.clone(), Py::Bool(false))?;
    v_s = py::mul(&v_sigma0.clone(), &py::pow(&py::truediv(&py::neg(&py::math1("log1p", &py::neg(&v_P_failure.clone())?)?)?, &v_V_ratio.clone())?, &py::truediv(&Py::Int(1i64), &v_m.clone())?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("sigma"), v_s.clone()), (Py::str("sigma_MPa"), py::truediv(&v_s.clone(), &Py::Float(1000000.0))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_wiedemann_franz(mut v_T: Py, mut v_sigma: Py, mut v_rho: Py) -> R {
    let mut v_s: Py = Py::None;
    let _ = f__pos(Py::str("the temperature"), Py::tuple(vec![v_T.clone()]))?;
    if ({ let t4007 = { let t4009 = v_sigma.clone(); Py::Bool({ let t4010 = Py::None; if py::is(&t4009, &t4010) { true } else { false } }) }; Py::Bool({ let t4008 = { let t4011 = v_rho.clone(); Py::Bool({ let t4012 = Py::None; if py::is(&t4011, &t4012) { true } else { false } }) }; if py::eq(&t4007, &t4008) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("give exactly one of the conductivity sigma and the resistivity rho"))));
    }
    v_s = (if ({ let t4013 = v_sigma.clone(); Py::Bool({ let t4014 = Py::None; if !py::is(&t4013, &t4014) { true } else { false } }) }).truthy() { v_sigma.clone() } else { (if ({ let t4015 = v_rho.clone(); Py::Bool({ let t4016 = Py::Int(0i64); if py::gt(&t4015, &t4016)? { true } else { false } }) }).truthy() { py::truediv(&Py::Int(1i64), &v_rho.clone())? } else { py::neg(&Py::Int(1i64))? }) });
    let _ = f__pos(Py::str("the conductivity"), Py::tuple(vec![v_s.clone()]))?;
    return Ok(f__out(Py::dict(vec![(Py::str("kappa"), py::mul(&py::mul(&c__LORENZ()?, &v_s.clone())?, &v_T.clone())?), (Py::str("lorenz"), c__LORENZ()?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_wt_to_at(mut v_element1: Py, mut v_element2: Py, mut v_wt1: Py) -> R {
    let mut v_A1: Py = Py::None;
    let mut v_A2: Py = Py::None;
    let mut v_at1: Py = Py::None;
    let mut v_e: Py = Py::None;
    for t3961 in py::iter(&(Py::tuple(vec![v_element1.clone(), v_element2.clone()])))? {
        v_e = t3961;
        if ({ let t3962 = v_e.clone(); Py::Bool({ let t3963 = c_ATOMIC_WEIGHT()?; if !py::contains(&t3963, &t3962)? { true } else { false } }) }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str(&["unknown element ".to_string(), py::format(&v_e.clone(), "")?].concat()))));
        }
    }
    if ({ let t3964 = v_element1.clone(); Py::Bool({ let t3965 = v_element2.clone(); if py::eq(&t3964, &t3965) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the two elements must differ"))));
    }
    if (py::not(&{ let t3966 = Py::Int(0i64); Py::Bool({ let t3967 = v_wt1.clone(); if py::le(&t3966, &t3967)? { { let t3968 = Py::Int(100i64); if py::le(&t3967, &t3968)? { true } else { false } } } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the weight percent must lie between 0 and 100"))));
    }
    let t3969 = (Py::tuple(vec![py::getitem(&c_ATOMIC_WEIGHT()?, &v_element1.clone())?, py::getitem(&c_ATOMIC_WEIGHT()?, &v_element2.clone())?])).items()?;
    if t3969.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_A1 = t3969[0].clone();
    v_A2 = t3969[1].clone();
    v_at1 = py::mul(&py::truediv(&py::mul(&v_wt1.clone(), &v_A2.clone())?, &py::add(&py::mul(&v_wt1.clone(), &v_A2.clone())?, &py::mul(&py::sub(&Py::Int(100i64), &v_wt1.clone())?, &v_A1.clone())?)?)?, &Py::Int(100i64))?;
    return Ok(f__out(Py::dict(vec![(Py::str("at1"), v_at1.clone()), (Py::str("at2"), py::sub(&Py::Int(100i64), &v_at1.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_youngs_modulus_fit(mut v_strain: Py, mut v_stress: Py) -> R {
    let mut v_b0: Py = Py::None;
    let mut v_b1: Py = Py::None;
    let mut v_r2: Py = Py::None;
    if ({ let t4049 = py::len(&v_strain.clone())?; Py::Bool({ let t4050 = py::len(&v_stress.clone())?; if py::ne(&t4049, &t4050) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("strain and stress must have the same length"))));
    }
    let t4065 = (f__linfit(Py::list((v_strain.clone()).items()?), Py::list((v_stress.clone()).items()?))?).items()?;
    if t4065.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_b0 = t4065[0].clone();
    v_b1 = t4065[1].clone();
    v_r2 = t4065[2].clone();
    if ({ let t4066 = v_b1.clone(); Py::Bool({ let t4067 = Py::Int(0i64); if py::le(&t4066, &t4067)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("stress must rise with strain in the elastic region"))));
    }
    v_b0 = (if ({ let t4068 = py::abs(&v_b0.clone())?; Py::Bool({ let t4069 = py::mul(&Py::Float(1e-12), &py::minmax(&[{ let mut out = vec![]; for t4070 in py::iter(&(v_stress.clone()))? { let mut v_v: Py; 
v_v = t4070;
out.push(py::abs(&v_v.clone())?); } Py::list(out) }], true)?)?; if py::le(&t4068, &t4069)? { true } else { false } }) }).truthy() { Py::Float(0.0) } else { v_b0.clone() });
    return Ok(f__out(Py::dict(vec![(Py::str("E_GPa"), py::truediv(&v_b1.clone(), &Py::Int(1000i64))?), (Py::str("E"), py::mul(&v_b1.clone(), &Py::Float(1000000.0))?), (Py::str("intercept_MPa"), v_b0.clone()), (Py::str("r_squared"), v_r2.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1031(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_x: Py = Py::None;
    v_x = py::dict_from(&v_inputs.clone(), vec![])?;
    let t1057: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            if ({ let t1032 = Py::str("cubic"); Py::Bool({ let t1033 = Py::str("cubic"); if py::eq(&t1032, &t1033) { true } else { false } }) }).truthy() {
                v_d = f__d_general(py::getitem(&v_x.clone(), &Py::str("a"))?, py::getitem(&v_x.clone(), &Py::str("a"))?, py::getitem(&v_x.clone(), &Py::str("a"))?, Py::Int(90i64), Py::Int(90i64), Py::Int(90i64), py::getitem(&v_x.clone(), &Py::str("h"))?, py::getitem(&v_x.clone(), &Py::str("k"))?, py::getitem(&v_x.clone(), &Py::str("l"))?)?;
            } else {
                if ({ let t1051 = Py::str("cubic"); Py::Bool({ let t1052 = Py::str("tetragonal"); if py::eq(&t1051, &t1052) { true } else { false } }) }).truthy() {
                    v_d = f__d_general(py::getitem(&v_x.clone(), &Py::str("a"))?, py::getitem(&v_x.clone(), &Py::str("a"))?, py::getitem(&v_x.clone(), &Py::str("c"))?, Py::Int(90i64), Py::Int(90i64), Py::Int(90i64), py::getitem(&v_x.clone(), &Py::str("h"))?, py::getitem(&v_x.clone(), &Py::str("k"))?, py::getitem(&v_x.clone(), &Py::str("l"))?)?;
                } else {
                    if ({ let t1053 = Py::str("cubic"); Py::Bool({ let t1054 = Py::str("orthorhombic"); if py::eq(&t1053, &t1054) { true } else { false } }) }).truthy() {
                        v_d = f__d_general(py::getitem(&v_x.clone(), &Py::str("a"))?, py::getitem(&v_x.clone(), &Py::str("b"))?, py::getitem(&v_x.clone(), &Py::str("c"))?, Py::Int(90i64), Py::Int(90i64), Py::Int(90i64), py::getitem(&v_x.clone(), &Py::str("h"))?, py::getitem(&v_x.clone(), &Py::str("k"))?, py::getitem(&v_x.clone(), &Py::str("l"))?)?;
                    } else {
                        if ({ let t1055 = Py::str("cubic"); Py::Bool({ let t1056 = Py::str("hexagonal"); if py::eq(&t1055, &t1056) { true } else { false } }) }).truthy() {
                            v_d = f__d_general(py::getitem(&v_x.clone(), &Py::str("a"))?, py::getitem(&v_x.clone(), &Py::str("a"))?, py::getitem(&v_x.clone(), &Py::str("c"))?, Py::Int(90i64), Py::Int(90i64), Py::Int(120i64), py::getitem(&v_x.clone(), &Py::str("h"))?, py::getitem(&v_x.clone(), &Py::str("k"))?, py::getitem(&v_x.clone(), &Py::str("l"))?)?;
                        } else {
                            v_d = f__d_general(py::getitem(&v_x.clone(), &Py::str("a"))?, py::getitem(&v_x.clone(), &Py::str("b"))?, py::getitem(&v_x.clone(), &Py::str("c"))?, py::getitem(&v_x.clone(), &Py::str("alpha"))?, py::getitem(&v_x.clone(), &Py::str("beta"))?, py::getitem(&v_x.clone(), &Py::str("gamma"))?, py::getitem(&v_x.clone(), &Py::str("h"))?, py::getitem(&v_x.clone(), &Py::str("k"))?, py::getitem(&v_x.clone(), &Py::str("l"))?)?;
                        }
                    }
                }
            }
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1057 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    return Ok(py::minmax(&[f__rel(py::method(&v_outputs.clone(), "get", &[Py::str("d")])?, v_d.clone())?, f__rel(py::method(&v_outputs.clone(), "get", &[Py::str("d_nm")])?, py::mul(&v_d.clone(), &Py::Float(1000000000.0))?)?], true)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1103(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_x: Py = Py::None;
    v_x = py::dict_from(&v_inputs.clone(), vec![])?;
    let t1129: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            if ({ let t1104 = Py::str("hexagonal"); Py::Bool({ let t1105 = Py::str("cubic"); if py::eq(&t1104, &t1105) { true } else { false } }) }).truthy() {
                v_d = f__d_general(py::getitem(&v_x.clone(), &Py::str("a"))?, py::getitem(&v_x.clone(), &Py::str("a"))?, py::getitem(&v_x.clone(), &Py::str("a"))?, Py::Int(90i64), Py::Int(90i64), Py::Int(90i64), py::getitem(&v_x.clone(), &Py::str("h"))?, py::getitem(&v_x.clone(), &Py::str("k"))?, py::getitem(&v_x.clone(), &Py::str("l"))?)?;
            } else {
                if ({ let t1123 = Py::str("hexagonal"); Py::Bool({ let t1124 = Py::str("tetragonal"); if py::eq(&t1123, &t1124) { true } else { false } }) }).truthy() {
                    v_d = f__d_general(py::getitem(&v_x.clone(), &Py::str("a"))?, py::getitem(&v_x.clone(), &Py::str("a"))?, py::getitem(&v_x.clone(), &Py::str("c"))?, Py::Int(90i64), Py::Int(90i64), Py::Int(90i64), py::getitem(&v_x.clone(), &Py::str("h"))?, py::getitem(&v_x.clone(), &Py::str("k"))?, py::getitem(&v_x.clone(), &Py::str("l"))?)?;
                } else {
                    if ({ let t1125 = Py::str("hexagonal"); Py::Bool({ let t1126 = Py::str("orthorhombic"); if py::eq(&t1125, &t1126) { true } else { false } }) }).truthy() {
                        v_d = f__d_general(py::getitem(&v_x.clone(), &Py::str("a"))?, py::getitem(&v_x.clone(), &Py::str("b"))?, py::getitem(&v_x.clone(), &Py::str("c"))?, Py::Int(90i64), Py::Int(90i64), Py::Int(90i64), py::getitem(&v_x.clone(), &Py::str("h"))?, py::getitem(&v_x.clone(), &Py::str("k"))?, py::getitem(&v_x.clone(), &Py::str("l"))?)?;
                    } else {
                        if ({ let t1127 = Py::str("hexagonal"); Py::Bool({ let t1128 = Py::str("hexagonal"); if py::eq(&t1127, &t1128) { true } else { false } }) }).truthy() {
                            v_d = f__d_general(py::getitem(&v_x.clone(), &Py::str("a"))?, py::getitem(&v_x.clone(), &Py::str("a"))?, py::getitem(&v_x.clone(), &Py::str("c"))?, Py::Int(90i64), Py::Int(90i64), Py::Int(120i64), py::getitem(&v_x.clone(), &Py::str("h"))?, py::getitem(&v_x.clone(), &Py::str("k"))?, py::getitem(&v_x.clone(), &Py::str("l"))?)?;
                        } else {
                            v_d = f__d_general(py::getitem(&v_x.clone(), &Py::str("a"))?, py::getitem(&v_x.clone(), &Py::str("b"))?, py::getitem(&v_x.clone(), &Py::str("c"))?, py::getitem(&v_x.clone(), &Py::str("alpha"))?, py::getitem(&v_x.clone(), &Py::str("beta"))?, py::getitem(&v_x.clone(), &Py::str("gamma"))?, py::getitem(&v_x.clone(), &Py::str("h"))?, py::getitem(&v_x.clone(), &Py::str("k"))?, py::getitem(&v_x.clone(), &Py::str("l"))?)?;
                        }
                    }
                }
            }
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1129 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    return Ok(py::minmax(&[f__rel(py::method(&v_outputs.clone(), "get", &[Py::str("d")])?, v_d.clone())?, f__rel(py::method(&v_outputs.clone(), "get", &[Py::str("d_nm")])?, py::mul(&v_d.clone(), &Py::Float(1000000000.0))?)?], true)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1170(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_x: Py = Py::None;
    v_x = py::dict_from(&v_inputs.clone(), vec![])?;
    let t1196: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            if ({ let t1171 = Py::str("orthorhombic"); Py::Bool({ let t1172 = Py::str("cubic"); if py::eq(&t1171, &t1172) { true } else { false } }) }).truthy() {
                v_d = f__d_general(py::getitem(&v_x.clone(), &Py::str("a"))?, py::getitem(&v_x.clone(), &Py::str("a"))?, py::getitem(&v_x.clone(), &Py::str("a"))?, Py::Int(90i64), Py::Int(90i64), Py::Int(90i64), py::getitem(&v_x.clone(), &Py::str("h"))?, py::getitem(&v_x.clone(), &Py::str("k"))?, py::getitem(&v_x.clone(), &Py::str("l"))?)?;
            } else {
                if ({ let t1190 = Py::str("orthorhombic"); Py::Bool({ let t1191 = Py::str("tetragonal"); if py::eq(&t1190, &t1191) { true } else { false } }) }).truthy() {
                    v_d = f__d_general(py::getitem(&v_x.clone(), &Py::str("a"))?, py::getitem(&v_x.clone(), &Py::str("a"))?, py::getitem(&v_x.clone(), &Py::str("c"))?, Py::Int(90i64), Py::Int(90i64), Py::Int(90i64), py::getitem(&v_x.clone(), &Py::str("h"))?, py::getitem(&v_x.clone(), &Py::str("k"))?, py::getitem(&v_x.clone(), &Py::str("l"))?)?;
                } else {
                    if ({ let t1192 = Py::str("orthorhombic"); Py::Bool({ let t1193 = Py::str("orthorhombic"); if py::eq(&t1192, &t1193) { true } else { false } }) }).truthy() {
                        v_d = f__d_general(py::getitem(&v_x.clone(), &Py::str("a"))?, py::getitem(&v_x.clone(), &Py::str("b"))?, py::getitem(&v_x.clone(), &Py::str("c"))?, Py::Int(90i64), Py::Int(90i64), Py::Int(90i64), py::getitem(&v_x.clone(), &Py::str("h"))?, py::getitem(&v_x.clone(), &Py::str("k"))?, py::getitem(&v_x.clone(), &Py::str("l"))?)?;
                    } else {
                        if ({ let t1194 = Py::str("orthorhombic"); Py::Bool({ let t1195 = Py::str("hexagonal"); if py::eq(&t1194, &t1195) { true } else { false } }) }).truthy() {
                            v_d = f__d_general(py::getitem(&v_x.clone(), &Py::str("a"))?, py::getitem(&v_x.clone(), &Py::str("a"))?, py::getitem(&v_x.clone(), &Py::str("c"))?, Py::Int(90i64), Py::Int(90i64), Py::Int(120i64), py::getitem(&v_x.clone(), &Py::str("h"))?, py::getitem(&v_x.clone(), &Py::str("k"))?, py::getitem(&v_x.clone(), &Py::str("l"))?)?;
                        } else {
                            v_d = f__d_general(py::getitem(&v_x.clone(), &Py::str("a"))?, py::getitem(&v_x.clone(), &Py::str("b"))?, py::getitem(&v_x.clone(), &Py::str("c"))?, py::getitem(&v_x.clone(), &Py::str("alpha"))?, py::getitem(&v_x.clone(), &Py::str("beta"))?, py::getitem(&v_x.clone(), &Py::str("gamma"))?, py::getitem(&v_x.clone(), &Py::str("h"))?, py::getitem(&v_x.clone(), &Py::str("k"))?, py::getitem(&v_x.clone(), &Py::str("l"))?)?;
                        }
                    }
                }
            }
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1196 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    return Ok(py::minmax(&[f__rel(py::method(&v_outputs.clone(), "get", &[Py::str("d")])?, v_d.clone())?, f__rel(py::method(&v_outputs.clone(), "get", &[Py::str("d_nm")])?, py::mul(&v_d.clone(), &Py::Float(1000000000.0))?)?], true)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1237(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_x: Py = Py::None;
    v_x = py::dict_from(&v_inputs.clone(), vec![])?;
    let t1263: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            if ({ let t1238 = Py::str("tetragonal"); Py::Bool({ let t1239 = Py::str("cubic"); if py::eq(&t1238, &t1239) { true } else { false } }) }).truthy() {
                v_d = f__d_general(py::getitem(&v_x.clone(), &Py::str("a"))?, py::getitem(&v_x.clone(), &Py::str("a"))?, py::getitem(&v_x.clone(), &Py::str("a"))?, Py::Int(90i64), Py::Int(90i64), Py::Int(90i64), py::getitem(&v_x.clone(), &Py::str("h"))?, py::getitem(&v_x.clone(), &Py::str("k"))?, py::getitem(&v_x.clone(), &Py::str("l"))?)?;
            } else {
                if ({ let t1257 = Py::str("tetragonal"); Py::Bool({ let t1258 = Py::str("tetragonal"); if py::eq(&t1257, &t1258) { true } else { false } }) }).truthy() {
                    v_d = f__d_general(py::getitem(&v_x.clone(), &Py::str("a"))?, py::getitem(&v_x.clone(), &Py::str("a"))?, py::getitem(&v_x.clone(), &Py::str("c"))?, Py::Int(90i64), Py::Int(90i64), Py::Int(90i64), py::getitem(&v_x.clone(), &Py::str("h"))?, py::getitem(&v_x.clone(), &Py::str("k"))?, py::getitem(&v_x.clone(), &Py::str("l"))?)?;
                } else {
                    if ({ let t1259 = Py::str("tetragonal"); Py::Bool({ let t1260 = Py::str("orthorhombic"); if py::eq(&t1259, &t1260) { true } else { false } }) }).truthy() {
                        v_d = f__d_general(py::getitem(&v_x.clone(), &Py::str("a"))?, py::getitem(&v_x.clone(), &Py::str("b"))?, py::getitem(&v_x.clone(), &Py::str("c"))?, Py::Int(90i64), Py::Int(90i64), Py::Int(90i64), py::getitem(&v_x.clone(), &Py::str("h"))?, py::getitem(&v_x.clone(), &Py::str("k"))?, py::getitem(&v_x.clone(), &Py::str("l"))?)?;
                    } else {
                        if ({ let t1261 = Py::str("tetragonal"); Py::Bool({ let t1262 = Py::str("hexagonal"); if py::eq(&t1261, &t1262) { true } else { false } }) }).truthy() {
                            v_d = f__d_general(py::getitem(&v_x.clone(), &Py::str("a"))?, py::getitem(&v_x.clone(), &Py::str("a"))?, py::getitem(&v_x.clone(), &Py::str("c"))?, Py::Int(90i64), Py::Int(90i64), Py::Int(120i64), py::getitem(&v_x.clone(), &Py::str("h"))?, py::getitem(&v_x.clone(), &Py::str("k"))?, py::getitem(&v_x.clone(), &Py::str("l"))?)?;
                        } else {
                            v_d = f__d_general(py::getitem(&v_x.clone(), &Py::str("a"))?, py::getitem(&v_x.clone(), &Py::str("b"))?, py::getitem(&v_x.clone(), &Py::str("c"))?, py::getitem(&v_x.clone(), &Py::str("alpha"))?, py::getitem(&v_x.clone(), &Py::str("beta"))?, py::getitem(&v_x.clone(), &Py::str("gamma"))?, py::getitem(&v_x.clone(), &Py::str("h"))?, py::getitem(&v_x.clone(), &Py::str("k"))?, py::getitem(&v_x.clone(), &Py::str("l"))?)?;
                        }
                    }
                }
            }
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1263 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    return Ok(py::minmax(&[f__rel(py::method(&v_outputs.clone(), "get", &[Py::str("d")])?, v_d.clone())?, f__rel(py::method(&v_outputs.clone(), "get", &[Py::str("d_nm")])?, py::mul(&v_d.clone(), &Py::Float(1000000000.0))?)?], true)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1320(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_x: Py = Py::None;
    v_x = py::dict_from(&v_inputs.clone(), vec![])?;
    let t1329: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            if ({ let t1321 = Py::str("triclinic"); Py::Bool({ let t1322 = Py::str("cubic"); if py::eq(&t1321, &t1322) { true } else { false } }) }).truthy() {
                v_d = f__d_general(py::getitem(&v_x.clone(), &Py::str("a"))?, py::getitem(&v_x.clone(), &Py::str("a"))?, py::getitem(&v_x.clone(), &Py::str("a"))?, Py::Int(90i64), Py::Int(90i64), Py::Int(90i64), py::getitem(&v_x.clone(), &Py::str("h"))?, py::getitem(&v_x.clone(), &Py::str("k"))?, py::getitem(&v_x.clone(), &Py::str("l"))?)?;
            } else {
                if ({ let t1323 = Py::str("triclinic"); Py::Bool({ let t1324 = Py::str("tetragonal"); if py::eq(&t1323, &t1324) { true } else { false } }) }).truthy() {
                    v_d = f__d_general(py::getitem(&v_x.clone(), &Py::str("a"))?, py::getitem(&v_x.clone(), &Py::str("a"))?, py::getitem(&v_x.clone(), &Py::str("c"))?, Py::Int(90i64), Py::Int(90i64), Py::Int(90i64), py::getitem(&v_x.clone(), &Py::str("h"))?, py::getitem(&v_x.clone(), &Py::str("k"))?, py::getitem(&v_x.clone(), &Py::str("l"))?)?;
                } else {
                    if ({ let t1325 = Py::str("triclinic"); Py::Bool({ let t1326 = Py::str("orthorhombic"); if py::eq(&t1325, &t1326) { true } else { false } }) }).truthy() {
                        v_d = f__d_general(py::getitem(&v_x.clone(), &Py::str("a"))?, py::getitem(&v_x.clone(), &Py::str("b"))?, py::getitem(&v_x.clone(), &Py::str("c"))?, Py::Int(90i64), Py::Int(90i64), Py::Int(90i64), py::getitem(&v_x.clone(), &Py::str("h"))?, py::getitem(&v_x.clone(), &Py::str("k"))?, py::getitem(&v_x.clone(), &Py::str("l"))?)?;
                    } else {
                        if ({ let t1327 = Py::str("triclinic"); Py::Bool({ let t1328 = Py::str("hexagonal"); if py::eq(&t1327, &t1328) { true } else { false } }) }).truthy() {
                            v_d = f__d_general(py::getitem(&v_x.clone(), &Py::str("a"))?, py::getitem(&v_x.clone(), &Py::str("a"))?, py::getitem(&v_x.clone(), &Py::str("c"))?, Py::Int(90i64), Py::Int(90i64), Py::Int(120i64), py::getitem(&v_x.clone(), &Py::str("h"))?, py::getitem(&v_x.clone(), &Py::str("k"))?, py::getitem(&v_x.clone(), &Py::str("l"))?)?;
                        } else {
                            v_d = f__d_general(py::getitem(&v_x.clone(), &Py::str("a"))?, py::getitem(&v_x.clone(), &Py::str("b"))?, py::getitem(&v_x.clone(), &Py::str("c"))?, py::getitem(&v_x.clone(), &Py::str("alpha"))?, py::getitem(&v_x.clone(), &Py::str("beta"))?, py::getitem(&v_x.clone(), &Py::str("gamma"))?, py::getitem(&v_x.clone(), &Py::str("h"))?, py::getitem(&v_x.clone(), &Py::str("k"))?, py::getitem(&v_x.clone(), &Py::str("l"))?)?;
                        }
                    }
                }
            }
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1329 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    return Ok(py::minmax(&[f__rel(py::method(&v_outputs.clone(), "get", &[Py::str("d")])?, v_d.clone())?, f__rel(py::method(&v_outputs.clone(), "get", &[Py::str("d_nm")])?, py::mul(&v_d.clone(), &Py::Float(1000000000.0))?)?], true)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1347(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1349: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1348 = v_inputs.clone(); f_debye_heat_capacity(py::getitem(&t1348, &Py::str("theta_D"))?, py::getitem(&t1348, &Py::str("T"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1349 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1350 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1370 = (t1350).items()?;
        if t1370.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1370[0].clone();
        v_v = t1370[1].clone();
        if ({ let t1351 = v_k.clone(); Py::Bool({ let t1352 = v_outputs.clone(); if !py::contains(&t1352, &t1351)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1407(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1409: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1408 = v_inputs.clone(); f_degree_of_polymerization(py::getitem(&t1408, &Py::str("formula"))?, py::getitem(&t1408, &Py::str("Mn"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1409 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1410 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1430 = (t1410).items()?;
        if t1430.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1430[0].clone();
        v_v = t1430[1].clone();
        if ({ let t1411 = v_k.clone(); Py::Bool({ let t1412 = v_outputs.clone(); if !py::contains(&t1412, &t1411)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_142(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t144: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t143 = v_inputs.clone(); f_angle_directions_cubic(py::getitem(&t143, &Py::str("u1"))?, py::getitem(&t143, &Py::str("v1"))?, py::getitem(&t143, &Py::str("w1"))?, py::getitem(&t143, &Py::str("u2"))?, py::getitem(&t143, &Py::str("v2"))?, py::getitem(&t143, &Py::str("w2"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t144 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t145 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t165 = (t145).items()?;
        if t165.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t165[0].clone();
        v_v = t165[1].clone();
        if ({ let t146 = v_k.clone(); Py::Bool({ let t147 = v_outputs.clone(); if !py::contains(&t147, &t146)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1470(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1472: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1471 = v_inputs.clone(); f_arrhenius_fit(py::getitem(&t1471, &Py::str("T"))?, py::getitem(&t1471, &Py::str("D"))?, py::method(&t1471, "get", &[Py::str("T_target"), Py::None])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1472 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1473 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1493 = (t1473).items()?;
        if t1493.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1493[0].clone();
        v_v = t1493[1].clone();
        if ({ let t1474 = v_k.clone(); Py::Bool({ let t1475 = v_outputs.clone(); if !py::contains(&t1475, &t1474)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1521(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1523: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1522 = v_inputs.clone(); f_diffusion_depth(py::getitem(&t1522, &Py::str("C0"))?, py::getitem(&t1522, &Py::str("Cs"))?, py::getitem(&t1522, &Py::str("Cx"))?, py::getitem(&t1522, &Py::str("D"))?, py::getitem(&t1522, &Py::str("t"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1523 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1524 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1544 = (t1524).items()?;
        if t1544.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1544[0].clone();
        v_v = t1544[1].clone();
        if ({ let t1525 = v_k.clone(); Py::Bool({ let t1526 = v_outputs.clone(); if !py::contains(&t1526, &t1525)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1572(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1574: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1573 = v_inputs.clone(); f_diffusion_time(py::getitem(&t1573, &Py::str("C0"))?, py::getitem(&t1573, &Py::str("Cs"))?, py::getitem(&t1573, &Py::str("Cx"))?, py::getitem(&t1573, &Py::str("x"))?, py::getitem(&t1573, &Py::str("D"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1574 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1575 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1595 = (t1575).items()?;
        if t1595.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1595[0].clone();
        v_v = t1595[1].clone();
        if ({ let t1576 = v_k.clone(); Py::Bool({ let t1577 = v_outputs.clone(); if !py::contains(&t1577, &t1576)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1611(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1613: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1612 = v_inputs.clone(); f_einstein_heat_capacity(py::getitem(&t1612, &Py::str("theta_E"))?, py::getitem(&t1612, &Py::str("T"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1613 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1614 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1634 = (t1614).items()?;
        if t1634.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1634[0].clone();
        v_v = t1634[1].clone();
        if ({ let t1615 = v_k.clone(); Py::Bool({ let t1616 = v_outputs.clone(); if !py::contains(&t1616, &t1615)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1651(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1653: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1652 = v_inputs.clone(); f_elastic_deformation(py::getitem(&t1652, &Py::str("F"))?, py::getitem(&t1652, &Py::str("d0"))?, py::getitem(&t1652, &Py::str("l0"))?, py::getitem(&t1652, &Py::str("E"))?, py::getitem(&t1652, &Py::str("nu"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1653 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1654 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1674 = (t1654).items()?;
        if t1674.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1674[0].clone();
        v_v = t1674[1].clone();
        if ({ let t1655 = v_k.clone(); Py::Bool({ let t1656 = v_outputs.clone(); if !py::contains(&t1656, &t1655)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1701(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1703: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1702 = v_inputs.clone(); f_eutectic_constituents(py::getitem(&t1702, &Py::str("C0"))?, py::getitem(&t1702, &Py::str("C_alpha"))?, py::getitem(&t1702, &Py::str("C_E"))?, py::getitem(&t1702, &Py::str("C_beta"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1703 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1704 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1724 = (t1704).items()?;
        if t1724.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1724[0].clone();
        v_v = t1724[1].clone();
        if ({ let t1705 = v_k.clone(); Py::Bool({ let t1706 = v_outputs.clone(); if !py::contains(&t1706, &t1705)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1755(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1757: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1756 = v_inputs.clone(); f_extrinsic_carriers(py::getitem(&t1756, &Py::str("Nd"))?, py::getitem(&t1756, &Py::str("Na"))?, py::getitem(&t1756, &Py::str("ni"))?, py::method(&t1756, "get", &[Py::str("T"), Py::None])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1757 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1758 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1778 = (t1758).items()?;
        if t1778.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1778[0].clone();
        v_v = t1778[1].clone();
        if ({ let t1759 = v_k.clone(); Py::Bool({ let t1760 = v_outputs.clone(); if !py::contains(&t1760, &t1759)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1792(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1794: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1793 = v_inputs.clone(); f_fermi_energy(py::getitem(&t1793, &Py::str("rho"))?, py::getitem(&t1793, &Py::str("A"))?, py::getitem(&t1793, &Py::str("valence"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1794 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1795 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1815 = (t1795).items()?;
        if t1815.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1815[0].clone();
        v_v = t1815[1].clone();
        if ({ let t1796 = v_k.clone(); Py::Bool({ let t1797 = v_outputs.clone(); if !py::contains(&t1797, &t1796)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1831(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1833: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1832 = v_inputs.clone(); f_fick_first(py::getitem(&t1832, &Py::str("D"))?, py::getitem(&t1832, &Py::str("C1"))?, py::getitem(&t1832, &Py::str("C2"))?, py::getitem(&t1832, &Py::str("x1"))?, py::getitem(&t1832, &Py::str("x2"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1833 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1834 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1854 = (t1834).items()?;
        if t1854.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1854[0].clone();
        v_v = t1854[1].clone();
        if ({ let t1835 = v_k.clone(); Py::Bool({ let t1836 = v_outputs.clone(); if !py::contains(&t1836, &t1835)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1870(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1872: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1871 = v_inputs.clone(); f_fick_second(py::getitem(&t1871, &Py::str("C0"))?, py::getitem(&t1871, &Py::str("Cs"))?, py::getitem(&t1871, &Py::str("x"))?, py::getitem(&t1871, &Py::str("D"))?, py::getitem(&t1871, &Py::str("t"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1872 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1873 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1893 = (t1873).items()?;
        if t1893.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1893[0].clone();
        v_v = t1893[1].clone();
        if ({ let t1874 = v_k.clone(); Py::Bool({ let t1875 = v_outputs.clone(); if !py::contains(&t1875, &t1874)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1907(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1909: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1908 = v_inputs.clone(); f_fracture_stress(py::getitem(&t1908, &Py::str("K_IC"))?, py::getitem(&t1908, &Py::str("a"))?, py::method(&t1908, "get", &[Py::str("Y"), Py::Float(1.0)])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1909 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1910 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1930 = (t1910).items()?;
        if t1930.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1930[0].clone();
        v_v = t1930[1].clone();
        if ({ let t1911 = v_k.clone(); Py::Bool({ let t1912 = v_outputs.clone(); if !py::contains(&t1912, &t1911)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1941(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1943: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1942 = v_inputs.clone(); f_gibbs_phase_rule(py::getitem(&t1942, &Py::str("C"))?, py::getitem(&t1942, &Py::str("P"))?, py::method(&t1942, "get", &[Py::str("pressure"), Py::str("variable")])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1943 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1944 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1964 = (t1944).items()?;
        if t1964.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1964[0].clone();
        v_v = t1964[1].clone();
        if ({ let t1945 = v_k.clone(); Py::Bool({ let t1946 = v_outputs.clone(); if !py::contains(&t1946, &t1945)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1978(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1980: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1979 = v_inputs.clone(); f_griffith_stress(py::getitem(&t1979, &Py::str("E"))?, py::getitem(&t1979, &Py::str("gamma_s"))?, py::getitem(&t1979, &Py::str("a"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1980 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1981 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2001 = (t1981).items()?;
        if t2001.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2001[0].clone();
        v_v = t2001[1].clone();
        if ({ let t1982 = v_k.clone(); Py::Bool({ let t1983 = v_outputs.clone(); if !py::contains(&t1983, &t1982)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2037(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2039: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2038 = v_inputs.clone(); f_hall_petch_fit(py::getitem(&t2038, &Py::str("d_um"))?, py::getitem(&t2038, &Py::str("yield_MPa"))?, py::method(&t2038, "get", &[Py::str("d_target_um"), Py::None])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2039 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2040 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2060 = (t2040).items()?;
        if t2060.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2060[0].clone();
        v_v = t2060[1].clone();
        if ({ let t2041 = v_k.clone(); Py::Bool({ let t2042 = v_outputs.clone(); if !py::contains(&t2042, &t2041)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2074(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2076: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2075 = v_inputs.clone(); f_hcp_cell(py::getitem(&t2075, &Py::str("a"))?, py::getitem(&t2075, &Py::str("c"))?, py::getitem(&t2075, &Py::str("A"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2076 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2077 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2097 = (t2077).items()?;
        if t2097.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2097[0].clone();
        v_v = t2097[1].clone();
        if ({ let t2078 = v_k.clone(); Py::Bool({ let t2079 = v_outputs.clone(); if !py::contains(&t2079, &t2078)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_209(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t211: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t210 = v_inputs.clone(); f_angle_planes_general(py::getitem(&t210, &Py::str("a"))?, py::getitem(&t210, &Py::str("b"))?, py::getitem(&t210, &Py::str("c"))?, py::getitem(&t210, &Py::str("alpha"))?, py::getitem(&t210, &Py::str("beta"))?, py::getitem(&t210, &Py::str("gamma"))?, py::getitem(&t210, &Py::str("h1"))?, py::getitem(&t210, &Py::str("k1"))?, py::getitem(&t210, &Py::str("l1"))?, py::getitem(&t210, &Py::str("h2"))?, py::getitem(&t210, &Py::str("k2"))?, py::getitem(&t210, &Py::str("l2"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t211 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t212 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t232 = (t212).items()?;
        if t232.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t232[0].clone();
        v_v = t232[1].clone();
        if ({ let t213 = v_k.clone(); Py::Bool({ let t214 = v_outputs.clone(); if !py::contains(&t214, &t213)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2114(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2116: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2115 = v_inputs.clone(); f_nucleation(py::getitem(&t2115, &Py::str("gamma"))?, py::getitem(&t2115, &Py::str("dHf"))?, py::getitem(&t2115, &Py::str("Tm"))?, py::getitem(&t2115, &Py::str("T"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2116 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2117 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2137 = (t2117).items()?;
        if t2137.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2137[0].clone();
        v_v = t2137[1].clone();
        if ({ let t2118 = v_k.clone(); Py::Bool({ let t2119 = v_outputs.clone(); if !py::contains(&t2119, &t2118)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2151(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2153: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2152 = v_inputs.clone(); f_interstitial_site(py::getitem(&t2152, &Py::str("structure"))?, py::getitem(&t2152, &Py::str("site"))?, py::getitem(&t2152, &Py::str("R"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2153 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2154 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2174 = (t2154).items()?;
        if t2174.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2174[0].clone();
        v_v = t2174[1].clone();
        if ({ let t2155 = v_k.clone(); Py::Bool({ let t2156 = v_outputs.clone(); if !py::contains(&t2156, &t2155)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2189(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2191: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2190 = v_inputs.clone(); f_intrinsic_carriers(py::getitem(&t2190, &Py::str("Eg"))?, py::getitem(&t2190, &Py::str("T"))?, py::method(&t2190, "get", &[Py::str("me"), Py::Float(1.0)])?, py::method(&t2190, "get", &[Py::str("mh"), Py::Float(1.0)])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2191 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2192 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2212 = (t2192).items()?;
        if t2212.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2212[0].clone();
        v_v = t2212[1].clone();
        if ({ let t2193 = v_k.clone(); Py::Bool({ let t2194 = v_outputs.clone(); if !py::contains(&t2194, &t2193)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2229(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2231: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2230 = v_inputs.clone(); f_iron_carbon(py::getitem(&t2230, &Py::str("C0"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2231 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2232 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2252 = (t2232).items()?;
        if t2252.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2252[0].clone();
        v_v = t2252[1].clone();
        if ({ let t2233 = v_k.clone(); Py::Bool({ let t2234 = v_outputs.clone(); if !py::contains(&t2234, &t2233)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2269(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2271: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2270 = v_inputs.clone(); f_elastic_constants(py::getitem(&t2270, &Py::str("E"))?, py::getitem(&t2270, &Py::str("nu"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2271 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2272 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2292 = (t2272).items()?;
        if t2292.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2292[0].clone();
        v_v = t2292[1].clone();
        if ({ let t2273 = v_k.clone(); Py::Bool({ let t2274 = v_outputs.clone(); if !py::contains(&t2274, &t2273)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2308(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2310: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2309 = v_inputs.clone(); f_kelvin_voigt_creep(py::getitem(&t2309, &Py::str("sigma0"))?, py::getitem(&t2309, &Py::str("E"))?, py::getitem(&t2309, &Py::str("eta"))?, py::getitem(&t2309, &Py::str("t"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2310 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2311 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2331 = (t2311).items()?;
        if t2331.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2331[0].clone();
        v_v = t2331[1].clone();
        if ({ let t2312 = v_k.clone(); Py::Bool({ let t2313 = v_outputs.clone(); if !py::contains(&t2313, &t2312)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2355(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2357: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2356 = v_inputs.clone(); f_larson_miller(py::getitem(&t2356, &Py::str("T"))?, py::getitem(&t2356, &Py::str("t_r"))?, py::method(&t2356, "get", &[Py::str("C"), Py::Float(20.0)])?, py::method(&t2356, "get", &[Py::str("T2"), Py::None])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2357 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2358 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2378 = (t2358).items()?;
        if t2378.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2378[0].clone();
        v_v = t2378[1].clone();
        if ({ let t2359 = v_k.clone(); Py::Bool({ let t2360 = v_outputs.clone(); if !py::contains(&t2360, &t2359)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2394(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2396: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2395 = v_inputs.clone(); f_lattice_from_density(py::getitem(&t2395, &Py::str("structure"))?, py::getitem(&t2395, &Py::str("rho"))?, py::getitem(&t2395, &Py::str("A"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2396 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2397 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2417 = (t2397).items()?;
        if t2417.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2417[0].clone();
        v_v = t2417[1].clone();
        if ({ let t2398 = v_k.clone(); Py::Bool({ let t2399 = v_outputs.clone(); if !py::contains(&t2399, &t2398)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2447(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2449: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2448 = v_inputs.clone(); f_lever_rule(py::getitem(&t2448, &Py::str("C0"))?, py::getitem(&t2448, &Py::str("C1"))?, py::getitem(&t2448, &Py::str("C2"))?, py::method(&t2448, "get", &[Py::str("mass"), Py::None])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2449 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2450 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2470 = (t2450).items()?;
        if t2470.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2470[0].clone();
        v_v = t2470[1].clone();
        if ({ let t2451 = v_k.clone(); Py::Bool({ let t2452 = v_outputs.clone(); if !py::contains(&t2452, &t2451)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_246(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t248: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t247 = v_inputs.clone(); f_arrhenius_diffusivity(py::getitem(&t247, &Py::str("D0"))?, py::getitem(&t247, &Py::str("Q"))?, py::getitem(&t247, &Py::str("T"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t248 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t249 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t269 = (t249).items()?;
        if t269.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t269[0].clone();
        v_v = t269[1].clone();
        if ({ let t250 = v_k.clone(); Py::Bool({ let t251 = v_outputs.clone(); if !py::contains(&t251, &t250)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2508(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2510: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2509 = v_inputs.clone(); f_linear_density(py::getitem(&t2509, &Py::str("structure"))?, py::getitem(&t2509, &Py::str("u"))?, py::getitem(&t2509, &Py::str("v"))?, py::getitem(&t2509, &Py::str("w"))?, py::getitem(&t2509, &Py::str("a"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2510 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2511 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2531 = (t2511).items()?;
        if t2531.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2531[0].clone();
        v_v = t2531[1].clone();
        if ({ let t2512 = v_k.clone(); Py::Bool({ let t2513 = v_outputs.clone(); if !py::contains(&t2513, &t2512)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2547(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2549: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2548 = v_inputs.clone(); f_linear_expansion(py::getitem(&t2548, &Py::str("alpha"))?, py::getitem(&t2548, &Py::str("L0"))?, py::getitem(&t2548, &Py::str("dT"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2549 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2550 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2570 = (t2550).items()?;
        if t2570.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2570[0].clone();
        v_v = t2570[1].clone();
        if ({ let t2551 = v_k.clone(); Py::Bool({ let t2552 = v_outputs.clone(); if !py::contains(&t2552, &t2551)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2608(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2610: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2609 = v_inputs.clone(); f_max_schmid(py::getitem(&t2609, &Py::str("structure"))?, py::getitem(&t2609, &Py::str("load"))?, py::method(&t2609, "get", &[Py::str("tau_crss"), Py::None])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2610 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2611 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2631 = (t2611).items()?;
        if t2631.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2631[0].clone();
        v_v = t2631[1].clone();
        if ({ let t2612 = v_k.clone(); Py::Bool({ let t2613 = v_outputs.clone(); if !py::contains(&t2613, &t2612)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2647(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2649: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2648 = v_inputs.clone(); f_maxwell_relaxation(py::getitem(&t2648, &Py::str("sigma0"))?, py::getitem(&t2648, &Py::str("E"))?, py::getitem(&t2648, &Py::str("eta"))?, py::getitem(&t2648, &Py::str("t"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2649 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2650 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2670 = (t2650).items()?;
        if t2670.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2670[0].clone();
        v_v = t2670[1].clone();
        if ({ let t2651 = v_k.clone(); Py::Bool({ let t2652 = v_outputs.clone(); if !py::contains(&t2652, &t2651)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2684(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f__check_uvtw(v_i.clone(), v_o.clone(), Py::Bool(true))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2711(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f__check_uvtw(v_i.clone(), v_o.clone(), Py::Bool(false))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2774(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2776: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2775 = v_inputs.clone(); f_resilience(py::getitem(&t2775, &Py::str("sy"))?, py::getitem(&t2775, &Py::str("E"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2776 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2777 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2797 = (t2777).items()?;
        if t2797.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2797[0].clone();
        v_v = t2797[1].clone();
        if ({ let t2778 = v_k.clone(); Py::Bool({ let t2779 = v_outputs.clone(); if !py::contains(&t2779, &t2778)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_280(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t282: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t281 = v_inputs.clone(); f_packing_factor(py::getitem(&t281, &Py::str("structure"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t282 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t283 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t303 = (t283).items()?;
        if t303.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t303[0].clone();
        v_v = t303[1].clone();
        if ({ let t284 = v_k.clone(); Py::Bool({ let t285 = v_outputs.clone(); if !py::contains(&t285, &t284)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2831(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2833: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2832 = v_inputs.clone(); f_mw_averages(py::getitem(&t2832, &Py::str("M"))?, py::getitem(&t2832, &Py::str("fraction"))?, py::method(&t2832, "get", &[Py::str("basis"), Py::str("number")])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2833 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2834 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2854 = (t2834).items()?;
        if t2854.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2854[0].clone();
        v_v = t2854[1].clone();
        if ({ let t2835 = v_k.clone(); Py::Bool({ let t2836 = v_outputs.clone(); if !py::contains(&t2836, &t2835)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2888(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2890: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2889 = v_inputs.clone(); f_offset_yield(py::getitem(&t2889, &Py::str("strain"))?, py::getitem(&t2889, &Py::str("stress"))?, py::getitem(&t2889, &Py::str("E"))?, py::method(&t2889, "get", &[Py::str("offset"), Py::Float(0.002)])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2890 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2891 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2911 = (t2891).items()?;
        if t2911.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2911[0].clone();
        v_v = t2911[1].clone();
        if ({ let t2892 = v_k.clone(); Py::Bool({ let t2893 = v_outputs.clone(); if !py::contains(&t2893, &t2892)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2947(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2949: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2948 = v_inputs.clone(); f_paris_life(py::getitem(&t2948, &Py::str("C"))?, py::getitem(&t2948, &Py::str("m"))?, py::getitem(&t2948, &Py::str("dsigma"))?, py::getitem(&t2948, &Py::str("a0"))?, py::method(&t2948, "get", &[Py::str("Y"), Py::Float(1.0)])?, py::method(&t2948, "get", &[Py::str("af"), Py::None])?, py::method(&t2948, "get", &[Py::str("K_IC"), Py::None])?, py::method(&t2948, "get", &[Py::str("sigma_max"), Py::None])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2949 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2950 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2970 = (t2950).items()?;
        if t2970.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2970[0].clone();
        v_v = t2970[1].clone();
        if ({ let t2951 = v_k.clone(); Py::Bool({ let t2952 = v_outputs.clone(); if !py::contains(&t2952, &t2951)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3005(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3007: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3006 = v_inputs.clone(); f_planar_density(py::getitem(&t3006, &Py::str("structure"))?, py::getitem(&t3006, &Py::str("h"))?, py::getitem(&t3006, &Py::str("k"))?, py::getitem(&t3006, &Py::str("l"))?, py::getitem(&t3006, &Py::str("a"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3007 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3008 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3028 = (t3008).items()?;
        if t3028.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3028[0].clone();
        v_v = t3028[1].clone();
        if ({ let t3009 = v_k.clone(); Py::Bool({ let t3010 = v_outputs.clone(); if !py::contains(&t3010, &t3009)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3052(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3054: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3053 = v_inputs.clone(); f_built_in_potential(py::getitem(&t3053, &Py::str("Na"))?, py::getitem(&t3053, &Py::str("Nd"))?, py::getitem(&t3053, &Py::str("ni"))?, py::getitem(&t3053, &Py::str("T"))?, py::method(&t3053, "get", &[Py::str("eps_r"), Py::None])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3054 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3055 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3075 = (t3055).items()?;
        if t3075.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3075[0].clone();
        v_v = t3075[1].clone();
        if ({ let t3056 = v_k.clone(); Py::Bool({ let t3057 = v_outputs.clone(); if !py::contains(&t3057, &t3056)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3098(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3100: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3099 = v_inputs.clone(); f_chain_dimensions(py::getitem(&t3099, &Py::str("n"))?, py::getitem(&t3099, &Py::str("l"))?, py::method(&t3099, "get", &[Py::str("theta"), Py::Float(109.47)])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3100 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3101 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3121 = (t3101).items()?;
        if t3121.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3121[0].clone();
        v_v = t3121[1].clone();
        if ({ let t3102 = v_k.clone(); Py::Bool({ let t3103 = v_outputs.clone(); if !py::contains(&t3103, &t3102)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_31(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t33: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t32 = v_inputs.clone(); f_allowed_reflections(py::getitem(&t32, &Py::str("structure"))?, py::method(&t32, "get", &[Py::str("count"), Py::Int(6i64)])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t33 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t34 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t54 = (t34).items()?;
        if t54.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t54[0].clone();
        v_v = t54[1].clone();
        if ({ let t35 = v_k.clone(); Py::Bool({ let t36 = v_outputs.clone(); if !py::contains(&t36, &t35)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3138(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3140: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3139 = v_inputs.clone(); f_crystallinity(py::getitem(&t3139, &Py::str("rho_s"))?, py::getitem(&t3139, &Py::str("rho_c"))?, py::getitem(&t3139, &Py::str("rho_a"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3140 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3141 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3161 = (t3141).items()?;
        if t3161.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3161[0].clone();
        v_v = t3161[1].clone();
        if ({ let t3142 = v_k.clone(); Py::Bool({ let t3143 = v_outputs.clone(); if !py::contains(&t3143, &t3142)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3175(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3177: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3176 = v_inputs.clone(); f_polymorph_volume_change(py::getitem(&t3176, &Py::str("R_bcc"))?, py::getitem(&t3176, &Py::str("R_fcc"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3177 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3178 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3198 = (t3178).items()?;
        if t3198.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3198[0].clone();
        v_v = t3198[1].clone();
        if ({ let t3179 = v_k.clone(); Py::Bool({ let t3180 = v_outputs.clone(); if !py::contains(&t3180, &t3179)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_321(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t323: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t322 = v_inputs.clone(); f_at_to_wt(py::getitem(&t322, &Py::str("element1"))?, py::getitem(&t322, &Py::str("element2"))?, py::getitem(&t322, &Py::str("at1"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t323 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t324 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t344 = (t324).items()?;
        if t344.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t344[0].clone();
        v_v = t344[1].clone();
        if ({ let t325 = v_k.clone(); Py::Bool({ let t326 = v_outputs.clone(); if !py::contains(&t326, &t325)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3218(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3220: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3219 = v_inputs.clone(); f_radius_ratio(py::getitem(&t3219, &Py::str("rc"))?, py::getitem(&t3219, &Py::str("ra"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3220 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3221 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3241 = (t3221).items()?;
        if t3241.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3241[0].clone();
        v_v = t3241[1].clone();
        if ({ let t3222 = v_k.clone(); Py::Bool({ let t3223 = v_outputs.clone(); if !py::contains(&t3223, &t3222)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3255(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3257: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3256 = v_inputs.clone(); f_rubber_elasticity(py::getitem(&t3256, &Py::str("rho"))?, py::getitem(&t3256, &Py::str("Mc"))?, py::getitem(&t3256, &Py::str("T"))?, py::getitem(&t3256, &Py::str("stretch"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3257 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3258 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3278 = (t3258).items()?;
        if t3278.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3278[0].clone();
        v_v = t3278[1].clone();
        if ({ let t3259 = v_k.clone(); Py::Bool({ let t3260 = v_outputs.clone(); if !py::contains(&t3260, &t3259)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3313(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3315: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3314 = v_inputs.clone(); f_schmid(py::getitem(&t3314, &Py::str("load"))?, py::getitem(&t3314, &Py::str("plane"))?, py::getitem(&t3314, &Py::str("direction"))?, py::method(&t3314, "get", &[Py::str("sigma"), Py::None])?, py::method(&t3314, "get", &[Py::str("tau_crss"), Py::None])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3315 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3316 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3336 = (t3316).items()?;
        if t3336.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3336[0].clone();
        v_v = t3336[1].clone();
        if ({ let t3317 = v_k.clone(); Py::Bool({ let t3318 = v_outputs.clone(); if !py::contains(&t3318, &t3317)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3342(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3344: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3343 = v_inputs.clone(); f_space_group(py::getitem(&t3343, &Py::str("structure"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3344 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3345 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3365 = (t3345).items()?;
        if t3365.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3365[0].clone();
        v_v = t3365[1].clone();
        if ({ let t3346 = v_k.clone(); Py::Bool({ let t3347 = v_outputs.clone(); if !py::contains(&t3347, &t3346)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3402(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3404: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3403 = v_inputs.clone(); f_hollomon_fit(py::getitem(&t3403, &Py::str("true_strain"))?, py::getitem(&t3403, &Py::str("true_stress"))?, py::method(&t3403, "get", &[Py::str("strain_target"), Py::None])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3404 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3405 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3425 = (t3405).items()?;
        if t3425.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3425[0].clone();
        v_v = t3425[1].clone();
        if ({ let t3406 = v_k.clone(); Py::Bool({ let t3407 = v_outputs.clone(); if !py::contains(&t3407, &t3406)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3488(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3490: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3489 = v_inputs.clone(); f_tensile_toughness(py::getitem(&t3489, &Py::str("strain"))?, py::getitem(&t3489, &Py::str("stress"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3490 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3491 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3511 = (t3491).items()?;
        if t3511.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3511[0].clone();
        v_v = t3511[1].clone();
        if ({ let t3492 = v_k.clone(); Py::Bool({ let t3493 = v_outputs.clone(); if !py::contains(&t3493, &t3492)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3527(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3529: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3528 = v_inputs.clone(); f_theoretical_density(py::getitem(&t3528, &Py::str("structure"))?, py::getitem(&t3528, &Py::str("A"))?, py::getitem(&t3528, &Py::str("R"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3529 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3530 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3550 = (t3530).items()?;
        if t3550.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3550[0].clone();
        v_v = t3550[1].clone();
        if ({ let t3531 = v_k.clone(); Py::Bool({ let t3532 = v_outputs.clone(); if !py::contains(&t3532, &t3531)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3573(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3575: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3574 = v_inputs.clone(); f_thermal_stress(py::getitem(&t3574, &Py::str("E"))?, py::getitem(&t3574, &Py::str("alpha"))?, py::getitem(&t3574, &Py::str("dT"))?, py::method(&t3574, "get", &[Py::str("constraint"), Py::str("uniaxial")])?, py::method(&t3574, "get", &[Py::str("nu"), Py::None])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3575 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3576 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3596 = (t3576).items()?;
        if t3596.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3596[0].clone();
        v_v = t3596[1].clone();
        if ({ let t3577 = v_k.clone(); Py::Bool({ let t3578 = v_outputs.clone(); if !py::contains(&t3578, &t3577)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3607(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3609: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3608 = v_inputs.clone(); f_true_stress_strain(py::getitem(&t3608, &Py::str("stress"))?, py::getitem(&t3608, &Py::str("strain"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3609 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3610 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3630 = (t3610).items()?;
        if t3630.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3630[0].clone();
        v_v = t3630[1].clone();
        if ({ let t3611 = v_k.clone(); Py::Bool({ let t3612 = v_outputs.clone(); if !py::contains(&t3612, &t3611)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3653(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3655: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3654 = v_inputs.clone(); f_cell_volume(py::getitem(&t3654, &Py::str("a"))?, py::getitem(&t3654, &Py::str("b"))?, py::getitem(&t3654, &Py::str("c"))?, py::getitem(&t3654, &Py::str("alpha"))?, py::getitem(&t3654, &Py::str("beta"))?, py::getitem(&t3654, &Py::str("gamma"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3655 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3656 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3676 = (t3656).items()?;
        if t3676.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3676[0].clone();
        v_v = t3676[1].clone();
        if ({ let t3657 = v_k.clone(); Py::Bool({ let t3658 = v_outputs.clone(); if !py::contains(&t3658, &t3657)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3690(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3692: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3691 = v_inputs.clone(); f_vacancy_concentration(py::getitem(&t3691, &Py::str("rho"))?, py::getitem(&t3691, &Py::str("A"))?, py::getitem(&t3691, &Py::str("Qv"))?, py::getitem(&t3691, &Py::str("T"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3692 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3693 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3713 = (t3693).items()?;
        if t3713.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3713[0].clone();
        v_v = t3713[1].clone();
        if ({ let t3694 = v_k.clone(); Py::Bool({ let t3695 = v_outputs.clone(); if !py::contains(&t3695, &t3694)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3757(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3759: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3758 = v_inputs.clone(); f_vacancy_energy_fit(py::getitem(&t3758, &Py::str("T"))?, py::getitem(&t3758, &Py::str("fraction"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3759 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3760 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3780 = (t3760).items()?;
        if t3780.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3780[0].clone();
        v_v = t3780[1].clone();
        if ({ let t3761 = v_k.clone(); Py::Bool({ let t3762 = v_outputs.clone(); if !py::contains(&t3762, &t3761)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3796(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3798: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3797 = v_inputs.clone(); f_volumetric_expansion(py::getitem(&t3797, &Py::str("alpha"))?, py::getitem(&t3797, &Py::str("V0"))?, py::getitem(&t3797, &Py::str("dT"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3798 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3799 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3819 = (t3799).items()?;
        if t3819.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3819[0].clone();
        v_v = t3819[1].clone();
        if ({ let t3800 = v_k.clone(); Py::Bool({ let t3801 = v_outputs.clone(); if !py::contains(&t3801, &t3800)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3840(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3842: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3841 = v_inputs.clone(); f_weibull_stress(py::getitem(&t3841, &Py::str("P_failure"))?, py::getitem(&t3841, &Py::str("sigma0"))?, py::getitem(&t3841, &Py::str("m"))?, py::method(&t3841, "get", &[Py::str("V_ratio"), Py::Float(1.0)])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3842 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3843 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3863 = (t3843).items()?;
        if t3863.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3863[0].clone();
        v_v = t3863[1].clone();
        if ({ let t3844 = v_k.clone(); Py::Bool({ let t3845 = v_outputs.clone(); if !py::contains(&t3845, &t3844)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3877(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3879: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3878 = v_inputs.clone(); f_weibull_failure(py::getitem(&t3878, &Py::str("sigma"))?, py::getitem(&t3878, &Py::str("sigma0"))?, py::getitem(&t3878, &Py::str("m"))?, py::method(&t3878, "get", &[Py::str("V_ratio"), Py::Float(1.0)])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3879 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3880 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3900 = (t3880).items()?;
        if t3900.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3900[0].clone();
        v_v = t3900[1].clone();
        if ({ let t3881 = v_k.clone(); Py::Bool({ let t3882 = v_outputs.clone(); if !py::contains(&t3882, &t3881)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_391(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t393: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t392 = v_inputs.clone(); f_avrami_fit(py::getitem(&t392, &Py::str("t"))?, py::getitem(&t392, &Py::str("y"))?, py::method(&t392, "get", &[Py::str("y_target"), Py::None])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t393 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t394 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t414 = (t394).items()?;
        if t414.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t414[0].clone();
        v_v = t414[1].clone();
        if ({ let t395 = v_k.clone(); Py::Bool({ let t396 = v_outputs.clone(); if !py::contains(&t396, &t395)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3937(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3939: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3938 = v_inputs.clone(); f_weibull_fit(py::getitem(&t3938, &Py::str("strengths"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3939 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3940 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3960 = (t3940).items()?;
        if t3960.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3960[0].clone();
        v_v = t3960[1].clone();
        if ({ let t3941 = v_k.clone(); Py::Bool({ let t3942 = v_outputs.clone(); if !py::contains(&t3942, &t3941)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3978(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3980: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3979 = v_inputs.clone(); f_wt_to_at(py::getitem(&t3979, &Py::str("element1"))?, py::getitem(&t3979, &Py::str("element2"))?, py::getitem(&t3979, &Py::str("wt1"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3980 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3981 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4001 = (t3981).items()?;
        if t4001.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4001[0].clone();
        v_v = t4001[1].clone();
        if ({ let t3982 = v_k.clone(); Py::Bool({ let t3983 = v_outputs.clone(); if !py::contains(&t3983, &t3982)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4025(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4027: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4026 = v_inputs.clone(); f_wiedemann_franz(py::getitem(&t4026, &Py::str("T"))?, py::method(&t4026, "get", &[Py::str("sigma"), Py::None])?, py::method(&t4026, "get", &[Py::str("rho"), Py::None])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4027 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4028 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4048 = (t4028).items()?;
        if t4048.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4048[0].clone();
        v_v = t4048[1].clone();
        if ({ let t4029 = v_k.clone(); Py::Bool({ let t4030 = v_outputs.clone(); if !py::contains(&t4030, &t4029)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4079(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4081: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4080 = v_inputs.clone(); f_youngs_modulus_fit(py::getitem(&t4080, &Py::str("strain"))?, py::getitem(&t4080, &Py::str("stress"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4081 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4082 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4102 = (t4082).items()?;
        if t4102.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4102[0].clone();
        v_v = t4102[1].clone();
        if ({ let t4083 = v_k.clone(); Py::Bool({ let t4084 = v_outputs.clone(); if !py::contains(&t4084, &t4083)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_450(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t452: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t451 = v_inputs.clone(); f_band_gap_fit(py::getitem(&t451, &Py::str("T"))?, py::getitem(&t451, &Py::str("sigma"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t452 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t453 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t473 = (t453).items()?;
        if t473.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t473[0].clone();
        v_v = t473[1].clone();
        if ({ let t454 = v_k.clone(); Py::Bool({ let t455 = v_outputs.clone(); if !py::contains(&t455, &t454)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_491(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t493: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t492 = v_inputs.clone(); f_bragg_angle(py::getitem(&t492, &Py::str("d"))?, py::getitem(&t492, &Py::str("lam"))?, py::method(&t492, "get", &[Py::str("n"), Py::Int(1i64)])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t493 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t494 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t514 = (t494).items()?;
        if t514.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t514[0].clone();
        v_v = t514[1].clone();
        if ({ let t495 = v_k.clone(); Py::Bool({ let t496 = v_outputs.clone(); if !py::contains(&t496, &t495)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_533(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t535: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t534 = v_inputs.clone(); f_bragg_spacing(py::getitem(&t534, &Py::str("lam"))?, py::getitem(&t534, &Py::str("theta"))?, py::method(&t534, "get", &[Py::str("n"), Py::Int(1i64)])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t535 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t536 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t556 = (t536).items()?;
        if t556.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t556[0].clone();
        v_v = t556[1].clone();
        if ({ let t537 = v_k.clone(); Py::Bool({ let t538 = v_outputs.clone(); if !py::contains(&t538, &t537)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_575(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t577: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t576 = v_inputs.clone(); f_bragg_wavelength(py::getitem(&t576, &Py::str("d"))?, py::getitem(&t576, &Py::str("theta"))?, py::method(&t576, "get", &[Py::str("n"), Py::Int(1i64)])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t577 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t578 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t598 = (t578).items()?;
        if t598.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t598[0].clone();
        v_v = t598[1].clone();
        if ({ let t579 = v_k.clone(); Py::Bool({ let t580 = v_outputs.clone(); if !py::contains(&t580, &t579)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_610(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t612: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t611 = v_inputs.clone(); f_burgers_vector(py::getitem(&t611, &Py::str("structure"))?, py::getitem(&t611, &Py::str("a"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t612 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t613 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t633 = (t613).items()?;
        if t633.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t633[0].clone();
        v_v = t633[1].clone();
        if ({ let t614 = v_k.clone(); Py::Bool({ let t615 = v_outputs.clone(); if !py::contains(&t615, &t614)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_648(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t650: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t649 = v_inputs.clone(); f_ceramic_density(py::getitem(&t649, &Py::str("prototype"))?, py::getitem(&t649, &Py::str("rc"))?, py::getitem(&t649, &Py::str("ra"))?, py::getitem(&t649, &Py::str("Ac"))?, py::getitem(&t649, &Py::str("Aa"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t650 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t651 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t671 = (t651).items()?;
        if t671.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t671[0].clone();
        v_v = t671[1].clone();
        if ({ let t652 = v_k.clone(); Py::Bool({ let t653 = v_outputs.clone(); if !py::contains(&t653, &t652)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_692(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t694: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t693 = v_inputs.clone(); f_rule_of_mixtures(py::getitem(&t693, &Py::str("Ef"))?, py::getitem(&t693, &Py::str("Em"))?, py::getitem(&t693, &Py::str("Vf"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t694 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t695 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t715 = (t695).items()?;
        if t715.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t715[0].clone();
        v_v = t715[1].clone();
        if ({ let t696 = v_k.clone(); Py::Bool({ let t697 = v_outputs.clone(); if !py::contains(&t697, &t696)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_760(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t762: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t761 = v_inputs.clone(); f_copolymer_dp(py::getitem(&t761, &Py::str("formula1"))?, py::getitem(&t761, &Py::str("formula2"))?, py::getitem(&t761, &Py::str("f1"))?, py::getitem(&t761, &Py::str("Mn"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t762 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t763 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t783 = (t763).items()?;
        if t783.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t783[0].clone();
        v_v = t783[1].clone();
        if ({ let t764 = v_k.clone(); Py::Bool({ let t765 = v_outputs.clone(); if !py::contains(&t765, &t764)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_824(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t826: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t825 = v_inputs.clone(); f_creep_fit(py::getitem(&t825, &Py::str("stress_MPa"))?, py::getitem(&t825, &Py::str("T"))?, py::getitem(&t825, &Py::str("rate"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t826 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t827 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t847 = (t827).items()?;
        if t847.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t847[0].clone();
        v_v = t847[1].clone();
        if ({ let t828 = v_k.clone(); Py::Bool({ let t829 = v_outputs.clone(); if !py::contains(&t829, &t828)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_861(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t863: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t862 = v_inputs.clone(); f_critical_crack(py::getitem(&t862, &Py::str("K_IC"))?, py::getitem(&t862, &Py::str("sigma"))?, py::method(&t862, "get", &[Py::str("Y"), Py::Float(1.0)])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t863 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t864 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t884 = (t864).items()?;
        if t884.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t884[0].clone();
        v_v = t884[1].clone();
        if ({ let t865 = v_k.clone(); Py::Bool({ let t866 = v_outputs.clone(); if !py::contains(&t866, &t865)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_898(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t900: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t899 = v_inputs.clone(); f_critical_fiber_length(py::getitem(&t899, &Py::str("sigma_f"))?, py::getitem(&t899, &Py::str("d"))?, py::getitem(&t899, &Py::str("tau_c"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t900 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t901 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t921 = (t901).items()?;
        if t921.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t921[0].clone();
        v_v = t921[1].clone();
        if ({ let t902 = v_k.clone(); Py::Bool({ let t903 = v_outputs.clone(); if !py::contains(&t903, &t902)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_935(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t937: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t936 = v_inputs.clone(); f_cubic_lattice_parameter(py::getitem(&t936, &Py::str("structure"))?, py::getitem(&t936, &Py::str("R"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t937 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t938 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t958 = (t938).items()?;
        if t958.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t958[0].clone();
        v_v = t958[1].clone();
        if ({ let t939 = v_k.clone(); Py::Bool({ let t940 = v_outputs.clone(); if !py::contains(&t940, &t939)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_94(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t96: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t95 = v_inputs.clone(); f_angle_directions_general(py::getitem(&t95, &Py::str("a"))?, py::getitem(&t95, &Py::str("b"))?, py::getitem(&t95, &Py::str("c"))?, py::getitem(&t95, &Py::str("alpha"))?, py::getitem(&t95, &Py::str("beta"))?, py::getitem(&t95, &Py::str("gamma"))?, py::getitem(&t95, &Py::str("u1"))?, py::getitem(&t95, &Py::str("v1"))?, py::getitem(&t95, &Py::str("w1"))?, py::getitem(&t95, &Py::str("u2"))?, py::getitem(&t95, &Py::str("v2"))?, py::getitem(&t95, &Py::str("w2"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t96 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t97 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t117 = (t97).items()?;
        if t117.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t117[0].clone();
        v_v = t117[1].clone();
        if ({ let t98 = v_k.clone(); Py::Bool({ let t99 = v_outputs.clone(); if !py::contains(&t99, &t98)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_984(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t986: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t985 = v_inputs.clone(); f_cubic_reflection(py::getitem(&t985, &Py::str("a"))?, py::getitem(&t985, &Py::str("h"))?, py::getitem(&t985, &Py::str("k"))?, py::getitem(&t985, &Py::str("l"))?, py::getitem(&t985, &Py::str("lam"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t986 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t987 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1007 = (t987).items()?;
        if t1007.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1007[0].clone();
        v_v = t1007[1].clone();
        if ({ let t988 = v_k.clone(); Py::Bool({ let t989 = v_outputs.clone(); if !py::contains(&t989, &t988)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}
