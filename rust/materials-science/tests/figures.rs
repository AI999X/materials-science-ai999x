//! Every Materials Science capability's own live figures (src/figures*.rs): its worked example gives each published
//! figure kind, a changed input changes the drawing, and no drawing holds a non-finite number or text outside its
//! canvas.

use ax_engine::figures::{figures, Figure};
use ax_engine::Input;
use materials_science_ai999x::pack;

/// The published figure kinds of every capability (publish/website/models/materials-science-ai999x/figures, except
/// the generic "curve"), as tools/figure_parity.py lists them.
const KINDS: &[(&str, &[&str])] = &[
    ("allowed_reflections_cubic", &["series"]),
    ("atomic_packing_factor", &["cell"]),
    ("avrami_kinetics", &["sensitivity"]),
    ("band_gap_from_conductivity", &["sensitivity"]),
    ("bragg_angle", &["geometry"]),
    ("bragg_spacing", &["geometry"]),
    ("bragg_wavelength", &["field"]),
    ("burgers_vector", &["cell"]),
    ("copolymer_degree_of_polymerization", &["field"]),
    ("creep_parameters_fit", &["io"]),
    ("cubic_lattice_parameter", &["cell"]),
    ("cubic_reflection_angle", &["geometry", "pattern"]),
    ("diffusion_arrhenius_fit", &["sensitivity"]),
    ("einstein_heat_capacity", &["field"]),
    ("eutectic_microconstituents", &["lever"]),
    ("extrinsic_carrier_concentration", &["decision"]),
    ("fick_second_law_concentration", &["profile", "profile-2"]),
    ("gibbs_phase_rule", &["table"]),
    ("hall_petch", &["fit"]),
    ("interstitial_site_size", &["cell"]),
    ("iron_carbon_microconstituents", &["bars", "decision"]),
    ("isotropic_elastic_constants", &["field"]),
    ("larson_miller_parameter", &["field"]),
    ("lattice_parameter_from_density", &["cell"]),
    ("lever_rule", &["lever"]),
    ("linear_density", &["cell"]),
    ("max_schmid_factor", &["data"]),
    ("miller_bravais_direction", &["cell"]),
    ("miller_indices_from_intercepts", &["cell"]),
    ("modulus_of_resilience", &["field"]),
    ("molecular_weight_averages", &["xy"]),
    ("offset_yield_strength", &["stress-strain"]),
    ("planar_density", &["cell"]),
    ("polymer_chain_dimensions", &["field"]),
    ("radius_ratio_coordination", &["cn-vs-rc", "decision"]),
    ("schmid_factor", &["sensitivity"]),
    ("space_group", &["cell"]),
    ("strain_hardening_exponent", &["fit"]),
    ("structure_factor_cubic", &["cell", "decision"]),
    ("tensile_toughness", &["stress-strain"]),
    ("theoretical_density", &["cell"]),
    ("vacancy_formation_energy_fit", &["sensitivity"]),
    ("weibull_modulus_fit", &["sensitivity"]),
    ("wiedemann_franz", &["field"]),
    ("youngs_modulus_from_data", &["stress-strain"]),
];

/// Inputs to change for a capability whose inputs are not plain quantities (a second, different example).
const OTHER: &[(&str, &str, &str)] = &[
    ("allowed_reflections_cubic", "structure", "FCC"),
    ("atomic_packing_factor", "structure", "HCP"),
    ("atomic_packing_factor", "structure", "BCC"),
    ("burgers_vector", "structure", "BCC"),
    ("cubic_lattice_parameter", "structure", "diamond"),
    ("interstitial_site_size", "site", "tetrahedral"),
    ("lattice_parameter_from_density", "structure", "BCC"),
    ("linear_density", "u", "1"),
    ("linear_density", "w", "1"),
    ("planar_density", "k", "1"),
    ("space_group", "structure", "rocksalt"),
    ("space_group", "structure", "fluorite"),
    ("structure_factor_cubic", "l", "0"),
    ("theoretical_density", "structure", "BCC"),
    ("miller_bravais_direction", "V", "1"),
    ("miller_indices_from_intercepts", "z", "1"),
    ("cubic_reflection_angle", "h", "2"),
    ("gibbs_phase_rule", "C", "2"),
    ("max_schmid_factor", "load", "[1, 2, 3]"),
    ("molecular_weight_averages", "basis", "weight"),
    ("iron_carbon_microconstituents", "C0", "1.2"),
    ("eutectic_microconstituents", "C0", "80"),
    ("polymer_chain_dimensions", "n", "4"),
    ("hall_petch", "yield_MPa", "[160, 380]"),
    ("strain_hardening_exponent", "true_stress", "[400, 480]"),
    ("youngs_modulus_from_data", "stress", "[0, 150, 300]"),
    ("tensile_toughness", "stress", "[0, 300, 450, 470, 430]"),
    ("offset_yield_strength", "stress", "[0, 380, 500, 540, 560]"),
    ("schmid_factor", "load", "[0, 1, 1]"),
    ("weibull_modulus_fit", "strengths", "[300, 342, 365, 380, 395, 410, 450]"),
    ("creep_parameters_fit", "rate", "[1e-07, 1.8e-06, 2.5e-06]"),
    ("extrinsic_carrier_concentration", "Na", "3e+22 1/m^3"),
];

