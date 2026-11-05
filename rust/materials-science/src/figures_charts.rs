//! Materials Science's charts: sensitivity to one data value, a diffraction stick pattern, lever-rule tie lines,
//! steel microconstituents, the phase-rule table, stress-strain data, fits, diffusion profiles and data views. Every
//! curve point is a run of the model; lines drawn from returned fit parameters are said to be so.

use crate::figs_common::{self as c, clip, esc, getv, meaning, tick, Axis, Chart, CX0, CY0, CY1};
use crate::figs_extra::{fill_legend, num_table};
use crate::figures::{ti, val_text};
use ax_engine::figkit::{self as k, Solved};
use ax_engine::figures::Figure;
use ax_engine::Input;

fn json_list(v: &[f64]) -> Input {
    Input::Json(serde_json::json!(v))
}

/// An output as one value of a list input moves (the last one), the other values as given.
pub fn list_sensitivity(s: &Solved, title: &str, out: &str, list: &str) -> Option<Figure> {
    let v = s.vec(list)?;
    let y0 = s.out(out)?;
    let i = v.len().checked_sub(1)?;
    let x0 = v[i];
    let m = v.iter().fold(0.0f64, |a, x| a.max(x.abs()));
    let (lo, hi) = if x0 != 0.0 { c::around(x0, false)? } else if m > 0.0 { (-m, m) } else { (-1.0, 1.0) };
    let mut pts = vec![];
    for xv in k::span(lo, hi, 41, false) {
        let mut vv = v.clone();
        vv[i] = xv;
        if let Some(a) = s.with_input(&[(list, json_list(&vv))]) {
            if let Some(y) = k::num_of(&a, out) {
                pts.push((xv, y));
            }
        }
    }
    if pts.len() < 2 {
        return None;
    }
    let ys: Vec<f64> = pts.iter().map(|p| p.1).chain([y0]).collect();
    let mut ch = Chart::new((lo, hi), Chart::range(&ys), (false, false), (false, true),
                            &format!("value {} of {list}", i + 1), &format!("{out}"), &format!("{out} = {}", tick(y0)));
    ch.line(&pts, "axp-c", "", false);
    ch.mark((x0, y0), &format!("{list}[{}] = {}", i + 1, tick(x0)));
    let body = ch.body();
    let given = v.iter().map(|x| tick(*x)).collect::<Vec<_>>().join(", ");
    let caption = format!("{out} ({}) as value {} of {list} ({}) moves from {} to {}, the other values as given ([{given}]); \
                           {} runs of the model. Marked: the value given, {list}[{}] = {}, {out} = {}.", meaning(s, out), i + 1,
                          meaning(s, list), tick(lo), tick(hi), pts.len(), i + 1, tick(x0), tick(y0));
    Some(s.chart("sensitivity", title, &caption, &body))
}

