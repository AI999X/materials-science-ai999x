//! Drawing helpers shared by this model's own figures (figures*.rs): a value over two inputs with iso-lines (field),
//! one output against one input (a sweep), a family of curves, bars over the options of a choice, a verdict over two
//! inputs, an x-y chart with dots and lines, and strips of letters (sequences). Every number drawn is one the model
//! returned (an answer, or a further solve of the model through `Solved`); a figure that cannot be drawn for the
//! inputs is None. The same file is used by several models of the portfolio (each keeps its own copy).
#![allow(dead_code)]

use ax_engine::figkit::{self as k, num, Solved};
use ax_engine::figures::Figure;
use ax_engine::Input;

pub fn esc(s: &str) -> String {
    s.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;").replace('"', "&quot;")
}

/// A label cut to n characters ("..." when cut).
pub fn clip(s: &str, n: usize) -> String {
    let c: Vec<char> = s.chars().collect();
    if c.len() <= n { s.to_string() } else { format!("{}...", c[..n.saturating_sub(3)].iter().collect::<String>()) }
}

// ------------------------------------------------------------------------------------------------ axes
/// A value axis mapped onto pixels a..b (logarithmic when `log`).
#[derive(Clone, Copy, Debug)]
pub struct Axis {
    pub lo: f64,
    pub hi: f64,
    pub log: bool,
    pub a: f64,
    pub b: f64,
}

impl Axis {
    /// An axis over lo..hi; `pad`: a margin of 5 % (linear) or a factor 1.25 (log) on each side.
    pub fn new(lo: f64, hi: f64, log: bool, a: f64, b: f64, pad: bool) -> Axis {
        let (mut lo, mut hi) = if lo <= hi { (lo, hi) } else { (hi, lo) };
        if log {
            if !(lo > 0.0) {
                lo = hi / 100.0;
            }
            if !(hi > lo) {
                hi = lo * 10.0;
                lo /= 10.0;
            }
            if pad {
                lo /= 1.25;
                hi *= 1.25;
            }
        } else {
            if !(hi > lo) || !(hi - lo).is_finite() {
                let d = if lo == 0.0 { 1.0 } else { lo.abs() * 0.1 };
                lo -= d;
                hi += d;
            }
            if pad {
                let d = (hi - lo) * 0.05;
                lo -= d;
                hi += d;
            }
        }
        Axis { lo, hi, log, a, b }
    }
    pub fn px(&self, v: f64) -> f64 {
        let f = if self.log { (v.log10() - self.lo.log10()) / (self.hi.log10() - self.lo.log10()) } else {
            (v - self.lo) / (self.hi - self.lo)
        };
        self.a + f * (self.b - self.a)
    }
    pub fn inside(&self, v: f64) -> bool {
        v.is_finite() && (!self.log || v > 0.0) && v >= self.lo.min(self.hi) - 1e-12 * self.hi.abs().max(1e-300)
            && v <= self.hi.max(self.lo) + 1e-12 * self.hi.abs().max(1e-300)
    }
    pub fn ticks(&self) -> Vec<f64> {
        if self.log {
            let (e0, e1) = (self.lo.log10().ceil() as i32, self.hi.log10().floor() as i32);
            let n = (e1 - e0 + 1).max(1);
            let step = ((n as f64) / 6.0).ceil().max(1.0) as usize;
            let mut v: Vec<f64> = (e0..=e1).step_by(step).map(|e| 10f64.powi(e)).collect();
            if v.len() < 2 {
                // less than a decade: 1-2-5 marks
                v.clear();
                for e in (self.lo.log10().floor() as i32)..=(self.hi.log10().ceil() as i32) {
                    for m in [1.0, 2.0, 5.0] {
                        let t = m * 10f64.powi(e);
                        if t >= self.lo && t <= self.hi {
                            v.push(t);
                        }
                    }
                }
            }
            return v;
        }
        nice_ticks(self.lo, self.hi, 5)
    }
}

pub fn nice_step(span: f64, n: usize) -> f64 {
    let raw = span / n as f64;
    let mag = 10f64.powf(raw.log10().floor());
    let norm = raw / mag;
    mag * if norm < 1.5 { 1.0 } else if norm < 3.0 { 2.0 } else if norm < 7.0 { 5.0 } else { 10.0 }
}

pub fn nice_ticks(lo: f64, hi: f64, n: usize) -> Vec<f64> {
    let span = hi - lo;
    if !(span > 0.0) || !span.is_finite() {
        return vec![lo];
    }
    let step = nice_step(span, n);
    let mut v = vec![];
    let mut t = (lo / step).ceil() * step;
    while t <= hi + step * 1e-9 && v.len() < 12 {
        v.push(if t.abs() < step * 1e-9 { 0.0 } else { t });
        t += step;
    }
    v
}

/// A tick's text: 4 significant digits, no trailing noise.
pub fn tick(v: f64) -> String {
    // a compact exponent: 2.5e11 rather than 2.5e+11, 1e-5 rather than 1e-05
    let t = num(v);
    match t.split_once('e') {
        Some((m, e)) => {
            let (sign, d) = if let Some(d) = e.strip_prefix('-') { ("-", d) } else { ("", e.trim_start_matches('+')) };
            format!("{m}e{sign}{}", d.trim_start_matches('0'))
        }
        None => t,
    }
}

// ------------------------------------------------------------------------------------------------ x-y charts
/// An x-y chart of the generic 520 x 340 frame: lines, dots, a marked point, a legend, text. Build, then `body()`.
pub struct Chart {
    pub xa: Axis,
    pub ya: Axis,
    pub s: String,
    pub legend: Vec<(String, String)>,
    /// the drawn points in pixels (the legend goes to the emptiest corner)
    pub pix: Vec<(f64, f64)>,
    pub h: f64,
}

pub const CX0: f64 = 84.0;
pub const CX1: f64 = 500.0;
pub const CY0: f64 = 44.0;
pub const CY1: f64 = 280.0;

