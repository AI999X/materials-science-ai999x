//! Crystal drawings for Materials Science's figures: a cubic unit cell in an oblique view with the atoms of its
//! structure, a plane (hkl) or a direction [uvw] through it, a hexagonal prism, the basal plane of a hexagonal cell and
//! the Bragg reflection geometry. The atom positions are the standard positions of the structure the visitor chose;
//! every number written is one the model returned or an input as given.

use crate::figs_common::esc;

/// A point of the unit cell in lattice coordinates (0..1 each).
pub type V3 = (f64, f64, f64);

/// The oblique view of a cube: x across, y up, z into the page (up and to the right).
pub struct View {
    pub ox: f64,
    pub oy: f64,
    pub l: f64,
    pub d: f64,
}

impl View {
    pub fn p(&self, v: V3) -> (f64, f64) {
        (self.ox + self.l * v.0 + self.d * v.2 * 0.766, self.oy - self.l * v.1 - self.d * v.2 * 0.643)
    }
}

/// The view every cell is drawn in.
/// (depth 0.42 of an edge at 40 degrees: no plane of small indices, (111), (121), (211), ..., is seen edge-on)
pub const VIEW: View = View { ox: 64.0, oy: 300.0, l: 160.0, d: 67.0 };

/// An atom: its position and its colour (0 corner, 1 another lattice site, 2 the second atom of the basis or a second
/// element).
pub type Atom = (V3, u8);

fn corners() -> Vec<Atom> {
    let mut v = vec![];
    for x in [0.0, 1.0] {
        for y in [0.0, 1.0] {
            for z in [0.0, 1.0] {
                v.push(((x, y, z), 0));
            }
        }
    }
    v
}

fn faces(c: u8) -> Vec<Atom> {
    vec![((0.5, 0.5, 0.0), c), ((0.5, 0.5, 1.0), c), ((0.5, 0.0, 0.5), c), ((0.5, 1.0, 0.5), c), ((0.0, 0.5, 0.5), c),
         ((1.0, 0.5, 0.5), c)]
}

/// The atoms of a cubic structure's conventional cell (None for a structure not drawn as a cube).
pub fn atoms(structure: &str) -> Option<Vec<Atom>> {
    let s = structure.to_ascii_lowercase();
    let mut v = corners();
    match s.as_str() {
        "sc" => {}
        "bcc" => v.push(((0.5, 0.5, 0.5), 1)),
        "fcc" => v.extend(faces(1)),
        "diamond" | "zincblende" => {
            v.extend(faces(1));
            for p in [(0.25, 0.25, 0.25), (0.75, 0.75, 0.25), (0.75, 0.25, 0.75), (0.25, 0.75, 0.75)] {
                v.push((p, 2));
            }
        }
        "rocksalt" => {
            v.extend(faces(1));
            v.push(((0.5, 0.5, 0.5), 2));
            for (a, b) in [(0.0, 0.0), (0.0, 1.0), (1.0, 0.0), (1.0, 1.0)] {
                v.push(((0.5, a, b), 2));
                v.push(((a, 0.5, b), 2));
                v.push(((a, b, 0.5), 2));
            }
        }
        "cscl" => v.push(((0.5, 0.5, 0.5), 2)),
        "fluorite" => {
            v.extend(faces(1));
            for x in [0.25, 0.75] {
                for y in [0.25, 0.75] {
                    for z in [0.25, 0.75] {
                        v.push(((x, y, z), 2));
                    }
                }
            }
        }
        _ => return None,
    }
    Some(v)
}

/// What is drawn over a cell: a shaded plane (its corner points), a direction arrow (from, to), a marked site with
/// its ring radius in pixels.
#[derive(Default)]
pub struct Extras {
    pub plane: Vec<V3>,
    pub arrow: Option<(V3, V3)>,
    pub site: Option<(V3, f64)>,
}

const RING: [&str; 3] = ["axp-c", "axp-c3", "axp-c2"];
const DOT: [&str; 3] = ["axp-d", "axp-d3", "axp-d2"];