/// The stick pattern of the cubic lattices for the stated a and wavelength: every (hkl) up to 4 at the 2-theta the
/// model returns, its height the structure factor |F|^2/f^2 the model returns for SC, BCC and FCC.
pub fn pattern(s: &Solved) -> Option<Figure> {
    let (h0, k0, l0) = (getv(s, "h")?, getv(s, "k")?, getv(s, "l")?);
    let t0 = s.out("two_theta_deg")?;
    let mut lines: Vec<(f64, [f64; 3], String)> = vec![];
    for h in 0..=4i64 {
        for kk in 0..=h {
            for l in 0..=kk {
                if h == 0 {
                    continue;
                }
                let Some(a) = s.with_input(&[("h", Input::Int(h)), ("k", Input::Int(kk)), ("l", Input::Int(l))]) else { continue };
                let Some(tt) = k::num_of(&a, "two_theta_deg") else { continue };
                let mut f = [0.0; 3];
                for (j, st) in ["SC", "BCC", "FCC"].iter().enumerate() {
                    let b = s.solve_other("structure_factor_cubic", &[("structure", ti(st)), ("h", Input::Int(h)),
                                                                      ("k", Input::Int(kk)), ("l", Input::Int(l))])?;
                    f[j] = k::num_of(&b, "F2_over_f2")?;
                }
                lines.push((tt, f, format!("{h}{kk}{l}")));
            }
        }
    }
    if lines.len() < 2 {
        return None;
    }
    let mut ch = Chart::new((0.0, 180.0), (0.0, 19.0), (false, false), (false, false), "diffraction angle 2θ (°)",
                            "|F|² / f²", &format!("2θ = {}° for ({} {} {})", tick(t0), tick(h0), tick(k0), tick(l0)));
    let classes = [("axp-c4", "axp-d4"), ("axp-c3", "axp-d3"), ("axp-c", "axp-d")];
    // tallest first, so a shorter line at the same angle stays visible
    for j in [2usize, 1, 0] {
        for (tt, f, _) in &lines {
            if f[j] > 0.0 {
                ch.stem(*tt, f[j], classes[j].0, classes[j].1);
            }
        }
    }
    for (tt, f, _) in &lines {
        if f.iter().all(|x| *x == 0.0) {
            ch.stem(*tt, 0.4, "axp-c4", "axp-d4");
        }
    }
    let mut n = 0;
    for (tt, f, name) in &lines {
        if f[2] > 0.0 {
            let x = ch.xa.px(*tt);
            let y = ch.ya.px(f[2]) - 6.0 - 13.0 * (n % 2) as f64;
            ch.text(x, y, "middle", "axp-t", name);
            ch.pix.push((x, y));
            n += 1;
        }
    }
    ch.vguide(t0, "");
    ch.legend = vec![("axp-c".into(), "FCC".into()), ("axp-c3".into(), "BCC".into()), ("axp-c4".into(), "SC".into())];
    let body = ch.body();
    let fcc: Vec<String> = lines.iter().filter(|l| l.1[2] > 0.0).take(4).map(|l| format!("({}) {}°", l.2, tick(l.0))).collect();
    let caption = format!("Diffraction stick pattern of the cubic lattices with a = {} nm under λ = {} nm: every (hkl) up to 4 \
                           ({} reflections that occur) placed at the 2θ the model returns and given the structure factor \
                           |F|²/f² it returns for SC (1), BCC (4) and FCC (16); reflections absent in all three are short \
                           grey stems. First FCC lines: {}. Dashed: the reflection given, 2θ = {}°.",
                          tick(s.inp("a")? * 1e9), tick(s.inp("lam")? * 1e9), lines.len(), fcc.join(", "), tick(t0));
    Some(s.chart("pattern", "X-ray diffraction pattern of the cubic lattices", &caption, &body))
}