impl Chart {
    /// A chart over the x and y ranges given (`pad` adds a margin), axis labels and a header line.
    #[allow(clippy::too_many_arguments)]
    pub fn new(xr: (f64, f64), yr: (f64, f64), log: (bool, bool), pad: (bool, bool), xlab: &str, ylab: &str,
               header: &str) -> Chart {
        let xa = Axis::new(xr.0, xr.1, log.0, CX0, CX1, pad.0);
        let ya = Axis::new(yr.0, yr.1, log.1, CY1, CY0, pad.1);
        let mut c = Chart { xa, ya, s: String::new(), legend: vec![], pix: vec![], h: 340.0 };
        c.frame(xlab, ylab, header);
        c.s.push_str(&format!("<clipPath id=\"CLIPID\"><rect x=\"{CX0}\" y=\"{CY0}\" width=\"{}\" height=\"{}\"/></clipPath>",
                              CX1 - CX0, CY1 - CY0));
        c
    }
    /// The range of a set of values (both ends finite; a single value widened).
    pub fn range(v: &[f64]) -> (f64, f64) {
        let f: Vec<f64> = v.iter().copied().filter(|x| x.is_finite()).collect();
        if f.is_empty() {
            return (0.0, 1.0);
        }
        let lo = f.iter().cloned().fold(f64::INFINITY, f64::min);
        let hi = f.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        (lo, hi)
    }
    /// The range of positive values (for a logarithmic axis).
    pub fn range_pos(v: &[f64]) -> (f64, f64) {
        let f: Vec<f64> = v.iter().copied().filter(|x| x.is_finite() && *x > 0.0).collect();
        Chart::range(&f)
    }
    fn frame(&mut self, xlab: &str, ylab: &str, header: &str) {
        for t in self.xa.ticks() {
            let x = self.xa.px(t);
            self.s.push_str(&format!("<line class=\"axp-g\" x1=\"{x:.1}\" y1=\"{CY0}\" x2=\"{x:.1}\" y2=\"{CY1}\"/>\
                                      <text class=\"axp-t\" x=\"{x:.1}\" y=\"{}\" text-anchor=\"middle\">{}</text>",
                                     CY1 + 20.0, esc(&tick(t))));
        }
        for t in self.ya.ticks() {
            let y = self.ya.px(t);
            self.s.push_str(&format!("<line class=\"axp-g\" x1=\"{CX0}\" y1=\"{y:.1}\" x2=\"{CX1}\" y2=\"{y:.1}\"/>\
                                      <text class=\"axp-t\" x=\"{}\" y=\"{:.1}\" text-anchor=\"end\">{}</text>",
                                     CX0 - 6.0, y + 5.0, esc(&tick(t))));
        }
        self.s.push_str(&format!("<rect class=\"axp-f\" x=\"{CX0}\" y=\"{CY0}\" width=\"{}\" height=\"{}\"/>", CX1 - CX0,
                                 CY1 - CY0));
        if !xlab.is_empty() {
            self.s.push_str(&format!("<text class=\"axp-l\" x=\"{:.1}\" y=\"326\" text-anchor=\"middle\">{}</text>",
                                     (CX0 + CX1) / 2.0, esc(&clip(xlab, 60))));
        }
        if !ylab.is_empty() {
            self.s.push_str(&format!("<text class=\"axp-l\" transform=\"translate(16,{:.1}) rotate(-90)\" \
                                      text-anchor=\"middle\">{}</text>", (CY0 + CY1) / 2.0, esc(&clip(ylab, 32))));
        }
        if !header.is_empty() {
            self.s.push_str(&format!("<text class=\"axp-mt\" x=\"{CX0}\" y=\"22\">{}</text>", esc(&clip(header, 46))));
        }
    }
    fn ok(&self, p: (f64, f64)) -> bool {
        p.0.is_finite() && p.1.is_finite() && (!self.xa.log || p.0 > 0.0) && (!self.ya.log || p.1 > 0.0)
    }
    /// A polyline (class axp-c, axp-c2, axp-c3, axp-c4); a point outside the domain breaks it. `name`: a legend entry.
    pub fn line(&mut self, pts: &[(f64, f64)], class: &str, name: &str, dashed: bool) {
        let mut d = String::new();
        let mut pen = false;
        for p in pts {
            if !self.ok(*p) {
                pen = false;
                continue;
            }
            let (x, y) = (self.xa.px(p.0), self.ya.px(p.1));
            self.pix.push((x, y));
            d.push_str(&format!("{}{x:.1},{y:.1} ", if pen { "L" } else { "M" }));
            pen = true;
        }
        if d.is_empty() {
            return;
        }
        let dash = if dashed { " stroke-dasharray=\"6 4\"" } else { "" };
        self.s.push_str(&format!("<g clip-path=\"url(#CLIPID)\"><path class=\"{class}\"{dash} d=\"{}\"/></g>", d.trim_end()));
        if !name.is_empty() {
            self.legend.push((class.to_string(), name.to_string()));
        }
    }
    /// Dots (class axp-d, axp-d2, axp-d3, axp-d4).
    pub fn dots(&mut self, pts: &[(f64, f64)], class: &str, r: f64) {
        for p in pts {
            if self.ok(*p) && self.xa.inside(p.0) && self.ya.inside(p.1) {
                self.pix.push((self.xa.px(p.0), self.ya.px(p.1)));
                self.s.push_str(&format!("<circle class=\"{class}\" cx=\"{:.1}\" cy=\"{:.1}\" r=\"{r}\"/>", self.xa.px(p.0),
                                         self.ya.px(p.1)));
            }
        }
    }
    /// A shaded band under or between two y values over x from x0 to x1 (class axp-s ...).
    pub fn xband(&mut self, x0: f64, x1: f64, class: &str) {
        let (a, b) = (self.xa.px(x0).clamp(CX0, CX1), self.xa.px(x1).clamp(CX0, CX1));
        self.s.push_str(&format!("<rect class=\"{class}\" x=\"{:.1}\" y=\"{CY0}\" width=\"{:.1}\" height=\"{}\"/>",
                                 a.min(b), (b - a).abs(), CY1 - CY0));
    }
    /// A horizontal guide line at y with a label at its right end.
    pub fn hguide(&mut self, y: f64, label: &str) {
        if !self.ok((self.xa.lo.max(1e-300), y)) || !self.ya.inside(y) {
            return;
        }
        let py = self.ya.px(y);
        self.s.push_str(&format!("<path class=\"axp-c4\" stroke-dasharray=\"5 4\" d=\"M{CX0},{py:.1} L{CX1},{py:.1}\"/>"));
        if !label.is_empty() {
            self.text(CX1 - 4.0, py - 5.0, "end", "axp-lt", label);
        }
    }
    /// A vertical guide line at x with a label at its top.
    pub fn vguide(&mut self, x: f64, label: &str) {
        if !self.xa.inside(x) {
            return;
        }
        let px = self.xa.px(x);
        self.s.push_str(&format!("<path class=\"axp-c4\" stroke-dasharray=\"5 4\" d=\"M{px:.1},{CY0} L{px:.1},{CY1}\"/>"));
        if !label.is_empty() {
            let anchor = if px > (CX0 + CX1) / 2.0 { "end" } else { "start" };
            let dx = if anchor == "end" { -4.0 } else { 4.0 };
            self.text(px + dx, CY0 + 14.0, anchor, "axp-lt", label);
        }
    }
    /// The visitor's point, ringed, with its label placed inside the plot.
    pub fn mark(&mut self, p: (f64, f64), label: &str) {
        if !self.ok(p) || !self.xa.inside(p.0) || !self.ya.inside(p.1) {
            return;
        }
        let (x, y) = (self.xa.px(p.0), self.ya.px(p.1));
        for k in 0..6 {
            self.pix.push((x + 14.0 * k as f64, y - 10.0));
        }
        self.s.push_str(&format!("<circle class=\"axp-m\" cx=\"{x:.1}\" cy=\"{y:.1}\" r=\"6\"/>"));
        if !label.is_empty() {
            let w = k::text_w(label, true) * 1.15;
            let (tx, anchor) = if x + 10.0 + w <= CX1 { (x + 10.0, "start") } else { (x - 10.0, "end") };
            let ty = if y - 10.0 < CY0 + 14.0 { y + 20.0 } else { y - 10.0 };
            self.text(tx, ty, anchor, "axp-mt", label);
        }
    }
    /// Text kept inside the canvas.
    pub fn text(&mut self, x: f64, y: f64, anchor: &str, class: &str, t: &str) {
        if t.is_empty() {
            return;
        }
        let per = if class == "axp-mt" { 9.0 } else if class == "axp-vt" || class == "axp-t" { 8.0 } else { 7.6 };
        let w = t.chars().count() as f64 * per;
        let left = match anchor {
            "middle" => x - w / 2.0,
            "end" => x - w,
            _ => x,
        };
        let shift = if left < 4.0 { 4.0 - left } else if left + w > 516.0 { 516.0 - (left + w) } else { 0.0 };
        self.s.push_str(&format!("<text class=\"{class}\" x=\"{:.1}\" y=\"{:.1}\" text-anchor=\"{anchor}\">{}</text>",
                                 x + shift, y.clamp(14.0, self.h - 4.0), esc(t)));
    }
    /// The drawing: the legend (when several named lines) placed at the top right.
    pub fn body(&mut self) -> String {
        let mut out = self.s.clone();
        if !self.legend.is_empty() {
            let w = self.legend.iter().map(|l| l.1.chars().count()).max().unwrap_or(0) as f64 * 7.6 + 30.0;
            let hgt = 18.0 * self.legend.len() as f64 + 4.0;
            // the corner of the plot where the fewest drawn points fall
            let corners = [(CX1 - w - 6.0, CY0 + 4.0), (CX0 + 10.0, CY0 + 4.0), (CX1 - w - 6.0, CY1 - hgt - 4.0),
                           (CX0 + 10.0, CY1 - hgt - 4.0)];
            let busy = |c: &(f64, f64)| self.pix.iter().filter(|p| p.0 >= c.0 - 10.0 && p.0 <= c.0 + w + 10.0
                                                            && p.1 >= c.1 - 6.0 && p.1 <= c.1 + hgt + 6.0).count();
            let best = corners.iter().min_by_key(|c| busy(c)).copied().unwrap_or(corners[0]);
            // every corner crossed by a curve: one row above the plot, right-aligned, when it fits
            let row: f64 = self.legend.iter().map(|l| l.1.chars().count() as f64 * 7.6 + 34.0).sum();
            if busy(&best) > 2 && row <= CX1 - CX0 {
                let mut x = CX1 - row + 4.0;
                for (class, name) in &self.legend {
                    out.push_str(&format!("<path class=\"{class}\" d=\"M{x:.1},34 L{:.1},34\"/>\
                                           <text class=\"axp-lt\" x=\"{:.1}\" y=\"38\">{}</text>", x + 20.0, x + 26.0,
                                          esc(name)));
                    x += name.chars().count() as f64 * 7.6 + 34.0;
                }
                return out;
            }
            let x = best.0 + 4.0;
            let mut y = best.1 + 12.0;
            out.push_str(&format!("<rect class=\"axp-ka\" x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\"/>", x - 4.0,
                                  best.1, w + 8.0, hgt));
            for (class, name) in &self.legend {
                out.push_str(&format!("<path class=\"{class}\" d=\"M{x:.1},{:.1} L{:.1},{:.1}\"/>\
                                       <text class=\"axp-lt\" x=\"{:.1}\" y=\"{:.1}\">{}</text>", y - 4.0, x + 20.0, y - 4.0,
                                      x + 26.0, y, esc(name)));
                y += 18.0;
            }
        }
        out
    }
}