/// The capability refused by the model for these inputs gives no figure (capability, inputs).
const REFUSED: (&str, &[(&str, &str)]) = ("lever_rule", &[("C0", "50"), ("C1", "31.5"), ("C2", "42.5")]);



/// The arguments of a structured example: solve('key', a='1 V', b=8, c="(A·B)'").
fn example(key: &str) -> Vec<(String, Input)> {
    let p = pack();
    let ex = &p.procedure(key).expect(key).structured_example;
    let cs: Vec<char> = ex.chars().collect();
    let mut i = ex.find(',').map(|x| ex[..x].chars().count() + 1).unwrap_or(cs.len());
    let mut out = vec![];
    while i < cs.len() {
        while i < cs.len() && (cs[i] == ' ' || cs[i] == ',') {
            i += 1;
        }
        if i >= cs.len() || cs[i] == ')' {
            break;
        }
        let st = i;
        while cs[i] != '=' {
            i += 1;
        }
        let name: String = cs[st..i].iter().collect();
        i += 1;
        let value: String;
        if cs[i] == '\'' || cs[i] == '"' {
            let qch = cs[i];
            let s0 = i + 1;
            i += 1;
            while cs[i] != qch {
                i += 1;
            }
            value = cs[s0..i].iter().collect();
            i += 1;
        } else if cs[i] == '[' {
            let s0 = i;
            let mut depth = 0;
            loop {
                if cs[i] == '[' {
                    depth += 1;
                } else if cs[i] == ']' {
                    depth -= 1;
                    if depth == 0 {
                        break;
                    }
                }
                i += 1;
            }
            i += 1;
            value = cs[s0..i].iter().collect();
        } else {
            let s0 = i;
            while i < cs.len() && cs[i] != ',' && cs[i] != ')' {
                i += 1;
            }
            value = cs[s0..i].iter().collect();
        }
        out.push((name.trim().to_string(), Input::Text(value.trim().to_string())));
    }
    out
}

fn check_texts(f: &Figure) {
    if let Err(e) = ax_engine::figkit::check_figure(f) {
        panic!("{e}");
    }
}

#[test]
fn every_capability_draws_its_published_kinds() {
    let p = pack();
    let mut missing = vec![];
    for (key, kinds) in KINDS {
        let given = example(key);
        assert!(ax_engine::solve(p, key, &given, None).solved(), "{key}: the worked example is not solved");
        let figs = figures(p, key, &given, None, None);
        for kind in *kinds {
            match figs.iter().find(|f| f.kind == *kind) {
                Some(f) => {
                    assert_eq!(f.slug, format!("{}-{kind}", key.replace('_', "-")));
                    // an intercept may be written "inf" by the visitor (and repeated in the caption); a drawn number
                    // never is (check_texts looks for one in an attribute)
                    assert!(!f.svg.contains("NaN") && !f.svg.contains("=\"inf") && !f.svg.contains("inf\""),
                            "{key} {kind}: a non-finite number");
                    check_texts(f);
                }
                None => missing.push(format!("{key}: {kind}")),
            }
        }
        if let Some(i) = figs.iter().position(|f| f.slug.contains("-live-")) {
            assert!(figs[i..].iter().all(|f| f.slug.contains("-live-")), "{key}: generic figures before own ones");
        }
    }
    assert!(missing.is_empty(), "missing figures: {missing:?}");
}

