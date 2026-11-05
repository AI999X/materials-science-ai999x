//! Materials Science's own live figures (ax_engine::figkit), drawn for the visitor's inputs from what the model
//! computes: unit cells with planes and directions, Bragg geometry and a diffraction pattern, value maps over two
//! inputs, sensitivity to one data value, verdict maps, lever-rule tie lines, stress-strain data, fits and diffusion
//! profiles (docs/FIGURE_PARITY.md). Crystal drawings: figs_cells.rs; charts: figures_charts.rs.

use crate::figs_cells::{self as cl, Extras};
use crate::figs_common::{self as c, getv, tick, FieldSpec};
use ax_engine::figkit::Solved;
use ax_engine::figures::Figure;
use ax_engine::Input;

/// The field maps: capability, output shown, input across, input up, logarithmic (across, up), title.
const FIELDS: &[(&str, &str, &str, &str, (bool, bool), &str)] = &[
    ("bragg_wavelength", "lam_nm", "d", "theta", (false, false), "X-ray wavelength over interplanar spacing and Bragg angle"),
    ("copolymer_degree_of_polymerization", "DP", "Mn", "f1", (false, false),
     "Degree of polymerization of a copolymer over molecular weight and composition"),
    ("einstein_heat_capacity", "Cv", "theta_E", "T", (true, true),
     "Molar heat capacity (Einstein model) over Einstein temperature and temperature"),
    ("isotropic_elastic_constants", "G_GPa", "E", "nu", (false, false),
     "Shear modulus over Young's modulus and Poisson's ratio"),
    ("larson_miller_parameter", "LMP", "T", "t_r", (false, false),
     "Larson-Miller parameter over temperature and rupture time"),
    ("modulus_of_resilience", "Ur_MJ_m3", "sy", "E", (false, false),
     "Modulus of resilience over yield strength and Young's modulus"),
    ("polymer_chain_dimensions", "r_rms_nm", "l", "n", (false, false),
     "End-to-end distance of a freely rotating chain over bond length and number of bonds"),
    ("wiedemann_franz", "kappa", "T", "sigma", (false, false),
     "Electronic thermal conductivity (Wiedemann-Franz) over temperature and electrical conductivity"),
];

/// Sensitivity to the last value of a list: capability, output, list input, title.
const SENS: &[(&str, &str, &str, &str)] = &[
    ("avrami_kinetics", "n", "t", "Avrami exponent as one measured time moves"),
    ("band_gap_from_conductivity", "Eg_eV", "T", "Band gap as one measurement temperature moves"),
    ("diffusion_arrhenius_fit", "Q_kJmol", "T", "Activation energy of diffusion as one temperature moves"),
    ("schmid_factor", "schmid_factor", "load", "Schmid factor as one component of the loading direction moves"),
    ("vacancy_formation_energy_fit", "Qv_eV", "T", "Vacancy formation energy as one temperature moves"),
    ("weibull_modulus_fit", "m", "strengths", "Weibull modulus as one measured strength moves"),
];