pub const LINES: [&str; 4] = ["axp-c", "axp-c2", "axp-c3", "axp-c4"];
pub const DOTS: [&str; 4] = ["axp-d", "axp-d2", "axp-d3", "axp-d4"];
pub const FILLS: [&str; 4] = ["axp-b", "axp-b2", "axp-b3", "axp-b4"];

// ------------------------------------------------------------------------------------------------ inputs
/// The meaning of an input or output as the model words it (its symbol when none).
pub fn meaning(s: &Solved, sym: &str) -> String {
    let Some(p) = s.p.procedure(s.key) else { return sym.to_string() };
    let m = p.inputs.iter().find(|i| i.symbol == sym).map(|i| i.meaning.clone())
        .or_else(|| p.outputs.iter().find(|o| o.symbol == sym).map(|o| o.meaning.clone()))
        .unwrap_or_default();
    // "(e.g. in kg)" and "(optional; ...)" are for the form, not for a figure
    let mut m = m.split(" (e.g.").next().unwrap_or("").to_string();
    if let Some(i) = m.find(" (optional") {
        m.truncate(i);
    }
    if m.is_empty() { sym.to_string() } else { m }
}

/// A quantity of the request: a number input (an integer input too), an output, or a formula's answer.
pub fn getv(s: &Solved, sym: &str) -> Option<f64> {
    s.get(sym).or_else(|| match s.val(sym)? {
        ax_engine::Val::Int(i) => Some(i as f64),
        ax_engine::Val::Num(x) if x.is_finite() => Some(x),
        _ => None,
    })
}

/// Whether an input is read as an integer.
pub fn is_int(s: &Solved, sym: &str) -> bool {
    s.p.procedure(s.key).and_then(|p| p.inputs.iter().find(|i| i.symbol == sym).map(|i| i.ty == "integer"))
        .unwrap_or(false)
}