/// The lever rule on a tie line: the two ends, the alloy, each fraction written over the opposite segment.
pub fn lever(s: &Solved) -> Option<Figure> {
    let c0 = getv(s, "C0")?;
    let (l, r, wl, wr, nl, nr, wln, wrn) = if s.key == "lever_rule" {
        (getv(s, "C1")?, getv(s, "C2")?, s.out("W1")?, s.out("W2")?, "phase 1", "phase 2", "W1", "W2")
    } else {
        let (ca, ce, cb) = (getv(s, "C_alpha")?, getv(s, "C_E")?, getv(s, "C_beta")?);
        let (wp, we) = (s.out("W_primary")?, s.out("W_eutectic")?);
        let ph = s.out_text("primary_phase")?;
        if ph.to_ascii_lowercase().starts_with("alpha") {
            (ca, ce, wp, we, "α", "eutectic", "W_primary", "W_eutectic")
        } else {
            (ce, cb, we, wp, "eutectic", "β", "W_eutectic", "W_primary")
        }
    };
    if !(r > l) || !(c0 >= l && c0 <= r) {
        return None;
    }
    // the drawing must agree with the model: the left phase's share is the right-hand segment over the whole
    if ((r - c0) / (r - l) - wl).abs() > 1e-6 {
        return None;
    }
    let ax = Axis::new(l, r, false, 80.0, 440.0, true);
    let y = 120.0;
    let (pl, p0, pr) = (ax.px(l), ax.px(c0), ax.px(r));
    let mut out = String::new();
    // fractions over the opposite segments
    out.push_str(&format!("<rect class=\"axp-b3\" x=\"{pl:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"16\"/>\
                           <rect class=\"axp-b2\" x=\"{p0:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"16\"/>", y - 48.0, p0 - pl,
                          y - 48.0, pr - p0));
    let lab = |out: &mut String, x: f64, t: &str| {
        out.push_str(&format!("<text class=\"axp-vt\" x=\"{x:.1}\" y=\"{:.1}\" text-anchor=\"middle\">{}</text>", y - 56.0, esc(t)));
    };
    lab(&mut out, (pl + p0) / 2.0, &format!("{wrn} = {}", tick(wr)));
    lab(&mut out, (p0 + pr) / 2.0, &format!("{wln} = {}", tick(wl)));
    out.push_str(&format!("<path class=\"axp-k\" style=\"stroke-width:3\" d=\"M{pl:.1},{y} L{pr:.1},{y}\"/>\
                           <circle class=\"axp-kd\" cx=\"{pl:.1}\" cy=\"{y}\" r=\"5\"/><circle class=\"axp-kd\" cx=\"{pr:.1}\" \
                           cy=\"{y}\" r=\"5\"/><circle class=\"axp-m\" cx=\"{p0:.1}\" cy=\"{y}\" r=\"6\"/>"));
    out.push_str(&format!("<text class=\"axp-lt\" x=\"{pl:.1}\" y=\"{:.1}\" text-anchor=\"middle\">{nl} {}</text>\
                           <text class=\"axp-lt\" x=\"{pr:.1}\" y=\"{:.1}\" text-anchor=\"middle\">{nr} {}</text>\
                           <text class=\"axp-mt\" x=\"{p0:.1}\" y=\"{:.1}\" text-anchor=\"middle\">C0 = {}</text>", y - 14.0,
                          tick(l), y - 14.0, tick(r), y + 26.0, tick(c0)));
    for t in ax.ticks() {
        let x = ax.px(t);
        out.push_str(&format!("<path class=\"axp-g\" d=\"M{x:.1},{} L{x:.1},{}\"/><text class=\"axp-t\" x=\"{x:.1}\" y=\"{}\" \
                               text-anchor=\"middle\">{}</text>", y + 40.0, y + 46.0, y + 62.0, esc(&tick(t))));
    }
    out.push_str(&format!("<path class=\"axp-k\" d=\"M{:.1},{} L{:.1},{}\"/><text class=\"axp-l\" x=\"260\" y=\"{}\" \
                           text-anchor=\"middle\">composition (as stated)</text>", ax.px(ax.lo), y + 40.0, ax.px(ax.hi), y + 40.0,
                          y + 86.0));
    let caption = format!("Tie line from {nl} at {} to {nr} at {} (stated compositions) with the alloy at C0 = {}. The \
                           fraction of each is the length of the opposite segment over the whole tie line; the model returned \
                           {wln} = {} and {wrn} = {} (written above the segments).", tick(l), tick(r), tick(c0), tick(wl),
                          tick(wr));
    let title = if s.key == "lever_rule" { "Lever rule on the tie line" } else { "Primary and eutectic fractions on the tie line" };
    Some(s.figure("lever", title, &caption, &out, 220.0))
}

/// Coordination number against the cation radius, the anion radius as given.
pub fn cn_vs_rc(s: &Solved) -> Option<Figure> {
    let ra = s.inp("ra")?;
    let rc = s.inp("rc")?;
    let hi = (1.2 * ra).max(1.1 * rc);
    c::sweep(s, "cn-vs-rc", "Coordination number against the cation radius", "CN", "rc", 0.05 * ra, hi, 21, false)
}

/// Steel microconstituents along the carbon content (runs of the model): the C0 values drawn.
fn steel_runs(s: &Solved, xs: &[f64]) -> Vec<(f64, f64, f64, String)> {
    xs.iter().filter_map(|x| {
        let a = s.with(&[("C0", *x)])?;
        let p = k::num_of(&a, "W_pearlite")?;
        let w = k::num_of(&a, "W_proeutectoid")?;
        let ph = a.values.iter().find(|v| v.0 == "proeutectoid_phase").and_then(|v| v.1.as_str().map(String::from))?;
        Some((*x, p, w, ph))
    }).collect()
}