/// The figures for one solved request (the dispatcher ax_engine calls: Pack::figures).
pub fn live(s: &Solved) -> Vec<Figure> {
    let mut v: Vec<Option<Figure>> = vec![];
    if let Some(f) = FIELDS.iter().find(|f| f.0 == s.key) {
        let spec = FieldSpec { z: f.1, x: f.2, y: f.3, log: f.4, title: f.5, zunit: "" };
        v.push(match s.key {
            "copolymer_degree_of_polymerization" => c::field_map_in(s, &spec, None, Some((0.02, 0.98))),
            "polymer_chain_dimensions" => {
                let n = getv(s, "n").unwrap_or(1.0);
                c::field_map_in(s, &spec, None, Some((1.0, (2.0 * n).max(9.0).min(40.0))))
            }
            _ => c::field_map(s, &spec),
        });
    }
    if let Some(w) = SENS.iter().find(|w| w.0 == s.key) {
        v.push(crate::figures_charts::list_sensitivity(s, w.3, w.1, w.2));
    }
    use crate::figures_charts as ch;
    match s.key {
        "atomic_packing_factor" => v.push(cell(s, "The unit cell with the returned values",
                                               &["apf", "coordination", "atoms_per_cell"], Extras::default(), "")),
        "burgers_vector" => v.push(burgers(s)),
        "cubic_lattice_parameter" => v.push(cell(s, "The unit cell with the returned values", &["a_nm", "nearest_neighbour_nm"],
                                                 Extras::default(), "")),
        "interstitial_site_size" => v.push(interstitial(s)),
        "lattice_parameter_from_density" => v.push(cell(s, "The unit cell with the returned values", &["a_nm", "R_nm"],
                                                        Extras::default(), "")),
        "linear_density" => v.push(linear(s)),
        "planar_density" => v.push(plane_cell(s, &["PD_per_nm2", "PPF"])),
        "space_group" => v.push(cell(s, "The structure's unit cell and space group", &["symbol", "number", "crystal_system"],
                                     Extras::default(), "")),
        "structure_factor_cubic" => {
            v.push(plane_cell(s, &["F_over_f", "F2_over_f2", "allowed"]));
            v.push(c::verdict_map(s, "Structure factor of a cubic reflection: where the reflection appears", "allowed", "h",
                                  (1.0, 9.0), "k", (1.0, 9.0), (false, false)));
        }
        "theoretical_density" => v.push(cell(s, "The unit cell with the returned values", &["rho_gcc", "a_nm"],
                                             Extras::default(), "")),
        "miller_bravais_direction" => v.push(bravais(s)),
        "miller_indices_from_intercepts" => v.push(intercepts(s)),
        "bragg_angle" | "bragg_spacing" => v.push(bragg_geom(s)),
        "cubic_reflection_angle" => {
            v.push(bragg_geom(s));
            v.push(ch::pattern(s));
        }
        "lever_rule" | "eutectic_microconstituents" => v.push(ch::lever(s)),
        "extrinsic_carrier_concentration" => {
            let m = getv(s, "Nd").unwrap_or(0.0).max(getv(s, "Na").unwrap_or(0.0));
            v.push(c::verdict_map(s, "Doped semiconductor: where the carrier type changes", "carrier_type", "Nd",
                                  (0.0, 2.0 * m), "Na", (0.0, 2.0 * m), (false, false)));
        }
        "radius_ratio_coordination" => {
            v.push(ch::cn_vs_rc(s));
            let (rc, ra) = (getv(s, "rc").unwrap_or(0.0), getv(s, "ra").unwrap_or(0.0));
            v.push(c::verdict_map(s, "Coordination from the cation-anion radius ratio: where the polyhedron changes",
                                  "geometry", "rc", (0.05 * ra.max(rc), 1.2 * ra.max(rc)), "ra", (0.5 * ra, 1.5 * ra),
                                  (false, false)));
        }
        "iron_carbon_microconstituents" => {
            v.push(ch::steel_bars(s));
            v.push(ch::steel_decision(s));
        }
        "gibbs_phase_rule" => v.push(ch::phase_table(s)),
        "offset_yield_strength" | "tensile_toughness" | "youngs_modulus_from_data" => v.push(ch::stress_strain(s)),
        "hall_petch" => v.push(ch::hall_petch(s)),
        "strain_hardening_exponent" => v.push(ch::hollomon(s)),
        "allowed_reflections_cubic" => v.push(ch::reflections(s)),
        "creep_parameters_fit" => v.push(ch::io(s, &["stress_MPa", "T", "rate"], &["n", "Q_kJmol"])),
        "max_schmid_factor" => v.push(ch::load_data(s)),
        "molecular_weight_averages" => v.push(ch::mw_xy(s)),
        "fick_second_law_concentration" => {
            v.push(ch::profiles(s, "profile", &[1.0 / 7.0, 1.0, 24.0 / 7.0]));
            v.push(ch::profiles(s, "profile-2", &[0.25, 1.0, 4.0]));
        }
        _ => {}
    }
    v.into_iter().flatten().collect()
}