/// The choices of a choice input.
pub fn choices(s: &Solved, sym: &str) -> Vec<String> {
    s.p.procedure(s.key).and_then(|p| p.inputs.iter().find(|i| i.symbol == sym).map(|i| i.choices.clone()))
        .unwrap_or_default()
}

/// The range a figure explores round the visitor's value: half to one and a half times it (a tenth to ten times on a
/// logarithmic axis), -1..1 round zero.
pub fn around(v: f64, log: bool) -> Option<(f64, f64)> {
    if !v.is_finite() {
        return None;
    }
    if log {
        return (v > 0.0).then_some((v / 10.0, v * 10.0));
    }
    if v == 0.0 {
        return Some((-1.0, 1.0));
    }
    let (a, b) = (v * 0.5, v * 1.5);
    Some((a.min(b), a.max(b)))
}

/// Grid values over lo..hi (integers only for an integer input: the distinct whole numbers, at most n).
pub fn grid(lo: f64, hi: f64, n: usize, log: bool, int: bool) -> Vec<f64> {
    if int {
        let (a, b) = (lo.ceil().max(if log { 1.0 } else { f64::NEG_INFINITY }), hi.floor());
        if b < a {
            return vec![];
        }
        let cnt = (b - a) as usize + 1;
        if cnt <= n {
            return (0..cnt).map(|i| a + i as f64).collect();
        }
        let mut v: Vec<f64> = k::span(a, b, n, log).into_iter().map(|x| x.round()).collect();
        v.dedup();
        return v;
    }
    k::span(lo, hi, n, log)
}

/// The model solved again with two number inputs set, one output read.
pub fn solve2(s: &Solved, x: &str, xv: f64, y: &str, yv: f64, z: &str) -> Option<f64> {
    let set: Vec<(&str, Input)> = vec![(x, num_input(s, x, xv)), (y, num_input(s, y, yv))];
    let a = s.with_input(&set)?;
    k::num_of(&a, z)
}

/// A number as the input expects it (an integer input gets an integer).
pub fn num_input(s: &Solved, sym: &str, v: f64) -> Input {
    if is_int(s, sym) { Input::Int(v.round() as i64) } else { Input::Float(v) }
}

/// One output against one input, the other inputs as given.
pub fn curve(s: &Solved, x: &str, xs: &[f64], y: &str) -> Vec<(f64, f64)> {
    xs.iter().filter_map(|xv| {
        let a = s.with_input(&[(x, num_input(s, x, *xv))])?;
        k::num_of(&a, y).map(|yv| (*xv, yv))
    }).collect()
}

// ------------------------------------------------------------------------------------------------ field
/// What a field map shows: the output z over the inputs x (across) and y (up).
pub struct FieldSpec<'a> {
    pub z: &'a str,
    pub x: &'a str,
    pub y: &'a str,
    pub log: (bool, bool),
    pub title: &'a str,
    /// the unit written after z ("" for none)
    pub zunit: &'a str,
}

/// A value over two inputs: each of the up to 21 x 15 cells one run of the model, shaded by its value (darker =
/// larger; negative values in the second colour), iso-lines at round values, a colour scale, the visitor's point.
pub fn field_map(s: &Solved, f: &FieldSpec) -> Option<Figure> {
    field_map_in(s, f, None, None)
}