pub fn steel_bars(s: &Solved) -> Option<Figure> {
    let c0 = getv(s, "C0")?;
    let (wp0, wq0) = (s.out("W_pearlite")?, s.out("W_proeutectoid")?);
    let mut xs: Vec<f64> = (0..10).map(|i| 0.1 + 0.2 * i as f64).filter(|x| (x - c0).abs() > 0.06).collect();
    xs.push(c0);
    xs.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let runs = steel_runs(s, &xs);
    if runs.len() < 3 {
        return None;
    }
    let mut ch = Chart::new((0.0, 2.1f64.max(c0 + 0.1)), (0.0, 1.0), (false, false), (false, true), "carbon content C0 (wt%)",
                            "fraction", &format!("C0 = {}: pearlite {}", tick(c0), tick(wp0)));
    for (x, p, w, ph) in &runs {
        let px = ch.xa.px(*x);
        let (y0, y1, y2) = (ch.ya.px(0.0), ch.ya.px(*p), ch.ya.px(p + w));
        let top = if ph.starts_with("ferrite") { "axp-b3" } else { "axp-b2" };
        ch.s.push_str(&format!("<rect class=\"axp-b\" x=\"{:.1}\" y=\"{y1:.1}\" width=\"22\" height=\"{:.1}\"/>\
                                <rect class=\"{top}\" x=\"{:.1}\" y=\"{y2:.1}\" width=\"22\" height=\"{:.1}\"/>", px - 11.0,
                               (y0 - y1).max(0.0), px - 11.0, (y1 - y2).max(0.0)));
        if (x - c0).abs() < 1e-12 {
            ch.s.push_str(&format!("<rect class=\"axp-k\" x=\"{:.1}\" y=\"{y2:.1}\" width=\"26\" height=\"{:.1}\"/>", px - 13.0,
                                   (y0 - y2).max(0.0)));
        }
    }
    let mut body = ch.body();
    body.push_str(&fill_legend(&[("axp-b", "pearlite"), ("axp-b3", "proeutectoid ferrite"), ("axp-b2", "proeutectoid cementite")]));
    let caption = format!("Microconstituents of slowly cooled plain-carbon steel just below the eutectoid temperature, from \
                           the lever rule as computed by the model, as stacked bars at {} carbon contents: pearlite at the \
                           bottom, the proeutectoid constituent on top (ferrite or cementite, as the model returns). Outlined: \
                           C0 = {} wt%, pearlite {}, proeutectoid {}.", runs.len(), tick(c0), tick(wp0), tick(wq0));
    Some(s.chart("bars", "Steel microconstituents across the eutectoid (lever rule)", &caption, &body))
}

pub fn steel_decision(s: &Solved) -> Option<Figure> {
    let c0 = getv(s, "C0")?;
    let wp0 = s.out("W_pearlite")?;
    let ph0 = s.out_text("proeutectoid_phase")?;
    let xs = k::span(0.03, 2.13, 61, false);
    let runs = steel_runs(s, &xs);
    if runs.len() < 3 {
        return None;
    }
    let mut ch = Chart::new((0.0, 2.2), (0.0, 1.0), (false, false), (false, true), "carbon content C0 (wt%)",
                            "fraction of pearlite", &format!("C0 = {} gives {}", tick(c0), clip(&ph0, 22)));
    let mut names: Vec<String> = vec![];
    for (i, r) in runs.iter().enumerate() {
        let a = if i == 0 { r.0 } else { (runs[i - 1].0 + r.0) / 2.0 };
        let b = if i + 1 == runs.len() { r.0 } else { (runs[i + 1].0 + r.0) / 2.0 };
        if !names.contains(&r.3) {
            names.push(r.3.clone());
        }
        ch.xband(a, b, if r.3.starts_with("ferrite") { "axp-s3" } else { "axp-s2" });
    }
    let pts: Vec<(f64, f64)> = runs.iter().map(|r| (r.0, r.1)).collect();
    ch.line(&pts, "axp-c", "", false);
    ch.mark((c0, wp0), &format!("{}", clip(&ph0, 20)));
    let mut body = ch.body();
    body.push_str(&fill_legend(&[("axp-s3", "proeutectoid ferrite"), ("axp-s2", "proeutectoid cementite")]));
    let change = runs.windows(2).find(|w| w[0].3 != w[1].3).map(|w| (w[0].0 + w[1].0) / 2.0);
    let caption = format!("The proeutectoid phase along C0 from 0.03 to 2.13 wt%: {} runs of the model, each band coloured by \
                           the phase it returned ({}){}; the line is the fraction of pearlite. Marked: C0 = {} gives {}.",
                          runs.len(), names.join(", "),
                          change.map(|x| format!(", changing near C0 = {}", tick(x))).unwrap_or_default(), tick(c0), ph0);
    Some(s.chart("decision", "Proeutectoid phase and pearlite in a plain-carbon steel", &caption, &body))
}