/// A cubic cell with its atoms and extras, in the box x 40..300, y 40..300 of the canvas.
pub fn cube(atoms: &[Atom], ex: &Extras) -> String {
    let v = VIEW;
    let mut out = String::new();
    let edges: Vec<(V3, V3)> = {
        let mut e = vec![];
        let c = [0.0, 1.0];
        for a in c {
            for b in c {
                e.push(((0.0, a, b), (1.0, a, b)));
                e.push(((a, 0.0, b), (a, 1.0, b)));
                e.push(((a, b, 0.0), (a, b, 1.0)));
            }
        }
        e
    };
    let hidden = (0.0, 0.0, 1.0);
    for (a, b) in &edges {
        let (pa, pb) = (v.p(*a), v.p(*b));
        let dash = if *a == hidden || *b == hidden { " stroke-dasharray=\"5 4\"" } else { "" };
        out.push_str(&format!("<path class=\"axp-k\"{dash} d=\"M{:.1},{:.1} L{:.1},{:.1}\"/>", pa.0, pa.1, pb.0, pb.1));
    }
    if ex.plane.len() >= 3 {
        let pts: Vec<(f64, f64)> = ex.plane.iter().map(|p| v.p(*p)).collect();
        let d: Vec<String> = pts.iter().enumerate()
            .map(|(i, p)| format!("{}{:.1},{:.1}", if i == 0 { "M" } else { "L" }, p.0, p.1)).collect();
        out.push_str(&format!("<path class=\"axp-s3\" d=\"{} Z\"/><path class=\"axp-c3\" d=\"{} Z\"/>", d.join(" "),
                              d.join(" ")));
    }
    // back atoms first
    let mut order: Vec<&Atom> = atoms.iter().collect();
    order.sort_by(|a, b| (b.0 .2).partial_cmp(&a.0 .2).unwrap_or(std::cmp::Ordering::Equal));
    for (p, c) in order {
        let q = v.p(*p);
        out.push_str(&format!("<circle class=\"{}\" cx=\"{:.1}\" cy=\"{:.1}\" r=\"8.5\"/><circle class=\"{}\" cx=\"{:.1}\" \
                               cy=\"{:.1}\" r=\"2.6\"/>", RING[*c as usize], q.0, q.1, DOT[*c as usize], q.0, q.1));
    }
    if let Some((a, b)) = ex.arrow {
        out.push_str(&arrow(v.p(a), v.p(b), "axp-c2", "axp-d2"));
    }
    if let Some((p, r)) = ex.site {
        let q = v.p(p);
        out.push_str(&format!("<circle class=\"axp-s2\" cx=\"{:.1}\" cy=\"{:.1}\" r=\"{r:.1}\"/><circle class=\"axp-c2\" \
                               cx=\"{:.1}\" cy=\"{:.1}\" r=\"{r:.1}\"/>", q.0, q.1, q.0, q.1));
    }
    // the axes
    let o = (24.0, 344.0);
    out.push_str(&arrow(o, (o.0 + 40.0, o.1), "axp-k", "axp-kd"));
    out.push_str(&arrow(o, (o.0, o.1 - 40.0), "axp-k", "axp-kd"));
    out.push_str(&arrow(o, (o.0 + 26.0, o.1 - 15.0), "axp-k", "axp-kd"));
    out.push_str(&format!("<text class=\"axp-lt\" x=\"{:.1}\" y=\"{:.1}\">x</text><text class=\"axp-lt\" x=\"{:.1}\" \
                           y=\"{:.1}\">y</text><text class=\"axp-lt\" x=\"{:.1}\" y=\"{:.1}\">z</text>", o.0 + 44.0, o.1 + 4.0,
                          o.0 - 4.0, o.1 - 44.0, o.0 + 28.0, o.1 - 18.0));
    out
}

/// An arrow between two canvas points.
pub fn arrow(a: (f64, f64), b: (f64, f64), line: &str, head: &str) -> String {
    let (dx, dy) = (b.0 - a.0, b.1 - a.1);
    let d = (dx * dx + dy * dy).sqrt();
    if !(d > 4.0) {
        return String::new();
    }
    let (ux, uy) = (dx / d, dy / d);
    let (hx, hy) = (b.0 - ux * 9.0, b.1 - uy * 9.0);
    format!("<path class=\"{line}\" style=\"stroke-width:2.6\" d=\"M{:.1},{:.1} L{hx:.1},{hy:.1}\"/><path class=\"{head}\" \
             d=\"M{:.1},{:.1} L{:.1},{:.1} L{:.1},{:.1} Z\"/>", a.0, a.1, b.0, b.1, hx - uy * 5.5, hy + ux * 5.5,
            hx + uy * 5.5, hy - ux * 5.5)
}