/// `field_map` over given input ranges (None: round the visitor's value).
pub fn field_map_in(s: &Solved, f: &FieldSpec, xr: Option<(f64, f64)>, yr: Option<(f64, f64)>) -> Option<Figure> {
    let (x0, y0, z0) = (getv(s, f.x)?, getv(s, f.y)?, getv(s, f.z)?);
    let xr = match xr { Some(r) => r, None => around(x0, f.log.0)? };
    let yr = match yr { Some(r) => r, None => around(y0, f.log.1)? };
    let (ix, iy) = (is_int(s, f.x), is_int(s, f.y));
    let xs = grid(xr.0, xr.1, 21, f.log.0, ix);
    let ys = grid(yr.0, yr.1, 15, f.log.1, iy);
    if xs.len() < 2 || ys.len() < 2 || xs.len() * ys.len() > s.left() {
        return None;
    }
    let mut vals: Vec<Vec<Option<f64>>> = vec![vec![None; ys.len()]; xs.len()];
    for (i, xv) in xs.iter().enumerate() {
        for (j, yv) in ys.iter().enumerate() {
            vals[i][j] = solve2(s, f.x, *xv, f.y, *yv, f.z);
        }
    }
    let all: Vec<f64> = vals.iter().flatten().flatten().copied().collect();
    if all.len() < 4 {
        return None;
    }
    let (lo, hi) = Chart::range(&all);
    let lo = k::snap(lo, hi);
    let logz = lo > 0.0 && hi / lo > 300.0;
    let (px0, px1, py0, py1) = (78.0, 418.0, 20.0, 298.0);
    let xa = Axis::new(xs[0], xs[xs.len() - 1], f.log.0, px0, px1, false);
    let ya = Axis::new(ys[0], ys[ys.len() - 1], f.log.1, py1, py0, false);
    let edge = |v: &[f64], i: usize, lg: bool| -> (f64, f64) {
        let m = |a: f64, b: f64| if lg { (a * b).sqrt() } else { (a + b) / 2.0 };
        let l = if i == 0 { v[0] } else { m(v[i - 1], v[i]) };
        let r = if i + 1 >= v.len() { v[i] } else { m(v[i], v[i + 1]) };
        (l, r)
    };
    let amax = all.iter().fold(0.0f64, |a, v| a.max(v.abs()));
    let shade = |v: f64| -> (&'static str, f64) {
        if lo < 0.0 && hi > 0.0 {
            // a signed value: its magnitude, positive and negative in two colours
            (if v < 0.0 { "axp-hn" } else { "axp-hp" }, 0.08 + 0.74 * ((v.abs() / amax.max(1e-300)) * 9.999).floor() / 9.0)
        } else if logz {
            ("axp-hp", 0.08 + 0.74 * (((v.log10() - lo.log10()) / (hi.log10() - lo.log10())) * 9.999).floor() / 9.0)
        } else if hi > lo {
            // ten bands, light to dark
            ("axp-hp", 0.08 + 0.74 * (((v - lo) / (hi - lo)) * 9.999).floor() / 9.0)
        } else {
            ("axp-hp", 0.4)
        }
    };
    let mut out = String::new();
    out.push_str(&format!("<clipPath id=\"CLIPID\"><rect x=\"{px0}\" y=\"{py0}\" width=\"{}\" height=\"{}\"/></clipPath>",
                          px1 - px0, py1 - py0));
    out.push_str("<g clip-path=\"url(#CLIPID)\">");
    for (i, col) in vals.iter().enumerate() {
        for (j, v) in col.iter().enumerate() {
            let Some(v) = v else { continue };
            let (l, r) = edge(&xs, i, f.log.0);
            let (b, t) = edge(&ys, j, f.log.1);
            let (x1, x2) = (xa.px(l), xa.px(r));
            let (y1, y2) = (ya.px(t), ya.px(b));
            let (class, op) = shade(*v);
            out.push_str(&format!("<rect class=\"{class}\" x=\"{x1:.1}\" y=\"{y1:.1}\" width=\"{:.1}\" height=\"{:.1}\" \
                                   fill-opacity=\"{:.2}\" shape-rendering=\"crispEdges\"/>", (x2 - x1).abs(), (y2 - y1).abs(),
                                  op.clamp(0.04, 0.9)));
        }
    }
    // iso-lines by marching squares over the cell centres
    let levels: Vec<f64> = if logz {
        let mut v = vec![];
        for e in (lo.log10().ceil() as i32)..=(hi.log10().floor() as i32) {
            v.push(10f64.powi(e));
        }
        v
    } else {
        nice_ticks(lo, hi, 5).into_iter().filter(|l| *l > lo && *l < hi).collect()
    };
    let mut labels: Vec<(f64, f64, String)> = vec![];
    for lv in &levels {
        let mut d = String::new();
        let mut mids: Vec<(f64, f64)> = vec![];
        for i in 0..xs.len() - 1 {
            for j in 0..ys.len() - 1 {
                let c = [vals[i][j], vals[i + 1][j], vals[i + 1][j + 1], vals[i][j + 1]];
                if c.iter().any(|v| v.is_none()) {
                    continue;
                }
                let c: Vec<f64> = c.iter().map(|v| v.unwrap()).collect();
                let p = [(xs[i], ys[j]), (xs[i + 1], ys[j]), (xs[i + 1], ys[j + 1]), (xs[i], ys[j + 1])];
                let mut pts: Vec<(f64, f64)> = vec![];
                for e in 0..4 {
                    let (a, b) = (e, (e + 1) % 4);
                    let (va, vb) = (c[a], c[b]);
                    if (va - lv) * (vb - lv) < 0.0 || (va == *lv && vb != *lv) {
                        let t = (lv - va) / (vb - va);
                        let (pa, pb) = ((xa.px(p[a].0), ya.px(p[a].1)), (xa.px(p[b].0), ya.px(p[b].1)));
                        pts.push((pa.0 + t * (pb.0 - pa.0), pa.1 + t * (pb.1 - pa.1)));
                    }
                }
                if pts.len() >= 2 {
                    d.push_str(&format!("M{:.1},{:.1} L{:.1},{:.1} ", pts[0].0, pts[0].1, pts[1].0, pts[1].1));
                    if pts.len() == 4 {
                        d.push_str(&format!("M{:.1},{:.1} L{:.1},{:.1} ", pts[2].0, pts[2].1, pts[3].0, pts[3].1));
                    }
                    let m = ((pts[0].0 + pts[1].0) / 2.0, (pts[0].1 + pts[1].1) / 2.0);
                    if m.0 > px0 + 20.0 && m.0 < px1 - 24.0 && m.1 > py0 + 16.0 && m.1 < py1 - 8.0 {
                        mids.push(m);
                    }
                }
            }
        }
        if !d.is_empty() {
            out.push_str(&format!("<path class=\"axp-c4\" style=\"stroke-width:1.3\" d=\"{}\"/>", d.trim_end()));
            // the label near the middle of the line, away from the labels already placed and from the marked point
            if !mids.is_empty() {
                let cx = mids.iter().map(|m| m.0).sum::<f64>() / mids.len() as f64;
                let cy = mids.iter().map(|m| m.1).sum::<f64>() / mids.len() as f64;
                mids.sort_by(|a, b| {
                    let da = (a.0 - cx).powi(2) + (a.1 - cy).powi(2);
                    let db = (b.0 - cx).powi(2) + (b.1 - cy).powi(2);
                    da.partial_cmp(&db).unwrap_or(std::cmp::Ordering::Equal)
                });
                let mk = (xa.px(x0), ya.px(y0));
                if let Some(m) = mids.iter().find(|m| {
                    labels.iter().all(|l| (l.0 - m.0).abs() > (l.2.chars().count() + tick(*lv).chars().count()) as f64 * 3.6 + 8.0 || (l.1 - m.1).abs() > 16.0)
                        && ((m.0 - mk.0).abs() > 30.0 || (m.1 - mk.1).abs() > 22.0)
                }) {
                    labels.push((m.0, m.1, tick(*lv)));
                }
            }
        }
    }
    out.push_str("</g>");
    for (x, y, t) in labels {
        out.push_str(&format!("<text class=\"axp-t\" x=\"{:.1}\" y=\"{:.1}\" text-anchor=\"middle\" font-size=\"11\">{}</text>",
                              x, y - 3.0, esc(&t)));
    }
    // axes
    for t in xa.ticks() {
        let x = xa.px(t);
        out.push_str(&format!("<text class=\"axp-t\" x=\"{x:.1}\" y=\"318\" text-anchor=\"middle\">{}</text>", esc(&tick(t))));
    }
    for t in ya.ticks() {
        let y = ya.px(t);
        out.push_str(&format!("<text class=\"axp-t\" x=\"70\" y=\"{:.1}\" text-anchor=\"end\">{}</text>", y + 5.0, esc(&tick(t))));
    }
    out.push_str(&format!("<rect class=\"axp-f\" x=\"{px0}\" y=\"{py0}\" width=\"{}\" height=\"{}\"/>", px1 - px0, py1 - py0));
    let xlab = axis_label(s, f.x);
    let ylab = axis_label(s, f.y);
    out.push_str(&format!("<text class=\"axp-l\" x=\"{:.1}\" y=\"346\" text-anchor=\"middle\">{}</text>", (px0 + px1) / 2.0,
                          esc(&clip(&xlab, 44))));
    out.push_str(&format!("<text class=\"axp-l\" transform=\"translate(16,{:.1}) rotate(-90)\" text-anchor=\"middle\">{}</text>",
                          (py0 + py1) / 2.0, esc(&clip(&ylab, 34))));
    // the colour scale
    let (sx, sw): (f64, f64) = (440.0, 14.0);
    for kk in 0..20 {
        let t = kk as f64 / 19.0;
        let v = if logz { lo * (hi / lo).powf(t) } else { lo + (hi - lo) * t };
        let (class, op) = shade(v);
        let y = py1 - (py1 - py0) * (kk as f64 + 1.0) / 20.0;
        out.push_str(&format!("<rect class=\"{class}\" x=\"{sx}\" y=\"{y:.1}\" width=\"{sw}\" height=\"{:.1}\" fill-opacity=\"{:.2}\"/>",
                              (py1 - py0) / 20.0 + 0.4, op.clamp(0.04, 0.9)));
    }
    out.push_str(&format!("<rect class=\"axp-f\" x=\"{sx}\" y=\"{py0}\" width=\"{sw}\" height=\"{}\"/>", py1 - py0));
    // a long scale value moves left over the bar rather than leave the canvas
    let lx = |t: &str| (sx + sw + 4.0).min(516.0 - t.chars().count() as f64 * 7.6);
    let (th, tl) = (tick(hi), tick(lo));
    out.push_str(&format!("<text class=\"axp-t\" x=\"{:.1}\" y=\"{}\" text-anchor=\"start\" font-size=\"11\">{}</text>",
                          lx(&th), py0 - 4.0 + if lx(&th) < sx + sw { 0.0 } else { 14.0 }, esc(&th)));
    out.push_str(&format!("<text class=\"axp-t\" x=\"{:.1}\" y=\"{}\" text-anchor=\"start\" font-size=\"11\">{}</text>",
                          lx(&tl), if lx(&tl) < sx + sw { py1 + 14.0 } else { py1 }, esc(&tl)));
    out.push_str(&format!("<text class=\"axp-lt\" x=\"{}\" y=\"{:.1}\" text-anchor=\"middle\" font-size=\"11\">{}</text>",
                          sx + sw / 2.0 + 18.0, py1 + 32.0, esc(&clip(f.z, 12))));
    // the visitor's point
    if xa.inside(x0) && ya.inside(y0) {
        let (px, py) = (xa.px(x0), ya.px(y0));
        let lab = format!("{} = {}", f.z, tick(z0));
        let w = lab.chars().count() as f64 * 9.0;
        let (tx, anchor) = if px + 12.0 + w <= px1 { (px + 12.0, "start") } else { (px - 12.0, "end") };
        let ty = if py - 12.0 < py0 + 14.0 { py + 22.0 } else { py - 12.0 };
        out.push_str(&format!("<circle class=\"axp-k\" cx=\"{px:.1}\" cy=\"{py:.1}\" r=\"8\"/><circle class=\"axp-m\" cx=\"{px:.1}\" \
                               cy=\"{py:.1}\" r=\"4\"/><rect class=\"axp-ka\" x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"20\"/>\
                               <text class=\"axp-mt\" x=\"{tx:.1}\" y=\"{ty:.1}\" text-anchor=\"{anchor}\">{}</text>",
                              if anchor == "start" { tx - 3.0 } else { tx - w - 3.0 }, ty - 15.0, w + 6.0, esc(&lab)));
    }
    let unit = if f.zunit.is_empty() { String::new() } else { format!(" {}", f.zunit) };
    let caption = format!("{} ({}) over {} from {} to {} and {} from {} to {}{}: each of the {} x {} cells is one run of the \
                           model, shaded by its value (light {} to dark {}{unit}){}{}. Marked: the inputs given, {} = {}, \
                           {} = {}, {} = {}{unit}.",
                          f.z, meaning(s, f.z), f.x, tick(xs[0]), tick(xs[xs.len() - 1]), f.y, tick(ys[0]),
                          tick(ys[ys.len() - 1]),
                          match f.log { (true, true) => " (logarithmic axes)", (true, false) => " (logarithmic horizontal axis)",
                                        (false, true) => " (logarithmic vertical axis)", _ => "" },
                          xs.len(), ys.len(), tick(lo), tick(hi),
                          if levels.is_empty() { String::new() } else {
                              format!("; iso-lines at {} = {}", f.z, levels.iter().map(|l| tick(*l)).collect::<Vec<_>>().join(", "))
                          },
                          if vals.iter().flatten().any(|v| v.is_none()) { "; blank cells: inputs the model refuses" } else { "" },
                          f.x, tick(x0), f.y, tick(y0), f.z, tick(z0));
    Some(s.figure("field", f.title, &caption, &out, 360.0))
}