pub fn phase_table(s: &Solved) -> Option<Figure> {
    let (c0, p0) = (getv(s, "C")?, getv(s, "P")?);
    let f0 = s.out("F")?;
    let nc = (c0 as usize).clamp(3, 6);
    let np = (p0 as usize).clamp(3, 6);
    let mut m = vec![vec![None; np]; nc];
    for (i, row) in m.iter_mut().enumerate() {
        for (j, cell) in row.iter_mut().enumerate() {
            *cell = s.with_input(&[("C", Input::Int(i as i64 + 1)), ("P", Input::Int(j as i64 + 1))])
                .and_then(|a| k::num_of(&a, "F"));
        }
    }
    let rows: Vec<String> = (1..=nc).map(|i| format!("C = {i}")).collect();
    let cols: Vec<String> = (1..=np).map(|j| format!("P = {j}")).collect();
    let mark = (c0 >= 1.0 && p0 >= 1.0).then(|| (c0 as usize - 1, p0 as usize - 1));
    let pressure = s.txt("pressure").map(|p| format!(", pressure {p}")).unwrap_or_default();
    let (body, h) = num_table(&m, &rows, &cols, "components", "coexisting phases", &format!("F = {} at C = {}, P = {}",
                                                                                            tick(f0), tick(c0), tick(p0)), mark);
    let caption = format!("Degrees of freedom F returned by the model for C = 1 to {nc} components (rows) and P = 1 to {np} \
                           coexisting phases (columns){pressure}, one run per cell; F = 0 is an invariant point, a dot a \
                           combination the model refuses (it cannot exist). Outlined: the inputs given, C = {}, P = {}: F = {}.",
                          tick(c0), tick(p0), tick(f0));
    Some(s.figure("table", "Phase rule table", &caption, &body, h.max(200.0)))
}

pub fn stress_strain(s: &Solved) -> Option<Figure> {
    let e = s.vec("strain")?;
    let sg = s.vec("stress")?;
    if e.len() != sg.len() || e.len() < 2 {
        return None;
    }
    let pts: Vec<(f64, f64)> = e.iter().cloned().zip(sg.iter().cloned()).collect();
    let smax = sg.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let emax = e.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let (header, caption_tail, extra): (String, String, Box<dyn Fn(&mut Chart)>);
    match s.key {
        "offset_yield_strength" => {
            let (ys, ye) = (s.out("yield_MPa")?, s.out("yield_strain")?);
            let em = s.inp("E")? / 1e6;
            let off = ye - ys / em;
            header = format!("yield {} MPa", tick(ys));
            caption_tail = format!("the model returned the offset yield strength {} MPa at strain {}; dashed: the elastic line \
                                    of slope E = {} GPa shifted to the offset {} (derived from the returned values).",
                                   tick(ys), tick(ye), tick(em / 1e3), tick(off));
            let top = smax * 1.05;
            extra = Box::new(move |ch: &mut Chart| {
                ch.line(&[(off, 0.0), (off + top / em, top)], "axp-c3", "", true);
                ch.mark((ye, ys), &format!("{} MPa", tick(ys)));
            });
        }
        "tensile_toughness" => {
            let (tg, uts, fe) = (s.out("toughness_MJ_m3")?, s.out("uts_MPa")?, s.out("fracture_strain")?);
            header = format!("toughness {} MJ/m³", tick(tg));
            caption_tail = format!("the model returned the toughness {} MJ/m³ (the shaded area under the curve), UTS {} MPa and \
                                    fracture strain {}.", tick(tg), tick(uts), tick(fe));
            let p2 = pts.clone();
            extra = Box::new(move |ch: &mut Chart| {
                let base: Vec<(f64, f64)> = p2.iter().map(|p| (p.0, 0.0)).collect();
                ch.area(&base, &p2, "axp-s2", "");
                ch.hguide(uts, "");
                ch.vguide(fe, "");
                let (px, py) = (ch.xa.px(fe), ch.ya.px(uts));
                ch.text(px - 6.0, py + 18.0, "end", "axp-lt", &format!("UTS {} MPa", tick(uts)));
                ch.text(px - 6.0, CY1 - 10.0, "end", "axp-lt", &format!("fracture strain {}", tick(fe)));
            });
        }
        _ => {
            let (eg, b) = (s.out("E_GPa")?, s.out("intercept_MPa")?);
            header = format!("E = {} GPa", tick(eg));
            caption_tail = format!("the model returned E = {} GPa with intercept {} MPa; dashed: that line (drawn from the \
                                    returned values).", tick(eg), tick(b));
            extra = Box::new(move |ch: &mut Chart| {
                ch.line(&[(0.0, b), (emax, b + eg * 1e3 * emax)], "axp-c3", "", true);
            });
        }
    }
    let mut ch = Chart::new((0.0f64.min(e.iter().cloned().fold(0.0, f64::min)), emax), (0.0f64.min(sg.iter().cloned().fold(0.0, f64::min)), smax * 1.08),
                            (false, false), (true, true), "strain", "stress (MPa)", &header);
    extra(&mut ch);
    ch.line(&pts, "axp-c", "", false);
    ch.dots(&pts, "axp-d", 3.5);
    let body = ch.body();
    let caption = format!("Stress-strain points as given ({} points, joined in order); {caption_tail}", pts.len());
    Some(s.chart("stress-strain", "Stress-strain data with the returned properties", &caption, &body))
}

