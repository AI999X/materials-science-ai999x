//! Drawing helpers beyond the shared figs_common.rs (the same as Chemistry's, plus a shaded table of numbers):
//! arrows, filled areas and stems on an x-y chart, bars with their own colours and labels, formulas with subscripts.
#![allow(dead_code)]

use crate::figs_common::{clip, esc, tick, Axis, Chart, CX0, CX1, CY0, CY1};

/// A value to 3 significant digits, exponent form beyond 1e5 or under 1e-3 (2.74e23).
pub fn short(v: f64) -> String {
    let a = v.abs();
    if v != 0.0 && (a >= 1e5 || a < 1e-3) {
        format!("{:.2e}", v)
    } else {
        ax_engine::num::py_g(v, 3)
    }
}

/// A formula with its digits as subscripts (H2SO4 -> H₂SO₄); a leading coefficient stays as written.
pub fn sub(f: &str) -> String {
    let mut out = String::new();
    let mut prev_letter = false;
    for c in f.chars() {
        if c.is_ascii_digit() && prev_letter {
            out.push(char::from_u32(0x2080 + c.to_digit(10).unwrap()).unwrap_or(c));
            continue;
        }
        prev_letter = c.is_ascii_alphabetic() || c == ')' || (prev_letter && c.is_ascii_digit());
        out.push(c);
    }
    out
}

impl Chart {
    /// An arrow from a to b (data coordinates), in the third colour.
    pub fn arrow(&mut self, a: (f64, f64), b: (f64, f64)) {
        let (ax, ay, bx, by) = (self.xa.px(a.0), self.ya.px(a.1), self.xa.px(b.0), self.ya.px(b.1));
        if ![ax, ay, bx, by].iter().all(|v| v.is_finite()) {
            return;
        }
        let (dx, dy) = (bx - ax, by - ay);
        let d = (dx * dx + dy * dy).sqrt();
        if d < 4.0 {
            return;
        }
        let (ux, uy) = (dx / d, dy / d);
        let (hx, hy) = (bx - ux * 7.0, by - uy * 7.0);
        self.s.push_str(&format!("<path class=\"axp-c3\" d=\"M{ax:.1},{ay:.1} L{hx:.1},{hy:.1}\"/>\
                                  <path class=\"axp-d3\" d=\"M{bx:.1},{by:.1} L{:.1},{:.1} L{:.1},{:.1} Z\"/>",
                                 hx - uy * 5.0, hy + ux * 5.0, hx + uy * 5.0, hy - ux * 5.0));
    }
    /// A filled area between two curves sharing their x values (class axp-s ...).
    pub fn area(&mut self, lower: &[(f64, f64)], upper: &[(f64, f64)], class: &str, name: &str) {
        let ok: Vec<((f64, f64), (f64, f64))> = lower.iter().zip(upper).filter(|(l, u)| {
            l.0.is_finite() && l.1.is_finite() && u.1.is_finite() && (!self.ya.log || (l.1 > 0.0 && u.1 > 0.0))
        }).map(|(l, u)| (*l, *u)).collect();
        if ok.len() < 2 {
            return;
        }
        let mut d = String::new();
        for (i, (_, u)) in ok.iter().enumerate() {
            d.push_str(&format!("{}{:.1},{:.1} ", if i == 0 { "M" } else { "L" }, self.xa.px(u.0), self.ya.px(u.1)));
        }
        for (l, _) in ok.iter().rev() {
            d.push_str(&format!("L{:.1},{:.1} ", self.xa.px(l.0), self.ya.px(l.1)));
        }
        self.s.push_str(&format!("<g clip-path=\"url(#CLIPID)\"><path class=\"{class}\" d=\"{}Z\"/></g>", d));
        let _ = name;
    }
    /// A stem from the axis bottom up to (x, y) with a dot (a spectral line).
    pub fn stem(&mut self, x: f64, y: f64, class_line: &str, class_dot: &str) {
        if !self.xa.inside(x) || !y.is_finite() {
            return;
        }
        let (px, py) = (self.xa.px(x), self.ya.px(y).clamp(CY0, CY1));
        let mut yy = py;
        while yy < CY1 {
            self.pix.push((px, yy));
            yy += 10.0;
        }
        self.s.push_str(&format!("<path class=\"{class_line}\" style=\"stroke-width:1.6\" d=\"M{px:.1},{CY1} L{px:.1},{py:.1}\"/>\
                                  <circle class=\"{class_dot}\" cx=\"{px:.1}\" cy=\"{py:.1}\" r=\"3\"/>"));
    }
}