/// "meaning (symbol, unit)" for an axis.
pub fn axis_label(s: &Solved, sym: &str) -> String {
    let u = si_unit(s, sym);
    let m = meaning(s, sym);
    let m = if m == sym { String::new() } else { format!("{m}, ") };
    if u.is_empty() { format!("{m}{sym}") } else { format!("{m}{sym} ({u})") }
}

/// The SI unit an input is read in (the pack's unit of its kind), "" for a pure number.
pub fn si_unit(s: &Solved, sym: &str) -> String {
    let Some(p) = s.p.procedure(s.key) else { return String::new() };
    let kind = p.inputs.iter().find(|i| i.symbol == sym).map(|i| i.kind.clone())
        .or_else(|| p.outputs.iter().find(|o| o.symbol == sym).map(|o| o.kind.clone()));
    let Some(kind) = kind else { return String::new() };
    s.p.kinds.get(&kind).map(|k| k.unit.clone()).filter(|u| u != "1").unwrap_or_default()
}

// ------------------------------------------------------------------------------------------------ sweeps
/// One output against one input over lo..hi (n runs of the model), the visitor's point marked; `kind`: the published
/// kind (e.g. "e-vs-cout").
#[allow(clippy::too_many_arguments)]
pub fn sweep(s: &Solved, kind: &str, title: &str, y: &str, x: &str, lo: f64, hi: f64, n: usize, log_x: bool)
             -> Option<Figure> {
    let (x0, y0) = (getv(s, x)?, getv(s, y)?);
    let xs = grid(lo, hi, n, log_x, is_int(s, x));
    let pts = curve(s, x, &xs, y);
    if pts.len() < 2 {
        return None;
    }
    let ys: Vec<f64> = pts.iter().map(|p| p.1).chain([y0]).collect();
    let xr = Chart::range(&pts.iter().map(|p| p.0).chain([x0]).collect::<Vec<_>>());
    let mut c = Chart::new(xr, Chart::range(&ys), (log_x, false), (false, true), &axis_label(s, x), &axis_label(s, y),
                           &format!("{y} = {}", tick(y0)));
    c.line(&pts, "axp-c", "", false);
    c.mark((x0, y0), &format!("{x} = {}", tick(x0)));
    let body = c.body();
    let caption = format!("{y} ({}) against {x} ({}) from {} to {}, the other inputs as given: {} points, each a run of the \
                           model. Marked: {x} = {}, {y} = {}.", meaning(s, y), meaning(s, x), tick(pts[0].0),
                          tick(pts[pts.len() - 1].0), pts.len(), tick(x0), tick(y0));
    Some(s.chart(kind, title, &caption, &body))
}