/// An output as text: a number to 4 digits, a list as written, yes/no.
pub fn val_text(s: &Solved, sym: &str) -> Option<String> {
    if let Some(x) = s.out(sym) {
        return Some(tick(x));
    }
    let j = s.outj(sym)?;
    Some(match j {
        serde_json::Value::String(t) => t.clone(),
        serde_json::Value::Bool(b) => if *b { "yes".into() } else { "no".into() },
        serde_json::Value::Array(a) => format!("[{}]", a.iter().map(|x| x.as_f64().map(tick).unwrap_or_else(|| x.to_string()))
            .collect::<Vec<_>>().join(" ")),
        other => other.to_string(),
    })
}

fn structure_name(st: &str) -> String {
    match st.to_ascii_lowercase().as_str() {
        "sc" | "bcc" | "fcc" | "hcp" => st.to_ascii_uppercase(),
        "cscl" => "CsCl".into(),
        other => other.to_string(),
    }
}

/// The unit cell of the structure given, labelled with the returned values `outs`; `what`: words for the extras.
fn cell(s: &Solved, title: &str, outs: &[&str], ex: Extras, what: &str) -> Option<Figure> {
    let st = s.txt("structure")?;
    let items: Vec<(String, String)> = outs.iter().filter_map(|o| val_text(s, o).map(|v| (o.to_string(), v))).collect();
    if items.len() != outs.len() {
        return None;
    }
    let drawing = if st.eq_ignore_ascii_case("hcp") {
        if ex.plane.len() > 0 || ex.arrow.is_some() || ex.site.is_some() {
            return None;
        }
        cl::hcp_prism()
    } else {
        cl::cube(&cl::atoms(&st)?, &ex)
    };
    let name = structure_name(&st);
    let mut body = drawing;
    body.push_str(&cl::panel(330.0, 64.0, &name, &items));
    let listed = items.iter().map(|(n, v)| format!("{n} = {v}")).collect::<Vec<_>>().join(", ");
    let caption = format!("The {name} unit cell: atoms at the standard positions of the structure (corner atoms in the \
                           first colour, other lattice sites in the third, the second atom of the basis in the second; hidden \
                           edges dashed){what}. Labelled with what the model returned: {listed}.");
    Some(s.figure("cell", title, &caption, &body, 360.0))
}

fn burgers(s: &Solved) -> Option<Figure> {
    let dir = s.out_text("direction")?;
    let a = s.inp("a")?;
    let b = s.out("b")?;
    // "a/2<110>": the fraction and the family read from the returned text; drawn only when it gives the returned b
    let frac = if let Some(i) = dir.find("a/") {
        dir[i + 2..].chars().take_while(|c| c.is_ascii_digit()).collect::<String>().parse::<f64>().ok().map(|d| 1.0 / d)?
    } else { 1.0 };
    let inner: String = dir.split('<').nth(1)?.split('>').next()?.to_string();
    let digits: Vec<f64> = inner.chars().filter_map(|c| c.to_digit(10).map(|d| d as f64)).collect();
    if digits.len() != 3 {
        return None;
    }
    let len = (digits[0].powi(2) + digits[1].powi(2) + digits[2].powi(2)).sqrt() * frac * a;
    if ((len - b) / b).abs() > 1e-6 {
        return None;
    }
    let to = (digits[0] * frac, digits[1] * frac, digits[2] * frac);
    let ex = Extras { arrow: Some(((0.0, 0.0, 0.0), to)), ..Default::default() };
    cell(s, "The unit cell with its Burgers vector", &["b_nm", "direction"], ex,
         &format!(", with the Burgers vector {dir} drawn from the origin (second colour)"))
}