pub fn hall_petch(s: &Solved) -> Option<Figure> {
    let d = s.vec("d_um")?;
    let y = s.vec("yield_MPa")?;
    let (s0, kk) = (s.out("sigma0_MPa")?, s.out("k_MPa_sqrt_um")?);
    if d.len() != y.len() || d.iter().any(|x| !(*x > 0.0)) {
        return None;
    }
    let pts: Vec<(f64, f64)> = d.iter().map(|x| x.powf(-0.5)).zip(y.iter().cloned()).collect();
    let xmax = pts.iter().map(|p| p.0).fold(0.0, f64::max) * 1.15;
    let line = vec![(0.0, s0), (xmax, s0 + kk * xmax)];
    let ys: Vec<f64> = y.iter().cloned().chain([s0, s0 + kk * xmax]).collect();
    let mut ch = Chart::new((0.0, xmax), Chart::range(&ys), (false, false), (false, true), "d^(-1/2) (µm^(-1/2))",
                            "yield strength (MPa)", &format!("σ0 = {} MPa, k = {}", tick(s0), tick(kk)));
    ch.line(&line, "axp-c3", "", true);
    ch.dots(&pts, "axp-d", 4.0);
    ch.mark((0.0, s0), "σ0");
    if let (Some(dt), Some(yt)) = (getv(s, "d_target_um"), s.out("yield_target_MPa")) {
        if dt > 0.0 {
            ch.dots(&[(dt.powf(-0.5), yt)], "axp-d2", 5.0);
        }
    }
    let body = ch.body();
    let caption = format!("Hall-Petch plot: the stated yield strengths against d^(-1/2) for the stated grain sizes (the axis \
                           computed from d), with the line σ0 + k d^(-1/2) drawn from the returned σ0 = {} MPa and k = {} \
                           MPa µm^(1/2) (dashed). Marked: the intercept σ0.", tick(s0), tick(kk));
    Some(s.chart("fit", "Hall-Petch: yield strength against grain size", &caption, &body))
}

pub fn hollomon(s: &Solved) -> Option<Figure> {
    let e = s.vec("true_strain")?;
    let sg = s.vec("true_stress")?;
    let (n, kk) = (s.out("n")?, s.out("K_MPa")?);
    if e.len() != sg.len() || e.iter().chain(&sg).any(|x| !(*x > 0.0)) {
        return None;
    }
    let pts: Vec<(f64, f64)> = e.iter().cloned().zip(sg.iter().cloned()).collect();
    let (lo, hi) = Chart::range(&e);
    let xs = k::span(lo / 1.5, hi * 1.5, 30, true);
    let line: Vec<(f64, f64)> = xs.iter().map(|x| (*x, kk * x.powf(n))).collect();
    let ys: Vec<f64> = line.iter().map(|p| p.1).chain(sg.iter().cloned()).collect();
    let mut ch = Chart::new((lo / 1.5, hi * 1.5), Chart::range_pos(&ys), (true, true), (false, true), "true strain",
                            "true stress (MPa)", &format!("n = {}, K = {} MPa", tick(n), tick(kk)));
    ch.line(&line, "axp-c3", "", true);
    ch.dots(&pts, "axp-d", 4.0);
    let body = ch.body();
    let caption = format!("Hollomon hardening on logarithmic axes: the stated true stress-strain points with the power law \
                           K ε^n drawn from the returned K = {} MPa and n = {} (dashed; a straight line of slope n here).",
                          tick(kk), tick(n));
    Some(s.chart("fit", "Strain hardening: the Hollomon power law", &caption, &body))
}