/// The same sweep round the visitor's value (half to one and a half times it).
pub fn sweep_around(s: &Solved, kind: &str, title: &str, y: &str, x: &str, log_x: bool) -> Option<Figure> {
    let (lo, hi) = around(getv(s, x)?, log_x)?;
    sweep(s, kind, title, y, x, lo, hi, 31, log_x)
}

// ------------------------------------------------------------------------------------------------ bars
/// Bars with their values written on them; `hl`: the bars in the accent colour (the rest grey); `log`: a logarithmic
/// value axis (positive values).
pub fn bars_body(labels: &[String], vals: &[f64], hl: &[usize], ylab: &str, header: &str, log: bool) -> Option<String> {
    if labels.is_empty() || labels.len() != vals.len() || vals.iter().any(|v| !v.is_finite()) {
        return None;
    }
    let (mut lo, mut hi) = Chart::range(vals);
    if log {
        if vals.iter().any(|v| *v <= 0.0) {
            return None;
        }
    } else {
        lo = lo.min(0.0);
        hi = hi.max(0.0);
    }
    let ya = Axis::new(lo, hi, log, CY1, CY0, true);
    let mut c = Chart { xa: Axis::new(0.0, 1.0, false, CX0, CX1, false), ya, s: String::new(), legend: vec![], pix: vec![], h: 340.0 };
    for t in ya.ticks() {
        let y = ya.px(t);
        c.s.push_str(&format!("<line class=\"axp-g\" x1=\"{CX0}\" y1=\"{y:.1}\" x2=\"{CX1}\" y2=\"{y:.1}\"/>\
                               <text class=\"axp-t\" x=\"{}\" y=\"{:.1}\" text-anchor=\"end\">{}</text>", CX0 - 6.0, y + 5.0,
                              esc(&tick(t))));
    }
    let n = vals.len() as f64;
    let slot = (CX1 - CX0) / n;
    let bw = (slot * 0.7).min(70.0);
    let base = if log { ya.px(ya.lo) } else { ya.px(0.0) };
    let rot = labels.iter().map(|l| l.chars().count()).max().unwrap_or(0) as f64 * 7.4 > slot;
    for (i, (l, v)) in labels.iter().zip(vals).enumerate() {
        let x = CX0 + slot * (i as f64 + 0.5);
        let y = ya.px(*v);
        let class = if hl.is_empty() || hl.contains(&i) { "axp-b" } else { "axp-b4" };
        c.s.push_str(&format!("<rect class=\"{class}\" x=\"{:.1}\" y=\"{:.1}\" width=\"{bw:.1}\" height=\"{:.1}\"/>",
                              x - bw / 2.0, y.min(base), (y - base).abs().max(0.5)));
        if n <= 12.0 {
            let ty = if *v >= 0.0 || log { y - 5.0 } else { y + 15.0 };
            let t = tick(*v);
            if t.chars().count() as f64 * 7.0 <= slot + 4.0 {
                c.text(x, ty.max(CY0 + 12.0), "middle", "axp-vt", &t);
            }
        }
        if rot {
            c.s.push_str(&format!("<text class=\"axp-t\" transform=\"translate({x:.1},{:.1}) rotate(-35)\" \
                                   text-anchor=\"end\" font-size=\"12\">{}</text>", CY1 + 14.0, esc(&clip(l, 14))));
        } else {
            c.text(x, CY1 + 18.0, "middle", "axp-t", l);
        }
    }
    if !log && lo < 0.0 {
        c.s.push_str(&format!("<path class=\"axp-c4\" d=\"M{CX0},{base:.1} L{CX1},{base:.1}\"/>"));
    }
    c.s.push_str(&format!("<rect class=\"axp-f\" x=\"{CX0}\" y=\"{CY0}\" width=\"{}\" height=\"{}\"/>", CX1 - CX0, CY1 - CY0));
    c.s.push_str(&format!("<text class=\"axp-l\" transform=\"translate(16,{:.1}) rotate(-90)\" text-anchor=\"middle\">{}</text>",
                          (CY0 + CY1) / 2.0, esc(&clip(ylab, 32))));
    if !header.is_empty() {
        c.s.push_str(&format!("<text class=\"axp-mt\" x=\"{CX0}\" y=\"22\">{}</text>", esc(&clip(header, 46))));
    }
    Some(c.s)
}

/// One output for each option of a choice input, one run of the model each; the visitor's option highlighted.
pub fn option_bars(s: &Solved, kind: &str, title: &str, out: &str, choice: &str) -> Option<Figure> {
    let mine = s.txt(choice)?;
    let mut labels = vec![];
    let mut vals = vec![];
    let mut hl = vec![];
    for o in choices(s, choice) {
        let Some(a) = s.with_input(&[(choice, Input::Text(o.clone()))]) else { continue };
        let Some(v) = k::num_of(&a, out) else { continue };
        if o.eq_ignore_ascii_case(&mine) {
            hl.push(labels.len());
        }
        labels.push(o);
        vals.push(v);
    }
    if labels.len() < 2 {
        return None;
    }
    let body = bars_body(&labels, &vals, &hl, &format!("{out}"), &format!("{out} = {}", tick(s.out(out)?)), false)?;
    let caption = format!("{out} ({}) for each option of {choice}, one run of the model per option: {}. Highlighted: the \
                           option given, {mine}.", meaning(s, out),
                          labels.iter().zip(&vals).map(|(l, v)| format!("{l}: {}", tick(*v))).collect::<Vec<_>>().join(", "));
    Some(s.chart(kind, title, &caption, &body))
}