/// Bars, each with its own fill class, a text above it (its value when None) and a label under it; `split`: a dashed
/// divider before this bar (reactants | products); `log`: a logarithmic value axis.
#[allow(clippy::too_many_arguments)]
pub fn bars_full(labels: &[String], vals: &[f64], classes: &[&str], tops: Option<&[String]>, ylab: &str, header: &str,
                 log: bool, split: Option<usize>) -> Option<String> {
    if labels.is_empty() || labels.len() != vals.len() || vals.iter().any(|v| !v.is_finite()) {
        return None;
    }
    let (mut lo, mut hi) = Chart::range(vals);
    if log {
        if vals.iter().any(|v| *v <= 0.0) {
            return None;
        }
        lo /= 2.0;
    } else {
        lo = lo.min(0.0);
        hi = hi.max(0.0);
    }
    // room above the tallest bar for its text (two lines when given so)
    let lines = tops.map(|t| t.iter().map(|x| x.split('\n').count()).max().unwrap_or(1)).unwrap_or(1) as f64;
    let hi = if log { hi * 2.0 * 2f64.powf(lines - 1.0) } else { hi + (hi - lo) * 0.06 * lines };
    let ya = Axis::new(lo, hi, log, CY1, CY0, !log);
    let mut s = String::new();
    for t in ya.ticks() {
        let y = ya.px(t);
        s.push_str(&format!("<line class=\"axp-g\" x1=\"{CX0}\" y1=\"{y:.1}\" x2=\"{CX1}\" y2=\"{y:.1}\"/>\
                             <text class=\"axp-t\" x=\"{}\" y=\"{:.1}\" text-anchor=\"end\">{}</text>", CX0 - 6.0, y + 5.0,
                            esc(&tick(t))));
    }
    let n = vals.len() as f64;
    let slot = (CX1 - CX0) / n;
    let bw = (slot * 0.7).min(70.0);
    let base = if log { ya.px(ya.lo) } else { ya.px(0.0) };
    let maxlab = labels.iter().flat_map(|l| l.split('\n')).map(|l| l.chars().count()).max().unwrap_or(0) as f64 * 7.4;
    let rot = maxlab > slot;
    for (i, (l, v)) in labels.iter().zip(vals).enumerate() {
        let x = CX0 + slot * (i as f64 + 0.5);
        let y = ya.px(*v);
        let class = classes.get(i).copied().unwrap_or("axp-b");
        s.push_str(&format!("<rect class=\"{class}\" x=\"{:.1}\" y=\"{:.1}\" width=\"{bw:.1}\" height=\"{:.1}\"/>",
                            x - bw / 2.0, y.min(base), (y - base).abs().max(0.5)));
        let t = tops.and_then(|t| t.get(i).cloned()).unwrap_or_else(|| short(*v));
        if !t.is_empty() {
            let lines: Vec<&str> = t.split('\n').collect();
            let longest = lines.iter().map(|l| l.chars().count()).max().unwrap_or(0) as f64;
            let fs = if longest * 7.8 > slot - 6.0 { 10.0 } else { 13.0 };
            let n = lines.len() as f64;
            for (j, line) in lines.iter().enumerate() {
                // values too wide for their slot alternate in height
                let stagger = if longest * fs * 0.62 > slot - 8.0 && i % 2 == 1 { 12.0 } else { 0.0 };
                let ty = if *v >= 0.0 || log { y - 5.0 - stagger - (n - 1.0 - j as f64) * (fs + 2.0) } else {
                    y + 15.0 + stagger + j as f64 * (fs + 2.0)
                };
                s.push_str(&format!("<text class=\"axp-vt\" x=\"{x:.1}\" y=\"{:.1}\" text-anchor=\"middle\" font-size=\"{fs}\">{}</text>",
                                    ty.max(16.0), esc(&clip(line, ((slot + 10.0) / (fs * 0.6)) as usize))));
            }
        }
        if rot {
            s.push_str(&format!("<text class=\"axp-t\" transform=\"translate({x:.1},{:.1}) rotate(-30)\" \
                                 text-anchor=\"end\" font-size=\"12\">{}</text>", CY1 + 14.0, esc(&clip(&l.replace('\n', " "), 14))));
        } else {
            for (j, line) in l.split('\n').enumerate() {
                s.push_str(&format!("<text class=\"axp-t\" x=\"{x:.1}\" y=\"{}\" text-anchor=\"middle\">{}</text>",
                                    CY1 + 18.0 + 16.0 * j as f64, esc(line)));
            }
        }
    }
    if let Some(k) = split {
        let x = CX0 + slot * k as f64;
        s.push_str(&format!("<path class=\"axp-c4\" stroke-dasharray=\"5 4\" d=\"M{x:.1},{CY0} L{x:.1},{CY1}\"/>"));
    }
    if !log && lo < 0.0 {
        s.push_str(&format!("<path class=\"axp-c4\" d=\"M{CX0},{base:.1} L{CX1},{base:.1}\"/>"));
    }
    s.push_str(&format!("<rect class=\"axp-f\" x=\"{CX0}\" y=\"{CY0}\" width=\"{}\" height=\"{}\"/>", CX1 - CX0, CY1 - CY0));
    s.push_str(&format!("<text class=\"axp-l\" transform=\"translate(16,{:.1}) rotate(-90)\" text-anchor=\"middle\">{}</text>",
                        (CY0 + CY1) / 2.0, esc(&clip(ylab, 32))));
    if !header.is_empty() {
        s.push_str(&format!("<text class=\"axp-mt\" x=\"{CX0}\" y=\"22\">{}</text>", esc(&clip(header, 46))));
    }
    Some(s)
}