pub fn reflections(s: &Solved) -> Option<Figure> {
    let v = s.out_list("N_values")?;
    let labels: Vec<String> = (1..=v.len()).map(|i| i.to_string()).collect();
    let st = s.txt("structure")?;
    let body = c::bars_body(&labels, &v, &[], "h² + k² + l²", &format!("{st}: first {}", tick(v[0])), false)?;
    let caption = format!("h² + k² + l² of the first {} allowed reflections of the {st} structure, ascending, as the model \
                           returned them: {}.", v.len(), v.iter().map(|x| tick(*x)).collect::<Vec<_>>().join(", "));
    Some(s.chart("series", "First allowed reflections of a cubic crystal", &caption, &body))
}

/// Inputs and results of the request as a flow: input boxes, the capability, output values.
pub fn io(s: &Solved, ins: &[&str], outs: &[&str]) -> Option<Figure> {
    let name = s.p.procedure(s.key)?.name.clone();
    let iv: Vec<String> = ins.iter().filter_map(|i| s.vec(i).map(|v| format!("{i} = [{}]", v.iter().map(|x| tick(*x))
        .collect::<Vec<_>>().join(", ")))).collect();
    let ov: Vec<String> = outs.iter().filter_map(|o| val_text(s, o).map(|v| format!("{o} = {v}"))).collect();
    if iv.len() != ins.len() || ov.len() != outs.len() {
        return None;
    }
    let h = 40.0 + 44.0 * iv.len().max(ov.len()) as f64;
    let mid = h / 2.0;
    let mut out = String::new();
    for (i, t) in iv.iter().enumerate() {
        let y = mid + (i as f64 - (iv.len() as f64 - 1.0) / 2.0) * 44.0;
        out.push_str(&format!("<rect class=\"axp-ka\" x=\"10\" y=\"{:.1}\" width=\"182\" height=\"30\" rx=\"6\"/>\
                               <text class=\"axp-vt\" x=\"18\" y=\"{:.1}\" font-size=\"11\">{}</text>\
                               <path class=\"axp-k\" d=\"M192,{y:.1} L210,{mid:.1}\"/>", y - 15.0, y + 4.0, esc(&clip(t, 26))));
    }
    out.push_str(&format!("<rect class=\"axp-b\" x=\"204\" y=\"{:.1}\" width=\"146\" height=\"44\" rx=\"6\"/>", mid - 22.0));
    let words: Vec<&str> = name.split_whitespace().collect();
    let half = words.len().div_ceil(2);
    for (j, part) in [words[..half].join(" "), words[half..].join(" ")].iter().enumerate() {
        out.push_str(&format!("<text class=\"axp-bt\" x=\"275\" y=\"{:.1}\" text-anchor=\"middle\" font-size=\"11\">{}</text>",
                              mid - 4.0 + 14.0 * j as f64, esc(&clip(part, 22))));
    }
    for (i, t) in ov.iter().enumerate() {
        let y = mid + (i as f64 - (ov.len() as f64 - 1.0) / 2.0) * 44.0;
        out.push_str(&format!("<path class=\"axp-k\" d=\"M340,{mid:.1} L360,{y:.1}\"/>\
                               <text class=\"{}\" x=\"366\" y=\"{:.1}\">{}</text>", if i == 0 { "axp-mt" } else { "axp-vt" },
                              y + 5.0, esc(&clip(t, 17))));
    }
    let caption = format!("The request as inputs and results: stated {}; returned by the model: {}.", iv.join(", "),
                          ov.join(", "));
    Some(s.figure("io", &format!("{name}: inputs and results"), &caption, &out, h.max(140.0)))
}

/// The loading direction as given, with the returned slip-system values.
pub fn load_data(s: &Solved) -> Option<Figure> {
    let v = s.vec("load")?;
    let m = s.out("max_schmid_factor")?;
    let (act, sys) = (s.out("active_systems")?, s.out("systems")?);
    let labels: Vec<String> = ["u", "v", "w"].iter().take(v.len()).map(|x| x.to_string()).collect();
    if labels.len() != v.len() {
        return None;
    }
    let mut body = c::bars_body(&labels, &v, &[], "loading direction component", &format!("max_schmid_factor = {}", tick(m)), false)?;
    body.push_str(&format!("<text class=\"axp-vt\" x=\"{CX0}\" y=\"328\">active systems {} of {} considered ({})</text>",
                           tick(act), tick(sys), esc(&s.txt("structure").unwrap_or_default())));
    let caption = format!("The loading direction [{}] as given, one bar per component; returned by the model: the largest \
                           Schmid factor {}, reached on {} of the {} slip systems considered.",
                          v.iter().map(|x| tick(*x)).collect::<Vec<_>>().join(" "), tick(m), tick(act), tick(sys));
    Some(s.chart("data", "Most favourable slip system: the data and the returned values", &caption, &body))
}