/// The polygon where the plane n . r = c cuts the unit cube (its corners in order), empty when it misses the cube.
pub fn cut(n: V3, c: f64) -> Vec<V3> {
    let dot = |p: V3| n.0 * p.0 + n.1 * p.1 + n.2 * p.2;
    let mut pts: Vec<V3> = vec![];
    let cs = [0.0, 1.0];
    for a in cs {
        for b in cs {
            for (p, q) in [((0.0, a, b), (1.0, a, b)), ((a, 0.0, b), (a, 1.0, b)), ((a, b, 0.0), (a, b, 1.0))] {
                let (fp, fq) = (dot(p) - c, dot(q) - c);
                if fp == 0.0 {
                    pts.push(p);
                }
                if fq == 0.0 {
                    pts.push(q);
                }
                if fp * fq < 0.0 {
                    let t = fp / (fp - fq);
                    pts.push((p.0 + t * (q.0 - p.0), p.1 + t * (q.1 - p.1), p.2 + t * (q.2 - p.2)));
                }
            }
        }
    }
    let mut uniq: Vec<V3> = vec![];
    for p in pts {
        if !uniq.iter().any(|u| (u.0 - p.0).abs() + (u.1 - p.1).abs() + (u.2 - p.2).abs() < 1e-9) {
            uniq.push(p);
        }
    }
    if uniq.len() < 3 {
        return vec![];
    }
    // in order round the centre, as seen in the drawing (a convex polygon stays convex)
    let v = View { ox: 0.0, oy: 0.0, l: 1.0, d: 0.5 };
    let s: Vec<(f64, f64)> = uniq.iter().map(|p| v.p(*p)).collect();
    let cx = s.iter().map(|p| p.0).sum::<f64>() / s.len() as f64;
    let cy = s.iter().map(|p| p.1).sum::<f64>() / s.len() as f64;
    let mut idx: Vec<usize> = (0..uniq.len()).collect();
    idx.sort_by(|a, b| {
        let ta = (s[*a].1 - cy).atan2(s[*a].0 - cx);
        let tb = (s[*b].1 - cy).atan2(s[*b].0 - cx);
        ta.partial_cmp(&tb).unwrap_or(std::cmp::Ordering::Equal)
    });
    idx.into_iter().map(|i| uniq[i]).collect()
}

/// The plane (h k l) in the unit cube: through the points 1/h, 1/k, 1/l from the corner of the cell where every
/// negative index starts.
pub fn plane_hkl(h: f64, k: f64, l: f64) -> Vec<V3> {
    if h == 0.0 && k == 0.0 && l == 0.0 {
        return vec![];
    }
    let o = |x: f64| if x < 0.0 { 1.0 } else { 0.0 };
    let c = 1.0 + h * o(h) + k * o(k) + l * o(l);
    cut((h, k, l), c)
}

/// The direction [u v w] from the corner where its negative components start, scaled to end on the cell.
pub fn direction(u: f64, v: f64, w: f64) -> Option<(V3, V3)> {
    let m = u.abs().max(v.abs()).max(w.abs());
    if !(m > 0.0) {
        return None;
    }
    let o = |x: f64| if x < 0.0 { 1.0 } else { 0.0 };
    let a = (o(u), o(v), o(w));
    Some((a, (a.0 + u / m, a.1 + v / m, a.2 + w / m)))
}