/// The numbers of a text multiplied by f ("12 V" -> "16.44 V"; "[3, 6]" -> "[4.11, 8.22]"; "8e-05 m^3/s" ->
/// "0.0001096 m^3/s"): a number that follows a letter or "^" (a unit's power) is kept.
fn scale_text(t: &str, f: f64) -> Option<String> {
    let cs: Vec<char> = t.chars().collect();
    let mut out = String::new();
    let mut changed = false;
    let mut i = 0;
    while i < cs.len() {
        let prev = if i == 0 { ' ' } else { cs[i - 1] };
        let starts = cs[i].is_ascii_digit() || (cs[i] == '-' && i + 1 < cs.len() && cs[i + 1].is_ascii_digit());
        if starts && !(prev.is_alphanumeric() || prev == '^' || prev == '.' || prev == '_') {
            let s0 = i;
            i += 1;
            while i < cs.len() && (cs[i].is_ascii_digit() || cs[i] == '.') {
                i += 1;
            }
            if i + 1 < cs.len() && (cs[i] == 'e' || cs[i] == 'E')
                && (cs[i + 1].is_ascii_digit() || ((cs[i + 1] == '-' || cs[i + 1] == '+') && i + 2 < cs.len())) {
                i += 2;
                while i < cs.len() && cs[i].is_ascii_digit() {
                    i += 1;
                }
            }
            let tok: String = cs[s0..i].iter().collect();
            match tok.parse::<f64>() {
                Ok(x) if x != 0.0 => {
                    out.push_str(&format!("{}", x * f));
                    changed = true;
                }
                _ => out.push_str(&tok),
            }
        } else {
            out.push(cs[i]);
            i += 1;
        }
    }
    changed.then_some(out)
}

#[test]
fn a_changed_input_changes_the_drawing() {
    let p = pack();
    let mut unchanged = vec![];
    for (key, kinds) in KINDS {
        let given = example(key);
        let base = figures(p, key, &given, None, None);
        let mut tries: Vec<Vec<(String, Input)>> = vec![];
        for (_, sym, val) in OTHER.iter().filter(|o| o.0 == *key) {
            let mut g = given.clone();
            for e in g.iter_mut() {
                if e.0 == *sym {
                    e.1 = Input::Text(val.to_string());
                }
            }
            tries.push(g);
        }
        for f in [1.37, 0.73, 2.0] {
            for i in 0..given.len() {
                if let Input::Text(t) = &given[i].1 {
                    if let Some(t2) = scale_text(t, f) {
                        let mut g = given.clone();
                        g[i].1 = Input::Text(t2);
                        tries.push(g);
                    }
                }
            }
        }
        let mut ok = false;
        for g in tries {
            let a = ax_engine::solve(p, key, &g, None);
            if !a.solved() {
                continue;
            }
            let figs = figures(p, key, &g, None, None);
            let differs = kinds.iter().all(|kind| {
                match (base.iter().find(|f| f.kind == *kind), figs.iter().find(|f| f.kind == *kind)) {
                    (Some(a), Some(b)) => a.svg != b.svg,
                    _ => false,
                }
            });
            if differs {
                ok = true;
                for f in figs.iter().filter(|f| !f.slug.contains("-live-")) {
                    assert!(!f.svg.contains("NaN"), "{key}: a non-finite number");
                    check_texts(f);
                }
                break;
            }
        }
        if !ok {
            unchanged.push(*key);
        }
    }
    assert!(unchanged.is_empty(), "drawing did not follow a changed input: {unchanged:?}");
}

#[test]
fn a_refused_input_draws_nothing() {
    let p = pack();
    let (key, inputs) = REFUSED;
    let given: Vec<(String, Input)> = inputs.iter().map(|(k, v)| (k.to_string(), Input::Text(v.to_string()))).collect();
    let a = ax_engine::solve(p, key, &given, None);
    assert!(!a.solved(), "{key}: expected a refusal");
    assert!(figures(p, key, &given, None, None).is_empty());
}

/// Writes every worked example's own figures (and those of the OTHER inputs, prefixed "x-") as SVG files into
/// $FIG_DUMP, for a contact sheet: cargo test --test figures -- --ignored dump
#[test]
#[ignore]
fn dump() {
    let Ok(dir) = std::env::var("FIG_DUMP") else { return };
    let p = pack();
    for (key, _) in KINDS {
        let given = example(key);
        for f in figures(p, key, &given, None, None).iter().filter(|f| !f.slug.contains("-live-")) {
            std::fs::write(format!("{dir}/{}.svg", f.slug), &f.svg).unwrap();
        }
        for (_, sym, val) in OTHER.iter().filter(|o| o.0 == *key) {
            let mut g = given.clone();
            for e in g.iter_mut() {
                if e.0 == *sym {
                    e.1 = Input::Text(val.to_string());
                }
            }
            for f in figures(p, key, &g, None, None).iter().filter(|f| !f.slug.contains("-live-")) {
                std::fs::write(format!("{dir}/x-{}.svg", f.slug), &f.svg).unwrap();
            }
        }
    }
}