fn interstitial(s: &Solved) -> Option<Figure> {
    let st = s.txt("structure")?.to_ascii_lowercase();
    let site = s.txt("site")?.to_ascii_lowercase();
    let ratio = s.out("ratio")?;
    let p = match (st.as_str(), site.as_str()) {
        ("fcc", "octahedral") => (0.5, 0.5, 0.5),
        ("fcc", "tetrahedral") => (0.25, 0.25, 0.25),
        ("bcc", "octahedral") => (0.5, 0.5, 0.0),
        ("bcc", "tetrahedral") => (0.5, 0.25, 0.0),
        _ => return None,
    };
    let ex = Extras { site: Some((p, (24.0 * ratio).clamp(3.0, 30.0))), ..Default::default() };
    cell(s, "The unit cell with an interstitial site", &["r_nm", "ratio"], ex,
         &format!(", with one {site} site ringed in the second colour, the ring's size in proportion to the returned r/R"))
}

fn linear(s: &Solved) -> Option<Figure> {
    let (u, v, w) = (getv(s, "u")?, getv(s, "v")?, getv(s, "w")?);
    let ex = Extras { arrow: Some(cl::direction(u, v, w)?), ..Default::default() };
    cell(s, &format!("The direction [{u} {v} {w}] in the unit cell"), &["LD_per_nm", "repeat_nm", "LPF"], ex,
         &format!(", with the direction [{u} {v} {w}] as an arrow (second colour)"))
}

fn plane_cell(s: &Solved, outs: &[&str]) -> Option<Figure> {
    let (h, k, l) = (getv(s, "h")?, getv(s, "k")?, getv(s, "l")?);
    let plane = cl::plane_hkl(h, k, l);
    if plane.is_empty() {
        return None;
    }
    let st = structure_name(&s.txt("structure")?);
    cell(s, &format!("The ({h} {k} {l}) plane in the {st} cell"), outs, Extras { plane, ..Default::default() },
         &format!(", with the ({h} {k} {l}) plane shaded"))
}

fn bravais(s: &Solved) -> Option<Figure> {
    let (u, v, t, w) = (s.out("u")?, s.out("v")?, s.out("t")?, s.out("w")?);
    let (uu, vv, ww) = (getv(s, "U")?, getv(s, "V")?, getv(s, "W")?);
    if u == 0.0 && v == 0.0 && t == 0.0 {
        return None;
    }
    let mut body = cl::basal(u, v, t);
    let idx = format!("[{} {} {} {}]", tick(u), tick(v), tick(t), tick(w));
    body.push_str(&cl::panel(370.0, 64.0, &format!("[{uu} {vv} {ww}]"), &[("indices".into(), idx.clone()),
                                                                            ("w (along c)".into(), tick(w))]));
    let caption = format!("The basal plane of a hexagonal cell with its axes a1, a2, a3 and the direction the model returned \
                           for the three-index direction [{uu} {vv} {ww}]: {idx}, drawn as u a1 + v a2 + t a3 (to one third \
                           length, shortened to the cell when longer); w = {} is along c, out of the plane.", tick(w));
    Some(s.figure("cell", "Four-index direction on the basal plane", &caption, &body, 340.0))
}

/// An intercept as written ("1", "1/2", "inf", "-1"): its value (None for infinity).
fn intercept(t: &str) -> Option<Option<f64>> {
    let t = t.trim().to_ascii_lowercase();
    if t == "inf" || t == "infinity" || t == "∞" {
        return Some(None);
    }
    if let Some((a, b)) = t.split_once('/') {
        let (a, b): (f64, f64) = (a.trim().parse().ok()?, b.trim().parse().ok()?);
        return (b != 0.0).then_some(Some(a / b));
    }
    t.parse::<f64>().ok().map(Some)
}