/// A hexagonal close-packed cell: a hexagonal prism with its 17 atoms (12 corners, 2 face centres, 3 inside).
pub fn hcp_prism() -> String {
    let (cx, base, top, r) = (180.0, 280.0, 110.0, 120.0);
    let p = |th: f64, y: f64, f: f64| (cx + r * f * th.cos(), y - r * f * 0.36 * th.sin());
    let mut out = String::new();
    let ang: Vec<f64> = (0..6).map(|i| std::f64::consts::PI / 3.0 * i as f64).collect();
    for y in [base, top] {
        for i in 0..6 {
            let (a, b) = (p(ang[i], y, 1.0), p(ang[(i + 1) % 6], y, 1.0));
            let back = y == base && (ang[i].sin() > 0.1 || ang[(i + 1) % 6].sin() > 0.1) && !(i == 0 || i == 2);
            let dash = if back { " stroke-dasharray=\"5 4\"" } else { "" };
            out.push_str(&format!("<path class=\"axp-k\"{dash} d=\"M{:.1},{:.1} L{:.1},{:.1}\"/>", a.0, a.1, b.0, b.1));
        }
    }
    for th in &ang {
        let (a, b) = (p(*th, base, 1.0), p(*th, top, 1.0));
        let dash = if th.sin() > 0.1 && (th.cos() - 0.5).abs() > 0.1 && (th.cos() + 0.5).abs() > 0.1 { " stroke-dasharray=\"5 4\"" }
            else if (th.sin() - 0.866).abs() < 0.01 { " stroke-dasharray=\"5 4\"" } else { "" };
        out.push_str(&format!("<path class=\"axp-k\"{dash} d=\"M{:.1},{:.1} L{:.1},{:.1}\"/>", a.0, a.1, b.0, b.1));
    }
    let mut atoms: Vec<((f64, f64), u8, f64)> = vec![];
    for y in [base, top] {
        for th in &ang {
            atoms.push((p(*th, y, 1.0), 0, th.sin()));
        }
        atoms.push(((cx, y), 1, 0.0));
    }
    for i in 0..3 {
        let th = std::f64::consts::PI / 6.0 + 2.0 * std::f64::consts::PI / 3.0 * i as f64;
        atoms.push((p(th, (base + top) / 2.0, 0.577), 2, th.sin()));
    }
    atoms.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap_or(std::cmp::Ordering::Equal));
    for (q, c, _) in atoms {
        out.push_str(&format!("<circle class=\"{}\" cx=\"{:.1}\" cy=\"{:.1}\" r=\"8.5\"/><circle class=\"{}\" cx=\"{:.1}\" \
                               cy=\"{:.1}\" r=\"2.6\"/>", RING[c as usize], q.0, q.1, DOT[c as usize], q.0, q.1));
    }
    out
}

/// The text column beside a cell: a heading, then each (name, value) as a name line over a value line; the first
/// value is the marked one.
pub fn panel(x: f64, y: f64, heading: &str, items: &[(String, String)]) -> String {
    let mut out = format!("<text class=\"axp-bt\" x=\"{x:.1}\" y=\"{y:.1}\">{}</text>", esc(heading));
    let mut y = y + 26.0;
    for (i, (n, v)) in items.iter().enumerate() {
        let maxc = ((516.0 - x) / 8.6) as usize;
        let vt = crate::figs_common::clip(v, maxc.max(6));
        out.push_str(&format!("<text class=\"axp-lt\" x=\"{x:.1}\" y=\"{y:.1}\">{}</text>", esc(&crate::figs_common::clip(n, maxc + 4))));
        out.push_str(&format!("<text class=\"{}\" x=\"{x:.1}\" y=\"{:.1}\">{}</text>", if i == 0 { "axp-mt" } else { "axp-vt" },
                              y + 17.0, esc(&vt)));
        y += 42.0;
    }
    out
}