pub fn mw_xy(s: &Solved) -> Option<Figure> {
    let m = s.vec("M")?;
    let f = s.vec("fraction")?;
    let (mn, mw, pdi) = (s.out("Mn")?, s.out("Mw")?, s.out("PDI")?);
    if m.len() != f.len() {
        return None;
    }
    let pts: Vec<(f64, f64)> = m.iter().cloned().zip(f.iter().cloned()).collect();
    let xr = Chart::range(&m.iter().cloned().chain([mn, mw]).collect::<Vec<_>>());
    let mut ch = Chart::new(xr, (0.0, f.iter().cloned().fold(0.0, f64::max) * 1.25), (false, false), (true, false),
                            "molecular weight M (g/mol)", &format!("{} fraction", s.txt("basis").unwrap_or_default()),
                            &format!("PDI = {}", tick(pdi)));
    for p in &pts {
        ch.stem(p.0, p.1, "axp-c", "axp-d");
    }
    ch.vguide(mn, &format!("Mn = {}", tick(mn)));
    ch.vguide(mw, "");
    ch.text(ch.xa.px(mw) + 4.0, CY0 + 32.0, "start", "axp-lt", &format!("Mw = {}", tick(mw)));
    let body = ch.body();
    let caption = format!("The stated {} fractions against molecular weight ({} size ranges, stems), with the averages the \
                           model returned: Mn = {}, Mw = {} (dashed), dispersity {}.", s.txt("basis").unwrap_or_default(),
                          pts.len(), tick(mn), tick(mw), tick(pdi));
    Some(s.chart("xy", "Number- and weight-average molecular weight and dispersity", &caption, &body))
}

fn hours(t: f64) -> String {
    if t >= 3600.0 { format!("{} h", tick(t / 3600.0)) } else { format!("{} s", tick(t)) }
}

/// Concentration against depth at several multiples of the time given, each point a run of the model.
pub fn profiles(s: &Solved, kind: &str, factors: &[f64]) -> Option<Figure> {
    let (x0, t0, cx) = (s.inp("x")?, s.inp("t")?, s.out("Cx")?);
    let (c0, cs) = (getv(s, "C0")?, getv(s, "Cs")?);
    if !(x0 > 0.0 && t0 > 0.0) {
        return None;
    }
    let xs = k::span(0.0, 5.0 * x0, 41, false);
    let mut series = vec![];
    for (j, f) in factors.iter().enumerate() {
        let t = t0 * f;
        let pts: Vec<(f64, f64)> = xs.iter().filter_map(|x| {
            let a = s.with(&[("x", *x), ("t", t)])?;
            k::num_of(&a, "Cx").map(|y| (x * 1e3, y))
        }).collect();
        if pts.len() >= 2 {
            series.push((format!("t = {}", hours(t)), pts, j));
        }
    }
    if series.len() < 2 {
        return None;
    }
    let ys: Vec<f64> = series.iter().flat_map(|s| s.1.iter().map(|p| p.1)).chain([c0, cs]).collect();
    let mut ch = Chart::new((0.0, 5.0 * x0 * 1e3), Chart::range(&ys), (false, false), (false, true), "depth below the surface x (mm)",
                            "concentration C", &format!("C = {} at x = {} mm", tick(cx), tick(x0 * 1e3)));
    for (name, pts, j) in &series {
        ch.line(pts, c::LINES[*j % 3], name, false);
    }
    ch.hguide(c0, &format!("initial C0 = {}", tick(c0)));
    ch.mark((x0 * 1e3, cx), &tick(cx));
    let body = ch.body();
    let caption = format!("Concentration against depth for diffusion into a solid much deeper than the profile (surface Cs = {}, initial C0 = \
                           {}, D = {} m²/s as given) at {}, each curve 41 depths computed by the model to {} mm. Marked: x = {} \
                           mm, t = {}, C = {}.", tick(cs), tick(c0), tick(s.inp("D")?),
                          series.iter().map(|s| s.0.clone()).collect::<Vec<_>>().join(", "), tick(5.0 * x0 * 1e3),
                          tick(x0 * 1e3), hours(t0), tick(cx));
    let title = if kind == "profile" { "Diffusion into a solid: concentration profiles over time" } else {
        "Diffusion profiles at a quarter of, equal to and four times the time given"
    };
    Some(s.chart(kind, title, &caption, &body))
}