// ------------------------------------------------------------------------------------------------ verdicts
/// A verdict (a text output) over two number inputs, each cell one run of the model coloured by its verdict, the
/// visitor's point marked (kind "decision").
#[allow(clippy::too_many_arguments)]
pub fn verdict_map(s: &Solved, title: &str, out: &str, x: &str, xr: (f64, f64), y: &str, yr: (f64, f64),
                   log: (bool, bool)) -> Option<Figure> {
    let (x0, y0) = (getv(s, x)?, getv(s, y)?);
    let mine = s.out_text(out)?;
    let xs = grid(xr.0, xr.1, 25, log.0, is_int(s, x));
    let ys = grid(yr.0, yr.1, 15, log.1, is_int(s, y));
    if xs.len() < 2 || ys.len() < 2 || xs.len() * ys.len() > s.left() {
        return None;
    }
    let mut names: Vec<String> = vec![];
    let mut cells = vec![];
    for (i, xv) in xs.iter().enumerate() {
        for (j, yv) in ys.iter().enumerate() {
            let Some(a) = s.with_input(&[(x, num_input(s, x, *xv)), (y, num_input(s, y, *yv))]) else { continue };
            let Some(v) = a.values.iter().find(|v| v.0 == out).map(|v| match &v.1 {
                serde_json::Value::String(t) => t.clone(),
                serde_json::Value::Bool(b) => if *b { "yes".into() } else { "no".into() },
                other => other.to_string(),
            }) else { continue };
            if !names.contains(&v) {
                names.push(v.clone());
            }
            cells.push((i, j, v));
        }
    }
    if cells.len() < 4 {
        return None;
    }
    let counts: Vec<String> = names.iter().map(|n| format!("{n}: {} cells", cells.iter().filter(|c| c.2 == *n).count()))
        .collect();
    let body = k::decision(&xs, &ys, &cells, &names, (x0, y0), &axis_label(s, x), &axis_label(s, y),
                           &format!("{out}: {}", clip(&mine, 30)));
    let caption = format!("{out} ({}) over {x} from {} to {} and {y} from {} to {}: each of the {} x {} cells is one run of \
                           the model, coloured by the verdict it returned ({}). Marked: the inputs given, {x} = {}, {y} = {} \
                           gives {mine}.", meaning(s, out), tick(xs[0]), tick(xs[xs.len() - 1]), tick(ys[0]),
                          tick(ys[ys.len() - 1]), xs.len(), ys.len(), counts.join(", "), tick(x0), tick(y0));
    Some(s.chart("decision", title, &caption, &body))
}

// ------------------------------------------------------------------------------------------------ letter strips
/// A row of letters in boxes: (letter, class of its box: "axp-b" accent, "axp-b2" second, "axp-b3" third, "axp-b4"
/// grey, "" none).
pub struct Strip {
    pub cells: Vec<(String, String)>,
    pub label: String,
}

/// Rows of letter boxes, wrapped at `per_row` letters, starting at y; returns the drawing and the y below it.
pub fn strips(rows: &[Strip], y: f64, per_row: usize) -> (String, f64) {
    let n = rows.iter().map(|r| r.cells.len()).max().unwrap_or(0).min(per_row).max(1);
    let label_w = if rows.iter().any(|r| !r.label.is_empty()) { 92.0 } else { 16.0 };
    let cell = ((520.0 - label_w - 16.0) / n as f64).min(30.0);
    let mut out = String::new();
    let mut y = y;
    let chunks = rows.iter().map(|r| r.cells.len().div_ceil(per_row)).max().unwrap_or(1).max(1);
    for ch in 0..chunks {
        for r in rows {
            let part: Vec<&(String, String)> = r.cells.iter().skip(ch * per_row).take(per_row).collect();
            if !r.label.is_empty() {
                out.push_str(&format!("<text class=\"axp-lt\" x=\"{:.1}\" y=\"{:.1}\" text-anchor=\"end\">{}</text>",
                                      label_w - 6.0, y + cell * 0.68, esc(&clip(&r.label, 11))));
            }
            for (j, (t, class)) in part.iter().enumerate() {
                let x = label_w + cell * j as f64;
                if !class.is_empty() {
                    out.push_str(&format!("<rect class=\"{class}\" x=\"{x:.1}\" y=\"{y:.1}\" width=\"{:.1}\" height=\"{:.1}\"/>",
                                          cell - 2.0, cell - 2.0));
                } else {
                    out.push_str(&format!("<rect class=\"axp-f\" x=\"{x:.1}\" y=\"{y:.1}\" width=\"{:.1}\" height=\"{:.1}\"/>",
                                          cell - 2.0, cell - 2.0));
                }
                let fs = (cell * 0.55).clamp(8.0, 15.0);
                out.push_str(&format!("<text class=\"axp-bt\" x=\"{:.1}\" y=\"{:.1}\" text-anchor=\"middle\" font-size=\"{fs:.0}\">{}</text>",
                                      x + (cell - 2.0) / 2.0, y + cell * 0.66, esc(t)));
            }
            y += cell + 4.0;
        }
        y += 8.0;
    }
    (out, y)
}

/// Text lines (name, value) under a drawing, starting at y; returns the drawing and the y below it.
pub fn notes(lines: &[(String, String)], y: f64) -> (String, f64) {
    let mut out = String::new();
    let mut y = y;
    for (n, v) in lines {
        let nw = n.chars().count() as f64 * 7.6;
        out.push_str(&format!("<text class=\"axp-lt\" x=\"16\" y=\"{y:.1}\">{}</text>", esc(n)));
        let maxc = ((520.0 - 32.0 - nw) / 8.0).max(4.0) as usize;
        // a long value wraps at ", " onto further lines
        let mut rows: Vec<String> = vec![String::new()];
        for part in v.split(", ") {
            let last = rows.last_mut().unwrap();
            let add = if last.is_empty() { part.to_string() } else { format!("{last}, {part}") };
            if add.chars().count() <= maxc || last.is_empty() {
                *last = add;
            } else {
                last.push(',');
                rows.push(part.to_string());
            }
        }
        for r in rows {
            out.push_str(&format!("<text class=\"axp-vt\" x=\"{:.1}\" y=\"{y:.1}\">{}</text>", 24.0 + nw, esc(&clip(&r, maxc))));
            y += 18.0;
        }
        y += 2.0;
    }
    (out, y)
}

/// A text output's characters (letters only), at most `max`.
pub fn letters(t: &str, max: usize) -> Option<Vec<char>> {
    let v: Vec<char> = t.chars().filter(|c| !c.is_whitespace()).collect();
    (!v.is_empty() && v.len() <= max).then_some(v)
}

/// A JSON list output as text items.
pub fn out_items(s: &Solved, sym: &str) -> Option<Vec<String>> {
    let v = s.outj(sym)?.as_array()?;
    Some(v.iter().map(|x| match x {
        serde_json::Value::String(t) => t.clone(),
        other => other.to_string(),
    }).collect())
}

/// A dict output ({"A": 60.0, ...}) as (key, number) pairs in its order.
pub fn out_map(s: &Solved, sym: &str) -> Option<Vec<(String, f64)>> {
    let v = s.outj(sym)?.as_object()?;
    let out: Vec<(String, f64)> = v.iter().filter_map(|(k, x)| x.as_f64().map(|f| (k.clone(), f))).collect();
    (!out.is_empty() && out.len() == v.len()).then_some(out)
}