/// A table of numbers (None: a combination the model refuses, shown as a dot), each cell shaded by its value, its
/// value written; row and column labels; `mark`: one cell ringed. Returns the body and its height.
pub fn num_table(m: &[Vec<Option<f64>>], rows: &[String], cols: &[String], row_title: &str, col_title: &str,
                 header: &str, mark: Option<(usize, usize)>) -> (String, f64) {
    let r = m.len();
    let c = m.first().map(|x| x.len()).unwrap_or(0);
    let mut out = String::new();
    if r == 0 || c == 0 {
        return (out, 100.0);
    }
    let top = 60.0 + if col_title.is_empty() { 0.0 } else { 18.0 };
    let left = 40.0 + rows.iter().map(|x| x.chars().count()).max().unwrap_or(1) as f64 * 8.0;
    let cell = ((500.0 - left) / c as f64).min(64.0).min(300.0 / r as f64).max(16.0);
    let x0 = left + ((500.0 - left) - cell * c as f64) / 2.0;
    let y0 = top;
    if !header.is_empty() {
        out.push_str(&format!("<text class=\"axp-mt\" x=\"16\" y=\"22\">{}</text>", esc(&clip(header, 54))));
    }
    if !col_title.is_empty() {
        out.push_str(&format!("<text class=\"axp-lt\" x=\"{:.1}\" y=\"{:.1}\" text-anchor=\"middle\">{}</text>",
                              x0 + cell * c as f64 / 2.0, top - 26.0, esc(col_title)));
    }
    if !row_title.is_empty() {
        out.push_str(&format!("<text class=\"axp-lt\" x=\"{:.1}\" y=\"{:.1}\" text-anchor=\"end\">{}</text>", x0 - 6.0,
                              top - 26.0, esc(row_title)));
    }
    let mx = m.iter().flatten().flatten().fold(0.0f64, |a, v| a.max(v.abs()));
    for (i, row) in m.iter().enumerate() {
        for (j, v) in row.iter().enumerate() {
            let (x, y) = (x0 + cell * j as f64, y0 + cell * i as f64);
            match v {
                Some(v) => {
                    let op = if mx > 0.0 { 0.08 + 0.6 * v.abs() / mx } else { 0.08 };
                    out.push_str(&format!("<rect class=\"axp-hp\" x=\"{x:.1}\" y=\"{y:.1}\" width=\"{:.1}\" height=\"{:.1}\" \
                                           fill-opacity=\"{op:.2}\"/><text class=\"axp-ht\" x=\"{:.1}\" y=\"{:.1}\" \
                                           text-anchor=\"middle\" font-size=\"14\">{}</text>", cell - 2.0, cell - 2.0,
                                          x + cell / 2.0 - 1.0, y + cell / 2.0 + 4.0, esc(&tick(*v))));
                }
                None => out.push_str(&format!("<rect class=\"axp-f\" x=\"{x:.1}\" y=\"{y:.1}\" width=\"{:.1}\" height=\"{:.1}\"/>\
                                               <text class=\"axp-t\" x=\"{:.1}\" y=\"{:.1}\" text-anchor=\"middle\">.</text>",
                                              cell - 2.0, cell - 2.0, x + cell / 2.0 - 1.0, y + cell / 2.0 + 2.0)),
            }
        }
    }
    if let Some((i, j)) = mark {
        out.push_str(&format!("<rect class=\"axp-c2\" x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\" style=\"stroke-width:3\"/>",
                              x0 + cell * j as f64 - 1.0, y0 + cell * i as f64 - 1.0, cell, cell));
    }
    for (i, l) in rows.iter().enumerate() {
        out.push_str(&format!("<text class=\"axp-bt\" x=\"{:.1}\" y=\"{:.1}\" text-anchor=\"end\">{}</text>", x0 - 8.0,
                              y0 + cell * (i as f64 + 0.5) + 4.0, esc(l)));
    }
    for (j, l) in cols.iter().enumerate() {
        out.push_str(&format!("<text class=\"axp-bt\" x=\"{:.1}\" y=\"{:.1}\" text-anchor=\"middle\">{}</text>",
                              x0 + cell * (j as f64 + 0.5), y0 - 8.0, esc(l)));
    }
    (out, y0 + cell * r as f64 + 20.0)
}

/// A small legend of filled boxes at the top right of a chart body.
pub fn fill_legend(items: &[(&str, &str)]) -> String {
    let mut out = String::new();
    let mut x = CX1;
    for (class, name) in items.iter().rev() {
        let w = name.chars().count() as f64 * 7.4 + 22.0;
        x -= w;
        out.push_str(&format!("<rect class=\"{class}\" x=\"{x:.1}\" y=\"28\" width=\"12\" height=\"10\"/>\
                               <text class=\"axp-lt\" x=\"{:.1}\" y=\"38\">{}</text>", x + 16.0, esc(name)));
    }
    out
}