/// Bragg reflection from two lattice planes: rays arriving and leaving at the angle theta (degrees), the lower ray
/// travelling 2 d sin(theta) further; labels as given.
pub fn bragg(theta_deg: f64, labels: &[String]) -> Option<String> {
    if !(theta_deg > 0.0 && theta_deg < 90.0) {
        return None;
    }
    let th = theta_deg.to_radians();
    let (y1, y2, xc) = (130.0, 210.0, 250.0);
    let len = (200.0f64).min(110.0 / th.sin());
    let mut out = String::new();
    for y in [y1, y2] {
        out.push_str(&format!("<path class=\"axp-k\" d=\"M40,{y:.1} L470,{y:.1}\"/>"));
        let mut x = 50.0;
        while x <= 462.0 {
            out.push_str(&format!("<circle class=\"axp-kd\" cx=\"{x:.1}\" cy=\"{y:.1}\" r=\"3.2\"/>"));
            x += 50.0;
        }
    }
    let (dx, dy) = (len * th.cos(), len * th.sin());
    // upper ray hits at xc on the upper plane; the lower one, parallel, at xc on the lower plane
    for (y, line, head) in [(y1, "axp-c", "axp-d"), (y2, "axp-c2", "axp-d2")] {
        let a = (xc - dx, y - dy - if y == y2 { 0.0 } else { 0.0 });
        out.push_str(&arrow(a, (xc, y), line, head));
        out.push_str(&arrow((xc, y), (xc + dx, y - dy), line, head));
    }
    // the extra path: perpendiculars from the upper hit point to the lower rays
    let d = y2 - y1;
    let s = th.sin();
    let foot = |sign: f64| (xc + sign * d * s * th.cos(), y2 - d * s * s);
    let (fa, fb) = (foot(-1.0), foot(1.0));
    out.push_str(&format!("<path class=\"axp-c4\" stroke-dasharray=\"4 3\" d=\"M{xc:.1},{y1:.1} L{:.1},{:.1} M{xc:.1},{y1:.1} \
                           L{:.1},{:.1}\"/>", fa.0, fa.1, fb.0, fb.1));
    // the angle arc
    let r = 46.0;
    out.push_str(&format!("<path class=\"axp-c4\" d=\"M{:.1},{y1:.1} A{r},{r} 0 0 0 {:.1},{:.1}\"/>", xc + r, xc + r * th.cos(),
                          y1 - r * th.sin()));
    out.push_str(&format!("<text class=\"axp-lt\" x=\"{:.1}\" y=\"{:.1}\">θ</text>", xc + r + 4.0, y1 - 6.0));
    // the spacing
    out.push_str(&format!("<path class=\"axp-k\" d=\"M482,{y1:.1} L482,{y2:.1} M476,{y1:.1} L488,{y1:.1} M476,{y2:.1} L488,{y2:.1}\"/>\
                           <text class=\"axp-lt\" x=\"490\" y=\"{:.1}\">d</text>", (y1 + y2) / 2.0 + 4.0));
    let mut y = 250.0;
    for (i, t) in labels.iter().enumerate() {
        out.push_str(&format!("<text class=\"{}\" x=\"16\" y=\"{y:.1}\">{}</text>", if i == 0 { "axp-mt" } else { "axp-vt" },
                              esc(t)));
        y += 20.0;
    }
    Some(out)
}

/// The basal plane of a hexagonal cell with its axes a1, a2, a3 and the in-plane part u a1 + v a2 + t a3 of a
/// four-index direction (drawn to one third), as an arrow.
pub fn basal(u: f64, v: f64, t: f64) -> String {
    let (cx, cy, r) = (210.0, 175.0, 120.0);
    let mut out = String::new();
    let hex: Vec<(f64, f64)> = (0..6).map(|i| {
        let a = std::f64::consts::PI / 3.0 * i as f64;
        (cx + r * a.cos(), cy - r * a.sin())
    }).collect();
    let d: Vec<String> = hex.iter().enumerate().map(|(i, p)| format!("{}{:.1},{:.1}", if i == 0 { "M" } else { "L" }, p.0, p.1))
        .collect();
    out.push_str(&format!("<path class=\"axp-ka\" d=\"{} Z\"/><path class=\"axp-k\" d=\"{} Z\"/>", d.join(" "), d.join(" ")));
    // a1 along 0 deg... on the page: a1 to the right-down, a2 up-left, a3 down-left (120 deg apart)
    let axes = [("a1", 0.0f64), ("a2", 120.0), ("a3", 240.0)];
    let unit: Vec<(f64, f64)> = axes.iter().map(|(_, a)| (a.to_radians().cos(), -a.to_radians().sin())).collect();
    for (i, (n, _)) in axes.iter().enumerate() {
        let e = (cx + unit[i].0 * r * 0.75, cy + unit[i].1 * r * 0.75);
        out.push_str(&arrow((cx, cy), e, "axp-k", "axp-kd"));
        // each name beside its axis, off the line
        out.push_str(&format!("<text class=\"axp-lt\" x=\"{:.1}\" y=\"{:.1}\" text-anchor=\"middle\">{n}</text>",
                              cx + unit[i].0 * r * 0.8 - unit[i].1 * 16.0, cy + unit[i].1 * r * 0.8 + unit[i].0 * 16.0 + 5.0));
    }
    let x = (u * unit[0].0 + v * unit[1].0 + t * unit[2].0) / 3.0;
    let y = (u * unit[0].1 + v * unit[1].1 + t * unit[2].1) / 3.0;
    // a long direction is shortened to the hexagon's size (its direction is what is drawn)
    let len = (x * x + y * y).sqrt();
    let f = if len > 1.0 { 1.0 / len } else { 1.0 };
    let (x, y) = (x * f, y * f);
    let e =(cx + x * r, cy + y * r);
    out.push_str(&arrow((cx, cy), e, "axp-c", "axp-d"));
    out
}