fn intercepts(s: &Solved) -> Option<Figure> {
    let (xt, yt, zt) = (s.txt("x")?, s.txt("y")?, s.txt("z")?);
    let (x, y, z) = (intercept(&xt)?, intercept(&yt)?, intercept(&zt)?);
    let show = |v: Option<f64>, t: &str| if v.is_none() { "∞".to_string() } else { t.trim().to_string() };
    let (xt, yt, zt) = (show(x, &xt), show(y, &yt), show(z, &zt));
    let (h, k, l) = (s.out("h")?, s.out("k")?, s.out("l")?);
    let inv = |v: Option<f64>| v.map(|a| 1.0 / a).unwrap_or(0.0);
    let n = (inv(x), inv(y), inv(z));
    // the returned indices must be proportional to the reciprocal intercepts
    let m = n.0.abs().max(n.1.abs()).max(n.2.abs());
    let mh = h.abs().max(k.abs()).max(l.abs());
    if !(m > 0.0 && mh > 0.0) {
        return None;
    }
    for (a, b) in [(n.0, h), (n.1, k), (n.2, l)] {
        if (a / m - b / mh).abs() > 1e-9 {
            return None;
        }
    }
    // the plane through the stated intercepts, cut by the cell
    let poly = cl::cut(n, 1.0);
    if poly.is_empty() {
        return None;
    }
    let mut body = cl::cube(&[], &Extras { plane: poly, ..Default::default() });
    let v = cl::VIEW;
    for (val, axis) in [(x, 0), (y, 1), (z, 2)] {
        if let Some(a) = val {
            if (0.0..=1.0).contains(&a) {
                let p = match axis { 0 => (a, 0.0, 0.0), 1 => (0.0, a, 0.0), _ => (0.0, 0.0, a) };
                let q = v.p(p);
                body.push_str(&format!("<circle class=\"axp-m\" cx=\"{:.1}\" cy=\"{:.1}\" r=\"5\"/>", q.0, q.1));
            }
        }
    }
    let idx = format!("({} {} {})", tick(h), tick(k), tick(l));
    body.push_str(&cl::panel(330.0, 64.0, "intercepts", &[("Miller indices".into(), idx.clone()),
                                                          ("x, y, z".into(), format!("{xt}, {yt}, {zt}"))]));
    let caption = format!("The plane with the stated intercepts x = {xt}, y = {yt}, z = {zt} (in lattice parameters; ∞: \
                           parallel to that axis) drawn in the unit cell, shaded; marked: each intercept on the cell's edges. \
                           The model returned its Miller indices {idx}.");
    Some(s.figure("cell", &format!("The plane {idx} from its intercepts"), &caption, &body, 360.0))
}

fn bragg_geom(s: &Solved) -> Option<Figure> {
    let theta = s.out("theta_deg").or_else(|| if s.key == "bragg_spacing" { getv(s, "theta") } else { None })?;
    let lam = s.inp("lam")?;
    let d = s.inp("d").or_else(|| s.out("d")).or_else(|| s.out("d_nm").map(|x| x * 1e-9))?;
    let mut labels = vec![format!("θ = {}°", tick(theta)), format!("λ = {} nm, d = {} nm", tick(lam * 1e9), tick(d * 1e9))];
    if let Some(t2) = s.out("two_theta_deg") {
        labels.push(format!("2θ = {}°", tick(t2)));
    }
    if s.key == "cubic_reflection_angle" {
        labels.push(format!("({} {} {}) of a = {} nm", tick(getv(s, "h")?), tick(getv(s, "k")?), tick(getv(s, "l")?),
                            tick(s.inp("a")? * 1e9)));
    }
    let body = cl::bragg(theta, &labels)?;
    let caption = format!("Bragg reflection from two lattice planes a spacing d = {} nm apart: rays arriving and leaving at \
                           θ = {}°, the lower ray travelling 2d sin θ further (dashed); λ = {} nm{}.", tick(d * 1e9),
                          tick(theta), tick(lam * 1e9),
                          if s.key == "bragg_spacing" { format!("; the model returned d = {} nm", tick(d * 1e9)) }
                          else { format!("; the model returned θ = {}°", tick(theta)) });
    Some(s.figure("geometry", "Bragg reflection geometry", &caption, &body, 330.0))
}

/// Inputs for another solve: a text, an integer, a number.
pub fn ti(t: &str) -> Input {
    Input::Text(t.to_string())
}
