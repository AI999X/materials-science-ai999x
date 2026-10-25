"""Materials science procedures: deterministic capabilities with typed inputs and several outputs (domain code, loaded
by the pack; enabled only once tools/certify_procedures.py has certified them).

Topics: crystal structures and diffraction, defects and diffusion, mechanical properties and fracture, phase diagrams
and transformations, polymers, ceramics and composites, electronic and thermal properties.

Each procedure states its inputs, outputs, conditions, method, LaTeX, the documented question template it parses,
worked examples, an INDEPENDENT reference computation (used only by certification; the checking libraries pymatgen,
ASE, spglib, numpy, scipy, sympy and mpmath are imported inside the reference functions) and a deliberately wrong
variant that the checks must reject. The shipped code (run, verify, parse) is plain Python (math, re, fractions).

Conventions (fixed, so run and reference agree exactly):
- Constants: CODATA 2018 exact SI values (N_A = 6.02214076e23 /mol, k = 1.380649e-23 J/K, e = 1.602176634e-19 C,
  h = 6.62607015e-34 J s), m_e = 9.1093837139e-31 kg (CODATA 2022, as scipy.constants 1.18), R = N_A k.
- Hard-sphere crystal models: atoms touch along the close-packed direction (SC: edge, BCC: body diagonal, FCC: face
  diagonal, diamond: a quarter of the body diagonal); HCP uses the ideal ratio c/a = sqrt(8/3). Conventional cells:
  SC 1, BCC 2, FCC 4, diamond 8, HCP 6 atoms (the hexagonal prism).
- Miller indices are integers (h k l); hexagonal planes use the three indices (h k l) (i = -(h + k) may be given and
  is checked); directions [u v w]. Angles are reported in degrees.
- Atomic weights are pymatgen's element table (the table below is that table, checked against it in certification).
- Composition inputs (wt%, at%) are plain numbers in percent. Concentrations in diffusion problems may be in any one
  unit (wt%, kg/m^3, atoms/m^3) used for all of them; the answer is in the same unit.
- Results built from cos, sin, exp or log are rounded to 12 significant digits (identical on every platform); values
  that are zero by symmetry are written as exactly 0. Very small SI values also come in a well-scaled companion
  output (nm, g/cm^3, MPa, log10).

Question form (one per procedure): the capability phrase, a colon, then `name = value` pairs separated by commas
(`Lattice parameter from atomic radius: structure = FCC, R = 0.1278 nm`); physical values carry a unit (angstrom,
psi and ksi are converted here); lists are written in brackets (`strain = [0, 0.001, 0.002]`). Any other number or
an unknown name is refused, never guessed.
"""
from __future__ import annotations

import math
import re
from fractions import Fraction
from itertools import product as _iproduct

# ================================================================================================ constants
NA = 6.02214076e23
KB = 1.380649e-23
QE = 1.602176634e-19
HP = 6.62607015e-34
HBAR = HP / (2 * math.pi)
ME = 9.1093837139e-31
RGAS = NA * KB
SQ2, SQ3 = math.sqrt(2.0), math.sqrt(3.0)

# pymatgen's atomic weights (g/mol), Z = 1..94
ATOMIC_WEIGHT = {
    "H": 1.00794, "He": 4.002602, "Li": 6.941, "Be": 9.012182, "B": 10.811, "C": 12.0107, "N": 14.0067, "O": 15.9994,
    "F": 18.9984032, "Ne": 20.1797, "Na": 22.98976928, "Mg": 24.305, "Al": 26.9815386, "Si": 28.0855, "P": 30.973762,
    "S": 32.065, "Cl": 35.453, "Ar": 39.948, "K": 39.0983, "Ca": 40.078, "Sc": 44.955912, "Ti": 47.867, "V": 50.9415,
    "Cr": 51.9961, "Mn": 54.938045, "Fe": 55.845, "Co": 58.933195, "Ni": 58.6934, "Cu": 63.546, "Zn": 65.409,
    "Ga": 69.723, "Ge": 72.64, "As": 74.9216, "Se": 78.96, "Br": 79.904, "Kr": 83.798, "Rb": 85.4678, "Sr": 87.62,
    "Y": 88.90585, "Zr": 91.224, "Nb": 92.90638, "Mo": 95.94, "Tc": 98.0, "Ru": 101.07, "Rh": 102.9055, "Pd": 106.42,
    "Ag": 107.8682, "Cd": 112.411, "In": 114.818, "Sn": 118.71, "Sb": 121.76, "Te": 127.6, "I": 126.90447,
    "Xe": 131.293, "Cs": 132.9054519, "Ba": 137.327, "La": 138.90547, "Ce": 140.116, "Pr": 140.90765, "Nd": 144.242,
    "Pm": 145.0, "Sm": 150.36, "Eu": 151.964, "Gd": 157.25, "Tb": 158.92535, "Dy": 162.5, "Ho": 164.93032,
    "Er": 167.259, "Tm": 168.93421, "Yb": 173.04, "Lu": 174.967, "Hf": 178.49, "Ta": 180.94788, "W": 183.84,
    "Re": 186.207, "Os": 190.23, "Ir": 192.217, "Pt": 195.084, "Au": 196.966569, "Hg": 200.59, "Tl": 204.3833,
    "Pb": 207.2, "Bi": 208.9804, "Po": 210.0, "At": 210.0, "Rn": 220.0, "Fr": 223.0, "Ra": 226.0, "Ac": 227.0,
    "Th": 232.03806, "Pa": 231.03588, "U": 238.02891, "Np": 237.0, "Pu": 244.0}


# ================================================================================================ shared helpers
def _r(x):
    """12 significant digits, no negative zero (identical results on every platform and Python version)."""
    if isinstance(x, (bool, str, int)):
        return x
    if isinstance(x, list):
        return [_r(v) for v in x]
    if x == 0 or not math.isfinite(x):
        return 0.0 if x == 0 else x
    return float(f"{x:.12g}") + 0.0


def _out(**kw):
    return {k: _r(v) for k, v in kw.items()}


def _rel(a, b):
    if isinstance(a, (str, bool)) or isinstance(b, (str, bool)):
        return 0.0 if a == b else math.inf
    if isinstance(a, list) or isinstance(b, list):
        if not (isinstance(a, list) and isinstance(b, list)) or len(a) != len(b):
            return math.inf
        return max([_rel(x, y) for x, y in zip(a, b)] or [0.0])
    try:
        a, b = float(a), float(b)
    except (TypeError, ValueError):
        return math.inf
    d = abs(a - b)
    if d == 0:
        return 0.0
    s = max(abs(a), abs(b))
    return d / s if s > 0 else math.inf


def _recheck(fn):
    """A verifier: every output the procedure gives is recomputed and compared (relative); a missing output fails."""
    def verify(inputs, outputs):
        try:
            want = fn(**inputs)
        except Exception:
            return math.inf
        worst = 0.0
        for k, v in want.items():
            if k not in outputs:
                return math.inf
            worst = max(worst, _rel(outputs[k], v))
        return worst
    return verify


def _safe(fn):
    """Guard a wrong variant against exceptions (a variant that raises cannot be compared)."""
    def g(**kw):
        try:
            return fn(**kw)
        except Exception:
            return {"_failed": True}
    return g


def _lu(rng, a, b):
    return math.exp(rng.uniform(math.log(a), math.log(b)))


def _pos(name, *vals):
    for v in vals:
        if not (isinstance(v, (int, float)) and math.isfinite(v) and v > 0):
            raise ValueError(f"{name} must be a positive number")


def _frac01(name, v, closed=False):
    ok = (0 <= v <= 1) if closed else (0 < v < 1)
    if not (math.isfinite(v) and ok):
        raise ValueError(f"{name} must lie between 0 and 1")


# units the shared unit reader does not know: converted here to SI (factor, SI unit)
_EXTRA_UNITS = {"angstrom": (1e-10, "m"), "angstroms": (1e-10, "m"), "Å": (1e-10, "m"), "Angstrom": (1e-10, "m"),
                "psi": (6894.757293168361, "Pa"), "ksi": (6894757.293168361, "Pa"),
                "MPa*sqrt(m)": (1e6, "Pa*m^0.5"), "MPa sqrt(m)": (1e6, "Pa*m^0.5"), "MPa√m": (1e6, "Pa*m^0.5"),
                "MPa m^0.5": (1e6, "Pa*m^0.5"), "MPa*m^1/2": (1e6, "Pa*m^0.5"), "MPa m^1/2": (1e6, "Pa*m^0.5")}
_NUM = r"[-+]?(?:\d+(?:\.\d*)?|\.\d+)(?:[eE][-+]?\d+)?"
_SEG = re.compile(r"^\s*(?:and\s+)?([A-Za-z][A-Za-z_0-9]*)\s*=\s*(" + _NUM + r")\s*(.*?)\s*$")
_WORD = re.compile(r"^\s*(?:and\s+)?([A-Za-z][A-Za-z_0-9]*)\s*=\s*([A-Za-z][A-Za-z_0-9 \-]*?)\s*$")
_LIST = re.compile(r"^\s*(?:and\s+)?([A-Za-z][A-Za-z_0-9]*)\s*=\s*(\[[^\]]*\])\s*(.*?)\s*$")
_TEXT = re.compile(r"^\s*(?:and\s+)?([A-Za-z][A-Za-z_0-9]*)\s*=\s*(\S.*?)\s*$")


def _split(rest):
    """Split 'a = 1, b = [1, 2, 3], c = 4' on commas and semicolons outside brackets."""
    parts, depth, cur = [], 0, ""
    for ch in rest:
        if ch == "[":
            depth += 1
        elif ch == "]":
            depth -= 1
        if ch in ",;" and depth == 0:
            parts.append(cur)
            cur = ""
        else:
            cur += ch
    parts.append(cur)
    return parts if depth == 0 else None


def _pairs(question, phrase, names, phys=(), words=(), vectors=(), texts=(), num_units=None):
    """{symbol: value} from '<phrase>: name = value, ...'. None unless the phrase opens the question and every part is
    a 'name = value' pair of a known name. `phys`: symbols that need a unit (returned as 'value unit' strings, read by
    the runtime; angstrom, psi and ksi are converted here); `words`: symbols whose value is a word (a choice);
    `vectors`: list symbols written in brackets; `texts`: symbols whose value is free text (intercepts, formulas);
    `num_units`: {symbol: [allowed unit words]} for plain numbers (wt%, at%, percent ...)."""
    if not isinstance(question, str):
        return None
    m = re.match(r"\s*(?:" + phrase + r")\s*:\s*", question, re.I)
    if not m:
        return None
    rest = re.sub(r"[.?!]\s*$", "", question[m.end():].strip()).strip()
    if not rest:
        return {}
    segs = _split(rest)
    if segs is None:
        return None
    out = {}
    num_units = num_units or {}
    for seg in segs:
        if not seg.strip():
            continue
        ml = _LIST.match(seg)
        if ml:
            key = names.get(ml.group(1), names.get(ml.group(1).lower()))
            if key is None or key in out or key not in vectors or ml.group(3):
                return None
            out[key] = ml.group(2)
            continue
        mt = _TEXT.match(seg)
        if mt and names.get(mt.group(1), names.get(mt.group(1).lower())) in set(texts):
            key = names.get(mt.group(1), names.get(mt.group(1).lower()))
            if key in out:
                return None
            out[key] = mt.group(2).strip()
            continue
        mw = _WORD.match(seg)
        if mw and names.get(mw.group(1), names.get(mw.group(1).lower())) in set(words):
            key = names.get(mw.group(1), names.get(mw.group(1).lower()))
            if key in out:
                return None
            out[key] = mw.group(2).strip().lower()
            continue
        ms = _SEG.match(seg)
        if not ms:
            return None
        key = names.get(ms.group(1), names.get(ms.group(1).lower()))
        if key is None or key in out or key in words or key in vectors or key in texts:
            return None
        num, unit = ms.group(2), ms.group(3).strip()
        if key in phys:
            if not unit:
                return None
            if unit in _EXTRA_UNITS:
                f, si = _EXTRA_UNITS[unit]
                out[key] = f"{float(num) * f!r} {si}"
            else:
                out[key] = f"{num} {unit}"
        else:
            if unit and unit not in num_units.get(key, ()):
                return None
            out[key] = float(num)
    return out


def _phys(sym, kind, meaning, required=True):
    return {"symbol": sym, "kind": kind, "meaning": meaning, "required": required}


def _numin(sym, meaning, required=True, **extra):
    return {"symbol": sym, "kind": "number", "meaning": meaning, "required": required, **extra}


def _intin(sym, meaning, lo=None, hi=None, required=True):
    d = {"symbol": sym, "type": "integer", "meaning": meaning, "required": required}
    if lo is not None:
        d["min"] = lo
    if hi is not None:
        d["max"] = hi
    return d


def _choice(sym, choices, meaning, required=True):
    return {"symbol": sym, "type": "choice", "choices": list(choices), "meaning": meaning, "required": required}


def _vec(sym, meaning, lo=2, hi=200, required=True):
    return {"symbol": sym, "type": "vector", "meaning": meaning, "required": required, "min_length": lo, "max_length": hi}


def _text(sym, meaning, required=True):
    return {"symbol": sym, "type": "text", "meaning": meaning, "required": required}


def _o(sym, kind, meaning):
    return {"symbol": sym, "kind": kind, "meaning": meaning}


PROCEDURES: dict = {}


def _add(key, **spec):
    spec.setdefault("conditions", [])
    spec.setdefault("verify", _recheck(spec["run"]))
    PROCEDURES[key] = spec


def _gcd3(*v):
    g = 0
    for x in v:
        g = math.gcd(g, abs(int(x)))
    return g


def _ints(name, *v):
    for x in v:
        if not (isinstance(x, int) or (isinstance(x, float) and x == int(x))):
            raise ValueError(f"{name} must be whole numbers")
    if all(int(x) == 0 for x in v):
        raise ValueError(f"{name} must not all be zero")
    return [int(x) for x in v]


# ================================================================================================ crystal models
_CUBIC = ("SC", "BCC", "FCC", "diamond")
_ALL = ("SC", "BCC", "FCC", "HCP", "diamond")
_N_ATOMS = {"SC": 1, "BCC": 2, "FCC": 4, "diamond": 8, "HCP": 6}
_CN = {"SC": 6, "BCC": 8, "FCC": 12, "diamond": 4, "HCP": 12}
_A_PER_R = {"SC": 2.0, "BCC": 4 / SQ3, "FCC": 2 * SQ2, "diamond": 8 / SQ3, "HCP": 2.0}   # a / R (HCP: a, c/a ideal)
_COA = math.sqrt(8.0 / 3.0)


def _cell_volume(struct, a):
    if struct == "HCP":
        return 1.5 * SQ3 * a * a * (_COA * a)
    return a ** 3


def _ase_cell(struct, a):
    """ASE's own crystal builder (conventional cubic cell; HCP: the 2-atom primitive cell with ideal c/a)."""
    from ase.build import bulk
    if struct == "SC":
        return bulk("Cu", "sc", a=a)
    if struct == "BCC":
        return bulk("Fe", "bcc", a=a, cubic=True)
    if struct == "FCC":
        return bulk("Cu", "fcc", a=a, cubic=True)
    if struct == "diamond":
        return bulk("Si", "diamond", a=a, cubic=True)
    return bulk("Mg", "hcp", a=a, c=a * math.sqrt(8.0 / 3.0))


def _ase_nn(atoms, scale):
    """(nearest-neighbour distance, coordination number) from ASE's neighbour list."""
    import numpy as np
    from ase.neighborlist import neighbor_list
    i, d = neighbor_list("id", atoms, 1.2 * scale)
    dmin = float(d.min())
    cn = np.sum(np.abs(d - dmin) < 1e-9 * dmin) / len(atoms)
    return dmin, float(cn)


_STRUCT_WORDS = {"structure": "structure", "lattice": "structure", "crystal": "structure"}
_STRUCT_CHOICES = list(_ALL)


def _struct_in(choices, meaning="crystal structure"):
    return _choice("structure", choices, meaning + " (" + ", ".join(choices) + ")")


# ---------------------------------------------------------------------------------------- lattice parameter from R
def cubic_lattice_parameter(structure, R):
    _pos("the atomic radius", R)
    a = _A_PER_R[structure] * R
    return _out(a=a, a_nm=a * 1e9, nearest_neighbour_nm=2 * R * 1e9)


def ref_cubic_lattice_parameter(structure, R):
    d1, _ = _ase_nn(_ase_cell(structure, 1.0), 1.0)            # nearest-neighbour distance of a unit cell
    return {"a_nm": 2 * R / d1 * 1e9}


_add("cubic_lattice_parameter",
     name="Lattice parameter from the atomic radius", topic="crystal structures / cubic lattices",
     purpose="The cubic lattice parameter a of an SC, BCC, FCC or diamond-cubic crystal of hard-sphere atoms of radius R.",
     inputs=[_struct_in(_CUBIC), _phys("R", "length", "atomic radius")],
     outputs=[_o("a", "length", "lattice parameter"), _o("a_nm", "number", "lattice parameter in nm"),
              _o("nearest_neighbour_nm", "number", "nearest-neighbour distance 2R in nm")],
     primary="a_nm", conditions=["R > 0"],
     method="atoms touch along the close-packed direction: SC a = 2R, BCC a = 4R/sqrt(3), FCC a = 2 sqrt(2) R, diamond a = 8R/sqrt(3)",
     latex=r"a_{\mathrm{SC}} = 2R,\ a_{\mathrm{BCC}} = \frac{4R}{\sqrt3},\ a_{\mathrm{FCC}} = 2\sqrt2\,R,\ a_{\mathrm{dia}} = \frac{8R}{\sqrt3}",
     run=cubic_lattice_parameter,
     parse=lambda q: _pairs(q, r"lattice parameter from (?:the )?atomic radius", {"structure": "structure", "R": "R", "r": "R"},
                            phys={"R"}, words={"structure"}),
     reference=ref_cubic_lattice_parameter,
     wrong=_safe(lambda structure, R: (lambda a: _out(a=a, a_nm=a * 1e9, nearest_neighbour_nm=2 * R * 1e9))(
         {"SC": 2.0, "BCC": 4 / SQ2, "FCC": 4 / SQ3, "diamond": 4 / SQ3}[structure] * R if structure != "SC" else R)),
     random_inputs=lambda rng: {"structure": rng.choice(_CUBIC), "R": rng.uniform(0.05e-9, 0.25e-9)},
     templates=["Lattice parameter from atomic radius: structure = FCC, R = 0.1278 nm",
                "Lattice parameter from atomic radius: structure = BCC, R = 1.241 angstrom"],
     examples=[{"inputs": {"structure": "FCC", "R": 0.1278e-9}, "outputs": {"a_nm": 2 * SQ2 * 0.1278}},
               {"inputs": {"structure": "BCC", "R": 0.1241e-9}, "outputs": {"a_nm": 4 * 0.1241 / SQ3}},
               {"inputs": {"structure": "SC", "R": 0.167e-9}, "outputs": {"a_nm": 0.334}},
               {"inputs": {"structure": "diamond", "R": 0.1176e-9}, "outputs": {"a_nm": 8 * 0.1176 / SQ3}}],
     questions=[{"q": "Lattice parameter from atomic radius: structure = FCC, R = 0.1278 nm", "outputs": {"a_nm": 2 * SQ2 * 0.1278}},
                {"q": "Lattice parameter from atomic radius: structure = BCC, R = 1.241 angstrom", "outputs": {"a_nm": 4 * 0.1241 / SQ3}},
                {"q": "Lattice parameter from atomic radius: structure = FCC, R = -1 nm", "outcome": "INVALID_INPUT"},
                {"q": "Lattice parameter from atomic radius: structure = FCC", "outcome": "NEEDS_INPUT"},
                {"q": "Lattice parameter from atomic radius: structure = FCC, R = 0.1278", "outcome": "UNRECOGNIZED_FORMAT"}])


# ---------------------------------------------------------------------------------------- atomic packing factor
def packing_factor(structure):
    a = _A_PER_R[structure]                                     # with R = 1
    apf = _N_ATOMS[structure] * (4 / 3) * math.pi / _cell_volume(structure, a)
    return _out(apf=apf, coordination=_CN[structure], atoms_per_cell=_N_ATOMS[structure])


def ref_packing_factor(structure):
    atoms = _ase_cell(structure, 1.0)
    d, cn = _ase_nn(atoms, 1.0)
    apf = len(atoms) * (4 / 3) * math.pi * (d / 2) ** 3 / atoms.get_volume()
    return {"apf": apf, "coordination": int(round(cn))}


_add("atomic_packing_factor",
     name="Atomic packing factor and coordination number", topic="crystal structures / packing",
     purpose="The fraction of a crystal's volume filled by hard-sphere atoms, the coordination number and the atoms per conventional cell of SC, BCC, FCC, HCP or diamond cubic.",
     inputs=[_struct_in(_ALL)],
     outputs=[_o("apf", "number", "atomic packing factor"), _o("coordination", "number", "coordination number"),
              _o("atoms_per_cell", "number", "atoms per conventional cell (HCP: the hexagonal prism)")],
     primary="apf", conditions=["hard spheres touching along the close-packed direction; ideal c/a for HCP"],
     method="APF = n (4/3) pi R^3 / V_cell with the cell edge written in R",
     latex=r"\mathrm{APF} = \frac{n\,\tfrac43\pi R^3}{V_{\mathrm{cell}}}",
     run=packing_factor,
     parse=lambda q: _pairs(q, r"atomic packing factor|packing factor", {"structure": "structure"}, words={"structure"}),
     reference=ref_packing_factor,
     wrong=_safe(lambda structure: _out(apf=({"SC": 8, "BCC": 9, "FCC": 14, "diamond": 18, "HCP": 17}[structure]) * (4 / 3) * math.pi
                                        / _cell_volume(structure, _A_PER_R[structure]),
                                        coordination=_CN[structure], atoms_per_cell=_N_ATOMS[structure])),
     random_inputs=lambda rng: {"structure": rng.choice(_ALL)},
     templates=["Atomic packing factor: structure = FCC", "Atomic packing factor: structure = BCC"],
     examples=[{"inputs": {"structure": "FCC"}, "outputs": {"apf": math.pi / (3 * SQ2), "coordination": 12}},
               {"inputs": {"structure": "BCC"}, "outputs": {"apf": math.pi * SQ3 / 8, "coordination": 8}},
               {"inputs": {"structure": "SC"}, "outputs": {"apf": math.pi / 6, "coordination": 6}},
               {"inputs": {"structure": "HCP"}, "outputs": {"apf": math.pi / (3 * SQ2), "atoms_per_cell": 6}},
               {"inputs": {"structure": "diamond"}, "outputs": {"apf": math.pi * SQ3 / 16, "coordination": 4}}],
     questions=[{"q": "Atomic packing factor: structure = FCC", "outputs": {"apf": math.pi / (3 * SQ2)}},
                {"q": "Atomic packing factor: structure = diamond", "outputs": {"apf": math.pi * SQ3 / 16}},
                {"q": "Atomic packing factor: structure = BCT", "outcome": "INVALID_INPUT"},
                {"q": "Atomic packing factor:", "outcome": "NEEDS_INPUT"}])


# ---------------------------------------------------------------------------------------- theoretical density
def theoretical_density(structure, A, R):
    _pos("the molar mass", A)
    _pos("the atomic radius", R)
    a = _A_PER_R[structure] * R
    rho = _N_ATOMS[structure] * A / (_cell_volume(structure, a) * NA)
    return _out(rho=rho, rho_gcc=rho / 1000, a_nm=a * 1e9)


_BASE_CELLS: dict = {}


def _ase_density(structure, a, A):
    if structure not in _BASE_CELLS:
        _BASE_CELLS[structure] = _ase_cell(structure, 1.0)
    atoms = _BASE_CELLS[structure].copy()
    atoms.set_cell(atoms.get_cell() * a, scale_atoms=True)            # ASE's cell scaled to edge a
    atoms.set_masses([A * 1000.0] * len(atoms))                 # g/mol per atom
    return float(sum(atoms.get_masses())) * 1e-3 / NA / atoms.get_volume()


def ref_theoretical_density(structure, A, R):
    from scipy.constants import Avogadro
    d1, _ = _ase_nn(_ase_cell(structure, 1.0), 1.0)
    a = 2 * R / d1
    atoms = _ase_cell(structure, a * 1e10)                      # in angstrom
    atoms.set_masses([A * 1000.0] * len(atoms))
    rho = float(sum(atoms.get_masses())) * 1e-3 / Avogadro / (atoms.get_volume() * 1e-30)
    return {"rho_gcc": rho / 1000, "a_nm": a * 1e9}


_add("theoretical_density",
     name="Theoretical density of a metal", topic="crystal structures / density",
     purpose="The density of a crystal from its structure, the atomic radius and the molar mass: rho = n A / (V_cell N_A).",
     inputs=[_struct_in(_ALL), _phys("A", "molar_mass", "molar mass (atomic weight)"), _phys("R", "length", "atomic radius")],
     outputs=[_o("rho", "density", "theoretical density"), _o("rho_gcc", "number", "density in g/cm^3"),
              _o("a_nm", "number", "lattice parameter a in nm")],
     primary="rho_gcc", conditions=["A > 0", "R > 0"],
     method="a from R (hard spheres), V_cell = a^3 (HCP: (3 sqrt(3)/2) a^2 c with c = sqrt(8/3) a), rho = n A/(V_cell N_A)",
     latex=r"\rho = \frac{n A}{V_{\mathrm{cell}} N_A}",
     run=theoretical_density,
     parse=lambda q: _pairs(q, r"theoretical density", {"structure": "structure", "A": "A", "R": "R", "r": "R"},
                            phys={"A", "R"}, words={"structure"}),
     reference=ref_theoretical_density,
     wrong=_safe(lambda structure, A, R: (lambda a: (lambda rho: _out(rho=rho, rho_gcc=rho / 1000, a_nm=a * 1e9))(
         (_N_ATOMS[structure] + 1) * A / (_cell_volume(structure, a) * NA)))(_A_PER_R[structure] * R)),
     random_inputs=lambda rng: {"structure": rng.choice(_ALL), "A": rng.uniform(0.007, 0.240), "R": rng.uniform(0.1e-9, 0.2e-9)},
     templates=["Theoretical density: structure = FCC, A = 63.55 g/mol, R = 0.128 nm",
                "Theoretical density: structure = BCC, A = 55.85 g/mol, R = 0.1241 nm"],
     examples=[{"inputs": {"structure": "FCC", "A": 0.06355, "R": 0.128e-9},
                "outputs": {"rho_gcc": 4 * 63.55 / ((2 * SQ2 * 0.128e-7) ** 3 * NA)}},
               {"inputs": {"structure": "BCC", "A": 0.05585, "R": 0.1241e-9},
                "outputs": {"rho_gcc": 2 * 55.85 / ((4 * 0.1241e-7 / SQ3) ** 3 * NA)}},
               {"inputs": {"structure": "HCP", "A": 0.02431, "R": 0.160e-9},
                "outputs": {"rho_gcc": 6 * 24.31 / (1.5 * SQ3 * (0.32e-7) ** 3 * _COA * NA)}}],
     questions=[{"q": "Theoretical density: structure = FCC, A = 63.55 g/mol, R = 0.128 nm",
                 "outputs": {"rho_gcc": 4 * 63.55 / ((2 * SQ2 * 0.128e-7) ** 3 * NA)}},
                {"q": "Theoretical density: structure = BCC, A = 55.85 g/mol, R = 1.241 angstrom",
                 "outputs": {"rho_gcc": 2 * 55.85 / ((4 * 0.1241e-7 / SQ3) ** 3 * NA)}},
                {"q": "Theoretical density: structure = FCC, A = 63.55 g/mol", "outcome": "NEEDS_INPUT"},
                {"q": "Theoretical density: structure = FCC, A = 0 g/mol, R = 0.128 nm", "outcome": "INVALID_INPUT"}])


# ---------------------------------------------------------------------------------------- lattice parameter from density
def lattice_from_density(structure, rho, A):
    _pos("the density", rho)
    _pos("the molar mass", A)
    if structure == "HCP":
        a = (_N_ATOMS["HCP"] * A / (rho * NA * 1.5 * SQ3 * _COA)) ** (1 / 3)
    else:
        a = (_N_ATOMS[structure] * A / (rho * NA)) ** (1 / 3)
    R = a / _A_PER_R[structure]
    return _out(a=a, a_nm=a * 1e9, R_nm=R * 1e9)


def ref_lattice_from_density(structure, rho, A):
    from scipy.optimize import brentq
    f = lambda a_nm: _ase_density(structure, a_nm, A) * 1e27 - rho          # noqa: E731  (ASE volume in nm^3)
    a_nm = brentq(f, 0.01, 5.0, xtol=1e-15, rtol=8.9e-16, maxiter=500)
    d1, _ = _ase_nn(_ase_cell(structure, 1.0), 1.0)           # neighbour distance of the unit cell, scaled
    return {"a_nm": a_nm, "R_nm": d1 * a_nm / 2}


_add("lattice_parameter_from_density",
     name="Lattice parameter and atomic radius from the density", topic="crystal structures / density",
     purpose="The lattice parameter a and hard-sphere atomic radius R of a crystal of known structure, density and molar mass.",
     inputs=[_struct_in(_ALL), _phys("rho", "density", "measured density"), _phys("A", "molar_mass", "molar mass")],
     outputs=[_o("a", "length", "lattice parameter a"), _o("a_nm", "number", "lattice parameter a in nm"),
              _o("R_nm", "number", "atomic radius in nm")],
     primary="a_nm", conditions=["rho > 0", "A > 0"],
     method="V_cell = n A / (rho N_A); a = V_cell^(1/3) (HCP with ideal c/a); R from the touching condition",
     latex=r"a = \left(\frac{nA}{\rho N_A}\right)^{1/3}",
     run=lattice_from_density,
     parse=lambda q: _pairs(q, r"lattice parameter from (?:the )?density", {"structure": "structure", "rho": "rho", "density": "rho", "A": "A"},
                            phys={"rho", "A"}, words={"structure"}),
     reference=ref_lattice_from_density,
     wrong=_safe(lambda structure, rho, A: (lambda a: _out(a=a, a_nm=a * 1e9, R_nm=a / _A_PER_R[structure] * 1e9))(
         (A / (rho * NA)) ** (1 / 3))),
     random_inputs=lambda rng: {"structure": rng.choice(_ALL), "rho": rng.uniform(1500, 22000), "A": rng.uniform(0.007, 0.240)},
     templates=["Lattice parameter from density: structure = FCC, rho = 8.96 g/cm^3, A = 63.55 g/mol",
                "Lattice parameter from density: structure = BCC, rho = 7.87 g/cm^3, A = 55.85 g/mol"],
     examples=[{"inputs": {"structure": "FCC", "rho": 8960.0, "A": 0.06355},
                "outputs": {"a_nm": (4 * 0.06355 / (8960 * NA)) ** (1 / 3) * 1e9}},
               {"inputs": {"structure": "BCC", "rho": 7870.0, "A": 0.05585},
                "outputs": {"a_nm": (2 * 0.05585 / (7870 * NA)) ** (1 / 3) * 1e9}}],
     questions=[{"q": "Lattice parameter from density: structure = FCC, rho = 8.96 g/cm^3, A = 63.55 g/mol",
                 "outputs": {"a_nm": (4 * 0.06355 / (8960 * NA)) ** (1 / 3) * 1e9}},
                {"q": "Lattice parameter from density: structure = FCC, rho = 8.96 g/cm^3", "outcome": "NEEDS_INPUT"}])


# ---------------------------------------------------------------------------------------- hexagonal cell
def hcp_cell(a, c, A):
    _pos("a", a)
    _pos("c", c)
    _pos("the molar mass", A)
    V = 1.5 * SQ3 * a * a * c                                   # the hexagonal prism, 6 atoms
    rho = 6 * A / (V * NA)
    return _out(V=V, V_nm3=V * 1e27, rho=rho, rho_gcc=rho / 1000, c_over_a=c / a)


def ref_hcp_cell(a, c, A):
    from pymatgen.core import Lattice
    from scipy.constants import Avogadro
    Vp = Lattice.hexagonal(a * 1e9, c * 1e9).volume             # primitive cell (2 atoms), nm^3
    return {"V_nm3": 3 * Vp, "rho_gcc": 2 * A * 1000 / (Vp * 1e-21 * Avogadro), "c_over_a": c / a}


_add("hcp_cell_density",
     name="Volume and density of a hexagonal close-packed cell", topic="crystal structures / hexagonal cells",
     purpose="The volume of the HCP hexagonal prism (6 atoms), the density and the c/a ratio from a, c and the molar mass.",
     inputs=[_phys("a", "length", "lattice parameter a"), _phys("c", "length", "lattice parameter c"),
             _phys("A", "molar_mass", "molar mass")],
     outputs=[_o("V", "volume", "volume of the hexagonal prism (6 atoms)"), _o("V_nm3", "number", "prism volume in nm^3"),
              _o("rho", "density", "density"), _o("rho_gcc", "number", "density in g/cm^3"), _o("c_over_a", "number", "c/a ratio")],
     primary="rho_gcc", conditions=["a > 0", "c > 0", "A > 0"],
     method="V = (3 sqrt(3)/2) a^2 c, rho = 6 A/(V N_A)", latex=r"V = \frac{3\sqrt3}{2}a^2c,\quad \rho = \frac{6A}{V N_A}",
     run=hcp_cell,
     parse=lambda q: _pairs(q, r"hcp cell|hexagonal cell density", {"a": "a", "c": "c", "A": "A"}, phys={"a", "c", "A"}),
     reference=ref_hcp_cell,
     wrong=_safe(lambda a, c, A: (lambda V: _out(V=V, V_nm3=V * 1e27, rho=6 * A / (V * NA), rho_gcc=6 * A / (V * NA) / 1000,
                                                  c_over_a=c / a))(SQ3 / 2 * a * a * c)),
     random_inputs=lambda rng: (lambda a: {"a": a, "c": a * rng.uniform(1.55, 1.9), "A": rng.uniform(0.009, 0.2)})(rng.uniform(0.22e-9, 0.37e-9)),
     templates=["HCP cell: a = 0.3209 nm, c = 0.5211 nm, A = 24.31 g/mol"],
     examples=[{"inputs": {"a": 0.3209e-9, "c": 0.5211e-9, "A": 0.02431},
                "outputs": {"V_nm3": 1.5 * SQ3 * 0.3209 ** 2 * 0.5211, "rho_gcc": 6 * 24.31 / (1.5 * SQ3 * 0.3209e-7 ** 2 * 0.5211e-7 * NA)}}],
     questions=[{"q": "HCP cell: a = 0.3209 nm, c = 0.5211 nm, A = 24.31 g/mol",
                 "outputs": {"rho_gcc": 6 * 24.31 / (1.5 * SQ3 * 0.3209e-7 ** 2 * 0.5211e-7 * NA)}},
                {"q": "HCP cell: a = 0.3209 nm, A = 24.31 g/mol", "outcome": "NEEDS_INPUT"}])


# ---------------------------------------------------------------------------------------- general cell volume
def _cos_deg(x):
    """cos of an angle in degrees with exact values at multiples of 90 degrees."""
    if x % 90 == 0:
        return [1.0, 0.0, -1.0, 0.0][int(x // 90) % 4]
    return math.cos(math.radians(x))


def _metric(a, b, c, al, be, ga):
    ca, cb, cg = _cos_deg(al), _cos_deg(be), _cos_deg(ga)
    return [[a * a, a * b * cg, a * c * cb], [a * b * cg, b * b, b * c * ca], [a * c * cb, b * c * ca, c * c]]


def _det3(m):
    return (m[0][0] * (m[1][1] * m[2][2] - m[1][2] * m[2][1]) - m[0][1] * (m[1][0] * m[2][2] - m[1][2] * m[2][0])
            + m[0][2] * (m[1][0] * m[2][1] - m[1][1] * m[2][0]))


def _inv3(m):
    d = _det3(m)
    if d <= 0:
        raise ValueError("the cell parameters do not form a cell (the angles are incompatible)")
    c = [[m[1][1] * m[2][2] - m[1][2] * m[2][1], m[0][2] * m[2][1] - m[0][1] * m[2][2], m[0][1] * m[1][2] - m[0][2] * m[1][1]],
         [m[1][2] * m[2][0] - m[1][0] * m[2][2], m[0][0] * m[2][2] - m[0][2] * m[2][0], m[0][2] * m[1][0] - m[0][0] * m[1][2]],
         [m[1][0] * m[2][1] - m[1][1] * m[2][0], m[0][1] * m[2][0] - m[0][0] * m[2][1], m[0][0] * m[1][1] - m[0][1] * m[1][0]]]
    return [[x / d for x in row] for row in c]


def _quad(G, u, v):
    return sum(u[i] * G[i][j] * v[j] for i in range(3) for j in range(3))


def _cell_check(a, b, c, al, be, ga):
    _pos("the cell lengths", a, b, c)
    for x in (al, be, ga):
        if not (0 < x < 180):
            raise ValueError("cell angles must lie between 0 and 180 degrees")
    ca, cb, cg = _cos_deg(al), _cos_deg(be), _cos_deg(ga)
    rad = 1 - ca * ca - cb * cb - cg * cg + 2 * ca * cb * cg
    if rad <= 1e-12:
        raise ValueError("the cell angles are incompatible (no cell has them)")
    return rad


def cell_volume(a, b, c, alpha, beta, gamma):
    rad = _cell_check(a, b, c, alpha, beta, gamma)
    V = a * b * c * math.sqrt(rad)
    return _out(V=V, V_nm3=V * 1e27)


def ref_cell_volume(a, b, c, alpha, beta, gamma):
    from pymatgen.core import Lattice
    return {"V_nm3": Lattice.from_parameters(a * 1e9, b * 1e9, c * 1e9, alpha, beta, gamma).volume}


def _rand_cell(rng):
    while True:
        al, be, ga = (rng.uniform(60, 120) for _ in range(3))
        ca, cb, cg = (math.cos(math.radians(x)) for x in (al, be, ga))
        if 1 - ca * ca - cb * cb - cg * cg + 2 * ca * cb * cg > 0.05:
            return {"a": rng.uniform(0.3e-9, 1.2e-9), "b": rng.uniform(0.3e-9, 1.2e-9), "c": rng.uniform(0.3e-9, 1.2e-9),
                    "alpha": round(al, 3), "beta": round(be, 3), "gamma": round(ga, 3)}


_CELL_NAMES = {"a": "a", "b": "b", "c": "c", "alpha": "alpha", "beta": "beta", "gamma": "gamma"}
_DEG = {"alpha": ["deg", "degrees", "°"], "beta": ["deg", "degrees", "°"], "gamma": ["deg", "degrees", "°"]}


def _cell_in():
    return [_phys("a", "length", "cell edge a"), _phys("b", "length", "cell edge b"), _phys("c", "length", "cell edge c"),
            _numin("alpha", "cell angle alpha in degrees (between b and c)"), _numin("beta", "cell angle beta in degrees (between a and c)"),
            _numin("gamma", "cell angle gamma in degrees (between a and b)")]


_add("unit_cell_volume",
     name="Volume of a general (triclinic) unit cell", topic="crystal structures / unit cells",
     purpose="The volume of a unit cell from its six lattice parameters a, b, c, alpha, beta, gamma.",
     inputs=_cell_in(),
     outputs=[_o("V", "volume", "cell volume"), _o("V_nm3", "number", "cell volume in nm^3")],
     primary="V_nm3", conditions=["0 < angles < 180", "1 - cos^2(alpha) - cos^2(beta) - cos^2(gamma) + 2 cos(alpha) cos(beta) cos(gamma) > 0"],
     method="V = abc sqrt(1 - cos^2 alpha - cos^2 beta - cos^2 gamma + 2 cos alpha cos beta cos gamma)",
     latex=r"V = abc\sqrt{1-\cos^2\alpha-\cos^2\beta-\cos^2\gamma+2\cos\alpha\cos\beta\cos\gamma}",
     run=cell_volume,
     parse=lambda q: _pairs(q, r"unit cell volume|cell volume", _CELL_NAMES, phys={"a", "b", "c"}, num_units=_DEG),
     reference=ref_cell_volume,
     wrong=_safe(lambda a, b, c, alpha, beta, gamma: (lambda V: _out(V=V, V_nm3=V * 1e27))(
         a * b * c * math.sqrt(abs(1 - _cos_deg(alpha) ** 2 - _cos_deg(beta) ** 2 - _cos_deg(gamma) ** 2)))),
     random_inputs=_rand_cell,
     templates=["Unit cell volume: a = 0.5 nm, b = 0.6 nm, c = 0.7 nm, alpha = 90, beta = 100, gamma = 90"],
     examples=[{"inputs": {"a": 0.5e-9, "b": 0.6e-9, "c": 0.7e-9, "alpha": 90, "beta": 100, "gamma": 90},
                "outputs": {"V_nm3": 0.21 * math.sin(math.radians(100))}},
               {"inputs": {"a": 0.4e-9, "b": 0.4e-9, "c": 0.4e-9, "alpha": 90, "beta": 90, "gamma": 90}, "outputs": {"V_nm3": 0.064}}],
     questions=[{"q": "Unit cell volume: a = 0.5 nm, b = 0.6 nm, c = 0.7 nm, alpha = 90, beta = 100, gamma = 90",
                 "outputs": {"V_nm3": 0.21 * math.sin(math.radians(100))}},
                {"q": "Unit cell volume: a = 0.5 nm, b = 0.6 nm, c = 0.7 nm, alpha = 150, beta = 150, gamma = 150", "outcome": "INVALID_INPUT"},
                {"q": "Unit cell volume: a = 0.5 nm, b = 0.6 nm, c = 0.7 nm", "outcome": "NEEDS_INPUT"}])


# ---------------------------------------------------------------------------------------- interplanar spacings
def _d_general(a, b, c, al, be, ga, h, k, l):
    _cell_check(a, b, c, al, be, ga)
    h, k, l = _ints("the Miller indices", h, k, l)
    Gs = _inv3(_metric(a, b, c, al, be, ga))
    return 1 / math.sqrt(_quad(Gs, [h, k, l], [h, k, l]))


def d_cubic(a, h, k, l):
    _pos("the lattice parameter", a)
    h, k, l = _ints("the Miller indices", h, k, l)
    d = a / math.sqrt(h * h + k * k + l * l)
    return _out(d=d, d_nm=d * 1e9)


def d_tetragonal(a, c, h, k, l):
    _pos("the lattice parameters", a, c)
    h, k, l = _ints("the Miller indices", h, k, l)
    d = 1 / math.sqrt((h * h + k * k) / (a * a) + l * l / (c * c))
    return _out(d=d, d_nm=d * 1e9)


def d_orthorhombic(a, b, c, h, k, l):
    _pos("the lattice parameters", a, b, c)
    h, k, l = _ints("the Miller indices", h, k, l)
    d = 1 / math.sqrt(h * h / (a * a) + k * k / (b * b) + l * l / (c * c))
    return _out(d=d, d_nm=d * 1e9)


def d_hexagonal(a, c, h, k, l, i=None):
    _pos("the lattice parameters", a, c)
    h, k, l = _ints("the Miller indices", h, k, l)
    if i is not None and int(i) != -(h + k):
        raise ValueError("in (h k i l) the index i must equal -(h + k)")
    d = 1 / math.sqrt(4 / 3 * (h * h + h * k + k * k) / (a * a) + l * l / (c * c))
    return _out(d=d, d_nm=d * 1e9)


def d_triclinic(a, b, c, alpha, beta, gamma, h, k, l):
    d = _d_general(a, b, c, alpha, beta, gamma, h, k, l)
    return _out(d=d, d_nm=d * 1e9)


def _vd(kind):
    def v(inputs, outputs):
        x = dict(inputs)
        try:
            if kind == "cubic":
                d = _d_general(x["a"], x["a"], x["a"], 90, 90, 90, x["h"], x["k"], x["l"])
            elif kind == "tetragonal":
                d = _d_general(x["a"], x["a"], x["c"], 90, 90, 90, x["h"], x["k"], x["l"])
            elif kind == "orthorhombic":
                d = _d_general(x["a"], x["b"], x["c"], 90, 90, 90, x["h"], x["k"], x["l"])
            elif kind == "hexagonal":
                d = _d_general(x["a"], x["a"], x["c"], 90, 90, 120, x["h"], x["k"], x["l"])
            else:
                d = _d_general(x["a"], x["b"], x["c"], x["alpha"], x["beta"], x["gamma"], x["h"], x["k"], x["l"])
        except Exception:
            return math.inf
        return max(_rel(outputs.get("d"), d), _rel(outputs.get("d_nm"), d * 1e9))
    return v


def _ref_d(lat, h, k, l):
    return {"d_nm": float(lat.d_hkl((h, k, l)))}


def _rand_hkl(rng, lo=-3, hi=3):
    while True:
        h, k, l = (rng.randint(lo, hi) for _ in range(3))
        if (h, k, l) != (0, 0, 0):
            return {"h": h, "k": k, "l": l}


_HKL_IN = [_intin("h", "Miller index h"), _intin("k", "Miller index k"), _intin("l", "Miller index l")]
_HKL_N = {"h": "h", "k": "k", "l": "l"}

_add("d_spacing_cubic",
     name="Interplanar spacing in a cubic crystal", topic="diffraction / interplanar spacing",
     purpose="The spacing of the (h k l) planes of a cubic lattice: d = a / sqrt(h^2 + k^2 + l^2).",
     inputs=[_phys("a", "length", "lattice parameter")] + _HKL_IN,
     outputs=[_o("d", "length", "interplanar spacing"), _o("d_nm", "number", "interplanar spacing in nm")],
     primary="d_nm", conditions=["a > 0", "integer indices, not all zero"],
     method="d = a / sqrt(h^2 + k^2 + l^2)", latex=r"d_{hkl} = \frac{a}{\sqrt{h^2+k^2+l^2}}",
     run=d_cubic, verify=_vd("cubic"),
     parse=lambda q: _pairs(q, r"cubic interplanar spacing|interplanar spacing \(cubic\)", {"a": "a", **_HKL_N}, phys={"a"}),
     reference=lambda a, h, k, l: _ref_d(__import__("pymatgen.core", fromlist=["Lattice"]).Lattice.cubic(a * 1e9), h, k, l),
     wrong=_safe(lambda a, h, k, l: (lambda d: _out(d=d, d_nm=d * 1e9))(a / (h * h + k * k + l * l))),
     random_inputs=lambda rng: {"a": rng.uniform(0.25e-9, 0.7e-9), **_rand_hkl(rng)},
     templates=["Cubic interplanar spacing: a = 0.3615 nm, h = 1, k = 1, l = 1"],
     examples=[{"inputs": {"a": 0.3615e-9, "h": 1, "k": 1, "l": 1}, "outputs": {"d_nm": 0.3615 / SQ3}},
               {"inputs": {"a": 0.2866e-9, "h": 2, "k": 1, "l": 1}, "outputs": {"d_nm": 0.2866 / math.sqrt(6)}},
               {"inputs": {"a": 0.4e-9, "h": 2, "k": 0, "l": 0}, "outputs": {"d_nm": 0.2}}],
     questions=[{"q": "Cubic interplanar spacing: a = 0.3615 nm, h = 1, k = 1, l = 1", "outputs": {"d_nm": 0.3615 / SQ3}},
                {"q": "Cubic interplanar spacing: a = 2.866 angstrom, h = 2, k = 1, l = 1", "outputs": {"d_nm": 0.2866 / math.sqrt(6)}},
                {"q": "Cubic interplanar spacing: a = 0.3615 nm, h = 0, k = 0, l = 0", "outcome": "INVALID_INPUT"},
                {"q": "Cubic interplanar spacing: a = 0.3615 nm, h = 1, k = 1", "outcome": "NEEDS_INPUT"}])

_add("d_spacing_tetragonal",
     name="Interplanar spacing in a tetragonal crystal", topic="diffraction / interplanar spacing",
     purpose="The spacing of the (h k l) planes of a tetragonal lattice: 1/d^2 = (h^2 + k^2)/a^2 + l^2/c^2.",
     inputs=[_phys("a", "length", "lattice parameter a"), _phys("c", "length", "lattice parameter c")] + _HKL_IN,
     outputs=[_o("d", "length", "interplanar spacing"), _o("d_nm", "number", "interplanar spacing in nm")],
     primary="d_nm", conditions=["a, c > 0", "integer indices, not all zero"],
     method="1/d^2 = (h^2 + k^2)/a^2 + l^2/c^2", latex=r"\frac{1}{d^2} = \frac{h^2+k^2}{a^2}+\frac{l^2}{c^2}",
     run=d_tetragonal, verify=_vd("tetragonal"),
     parse=lambda q: _pairs(q, r"tetragonal interplanar spacing|interplanar spacing \(tetragonal\)", {"a": "a", "c": "c", **_HKL_N}, phys={"a", "c"}),
     reference=lambda a, c, h, k, l: _ref_d(__import__("pymatgen.core", fromlist=["Lattice"]).Lattice.tetragonal(a * 1e9, c * 1e9), h, k, l),
     wrong=_safe(lambda a, c, h, k, l: (lambda d: _out(d=d, d_nm=d * 1e9))(1 / math.sqrt((h * h + l * l) / (a * a) + k * k / (c * c)))),
     random_inputs=lambda rng: (lambda h: h if h["k"] != h["l"] else dict(h, l=h["l"] + 1))(
         {"a": rng.uniform(0.25e-9, 0.7e-9), "c": rng.uniform(0.3e-9, 1.2e-9), **_rand_hkl(rng)}),
     templates=["Tetragonal interplanar spacing: a = 0.3 nm, c = 0.5 nm, h = 1, k = 0, l = 1"],
     examples=[{"inputs": {"a": 0.3e-9, "c": 0.5e-9, "h": 1, "k": 0, "l": 1}, "outputs": {"d_nm": 1 / math.sqrt(1 / 0.09 + 1 / 0.25)}},
               {"inputs": {"a": 0.4e-9, "c": 0.6e-9, "h": 0, "k": 0, "l": 2}, "outputs": {"d_nm": 0.3}}],
     questions=[{"q": "Tetragonal interplanar spacing: a = 0.3 nm, c = 0.5 nm, h = 1, k = 0, l = 1",
                 "outputs": {"d_nm": 1 / math.sqrt(1 / 0.09 + 1 / 0.25)}},
                {"q": "Tetragonal interplanar spacing: a = 0.3 nm, h = 1, k = 0, l = 1", "outcome": "NEEDS_INPUT"}])

_add("d_spacing_orthorhombic",
     name="Interplanar spacing in an orthorhombic crystal", topic="diffraction / interplanar spacing",
     purpose="The spacing of the (h k l) planes of an orthorhombic lattice: 1/d^2 = h^2/a^2 + k^2/b^2 + l^2/c^2.",
     inputs=[_phys("a", "length", "lattice parameter a"), _phys("b", "length", "lattice parameter b"),
             _phys("c", "length", "lattice parameter c")] + _HKL_IN,
     outputs=[_o("d", "length", "interplanar spacing"), _o("d_nm", "number", "interplanar spacing in nm")],
     primary="d_nm", conditions=["a, b, c > 0", "integer indices, not all zero"],
     method="1/d^2 = h^2/a^2 + k^2/b^2 + l^2/c^2", latex=r"\frac{1}{d^2} = \frac{h^2}{a^2}+\frac{k^2}{b^2}+\frac{l^2}{c^2}",
     run=d_orthorhombic, verify=_vd("orthorhombic"),
     parse=lambda q: _pairs(q, r"orthorhombic interplanar spacing|interplanar spacing \(orthorhombic\)", {"a": "a", "b": "b", "c": "c", **_HKL_N},
                            phys={"a", "b", "c"}),
     reference=lambda a, b, c, h, k, l: _ref_d(__import__("pymatgen.core", fromlist=["Lattice"]).Lattice.orthorhombic(a * 1e9, b * 1e9, c * 1e9), h, k, l),
     wrong=_safe(lambda a, b, c, h, k, l: (lambda d: _out(d=d, d_nm=d * 1e9))(1 / (abs(h) / a + abs(k) / b + abs(l) / c))),
     random_inputs=lambda rng: {"a": rng.uniform(0.25e-9, 0.7e-9), "b": rng.uniform(0.3e-9, 0.9e-9), "c": rng.uniform(0.3e-9, 1.2e-9), **_rand_hkl(rng)},
     templates=["Orthorhombic interplanar spacing: a = 0.4 nm, b = 0.5 nm, c = 0.6 nm, h = 1, k = 1, l = 0"],
     examples=[{"inputs": {"a": 0.4e-9, "b": 0.5e-9, "c": 0.6e-9, "h": 1, "k": 1, "l": 0}, "outputs": {"d_nm": 1 / math.sqrt(1 / 0.16 + 1 / 0.25)}}],
     questions=[{"q": "Orthorhombic interplanar spacing: a = 0.4 nm, b = 0.5 nm, c = 0.6 nm, h = 1, k = 1, l = 0",
                 "outputs": {"d_nm": 1 / math.sqrt(1 / 0.16 + 1 / 0.25)}}])

_add("d_spacing_hexagonal",
     name="Interplanar spacing in a hexagonal crystal", topic="diffraction / interplanar spacing",
     purpose="The spacing of the (h k l) planes of a hexagonal lattice: 1/d^2 = 4(h^2 + hk + k^2)/(3a^2) + l^2/c^2.",
     inputs=[_phys("a", "length", "lattice parameter a"), _phys("c", "length", "lattice parameter c")] + _HKL_IN
     + [_intin("i", "Miller-Bravais index i = -(h + k) (optional, checked)", required=False)],
     outputs=[_o("d", "length", "interplanar spacing"), _o("d_nm", "number", "interplanar spacing in nm")],
     primary="d_nm", conditions=["a, c > 0", "integer indices, not all zero", "i = -(h + k) when given"],
     method="1/d^2 = 4(h^2 + hk + k^2)/(3a^2) + l^2/c^2", latex=r"\frac{1}{d^2} = \frac{4}{3}\frac{h^2+hk+k^2}{a^2}+\frac{l^2}{c^2}",
     run=d_hexagonal, verify=_vd("hexagonal"),
     parse=lambda q: _pairs(q, r"hexagonal interplanar spacing|interplanar spacing \(hexagonal\)", {"a": "a", "c": "c", "i": "i", **_HKL_N},
                            phys={"a", "c"}),
     reference=lambda a, c, h, k, l, i=None: _ref_d(__import__("pymatgen.core", fromlist=["Lattice"]).Lattice.hexagonal(a * 1e9, c * 1e9), h, k, l),
     wrong=_safe(lambda a, c, h, k, l, i=None: (lambda d: _out(d=d, d_nm=d * 1e9))(1 / math.sqrt((h * h + h * k + k * k) / (a * a) + l * l / (c * c)))),
     random_inputs=lambda rng: (lambda x: x if (x["h"], x["k"]) != (0, 0) else dict(x, h=1))(
         {"a": rng.uniform(0.22e-9, 0.4e-9), "c": rng.uniform(0.35e-9, 0.7e-9), **_rand_hkl(rng)}),
     templates=["Hexagonal interplanar spacing: a = 0.3209 nm, c = 0.5211 nm, h = 1, k = 0, l = 1",
                "Hexagonal interplanar spacing: a = 0.3209 nm, c = 0.5211 nm, h = 1, k = 0, i = -1, l = 1"],
     examples=[{"inputs": {"a": 0.3209e-9, "c": 0.5211e-9, "h": 1, "k": 0, "l": 1},
                "outputs": {"d_nm": 1 / math.sqrt(4 / 3 / 0.3209 ** 2 + 1 / 0.5211 ** 2)}},
               {"inputs": {"a": 0.25e-9, "c": 0.4e-9, "h": 0, "k": 0, "l": 2}, "outputs": {"d_nm": 0.2}}],
     questions=[{"q": "Hexagonal interplanar spacing: a = 0.3209 nm, c = 0.5211 nm, h = 1, k = 0, l = 1",
                 "outputs": {"d_nm": 1 / math.sqrt(4 / 3 / 0.3209 ** 2 + 1 / 0.5211 ** 2)}},
                {"q": "Hexagonal interplanar spacing: a = 0.3209 nm, c = 0.5211 nm, h = 1, k = 0, i = 1, l = 1", "outcome": "INVALID_INPUT"}])

_add("d_spacing_triclinic",
     name="Interplanar spacing in a general (triclinic) lattice", topic="diffraction / interplanar spacing",
     purpose="The spacing of the (h k l) planes of any lattice from its six cell parameters, through the reciprocal metric tensor.",
     inputs=_cell_in() + _HKL_IN,
     outputs=[_o("d", "length", "interplanar spacing"), _o("d_nm", "number", "interplanar spacing in nm")],
     primary="d_nm", conditions=["a valid cell", "integer indices, not all zero"],
     method="1/d^2 = h G* h^T with G* the inverse of the metric tensor G_ij = a_i . a_j",
     latex=r"\frac{1}{d_{hkl}^2} = \mathbf{h}^{\mathsf T} G^{-1}\mathbf{h}",
     run=d_triclinic, verify=_vd("triclinic"),
     parse=lambda q: _pairs(q, r"triclinic interplanar spacing|general interplanar spacing", {**_CELL_NAMES, **_HKL_N},
                            phys={"a", "b", "c"}, num_units=_DEG),
     reference=lambda a, b, c, alpha, beta, gamma, h, k, l: _ref_d(__import__("pymatgen.core", fromlist=["Lattice"]).Lattice.from_parameters(
         a * 1e9, b * 1e9, c * 1e9, alpha, beta, gamma), h, k, l),
     wrong=_safe(lambda a, b, c, alpha, beta, gamma, h, k, l: (lambda d: _out(d=d, d_nm=d * 1e9))(
         1 / math.sqrt(_quad(_metric(1 / a, 1 / b, 1 / c, alpha, beta, gamma), [h, k, l], [h, k, l])))),
     random_inputs=lambda rng: {**_rand_cell(rng), **_rand_hkl(rng)},
     templates=["Triclinic interplanar spacing: a = 0.5 nm, b = 0.6 nm, c = 0.7 nm, alpha = 80, beta = 95, gamma = 105, h = 1, k = 1, l = 0"],
     examples=[{"inputs": {"a": 0.5e-9, "b": 0.6e-9, "c": 0.7e-9, "alpha": 90, "beta": 90, "gamma": 90, "h": 1, "k": 1, "l": 0},
                "outputs": {"d_nm": 1 / math.sqrt(4 + 1 / 0.36)}},
               {"inputs": {"a": 0.5e-9, "b": 0.5e-9, "c": 0.7e-9, "alpha": 90, "beta": 90, "gamma": 120, "h": 1, "k": 0, "l": 0},
                "outputs": {"d_nm": 0.5 * SQ3 / 2}}],
     questions=[{"q": "Triclinic interplanar spacing: a = 0.5 nm, b = 0.5 nm, c = 0.7 nm, alpha = 90, beta = 90, gamma = 120, h = 1, k = 0, l = 0",
                 "outputs": {"d_nm": 0.5 * SQ3 / 2}}])


# ---------------------------------------------------------------------------------------- Bragg's law and X-ray diffraction
def _xrd_two_theta(a_ang, lam_ang, hkl, window=None, struct_species="Cu"):
    """2-theta of the (hkl) family in pymatgen's own X-ray diffraction calculator for a simple-cubic lattice of edge
    a (angstrom) at wavelength lam (angstrom); NaN if the reflection is not in the pattern."""
    from pymatgen.analysis.diffraction.xrd import XRDCalculator
    from pymatgen.core import Lattice, Structure
    s = Structure(Lattice.cubic(a_ang), [struct_species], [[0, 0, 0]])
    target = sorted(abs(x) for x in hkl)
    rng = (0, 179.999) if window is None else (max(0.001, window[0]), min(179.999, window[1]))
    pat = XRDCalculator(wavelength=lam_ang).get_pattern(s, scaled=False, two_theta_range=rng)
    for x, fam in zip(pat.x, pat.hkls):
        for f in fam:
            if sorted(abs(v) for v in f["hkl"]) == target:
                return float(x)
    return float("nan")


def bragg_angle(d, lam, n=1):
    _pos("the spacing and wavelength", d, lam)
    n = int(n)
    if n < 1:
        raise ValueError("the order n must be a positive whole number")
    s = n * lam / (2 * d)
    if not s < 1:
        raise ValueError("n lambda must be smaller than 2d (no reflection otherwise)")
    th = math.degrees(math.asin(s))
    return _out(theta_deg=th, two_theta_deg=2 * th)


def ref_bragg_angle(d, lam, n=1):
    tt = _xrd_two_theta(d * 1e10, lam * 1e10, (int(n), 0, 0))
    return {"theta_deg": tt / 2, "two_theta_deg": tt}


_ORDER = _intin("n", "order of the reflection (default 1)", lo=1, hi=10, required=False)

_add("bragg_angle",
     name="Bragg angle of a reflection", topic="diffraction / Bragg's law",
     purpose="The diffraction angle theta (and 2-theta) of planes of spacing d for X-rays of wavelength lambda: n lambda = 2 d sin(theta).",
     inputs=[_phys("d", "length", "interplanar spacing"), _phys("lam", "length", "X-ray wavelength"), _ORDER],
     outputs=[_o("theta_deg", "number", "Bragg angle theta in degrees"), _o("two_theta_deg", "number", "diffraction angle 2-theta in degrees")],
     primary="two_theta_deg", conditions=["d, lambda > 0", "n lambda < 2 d"],
     method="theta = asin(n lambda / (2 d))", latex=r"n\lambda = 2d\sin\theta",
     run=bragg_angle,
     parse=lambda q: _pairs(q, r"bragg angle", {"d": "d", "lam": "lam", "lambda": "lam", "wavelength": "lam", "n": "n"}, phys={"d", "lam"}),
     reference=ref_bragg_angle,
     wrong=_safe(lambda d, lam, n=1: (lambda th: _out(theta_deg=th, two_theta_deg=2 * th))(math.degrees(math.asin(n * lam / d)))),
     random_inputs=lambda rng: (lambda d, n: {"d": d, "n": n, "lam": rng.uniform(0.2, 0.95) * 2 * d / n})(
         rng.uniform(0.1e-9, 0.4e-9), rng.choice([1, 1, 1, 2, 3])),
     templates=["Bragg angle: d = 0.2087 nm, lambda = 0.1542 nm", "Bragg angle: d = 0.2087 nm, lambda = 1.542 angstrom, n = 2"],
     examples=[{"inputs": {"d": 0.2087e-9, "lam": 0.1542e-9}, "outputs": {"theta_deg": math.degrees(math.asin(0.1542 / 0.4174))}},
               {"inputs": {"d": 0.2e-9, "lam": 0.2e-9, "n": 1}, "outputs": {"theta_deg": 30.0, "two_theta_deg": 60.0}}],
     questions=[{"q": "Bragg angle: d = 0.2087 nm, lambda = 0.1542 nm", "outputs": {"theta_deg": math.degrees(math.asin(0.1542 / 0.4174))}},
                {"q": "Bragg angle: d = 0.2 nm, lambda = 0.2 nm", "outputs": {"two_theta_deg": 60.0}},
                {"q": "Bragg angle: d = 0.1 nm, lambda = 0.3 nm", "outcome": "INVALID_INPUT"},
                {"q": "Bragg angle: d = 0.2 nm", "outcome": "NEEDS_INPUT"}])


def bragg_wavelength(d, theta, n=1):
    _pos("the spacing", d)
    n = int(n)
    if n < 1:
        raise ValueError("the order n must be a positive whole number")
    if not 0 < theta < 90:
        raise ValueError("the Bragg angle theta must lie between 0 and 90 degrees")
    lam = 2 * d * math.sin(math.radians(theta)) / n
    return _out(lam=lam, lam_nm=lam * 1e9)


def ref_bragg_wavelength(d, theta, n=1):
    from scipy.optimize import brentq
    n = int(n)
    win = (2 * theta - 5, 2 * theta + 5)
    f = lambda lam: _xrd_two_theta(d * 1e10, lam, (n, 0, 0), win) - 2 * theta         # noqa: E731
    # the bracket (0.1 % around the closed form) only localises the root; the root itself is pymatgen's
    guess = 2 * d * 1e10 * math.sin(math.radians(theta)) / n
    lam = brentq(f, guess * (1 - 1e-3), guess * (1 + 1e-3), xtol=1e-15, rtol=8.9e-16, maxiter=200)
    return {"lam_nm": lam / 10}


_add("bragg_wavelength",
     name="X-ray wavelength from a Bragg reflection", topic="diffraction / Bragg's law",
     purpose="The wavelength that reflects from planes of spacing d at the Bragg angle theta in order n.",
     inputs=[_phys("d", "length", "interplanar spacing"), _numin("theta", "Bragg angle theta in degrees (half of 2-theta)"), _ORDER],
     outputs=[_o("lam", "length", "wavelength"), _o("lam_nm", "number", "wavelength in nm")],
     primary="lam_nm", conditions=["d > 0", "0 < theta < 90 degrees"],
     method="lambda = 2 d sin(theta) / n", latex=r"\lambda = \frac{2d\sin\theta}{n}",
     run=bragg_wavelength,
     parse=lambda q: _pairs(q, r"bragg wavelength", {"d": "d", "theta": "theta", "n": "n"}, phys={"d"}, num_units={"theta": ["deg", "degrees", "°"]}),
     reference=ref_bragg_wavelength,
     wrong=_safe(lambda d, theta, n=1: (lambda lam: _out(lam=lam, lam_nm=lam * 1e9))(d * math.sin(math.radians(theta)) / n)),
     random_inputs=lambda rng: {"d": rng.uniform(0.1e-9, 0.4e-9), "theta": round(rng.uniform(8, 70), 4), "n": rng.choice([1, 1, 2, 3])},
     templates=["Bragg wavelength: d = 0.2 nm, theta = 22.5 deg", "Bragg wavelength: d = 0.2 nm, theta = 40 deg, n = 2"],
     examples=[{"inputs": {"d": 0.2e-9, "theta": 30.0}, "outputs": {"lam_nm": 0.2}},
               {"inputs": {"d": 0.3e-9, "theta": 30.0, "n": 2}, "outputs": {"lam_nm": 0.15}}],
     questions=[{"q": "Bragg wavelength: d = 0.2 nm, theta = 30 deg", "outputs": {"lam_nm": 0.2}},
                {"q": "Bragg wavelength: d = 0.2 nm, theta = 95 deg", "outcome": "INVALID_INPUT"}])


def bragg_spacing(lam, theta, n=1):
    _pos("the wavelength", lam)
    n = int(n)
    if n < 1:
        raise ValueError("the order n must be a positive whole number")
    if not 0 < theta < 90:
        raise ValueError("the Bragg angle theta must lie between 0 and 90 degrees")
    d = n * lam / (2 * math.sin(math.radians(theta)))
    return _out(d=d, d_nm=d * 1e9)


def ref_bragg_spacing(lam, theta, n=1):
    from scipy.optimize import brentq
    n = int(n)
    win = (2 * theta - 5, 2 * theta + 5)
    f = lambda d: _xrd_two_theta(d, lam * 1e10, (n, 0, 0), win) - 2 * theta             # noqa: E731
    guess = n * lam * 1e10 / (2 * math.sin(math.radians(theta)))     # localises the root only (0.1 % bracket)
    d = brentq(f, guess * (1 - 1e-3), guess * (1 + 1e-3), xtol=1e-15, rtol=8.9e-16, maxiter=200)
    return {"d_nm": d / 10}


_add("bragg_spacing",
     name="Interplanar spacing from a Bragg reflection", topic="diffraction / Bragg's law",
     purpose="The interplanar spacing d of planes that reflect X-rays of wavelength lambda at the Bragg angle theta in order n.",
     inputs=[_phys("lam", "length", "X-ray wavelength"), _numin("theta", "Bragg angle theta in degrees (half of 2-theta)"), _ORDER],
     outputs=[_o("d", "length", "interplanar spacing"), _o("d_nm", "number", "interplanar spacing in nm")],
     primary="d_nm", conditions=["lambda > 0", "0 < theta < 90 degrees"],
     method="d = n lambda / (2 sin(theta))", latex=r"d = \frac{n\lambda}{2\sin\theta}",
     run=bragg_spacing,
     parse=lambda q: _pairs(q, r"bragg spacing|interplanar spacing from (?:a )?bragg (?:angle|reflection)",
                            {"lam": "lam", "lambda": "lam", "wavelength": "lam", "theta": "theta", "n": "n"}, phys={"lam"},
                            num_units={"theta": ["deg", "degrees", "°"]}),
     reference=ref_bragg_spacing,
     wrong=_safe(lambda lam, theta, n=1: (lambda d: _out(d=d, d_nm=d * 1e9))(n * lam / (2 * math.cos(math.radians(theta))))),
     random_inputs=lambda rng: {"lam": rng.uniform(0.05e-9, 0.25e-9), "theta": round(rng.uniform(8, 70), 4), "n": rng.choice([1, 1, 2, 3])},
     templates=["Bragg spacing: lambda = 0.1542 nm, theta = 21.65 deg", "Bragg spacing: lambda = 1.542 angstrom, theta = 30 deg, n = 2"],
     examples=[{"inputs": {"lam": 0.1542e-9, "theta": 30.0}, "outputs": {"d_nm": 0.1542}},
               {"inputs": {"lam": 0.1e-9, "theta": 30.0, "n": 2}, "outputs": {"d_nm": 0.2}}],
     questions=[{"q": "Bragg spacing: lambda = 0.1542 nm, theta = 30 deg", "outputs": {"d_nm": 0.1542}},
                {"q": "Bragg spacing: lambda = 0.1542 nm", "outcome": "NEEDS_INPUT"}])


def cubic_reflection(a, h, k, l, lam):
    _pos("the lattice parameter and wavelength", a, lam)
    h, k, l = _ints("the Miller indices", h, k, l)
    d = a / math.sqrt(h * h + k * k + l * l)
    s = lam / (2 * d)
    if not s < 1:
        raise ValueError("this reflection does not occur at this wavelength (lambda >= 2d)")
    th = math.degrees(math.asin(s))
    return _out(d_nm=d * 1e9, theta_deg=th, two_theta_deg=2 * th)


def ref_cubic_reflection(a, h, k, l, lam):
    tt = _xrd_two_theta(a * 1e10, lam * 1e10, (h, k, l))
    from pymatgen.core import Lattice
    return {"d_nm": float(Lattice.cubic(a * 1e9).d_hkl((h, k, l))), "two_theta_deg": tt, "theta_deg": tt / 2}


def _rand_reflection(rng):
    while True:
        x = {"a": rng.uniform(0.28e-9, 0.6e-9), **_rand_hkl(rng, 0, 3), "lam": rng.uniform(0.07e-9, 0.2e-9)}
        d = x["a"] / math.sqrt(x["h"] ** 2 + x["k"] ** 2 + x["l"] ** 2)
        if x["lam"] / (2 * d) < 0.97:
            return x


_add("cubic_reflection_angle",
     name="Diffraction angle of a cubic (h k l) reflection", topic="diffraction / X-ray diffraction",
     purpose="The interplanar spacing and the diffraction angle 2-theta of the (h k l) reflection of a cubic crystal of lattice parameter a at wavelength lambda.",
     inputs=[_phys("a", "length", "lattice parameter")] + _HKL_IN + [_phys("lam", "length", "X-ray wavelength")],
     outputs=[_o("d_nm", "number", "interplanar spacing in nm"), _o("theta_deg", "number", "Bragg angle in degrees"),
              _o("two_theta_deg", "number", "diffraction angle 2-theta in degrees")],
     primary="two_theta_deg", conditions=["a, lambda > 0", "lambda < 2 d"],
     method="d = a/sqrt(h^2 + k^2 + l^2), sin(theta) = lambda/(2 d) (first order)",
     latex=r"\sin\theta = \frac{\lambda\sqrt{h^2+k^2+l^2}}{2a}",
     run=cubic_reflection,
     parse=lambda q: _pairs(q, r"cubic reflection angle|diffraction angle of a cubic reflection",
                            {"a": "a", "lam": "lam", "lambda": "lam", "wavelength": "lam", **_HKL_N}, phys={"a", "lam"}),
     reference=ref_cubic_reflection,
     wrong=_safe(lambda a, h, k, l, lam: (lambda d: (lambda th: _out(d_nm=d * 1e9, theta_deg=th, two_theta_deg=th))(
         math.degrees(math.asin(lam / (2 * d)))))(a / math.sqrt(h * h + k * k + l * l))),
     random_inputs=_rand_reflection,
     templates=["Cubic reflection angle: a = 0.3615 nm, h = 1, k = 1, l = 1, lambda = 0.1542 nm"],
     examples=[{"inputs": {"a": 0.3615e-9, "h": 1, "k": 1, "l": 1, "lam": 0.1542e-9},
                "outputs": {"two_theta_deg": 2 * math.degrees(math.asin(0.1542 * SQ3 / (2 * 0.3615)))}},
               {"inputs": {"a": 0.4e-9, "h": 2, "k": 0, "l": 0, "lam": 0.2e-9}, "outputs": {"two_theta_deg": 60.0}}],
     questions=[{"q": "Cubic reflection angle: a = 0.3615 nm, h = 1, k = 1, l = 1, lambda = 0.1542 nm",
                 "outputs": {"two_theta_deg": 2 * math.degrees(math.asin(0.1542 * SQ3 / (2 * 0.3615)))}},
                {"q": "Cubic reflection angle: a = 0.3 nm, h = 3, k = 3, l = 3, lambda = 0.2 nm", "outcome": "INVALID_INPUT"}])


# ---------------------------------------------------------------------------------------- structure factor
_BASIS = {"SC": [(0, 0, 0)],
          "BCC": [(0, 0, 0), (0.5, 0.5, 0.5)],
          "FCC": [(0, 0, 0), (0.5, 0.5, 0), (0.5, 0, 0.5), (0, 0.5, 0.5)],
          "diamond": [(0, 0, 0), (0.5, 0.5, 0), (0.5, 0, 0.5), (0, 0.5, 0.5),
                      (0.25, 0.25, 0.25), (0.75, 0.75, 0.25), (0.75, 0.25, 0.75), (0.25, 0.75, 0.75)]}


def _f_rule(structure, h, k, l):
    """|F|/f from the extinction rules (no sum over atoms)."""
    if structure == "SC":
        return 1.0
    if structure == "BCC":
        return 2.0 if (h + k + l) % 2 == 0 else 0.0
    same = (h % 2 == k % 2 == l % 2)
    if structure == "FCC":
        return 4.0 if same else 0.0
    if not same:
        return 0.0
    if h % 2:                                                   # all odd
        return 4 * SQ2
    return 8.0 if (h + k + l) % 4 == 0 else 0.0


def _f_sum(structure, h, k, l):
    re_ = sum(math.cos(2 * math.pi * (h * x + k * y + l * z)) for x, y, z in _BASIS[structure])
    im_ = sum(math.sin(2 * math.pi * (h * x + k * y + l * z)) for x, y, z in _BASIS[structure])
    v = math.hypot(re_, im_)
    return 0.0 if v < 1e-9 else v


def structure_factor(structure, h, k, l):
    h, k, l = _ints("the Miller indices", h, k, l)
    F = _f_rule(structure, h, k, l)
    return _out(F_over_f=F, F2_over_f2=F * F, allowed=F > 0)


def verify_structure_factor(inputs, outputs):
    try:
        F = _f_sum(inputs["structure"], int(inputs["h"]), int(inputs["k"]), int(inputs["l"]))
    except Exception:
        return math.inf
    return max(abs(float(outputs["F_over_f"]) - F), abs(float(outputs["F2_over_f2"]) - F * F) / max(1.0, F * F),
               0.0 if outputs["allowed"] == (F > 0) else 1.0)


def ref_structure_factor(structure, h, k, l):
    import numpy as np
    pos = _ase_cell(structure, 1.0).get_scaled_positions()
    F = complex(np.sum(np.exp(2j * np.pi * (pos @ np.array([h, k, l], float)))))
    v = abs(F)
    v = 0.0 if v < 1e-9 else float(f"{v:.12g}")
    return {"F_over_f": v, "F2_over_f2": float(f"{v * v:.12g}") if v else 0.0, "allowed": v > 0}


def _wrong_sf(structure, h, k, l):
    """The classic slip: every atom scattering in phase (F = n f), extinctions forgotten."""
    F = float(_N_ATOMS[structure])
    return _out(F_over_f=F, F2_over_f2=F * F, allowed=True)


_add("structure_factor_cubic",
     name="Structure factor and extinction of a cubic reflection", topic="diffraction / structure factor",
     purpose="The structure factor |F|/f of the (h k l) reflection of a monatomic SC, BCC, FCC or diamond-cubic crystal, and whether the reflection is allowed.",
     inputs=[_struct_in(_CUBIC)] + _HKL_IN,
     outputs=[_o("F_over_f", "number", "|F| in units of the atomic scattering factor f"),
              _o("F2_over_f2", "number", "|F|^2 / f^2 (proportional to intensity)"), _o("allowed", "boolean", "whether the reflection appears")],
     primary="F_over_f", conditions=["integer indices, not all zero"],
     method="extinction rules: BCC h+k+l even (F = 2f); FCC unmixed (F = 4f); diamond: all odd (4 sqrt(2) f), all even with h+k+l = 4m (8f), otherwise 0",
     latex=r"F_{hkl} = f\sum_j e^{2\pi i(hx_j+ky_j+lz_j)}",
     run=structure_factor, verify=verify_structure_factor,
     parse=lambda q: _pairs(q, r"structure factor", {"structure": "structure", **_HKL_N}, words={"structure"}),
     reference=ref_structure_factor, wrong=_safe(_wrong_sf),
     random_inputs=lambda rng: {"structure": rng.choice(["BCC", "FCC", "diamond"]), **_rand_hkl(rng, -4, 4)},
     templates=["Structure factor: structure = FCC, h = 1, k = 1, l = 1", "Structure factor: structure = BCC, h = 1, k = 0, l = 0"],
     examples=[{"inputs": {"structure": "FCC", "h": 1, "k": 1, "l": 1}, "outputs": {"F_over_f": 4.0, "allowed": True}},
               {"inputs": {"structure": "FCC", "h": 1, "k": 1, "l": 0}, "outputs": {"F_over_f": 0.0, "allowed": False}},
               {"inputs": {"structure": "BCC", "h": 1, "k": 1, "l": 0}, "outputs": {"F_over_f": 2.0}},
               {"inputs": {"structure": "diamond", "h": 2, "k": 2, "l": 2}, "outputs": {"F_over_f": 0.0}},
               {"inputs": {"structure": "diamond", "h": 1, "k": 1, "l": 1}, "outputs": {"F_over_f": 4 * SQ2}}],
     questions=[{"q": "Structure factor: structure = FCC, h = 2, k = 0, l = 0", "outputs": {"F_over_f": 4.0, "allowed": True}},
                {"q": "Structure factor: structure = BCC, h = 1, k = 0, l = 0", "outputs": {"F_over_f": 0.0, "allowed": False}},
                {"q": "Structure factor: structure = FCC, h = 0, k = 0, l = 0", "outcome": "INVALID_INPUT"}])


def allowed_reflections(structure, count=6):
    count = int(count)
    if not 1 <= count <= 15:
        raise ValueError("count must be between 1 and 15")
    Ns = []
    N = 1
    while len(Ns) < count:
        ok = False
        r = int(math.isqrt(N))
        for h in range(r + 1):
            for k in range(h + 1):
                l2 = N - h * h - k * k
                if l2 < 0:
                    continue
                l = math.isqrt(l2)
                if l * l == l2 and l <= k and _f_rule(structure, h, k, l) > 0:
                    ok = True
        if ok:
            Ns.append(N)
        N += 1
    return {"N_values": Ns, "N_last": Ns[-1], "N_first": Ns[0]}


def ref_allowed_reflections(structure, count=6):
    import numpy as np
    pos = _ase_cell(structure, 1.0).get_scaled_positions()
    found = set()
    for h, k, l in _iproduct(range(0, 9), repeat=3):
        if (h, k, l) == (0, 0, 0):
            continue
        F = abs(complex(np.sum(np.exp(2j * np.pi * (pos @ np.array([h, k, l], float))))))
        if F > 1e-6:
            found.add(h * h + k * k + l * l)
    Ns = sorted(found)[:int(count)]
    return {"N_values": Ns, "N_last": Ns[-1], "N_first": Ns[0]}


def _wrong_allowed(structure, count=6):
    rule = {"SC": "BCC", "BCC": "FCC", "FCC": "BCC", "diamond": "FCC"}[structure]
    return allowed_reflections(rule, count)


_add("allowed_reflections_cubic",
     name="First allowed reflections of a cubic crystal", topic="diffraction / structure factor",
     purpose="The values of h^2 + k^2 + l^2 of the first allowed X-ray reflections of a monatomic SC, BCC, FCC or diamond-cubic crystal, in order of increasing angle.",
     inputs=[_struct_in(_CUBIC), _intin("count", "how many reflections (1 to 15, default 6)", lo=1, hi=15, required=False)],
     outputs=[_o("N_values", "number", "h^2 + k^2 + l^2 of the allowed reflections, ascending"),
              _o("N_last", "number", "the last of them"), _o("N_first", "number", "the first allowed reflection")],
     primary="N_last", conditions=["1 <= count <= 15"],
     method="each N = h^2 + k^2 + l^2 is allowed when some (h k l) with that sum passes the extinction rules",
     latex=r"N = h^2+k^2+l^2,\quad \sin^2\theta = \frac{\lambda^2}{4a^2}N",
     run=allowed_reflections,
     parse=lambda q: _pairs(q, r"allowed reflections", {"structure": "structure", "count": "count", "n": "count"}, words={"structure"}),
     reference=ref_allowed_reflections, wrong=_safe(_wrong_allowed),
     random_inputs=lambda rng: {"structure": rng.choice(_CUBIC), "count": rng.randint(4, 12)},
     templates=["Allowed reflections: structure = BCC, count = 6", "Allowed reflections: structure = FCC"],
     examples=[{"inputs": {"structure": "BCC", "count": 6}, "outputs": {"N_values": [2, 4, 6, 8, 10, 12]}},
               {"inputs": {"structure": "FCC", "count": 6}, "outputs": {"N_values": [3, 4, 8, 11, 12, 16]}},
               {"inputs": {"structure": "SC", "count": 7}, "outputs": {"N_values": [1, 2, 3, 4, 5, 6, 8]}},
               {"inputs": {"structure": "diamond", "count": 5}, "outputs": {"N_values": [3, 8, 11, 16, 19]}}],
     questions=[{"q": "Allowed reflections: structure = FCC, count = 6", "outputs": {"N_values": [3, 4, 8, 11, 12, 16]}},
                {"q": "Allowed reflections: structure = BCC", "outputs": {"N_values": [2, 4, 6, 8, 10, 12]}},
                {"q": "Allowed reflections: structure = FCC, count = 40", "outcome": "INVALID_INPUT"}])

# ---------------------------------------------------------------------------------------- linear and planar density
_BRAVAIS = ("SC", "BCC", "FCC")
_NN_PER_A = {"SC": 1.0, "BCC": SQ3 / 2, "FCC": 1 / SQ2}


def _lattice_points_half(structure, span):
    """Lattice points of ASE's cell, in units of a/2, as integer triples within [-span, span]^3."""
    key = (structure, span)
    if key not in _LP_CACHE:
        import numpy as np
        base = np.rint(_ase_cell(structure, 1.0).get_scaled_positions() * 2).astype(int) % 2
        m = span // 2 + 1
        r = np.arange(-m, m + 1)
        T = 2 * np.stack(np.meshgrid(r, r, r, indexing="ij"), axis=-1).reshape(-1, 3)
        P = (T[:, None, :] + base[None, :, :]).reshape(-1, 3)
        P = np.unique(P[np.abs(P).max(axis=1) <= span], axis=0)
        _LP_CACHE[key] = P
    return _LP_CACHE[key]


_LP_CACHE: dict = {}


def linear_density(structure, u, v, w, a):
    _pos("the lattice parameter", a)
    u, v, w = _ints("the direction indices", u, v, w)
    g = _gcd3(u, v, w)
    u, v, w = u // g, v // g, w // g
    L = a * math.sqrt(u * u + v * v + w * w)
    odd = sum(x % 2 for x in (u, v, w))
    half = (structure == "BCC" and odd == 3) or (structure == "FCC" and odd == 2)
    rep = L / 2 if half else L                                 # repeat distance along [uvw]
    ld = 1 / rep
    return _out(LD_per_nm=ld * 1e-9, LD=ld, repeat_nm=rep * 1e9, LPF=_NN_PER_A[structure] * a * ld)


def ref_linear_density(structure, u, v, w, a):
    import numpy as np
    pts = _lattice_points_half(structure, 14)
    d = np.array([u, v, w])
    on = pts[(np.abs(np.cross(pts, d)).sum(axis=1) == 0) & (pts @ d > 0)]
    rep = float(np.min(np.linalg.norm(on, axis=1))) * a / 2
    nn = _ase_nn(_ase_cell(structure, 1.0), 1.0)[0] * a
    return {"LD_per_nm": 1e-9 / rep, "repeat_nm": rep * 1e9, "LPF": nn / rep}


_UVW_N = {"u": "u", "v": "v", "w": "w"}
_UVW_IN = [_intin("u", "direction index u", -4, 4), _intin("v", "direction index v", -4, 4), _intin("w", "direction index w", -4, 4)]


def _rand_uvw(rng, lo=-3, hi=3):
    while True:
        u, v, w = (rng.randint(lo, hi) for _ in range(3))
        if (u, v, w) != (0, 0, 0):
            return {"u": u, "v": v, "w": w}


_add("linear_density",
     name="Linear density along a crystal direction", topic="crystal structures / linear and planar density",
     purpose="The number of atoms per unit length along the [u v w] direction of an SC, BCC or FCC crystal, and the linear packing fraction.",
     inputs=[_struct_in(_BRAVAIS)] + _UVW_IN + [_phys("a", "length", "lattice parameter")],
     outputs=[_o("LD_per_nm", "number", "atoms per nm along [uvw]"), _o("LD", "linear_atom_density", "atoms per metre along [uvw]"),
              _o("repeat_nm", "number", "repeat distance along [uvw] in nm"), _o("LPF", "number", "linear packing fraction (hard spheres)")],
     primary="LD_per_nm", conditions=["a > 0", "integer indices between -4 and 4, not all zero"],
     method="the shortest lattice translation along [uvw]: |[uvw]| a, halved when the half vector is a lattice vector (BCC all odd, FCC two odd)",
     latex=r"\mathrm{LD} = \frac{\text{atoms centred on the line}}{\text{length of the line}}",
     run=linear_density,
     parse=lambda q: _pairs(q, r"linear density", {"structure": "structure", "a": "a", **_UVW_N}, phys={"a"}, words={"structure"}),
     reference=ref_linear_density,
     wrong=_safe(lambda structure, u, v, w, a: (lambda o: _out(LD_per_nm=2 * o["LD_per_nm"], LD=2 * o["LD"], repeat_nm=o["repeat_nm"],
                                                               LPF=2 * o["LPF"]))(linear_density(structure, u, v, w, a))),   # end atoms counted whole
     random_inputs=lambda rng: {"structure": rng.choice(["BCC", "FCC", "FCC"]), **_rand_uvw(rng, -2, 2), "a": rng.uniform(0.28e-9, 0.6e-9)},
     templates=["Linear density: structure = FCC, u = 1, v = 1, w = 0, a = 0.3615 nm",
                "Linear density: structure = BCC, u = 1, v = 1, w = 1, a = 0.2866 nm"],
     examples=[{"inputs": {"structure": "FCC", "u": 1, "v": 1, "w": 0, "a": 0.3615e-9}, "outputs": {"LD_per_nm": SQ2 / 0.3615, "LPF": 1.0}},
               {"inputs": {"structure": "BCC", "u": 1, "v": 1, "w": 1, "a": 0.2866e-9}, "outputs": {"LD_per_nm": 2 / (SQ3 * 0.2866), "LPF": 1.0}},
               {"inputs": {"structure": "FCC", "u": 1, "v": 0, "w": 0, "a": 0.4e-9}, "outputs": {"LD_per_nm": 2.5, "LPF": 1 / SQ2}}],
     questions=[{"q": "Linear density: structure = FCC, u = 1, v = 1, w = 0, a = 0.3615 nm", "outputs": {"LD_per_nm": SQ2 / 0.3615}},
                {"q": "Linear density: structure = BCC, u = 1, v = 0, w = 0, a = 0.2866 nm", "outputs": {"LD_per_nm": 1 / 0.2866}},
                {"q": "Linear density: structure = FCC, u = 0, v = 0, w = 0, a = 0.3615 nm", "outcome": "INVALID_INPUT"}])


def _reciprocal_ok(structure, h, k, l):
    """(h k l) (coprime) is the shortest reciprocal-lattice vector along its direction (lattice planes, not reflections)."""
    if structure == "SC":
        return True
    if structure == "BCC":
        return (h + k + l) % 2 == 0
    return h % 2 == k % 2 == l % 2


def planar_density(structure, h, k, l, a):
    _pos("the lattice parameter", a)
    h, k, l = _ints("the Miller indices", h, k, l)
    g = _gcd3(h, k, l)
    h, k, l = h // g, k // g, l // g
    N = h * h + k * k + l * l
    d = a / math.sqrt(N) / (1 if _reciprocal_ok(structure, h, k, l) else 2)     # spacing of the lattice planes
    pd = _N_ATOMS[structure] / a ** 3 * d                       # atoms per area of one plane
    R = _NN_PER_A[structure] * a / 2
    return _out(PD_per_nm2=pd * 1e-18, PD=pd, PPF=pd * math.pi * R * R)


def ref_planar_density(structure, h, k, l, a):
    import numpy as np
    pts = _lattice_points_half(structure, 14)
    inplane = pts[(pts @ np.array([h, k, l]) == 0) & (np.abs(pts).sum(axis=1) > 0)]
    norms = np.linalg.norm(inplane, axis=1)
    order = np.argsort(norms, kind="stable")
    v1 = inplane[order[0]]
    area = None
    for idx in order[1:]:
        c = np.linalg.norm(np.cross(v1, inplane[idx]))
        if c > 1e-9:
            area = c                                            # shortest and next shortest independent: a 2-D basis
            break
    pd = 1 / (area * (a / 2) ** 2)
    nn = _ase_nn(_ase_cell(structure, 1.0), 1.0)[0] * a
    return {"PD_per_nm2": pd * 1e-18, "PPF": pd * math.pi * (nn / 2) ** 2}


_add("planar_density",
     name="Planar density of a crystal plane", topic="crystal structures / linear and planar density",
     purpose="The number of atoms per unit area on the (h k l) plane of an SC, BCC or FCC crystal, and the planar packing fraction.",
     inputs=[_struct_in(_BRAVAIS), _intin("h", "Miller index h", -3, 3), _intin("k", "Miller index k", -3, 3),
             _intin("l", "Miller index l", -3, 3), _phys("a", "length", "lattice parameter")],
     outputs=[_o("PD_per_nm2", "number", "atoms per nm^2 on the plane"), _o("PD", "planar_atom_density", "atoms per m^2 on the plane"),
              _o("PPF", "number", "planar packing fraction (hard spheres)")],
     primary="PD_per_nm2", conditions=["a > 0", "integer indices between -3 and 3, not all zero"],
     method="PD = (atoms per volume) x (spacing of the lattice planes parallel to (hkl)); the lattice-plane spacing is a/sqrt(N), halved when (hkl) is not a reciprocal-lattice vector (BCC h+k+l odd, FCC mixed)",
     latex=r"\mathrm{PD} = n_V\,d_{\mathrm{planes}},\quad \mathrm{PPF} = \mathrm{PD}\,\pi R^2",
     run=planar_density,
     parse=lambda q: _pairs(q, r"planar density", {"structure": "structure", "a": "a", **_HKL_N}, phys={"a"}, words={"structure"}),
     reference=ref_planar_density,
     wrong=_safe(lambda structure, h, k, l, a: (lambda pd: _out(PD_per_nm2=pd * 1e-18, PD=pd, PPF=pd * math.pi * (_NN_PER_A[structure] * a / 2) ** 2))(
         _N_ATOMS[structure] / a ** 3 * a / math.sqrt(h * h + k * k + l * l))),
     random_inputs=lambda rng: {"structure": rng.choice(_BRAVAIS), **_rand_hkl(rng, -2, 2), "a": rng.uniform(0.28e-9, 0.6e-9)},
     templates=["Planar density: structure = FCC, h = 1, k = 1, l = 1, a = 0.3615 nm",
                "Planar density: structure = BCC, h = 1, k = 1, l = 0, a = 0.2866 nm"],
     examples=[{"inputs": {"structure": "FCC", "h": 1, "k": 0, "l": 0, "a": 0.4e-9}, "outputs": {"PD_per_nm2": 2 / 0.16, "PPF": math.pi / 4}},
               {"inputs": {"structure": "FCC", "h": 1, "k": 1, "l": 1, "a": 0.4e-9}, "outputs": {"PD_per_nm2": 4 / (SQ3 * 0.16), "PPF": math.pi / (2 * SQ3)}},
               {"inputs": {"structure": "BCC", "h": 1, "k": 1, "l": 0, "a": 0.3e-9}, "outputs": {"PD_per_nm2": SQ2 / 0.09, "PPF": 3 * math.pi / (8 * SQ2)}}],
     questions=[{"q": "Planar density: structure = FCC, h = 1, k = 1, l = 1, a = 0.4 nm", "outputs": {"PD_per_nm2": 4 / (SQ3 * 0.16)}},
                {"q": "Planar density: structure = BCC, h = 1, k = 1, l = 0, a = 0.3 nm", "outputs": {"PD_per_nm2": SQ2 / 0.09}},
                {"q": "Planar density: structure = FCC, h = 1, k = 1, l = 1", "outcome": "NEEDS_INPUT"}])


# ---------------------------------------------------------------------------------------- Miller indices from intercepts
def _intercept(text, name):
    t = str(text).strip().lower().replace(" ", "")
    if t in ("inf", "infinity", "∞", "+inf", "-inf", "parallel"):
        return None
    try:
        v = Fraction(t)
    except (ValueError, ZeroDivisionError):
        raise ValueError(f"{name}: write a number, a fraction such as 1/2, or inf") from None
    if v == 0:
        raise ValueError("an intercept of 0 means the plane passes through the origin: move the origin first")
    return v


def miller_from_intercepts(x, y, z):
    ints = [_intercept(x, "x"), _intercept(y, "y"), _intercept(z, "z")]
    if all(v is None for v in ints):
        raise ValueError("at least one intercept must be finite")
    rec = [Fraction(0) if v is None else 1 / v for v in ints]
    den = 1
    for r_ in rec:
        den = den * r_.denominator // math.gcd(den, r_.denominator)
    hkl = [int(r_ * den) for r_ in rec]
    g = _gcd3(*hkl)
    h, k, l = (x_ // g for x_ in hkl)
    return {"h": h, "k": k, "l": l, "indices": f"({h} {k} {l})"}


def verify_intercepts(inputs, outputs):
    try:
        ints = [_intercept(inputs[s], s) for s in ("x", "y", "z")]
        h, k, l = (outputs[s] for s in ("h", "k", "l"))
        if any(not float(v).is_integer() for v in (h, k, l)) or _gcd3(int(h), int(k), int(l)) != 1:
            return math.inf
        hk = [int(h), int(k), int(l)]
        vals = [hk[i] * ints[i] for i in range(3) if ints[i] is not None]       # h x = k y = l z (= the plane's constant)
        if any(hk[i] != 0 for i in range(3) if ints[i] is None) or len(set(vals)) != 1 or vals[0] <= 0:
            return math.inf
        return 0.0 if outputs.get("indices") == f"({hk[0]} {hk[1]} {hk[2]})" else math.inf
    except Exception:
        return math.inf


def ref_intercepts(x, y, z):
    import numpy as np
    ints = [_intercept(v, s) for v, s in ((x, "x"), (y, "y"), (z, "z"))]
    E = np.eye(3)
    fin = [float(v) * E[i] for i, v in enumerate(ints) if v is not None]
    dirs = [E[i] for i, v in enumerate(ints) if v is None]
    vecs = [p - fin[0] for p in fin[1:]] + dirs
    n = np.cross(vecs[0], vecs[1])
    if n @ fin[0] < 0:
        n = -n
    nz = np.min(np.abs(n[np.abs(n) > 1e-12]))
    fr = [Fraction(float(c / nz)).limit_denominator(1000) for c in n]
    den = 1
    for f in fr:
        den = den * f.denominator // math.gcd(den, f.denominator)
    hkl = [int(f * den) for f in fr]
    g = _gcd3(*hkl)
    return {"h": hkl[0] // g, "k": hkl[1] // g, "l": hkl[2] // g}


_ICH = ["1", "2", "3", "-1", "-2", "1/2", "-1/2", "1/3", "2/3", "-2/3", "3/2", "inf"]


def _rand_intercepts(rng):
    while True:
        x = {s: rng.choice(_ICH) for s in ("x", "y", "z")}
        if not all(v == "inf" for v in x.values()):
            return x


_add("miller_indices_from_intercepts",
     name="Miller indices of a plane from its intercepts", topic="crystal structures / Miller indices",
     purpose="The Miller indices (h k l) of a plane cutting the axes at x, y, z (in units of the cell edges; inf for a plane parallel to an axis).",
     inputs=[_text("x", "intercept on the a axis (number, fraction or inf)"), _text("y", "intercept on the b axis"),
             _text("z", "intercept on the c axis")],
     outputs=[{"symbol": "h", "kind": "number", "meaning": "Miller index h"}, {"symbol": "k", "kind": "number", "meaning": "Miller index k"},
              {"symbol": "l", "kind": "number", "meaning": "Miller index l"}, {"symbol": "indices", "kind": "text", "meaning": "(h k l)"}],
     primary="h", conditions=["no intercept 0 (move the origin)", "at least one finite intercept"],
     method="take reciprocals of the intercepts, clear fractions, reduce to the smallest integers (signs kept)",
     latex=r"(hkl) \propto \left(\tfrac1x, \tfrac1y, \tfrac1z\right)",
     run=miller_from_intercepts, verify=verify_intercepts,
     parse=lambda q: _pairs(q, r"miller indices from intercepts", {"x": "x", "y": "y", "z": "z"}, texts={"x", "y", "z"}),
     reference=ref_intercepts,
     wrong=_safe(lambda x, y, z: (lambda iv: {"h": iv[0], "k": iv[1], "l": iv[2], "indices": "(%d %d %d)" % tuple(iv)})(
         [0 if v is None else (int(v) if v.denominator == 1 else v.denominator) for v in
          (_intercept(x, "x"), _intercept(y, "y"), _intercept(z, "z"))])),
     random_inputs=_rand_intercepts,
     templates=["Miller indices from intercepts: x = 1, y = 1/2, z = inf", "Miller indices from intercepts: x = 1/2, y = 1, z = 2/3"],
     examples=[{"inputs": {"x": "1", "y": "1/2", "z": "inf"}, "outputs": {"h": 1, "k": 2, "l": 0}},
               {"inputs": {"x": "1/2", "y": "1", "z": "2/3"}, "outputs": {"h": 4, "k": 2, "l": 3}},
               {"inputs": {"x": "-1", "y": "inf", "z": "1/2"}, "outputs": {"h": -1, "k": 0, "l": 2}}],
     questions=[{"q": "Miller indices from intercepts: x = 1, y = 1/2, z = inf", "outputs": {"h": 1, "k": 2, "l": 0}},
                {"q": "Miller indices from intercepts: x = 0, y = 1, z = 1", "outcome": "INVALID_INPUT"},
                {"q": "Miller indices from intercepts: x = 1, y = 1", "outcome": "NEEDS_INPUT"}])


# ---------------------------------------------------------------------------------------- Miller-Bravais directions
def _reduce_ints(vals):
    g = 0
    for v in vals:
        g = math.gcd(g, abs(v))
    return [v // g for v in vals]


def three_to_four(U, V, W):
    U, V, W = _ints("the direction indices", U, V, W)
    u, v, t, w = _reduce_ints([2 * U - V, 2 * V - U, -(U + V), 3 * W])
    return {"u": u, "v": v, "t": t, "w": w, "indices": f"[{u} {v} {t} {w}]"}


def four_to_three(u, v, t, w):
    u, v, t, w = _ints("the direction indices", u, v, t, w)
    if t != -(u + v):
        raise ValueError("in [u v t w] the index t must equal -(u + v)")
    U, V, W = _reduce_ints([u - t, v - t, w])
    return {"U": U, "V": V, "W": W, "indices": f"[{U} {V} {W}]"}


def _hex_axes():
    import numpy as np
    from pymatgen.core import Lattice
    m = Lattice.hexagonal(1.0, 1.633).matrix
    a1, a2, c = (np.array(r) for r in m)
    return a1, a2, -(a1 + a2), c


def _rationalize(vec):
    nz = min(abs(x) for x in vec if abs(x) > 1e-9)
    fr = [Fraction(float(x / nz)).limit_denominator(1000) for x in vec]
    den = 1
    for f in fr:
        den = den * f.denominator // math.gcd(den, f.denominator)
    return _reduce_ints([int(f * den) for f in fr])


def ref_three_to_four(U, V, W):
    import numpy as np
    a1, a2, a3, c = _hex_axes()
    r = U * a1 + V * a2 + W * c
    sol = np.linalg.solve(np.array([a1 - a3, a2 - a3, c]).T, r)       # r = u a1 + v a2 + t a3 + w c with t = -(u + v)
    u, v, w = sol
    out = _rationalize([u, v, -(u + v), w])
    return dict(zip("uvtw", out))


def ref_four_to_three(u, v, t, w):
    import numpy as np
    a1, a2, a3, c = _hex_axes()
    r = u * a1 + v * a2 + t * a3 + w * c
    sol = np.linalg.solve(np.array([a1, a2, c]).T, r)
    return dict(zip("UVW", _rationalize(list(sol))))


def _check_uvtw(inputs, outputs, three):
    try:
        if three:
            want = three_to_four(**inputs)
        else:
            want = four_to_three(**inputs)
    except Exception:
        return math.inf
    return 0.0 if all(outputs.get(k) == v for k, v in want.items()) else math.inf


def _rand_UVW(rng):
    while True:
        x = {s: rng.randint(-3, 3) for s in "UVW"}
        if any(x.values()):
            return x


_add("miller_bravais_direction",
     name="Three-index to four-index (Miller-Bravais) direction", topic="crystal structures / hexagonal indices",
     purpose="Convert a hexagonal direction [U V W] (axes a1, a2, c) to the Miller-Bravais form [u v t w] (axes a1, a2, a3, c).",
     inputs=[_intin("U", "index along a1", -6, 6), _intin("V", "index along a2", -6, 6), _intin("W", "index along c", -6, 6)],
     outputs=[{"symbol": s, "kind": "number", "meaning": f"Miller-Bravais index {s}"} for s in "uvtw"]
     + [{"symbol": "indices", "kind": "text", "meaning": "[u v t w]"}],
     primary="u", conditions=["integer indices, not all zero"],
     method="u = (2U - V)/3, v = (2V - U)/3, t = -(u + v), w = W, reduced to the smallest integers",
     latex=r"u = \tfrac13(2U-V),\ v = \tfrac13(2V-U),\ t = -(u+v),\ w = W",
     run=three_to_four, verify=lambda i, o: _check_uvtw(i, o, True),
     parse=lambda q: _pairs(q, r"miller-bravais direction|three to four index direction", {"U": "U", "V": "V", "W": "W"}),
     reference=ref_three_to_four,
     wrong=_safe(lambda U, V, W: (lambda r: {"u": r[0], "v": r[1], "t": r[2], "w": r[3], "indices": ""})(
         _reduce_ints([U, V, -(U + V), W]))),
     random_inputs=_rand_UVW,
     templates=["Miller-Bravais direction: U = 1, V = 0, W = 0", "Miller-Bravais direction: U = 1, V = 1, W = 1"],
     examples=[{"inputs": {"U": 1, "V": 0, "W": 0}, "outputs": {"u": 2, "v": -1, "t": -1, "w": 0}},
               {"inputs": {"U": 0, "V": 0, "W": 1}, "outputs": {"u": 0, "v": 0, "t": 0, "w": 1}},
               {"inputs": {"U": 1, "V": 1, "W": 0}, "outputs": {"u": 1, "v": 1, "t": -2, "w": 0}}],
     questions=[{"q": "Miller-Bravais direction: U = 1, V = 0, W = 0", "outputs": {"u": 2, "v": -1, "t": -1, "w": 0}},
                {"q": "Miller-Bravais direction: U = 0, V = 0, W = 0", "outcome": "INVALID_INPUT"}])


def _rand_uvtw(rng):
    while True:
        u, v, w = rng.randint(-3, 3), rng.randint(-3, 3), rng.randint(-3, 3)
        if (u, v, w) != (0, 0, 0):
            return {"u": u, "v": v, "t": -(u + v), "w": w}


_add("miller_bravais_to_three_index",
     name="Four-index (Miller-Bravais) to three-index direction", topic="crystal structures / hexagonal indices",
     purpose="Convert a Miller-Bravais direction [u v t w] of a hexagonal crystal to the three-index form [U V W].",
     inputs=[_intin("u", "index along a1", -6, 6), _intin("v", "index along a2", -6, 6), _intin("t", "index along a3 (= -(u + v))", -12, 12),
             _intin("w", "index along c", -6, 6)],
     outputs=[{"symbol": s, "kind": "number", "meaning": f"three-index component {s}"} for s in "UVW"]
     + [{"symbol": "indices", "kind": "text", "meaning": "[U V W]"}],
     primary="U", conditions=["integer indices", "t = -(u + v)"],
     method="U = u - t, V = v - t, W = w, reduced to the smallest integers", latex=r"U = u - t,\ V = v - t,\ W = w",
     run=four_to_three, verify=lambda i, o: _check_uvtw(i, o, False),
     parse=lambda q: _pairs(q, r"four to three index direction|three-index direction from miller-bravais", {"u": "u", "v": "v", "t": "t", "w": "w"}),
     reference=ref_four_to_three,
     wrong=_safe(lambda u, v, t, w: (lambda r: {"U": r[0], "V": r[1], "W": r[2], "indices": ""})(_reduce_ints([u, v, w]))),
     random_inputs=_rand_uvtw,
     templates=["Four to three index direction: u = 2, v = -1, t = -1, w = 0"],
     examples=[{"inputs": {"u": 2, "v": -1, "t": -1, "w": 0}, "outputs": {"U": 1, "V": 0, "W": 0}},
               {"inputs": {"u": 1, "v": 1, "t": -2, "w": 3}, "outputs": {"U": 1, "V": 1, "W": 1}}],
     questions=[{"q": "Four to three index direction: u = 2, v = -1, t = -1, w = 0", "outputs": {"U": 1, "V": 0, "W": 0}},
                {"q": "Four to three index direction: u = 1, v = 1, t = 1, w = 0", "outcome": "INVALID_INPUT"}])


# ---------------------------------------------------------------------------------------- angles between directions and planes
def _angle_from(G, p, q):
    pp, qq, pq = _quad(G, p, p), _quad(G, q, q), _quad(G, p, q)
    cross2 = pp * qq - pq * pq
    if cross2 <= 1e-12 * pp * qq:
        return 0.0 if pq > 0 else 180.0
    return math.degrees(math.atan2(math.sqrt(cross2), pq))


def _ref_angle(v1, v2):
    import numpy as np
    ang = float(np.degrees(np.arctan2(np.linalg.norm(np.cross(v1, v2)), np.dot(v1, v2))))
    if ang < 1e-4:
        return 0.0
    if ang > 180 - 1e-4:
        return 180.0
    return ang


def angle_directions_cubic(u1, v1, w1, u2, v2, w2):
    p = _ints("the first direction", u1, v1, w1)
    q = _ints("the second direction", u2, v2, w2)
    I = [[1, 0, 0], [0, 1, 0], [0, 0, 1]]
    return _out(angle_deg=_angle_from(I, p, q))


_DIR2 = {"u1": "u1", "v1": "v1", "w1": "w1", "u2": "u2", "v2": "v2", "w2": "w2"}
_DIR2_IN = [_intin(s, f"index {s[0]} of direction {s[1]}", -6, 6) for s in ("u1", "v1", "w1", "u2", "v2", "w2")]


def _rand_two(rng, names):
    while True:
        x = {s: rng.randint(-3, 3) for s in names}
        a, b = [x[n] for n in names[:3]], [x[n] for n in names[3:]]
        cr = [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]
        if any(a) and any(b) and any(cr):
            return x


_add("angle_between_directions_cubic",
     name="Angle between two directions in a cubic crystal", topic="crystal structures / crystal geometry",
     purpose="The angle between the directions [u1 v1 w1] and [u2 v2 w2] of a cubic crystal (also the angle between the planes with those indices).",
     inputs=_DIR2_IN, outputs=[_o("angle_deg", "number", "angle in degrees")],
     primary="angle_deg", conditions=["integer indices, neither direction zero"],
     method="cos(angle) = (u1 u2 + v1 v2 + w1 w2) / (|d1| |d2|), evaluated as atan2(|d1 x d2|, d1 . d2)",
     latex=r"\cos\varphi = \frac{u_1u_2+v_1v_2+w_1w_2}{\sqrt{u_1^2+v_1^2+w_1^2}\sqrt{u_2^2+v_2^2+w_2^2}}",
     run=angle_directions_cubic,
     parse=lambda q: _pairs(q, r"angle between cubic directions", _DIR2),
     reference=lambda u1, v1, w1, u2, v2, w2: {"angle_deg": _ref_angle(
         __import__("pymatgen.core", fromlist=["Lattice"]).Lattice.cubic(1.0).get_cartesian_coords([u1, v1, w1]),
         __import__("pymatgen.core", fromlist=["Lattice"]).Lattice.cubic(1.0).get_cartesian_coords([u2, v2, w2]))},
     wrong=_safe(lambda u1, v1, w1, u2, v2, w2: _out(angle_deg=math.degrees(math.acos(max(-1.0, min(1.0, (u1 * u2 + v1 * v2 + w1 * w2) / (
         (u1 * u1 + v1 * v1 + w1 * w1) * (u2 * u2 + v2 * v2 + w2 * w2)))))))),
     random_inputs=lambda rng: _rand_two(rng, ["u1", "v1", "w1", "u2", "v2", "w2"]),
     templates=["Angle between cubic directions: u1 = 1, v1 = 0, w1 = 0, u2 = 1, v2 = 1, w2 = 1"],
     examples=[{"inputs": {"u1": 1, "v1": 0, "w1": 0, "u2": 1, "v2": 1, "w2": 1}, "outputs": {"angle_deg": math.degrees(math.acos(1 / SQ3))}},
               {"inputs": {"u1": 1, "v1": 1, "w1": 0, "u2": 1, "v2": -1, "w2": 0}, "outputs": {"angle_deg": 90.0}},
               {"inputs": {"u1": 1, "v1": 1, "w1": 1, "u2": -1, "v2": -1, "w2": -1}, "outputs": {"angle_deg": 180.0}}],
     questions=[{"q": "Angle between cubic directions: u1 = 1, v1 = 0, w1 = 0, u2 = 1, v2 = 1, w2 = 1",
                 "outputs": {"angle_deg": math.degrees(math.acos(1 / SQ3))}},
                {"q": "Angle between cubic directions: u1 = 0, v1 = 0, w1 = 0, u2 = 1, v2 = 1, w2 = 1", "outcome": "INVALID_INPUT"}])


def angle_directions_general(a, b, c, alpha, beta, gamma, u1, v1, w1, u2, v2, w2):
    _cell_check(a, b, c, alpha, beta, gamma)
    p = _ints("the first direction", u1, v1, w1)
    q = _ints("the second direction", u2, v2, w2)
    G = _metric(a, b, c, alpha, beta, gamma)
    return _out(angle_deg=_angle_from(G, p, q))


def angle_planes_general(a, b, c, alpha, beta, gamma, h1, k1, l1, h2, k2, l2):
    _cell_check(a, b, c, alpha, beta, gamma)
    p = _ints("the first plane", h1, k1, l1)
    q = _ints("the second plane", h2, k2, l2)
    Gs = _inv3(_metric(a, b, c, alpha, beta, gamma))
    return _out(angle_deg=_angle_from(Gs, p, q))


def ref_angle_directions_general(a, b, c, alpha, beta, gamma, u1, v1, w1, u2, v2, w2):
    from pymatgen.core import Lattice
    L = Lattice.from_parameters(a * 1e9, b * 1e9, c * 1e9, alpha, beta, gamma)
    return {"angle_deg": _ref_angle(L.get_cartesian_coords([u1, v1, w1]), L.get_cartesian_coords([u2, v2, w2]))}


def ref_angle_planes_general(a, b, c, alpha, beta, gamma, h1, k1, l1, h2, k2, l2):
    from pymatgen.core import Lattice
    R = Lattice.from_parameters(a * 1e9, b * 1e9, c * 1e9, alpha, beta, gamma).reciprocal_lattice_crystallographic
    return {"angle_deg": _ref_angle(R.get_cartesian_coords([h1, k1, l1]), R.get_cartesian_coords([h2, k2, l2]))}


_PL2 = {"h1": "h1", "k1": "k1", "l1": "l1", "h2": "h2", "k2": "k2", "l2": "l2"}
_PL2_IN = [_intin(s, f"Miller index {s[0]} of plane {s[1]}", -6, 6) for s in ("h1", "k1", "l1", "h2", "k2", "l2")]

_add("angle_between_directions",
     name="Angle between two lattice directions in any crystal", topic="crystal structures / crystal geometry",
     purpose="The angle between the directions [u1 v1 w1] and [u2 v2 w2] of a crystal with any cell (a, b, c, alpha, beta, gamma).",
     inputs=_cell_in() + _DIR2_IN, outputs=[_o("angle_deg", "number", "angle in degrees")],
     primary="angle_deg", conditions=["a valid cell", "neither direction zero"],
     method="the metric tensor G: cos(angle) = d1 G d2 / sqrt((d1 G d1)(d2 G d2))",
     latex=r"\cos\varphi = \frac{\mathbf d_1^{\mathsf T}G\,\mathbf d_2}{\sqrt{\mathbf d_1^{\mathsf T}G\,\mathbf d_1}\sqrt{\mathbf d_2^{\mathsf T}G\,\mathbf d_2}}",
     run=angle_directions_general,
     parse=lambda q: _pairs(q, r"angle between lattice directions", {**_CELL_NAMES, **_DIR2}, phys={"a", "b", "c"}, num_units=_DEG),
     reference=ref_angle_directions_general,
     wrong=_safe(lambda a, b, c, alpha, beta, gamma, u1, v1, w1, u2, v2, w2: _out(angle_deg=_angle_from([[1, 0, 0], [0, 1, 0], [0, 0, 1]],
                                                                                                       [u1, v1, w1], [u2, v2, w2]))),
     random_inputs=lambda rng: (lambda cell: {**cell, **_rand_two(rng, ["u1", "v1", "w1", "u2", "v2", "w2"])})(
         dict(_rand_cell(rng), alpha=rng.uniform(65, 80), gamma=rng.uniform(100, 118))),
     templates=["Angle between lattice directions: a = 0.3 nm, b = 0.3 nm, c = 0.5 nm, alpha = 90, beta = 90, gamma = 120, u1 = 1, v1 = 0, w1 = 0, u2 = 0, v2 = 0, w2 = 1"],
     examples=[{"inputs": {"a": 0.3e-9, "b": 0.3e-9, "c": 0.5e-9, "alpha": 90, "beta": 90, "gamma": 120,
                           "u1": 1, "v1": 0, "w1": 0, "u2": 0, "v2": 1, "w2": 0}, "outputs": {"angle_deg": 120.0}},
               {"inputs": {"a": 0.3e-9, "b": 0.4e-9, "c": 0.5e-9, "alpha": 90, "beta": 90, "gamma": 90,
                           "u1": 1, "v1": 1, "w1": 0, "u2": 1, "v2": 0, "w2": 0}, "outputs": {"angle_deg": math.degrees(math.atan(4 / 3))}}],
     questions=[{"q": "Angle between lattice directions: a = 0.3 nm, b = 0.3 nm, c = 0.5 nm, alpha = 90, beta = 90, gamma = 120, u1 = 1, v1 = 0, w1 = 0, u2 = 0, v2 = 1, w2 = 0",
                 "outputs": {"angle_deg": 120.0}}])

_add("angle_between_planes",
     name="Angle between two lattice planes in any crystal", topic="crystal structures / crystal geometry",
     purpose="The angle between the normals of the planes (h1 k1 l1) and (h2 k2 l2) of a crystal with any cell.",
     inputs=_cell_in() + _PL2_IN, outputs=[_o("angle_deg", "number", "angle between the plane normals in degrees")],
     primary="angle_deg", conditions=["a valid cell", "neither plane (0 0 0)"],
     method="the reciprocal metric tensor G* = G^-1: cos(angle) = h1 G* h2 / sqrt((h1 G* h1)(h2 G* h2))",
     latex=r"\cos\varphi = \frac{\mathbf h_1^{\mathsf T}G^{-1}\mathbf h_2}{\sqrt{\mathbf h_1^{\mathsf T}G^{-1}\mathbf h_1}\sqrt{\mathbf h_2^{\mathsf T}G^{-1}\mathbf h_2}}",
     run=angle_planes_general,
     parse=lambda q: _pairs(q, r"angle between lattice planes|angle between planes", {**_CELL_NAMES, **_PL2}, phys={"a", "b", "c"}, num_units=_DEG),
     reference=ref_angle_planes_general,
     wrong=_safe(lambda a, b, c, alpha, beta, gamma, h1, k1, l1, h2, k2, l2: _out(angle_deg=_angle_from(
         _metric(a, b, c, alpha, beta, gamma), [h1, k1, l1], [h2, k2, l2]))),
     random_inputs=lambda rng: (lambda cell: {**cell, **_rand_two(rng, ["h1", "k1", "l1", "h2", "k2", "l2"])})(
         dict(_rand_cell(rng), alpha=rng.uniform(65, 80), gamma=rng.uniform(100, 118))),
     templates=["Angle between planes: a = 0.32 nm, b = 0.32 nm, c = 0.52 nm, alpha = 90, beta = 90, gamma = 120, h1 = 1, k1 = 0, l1 = 1, h2 = 0, k2 = 0, l2 = 1"],
     examples=[{"inputs": {"a": 0.4e-9, "b": 0.4e-9, "c": 0.4e-9, "alpha": 90, "beta": 90, "gamma": 90,
                           "h1": 1, "k1": 1, "l1": 1, "h2": 1, "k2": 1, "l2": -1}, "outputs": {"angle_deg": math.degrees(math.acos(1 / 3))}},
               {"inputs": {"a": 0.3e-9, "b": 0.3e-9, "c": 0.5e-9, "alpha": 90, "beta": 90, "gamma": 120,
                           "h1": 1, "k1": 0, "l1": 0, "h2": 0, "k2": 1, "l2": 0}, "outputs": {"angle_deg": 60.0}}],
     questions=[{"q": "Angle between planes: a = 0.4 nm, b = 0.4 nm, c = 0.4 nm, alpha = 90, beta = 90, gamma = 90, h1 = 1, k1 = 1, l1 = 1, h2 = 1, k2 = 1, l2 = -1",
                 "outputs": {"angle_deg": math.degrees(math.acos(1 / 3))}}])


# ---------------------------------------------------------------------------------------- interstitial sites
_SITE_RATIO = {("FCC", "octahedral"): SQ2 - 1, ("FCC", "tetrahedral"): math.sqrt(1.5) - 1,
               ("BCC", "octahedral"): 2 / SQ3 - 1, ("BCC", "tetrahedral"): math.sqrt(5.0 / 3.0) - 1}
_SITE_POS = {("FCC", "octahedral"): (0.5, 0.0, 0.0), ("FCC", "tetrahedral"): (0.25, 0.25, 0.25),
             ("BCC", "octahedral"): (0.5, 0.5, 0.0), ("BCC", "tetrahedral"): (0.5, 0.25, 0.0)}


def interstitial_site(structure, site, R):
    _pos("the atomic radius", R)
    q = _SITE_RATIO[(structure, site)]
    return _out(r=q * R, r_nm=q * R * 1e9, ratio=q)


def ref_interstitial_site(structure, site, R):
    import numpy as np
    d1, _ = _ase_nn(_ase_cell(structure, 1.0), 1.0)
    a = 2 * R / d1
    pos = _ase_cell(structure, 1.0).get_scaled_positions()
    pts = np.array([p + np.array(t) for p in pos for t in _iproduct((-1, 0, 1), repeat=3)])
    dist = np.linalg.norm(pts - np.array(_SITE_POS[(structure, site)]), axis=1).min() * a
    r = dist - R
    return {"r_nm": r * 1e9, "ratio": r / R}


_add("interstitial_site_size",
     name="Largest atom that fits an interstitial site", topic="defects / interstitials",
     purpose="The radius of the largest sphere that fits the octahedral or tetrahedral interstitial site of an FCC or BCC lattice of host atoms of radius R.",
     inputs=[_struct_in(["FCC", "BCC"]), _choice("site", ["octahedral", "tetrahedral"], "interstitial site (octahedral, tetrahedral)"),
             _phys("R", "length", "radius of the host atoms")],
     outputs=[_o("r", "length", "radius of the interstitial sphere"), _o("r_nm", "number", "interstitial radius in nm"),
              _o("ratio", "number", "r/R")],
     primary="ratio", conditions=["R > 0"],
     method="hard spheres: distance from the site centre to the nearest host atom minus R (FCC oct sqrt2-1, tet sqrt(3/2)-1; BCC oct 2/sqrt3-1, tet sqrt(5/3)-1)",
     latex=r"r = d_{\mathrm{site}\to\mathrm{atom}} - R",
     run=interstitial_site,
     parse=lambda q: _pairs(q, r"interstitial site size|interstitial site", {"structure": "structure", "site": "site", "R": "R"},
                            phys={"R"}, words={"structure", "site"}),
     reference=ref_interstitial_site,
     wrong=_safe(lambda structure, site, R: (lambda q: _out(r=q * R, r_nm=q * R * 1e9, ratio=q))(
         {("FCC", "octahedral"): math.sqrt(1.5) - 1, ("FCC", "tetrahedral"): SQ2 - 1,
          ("BCC", "octahedral"): math.sqrt(5.0 / 3.0) - 1, ("BCC", "tetrahedral"): 2 / SQ3 - 1}[(structure, site)])),
     random_inputs=lambda rng: {"structure": rng.choice(["FCC", "BCC"]), "site": rng.choice(["octahedral", "tetrahedral"]),
                                "R": rng.uniform(0.1e-9, 0.2e-9)},
     templates=["Interstitial site size: structure = FCC, site = octahedral, R = 0.129 nm"],
     examples=[{"inputs": {"structure": "FCC", "site": "octahedral", "R": 0.129e-9}, "outputs": {"ratio": SQ2 - 1, "r_nm": (SQ2 - 1) * 0.129}},
               {"inputs": {"structure": "BCC", "site": "tetrahedral", "R": 0.124e-9}, "outputs": {"ratio": math.sqrt(5 / 3) - 1}}],
     questions=[{"q": "Interstitial site size: structure = FCC, site = octahedral, R = 0.129 nm", "outputs": {"ratio": SQ2 - 1}},
                {"q": "Interstitial site size: structure = BCC, site = cubic, R = 0.129 nm", "outcome": "INVALID_INPUT"}])


# ---------------------------------------------------------------------------------------- radius ratio
_CRIT = [(2 / SQ3 - 1, 3, "triangular"), (math.sqrt(1.5) - 1, 4, "tetrahedral"), (SQ2 - 1, 6, "octahedral"), (SQ3 - 1, 8, "cubic")]


def radius_ratio(rc, ra):
    _pos("the ionic radii", rc, ra)
    q = rc / ra
    if q >= 1:
        raise ValueError("the cation must be smaller than the anion (rc/ra < 1) for the radius-ratio rules")
    cn, geo = 2, "linear"
    for lim, n, g in _CRIT:
        if q >= lim:
            cn, geo = n, g
    return {"ratio": _r(q), "CN": cn, "geometry": geo}


def ref_radius_ratio(rc, ra):
    """Critical ratios from the polyhedra themselves (numpy): anions of radius 1 touching along each edge, the cation
    touching every anion: r_c = (centre-to-vertex distance) - 1."""
    import numpy as np
    polys = {3: np.array([[math.cos(t), math.sin(t), 0] for t in (0, 2 * math.pi / 3, 4 * math.pi / 3)]),
             4: np.array([[1, 1, 1], [1, -1, -1], [-1, 1, -1], [-1, -1, 1]], float),
             6: np.array([[1, 0, 0], [-1, 0, 0], [0, 1, 0], [0, -1, 0], [0, 0, 1], [0, 0, -1]], float),
             8: np.array(list(_iproduct((-1, 1), repeat=3)), float)}
    q = rc / ra
    cn = 2
    for n, P in sorted(polys.items()):
        edge = min(np.linalg.norm(P[i] - P[j]) for i in range(len(P)) for j in range(i + 1, len(P)))
        crit = np.linalg.norm(P[0] - P.mean(axis=0)) / (edge / 2) - 1
        if q >= crit:
            cn = n
    return {"ratio": q, "CN": cn}


_add("radius_ratio_coordination",
     name="Coordination number from the cation-anion radius ratio", topic="ceramics / ionic structures",
     purpose="The expected coordination number and polyhedron of a cation from the ratio of the cation and anion radii (Pauling's radius-ratio rule).",
     inputs=[_phys("rc", "length", "cation radius"), _phys("ra", "length", "anion radius")],
     outputs=[_o("ratio", "number", "rc/ra"), _o("CN", "number", "coordination number"), {"symbol": "geometry", "kind": "text", "meaning": "coordination polyhedron"}],
     primary="CN", conditions=["0 < rc < ra"],
     method="critical ratios 0.155 (3), 0.225 (4), 0.414 (6), 0.732 (8): the largest one not above rc/ra",
     latex=r"\frac{r_c}{r_a}:\ 0.155\ (3),\ 0.225\ (4),\ 0.414\ (6),\ 0.732\ (8)",
     run=radius_ratio,
     parse=lambda q: _pairs(q, r"radius ratio|coordination from radius ratio", {"rc": "rc", "ra": "ra"}, phys={"rc", "ra"}),
     reference=ref_radius_ratio,
     wrong=_safe(lambda rc, ra: radius_ratio(rc * ra / (rc + ra), ra)),
     random_inputs=lambda rng: {"rc": rng.uniform(0.02e-9, 0.17e-9), "ra": rng.uniform(0.17e-9, 0.22e-9)},
     templates=["Radius ratio: rc = 0.102 nm, ra = 0.181 nm", "Radius ratio: rc = 0.072 nm, ra = 0.140 nm"],
     examples=[{"inputs": {"rc": 0.102e-9, "ra": 0.181e-9}, "outputs": {"CN": 6}},
               {"inputs": {"rc": 0.170e-9, "ra": 0.181e-9}, "outputs": {"CN": 8}},
               {"inputs": {"rc": 0.040e-9, "ra": 0.140e-9}, "outputs": {"CN": 4}}],
     questions=[{"q": "Radius ratio: rc = 0.102 nm, ra = 0.181 nm", "outputs": {"CN": 6, "geometry": "octahedral"}},
                {"q": "Radius ratio: rc = 0.2 nm, ra = 0.1 nm", "outcome": "INVALID_INPUT"}])


# ---------------------------------------------------------------------------------------- ceramic crystal densities
_CERAMIC = {"rocksalt": (4, 1, 2.0), "CsCl": (1, 1, 2 / SQ3), "zincblende": (4, 1, 4 / SQ3), "fluorite": (4, 2, 4 / SQ3)}
# formula units per cell, anions per formula unit, a / (rc + ra)


def ceramic_density(prototype, rc, ra, Ac, Aa):
    _pos("the ionic radii", rc, ra)
    _pos("the molar masses", Ac, Aa)
    n, k, f = _CERAMIC[prototype]
    a = f * (rc + ra)
    rho = n * (Ac + k * Aa) / (a ** 3 * NA)
    return _out(a=a, a_nm=a * 1e9, rho=rho, rho_gcc=rho / 1000)


def _ase_ceramic(prototype, a):
    from ase.build import bulk
    name, kind = {"rocksalt": ("NaCl", "rocksalt"), "CsCl": ("CsCl", "cesiumchloride"), "zincblende": ("ZnS", "zincblende"),
                  "fluorite": ("CaF2", "fluorite")}[prototype]
    return bulk(name, kind, a=a, cubic=True) if prototype != "CsCl" else bulk(name, kind, a=a)


def ref_ceramic_density(prototype, rc, ra, Ac, Aa):
    import numpy as np
    from ase.neighborlist import neighbor_list
    from scipy.constants import Avogadro
    at = _ase_ceramic(prototype, 1.0)
    cation = at.get_chemical_symbols()[0]
    i, j, d = neighbor_list("ijd", at, 1.0)
    sym = np.array(at.get_chemical_symbols())
    dca = float(d[sym[i] != sym[j]].min())                      # nearest cation-anion distance at a = 1
    a = (rc + ra) / dca
    at = _ase_ceramic(prototype, a * 1e10)
    at.set_masses([(Ac if s == cation else Aa) * 1000 for s in at.get_chemical_symbols()])
    rho = float(sum(at.get_masses())) * 1e-3 / Avogadro / (at.get_volume() * 1e-30)
    return {"a_nm": a * 1e9, "rho_gcc": rho / 1000}


_add("ceramic_density",
     name="Lattice parameter and density of an ionic crystal", topic="ceramics / crystal structures",
     purpose="The cubic lattice parameter and theoretical density of a rock-salt, cesium-chloride, zinc-blende or fluorite crystal from its ionic radii and molar masses.",
     inputs=[_choice("prototype", list(_CERAMIC), "crystal structure (rocksalt, CsCl, zincblende, fluorite)"),
             _phys("rc", "length", "cation radius"), _phys("ra", "length", "anion radius"),
             _phys("Ac", "molar_mass", "molar mass of the cation element"), _phys("Aa", "molar_mass", "molar mass of the anion element")],
     outputs=[_o("a", "length", "lattice parameter"), _o("a_nm", "number", "lattice parameter in nm"), _o("rho", "density", "density"),
              _o("rho_gcc", "number", "density in g/cm^3")],
     primary="rho_gcc", conditions=["radii and molar masses > 0", "cations and anions touch (the anions need not)"],
     method="cation-anion contact: rock salt a = 2(rc + ra), CsCl a = 2(rc + ra)/sqrt3, zinc blende and fluorite a = 4(rc + ra)/sqrt3; rho = n'(Ac + k Aa)/(a^3 N_A)",
     latex=r"\rho = \frac{n'\left(\sum A_C + \sum A_A\right)}{V_C N_A}",
     run=ceramic_density,
     parse=lambda q: _pairs(q, r"ceramic density|ionic crystal density", {"prototype": "prototype", "structure": "prototype", "rc": "rc",
                                                                         "ra": "ra", "Ac": "Ac", "Aa": "Aa"},
                            phys={"rc", "ra", "Ac", "Aa"}, words={"prototype"}),
     reference=ref_ceramic_density,
     wrong=_safe(lambda prototype, rc, ra, Ac, Aa: (lambda a: _out(a=a, a_nm=a * 1e9, rho=_CERAMIC[prototype][0] * (Ac + Aa) / (a ** 3 * NA),
                                                                     rho_gcc=_CERAMIC[prototype][0] * (Ac + Aa) / (a ** 3 * NA) / 1000))(
         2 * (rc + ra))),
     random_inputs=lambda rng: {"prototype": rng.choice(list(_CERAMIC)), "rc": rng.uniform(0.05e-9, 0.17e-9), "ra": rng.uniform(0.12e-9, 0.22e-9),
                                "Ac": rng.uniform(0.006, 0.14), "Aa": rng.uniform(0.016, 0.13)},
     templates=["Ceramic density: prototype = rocksalt, rc = 0.102 nm, ra = 0.181 nm, Ac = 22.99 g/mol, Aa = 35.45 g/mol"],
     examples=[{"inputs": {"prototype": "rocksalt", "rc": 0.102e-9, "ra": 0.181e-9, "Ac": 0.02299, "Aa": 0.03545},
                "outputs": {"a_nm": 0.566, "rho_gcc": 4 * (22.99 + 35.45) / ((0.566e-7) ** 3 * NA)}},
               {"inputs": {"prototype": "CsCl", "rc": 0.170e-9, "ra": 0.181e-9, "Ac": 0.13291, "Aa": 0.03545},
                "outputs": {"a_nm": 2 * 0.351 / SQ3}},
               {"inputs": {"prototype": "fluorite", "rc": 0.100e-9, "ra": 0.133e-9, "Ac": 0.04008, "Aa": 0.019},
                "outputs": {"rho_gcc": 4 * (40.08 + 38.0) / ((4 * 0.233e-7 / SQ3) ** 3 * NA)}}],
     questions=[{"q": "Ceramic density: prototype = rocksalt, rc = 0.102 nm, ra = 0.181 nm, Ac = 22.99 g/mol, Aa = 35.45 g/mol",
                 "outputs": {"a_nm": 0.566}},
                {"q": "Ceramic density: prototype = perovskite, rc = 0.102 nm, ra = 0.181 nm, Ac = 22.99 g/mol, Aa = 35.45 g/mol",
                 "outcome": "INVALID_INPUT"}])


# ---------------------------------------------------------------------------------------- polymorphic volume change
def polymorph_volume_change(R_bcc, R_fcc):
    _pos("the atomic radii", R_bcc, R_fcc)
    v_b = (4 * R_bcc / SQ3) ** 3 / 2
    v_f = (2 * SQ2 * R_fcc) ** 3 / 4
    return _out(volume_change_percent=(v_f - v_b) / v_b * 100, V_atom_bcc_nm3=v_b * 1e27, V_atom_fcc_nm3=v_f * 1e27)


def ref_polymorph_volume_change(R_bcc, R_fcc):
    out = {}
    for s, R in (("BCC", R_bcc), ("FCC", R_fcc)):
        d1, _ = _ase_nn(_ase_cell(s, 1.0), 1.0)
        at = _ase_cell(s, 2 * R / d1 * 1e9)
        out[s] = at.get_volume() / len(at)
    return {"volume_change_percent": (out["FCC"] - out["BCC"]) / out["BCC"] * 100,
            "V_atom_bcc_nm3": out["BCC"], "V_atom_fcc_nm3": out["FCC"]}


_add("polymorphic_volume_change",
     name="Volume change of a BCC to FCC transformation", topic="crystal structures / polymorphism",
     purpose="The percentage volume change when a metal transforms from BCC to FCC (as iron at 912 C), from the atomic radius in each structure.",
     inputs=[_phys("R_bcc", "length", "atomic radius in the BCC form"), _phys("R_fcc", "length", "atomic radius in the FCC form")],
     outputs=[_o("volume_change_percent", "number", "volume change BCC -> FCC in percent (negative = contraction)"),
              _o("V_atom_bcc_nm3", "number", "volume per atom in BCC, nm^3"), _o("V_atom_fcc_nm3", "number", "volume per atom in FCC, nm^3")],
     primary="volume_change_percent", conditions=["radii > 0"],
     method="volume per atom: BCC (4R/sqrt3)^3/2, FCC (2 sqrt2 R)^3/4", latex=r"\frac{\Delta V}{V} = \frac{a_F^3/4 - a_B^3/2}{a_B^3/2}",
     run=polymorph_volume_change,
     parse=lambda q: _pairs(q, r"bcc to fcc volume change|polymorphic volume change", {"R_bcc": "R_bcc", "R_fcc": "R_fcc"}, phys={"R_bcc", "R_fcc"}),
     reference=ref_polymorph_volume_change,
     wrong=_safe(lambda R_bcc, R_fcc: (lambda vb, vf: _out(volume_change_percent=(vf - vb) / vb * 100, V_atom_bcc_nm3=vb * 1e27,
                                                           V_atom_fcc_nm3=vf * 1e27))((4 * R_bcc / SQ3) ** 3, (2 * SQ2 * R_fcc) ** 3)),
     random_inputs=lambda rng: (lambda R: {"R_bcc": R, "R_fcc": R * rng.uniform(1.0, 1.06)})(rng.uniform(0.11e-9, 0.2e-9)),
     templates=["BCC to FCC volume change: R_bcc = 0.1258 nm, R_fcc = 0.1292 nm"],
     examples=[{"inputs": {"R_bcc": 0.1258e-9, "R_fcc": 0.1292e-9},
                "outputs": {"volume_change_percent": ((2 * SQ2 * 0.1292) ** 3 / 4 - (4 * 0.1258 / SQ3) ** 3 / 2) / ((4 * 0.1258 / SQ3) ** 3 / 2) * 100}}],
     questions=[{"q": "BCC to FCC volume change: R_bcc = 0.1258 nm, R_fcc = 0.1292 nm",
                 "outputs": {"volume_change_percent": ((2 * SQ2 * 0.1292) ** 3 / 4 - (4 * 0.1258 / SQ3) ** 3 / 2) / ((4 * 0.1258 / SQ3) ** 3 / 2) * 100}},
                {"q": "BCC to FCC volume change: R_bcc = 0.1258 nm", "outcome": "NEEDS_INPUT"}])


# ---------------------------------------------------------------------------------------- Burgers vector
_SLIP = {"SC": "<100>", "BCC": "a/2<111>", "FCC": "a/2<110>"}


def burgers_vector(structure, a):
    _pos("the lattice parameter", a)
    b = _NN_PER_A[structure] * a
    return {"b": _r(b), "b_nm": _r(b * 1e9), "direction": _SLIP[structure]}


def ref_burgers_vector(structure, a):
    import numpy as np
    pts = _lattice_points_half(structure, 4)
    nz = pts[np.abs(pts).sum(axis=1) > 0]
    return {"b_nm": float(np.min(np.linalg.norm(nz, axis=1))) * a / 2 * 1e9}


_add("burgers_vector",
     name="Burgers vector of a perfect dislocation", topic="defects / dislocations",
     purpose="The magnitude of the shortest lattice translation (the Burgers vector of a perfect dislocation) in an SC, BCC or FCC metal.",
     inputs=[_struct_in(_BRAVAIS), _phys("a", "length", "lattice parameter")],
     outputs=[_o("b", "length", "Burgers vector magnitude"), _o("b_nm", "number", "Burgers vector in nm"),
              {"symbol": "direction", "kind": "text", "meaning": "Burgers vector family"}],
     primary="b_nm", conditions=["a > 0"],
     method="shortest lattice translation: SC a<100>, BCC (a/2)<111> = a sqrt3/2, FCC (a/2)<110> = a/sqrt2",
     latex=r"b_{\mathrm{FCC}} = \frac{a}{\sqrt2},\ b_{\mathrm{BCC}} = \frac{a\sqrt3}{2},\ b_{\mathrm{SC}} = a",
     run=burgers_vector,
     parse=lambda q: _pairs(q, r"burgers vector", {"structure": "structure", "a": "a"}, phys={"a"}, words={"structure"}),
     reference=ref_burgers_vector,
     wrong=_safe(lambda structure, a: {"b": a, "b_nm": a * 1e9, "direction": "<100>"}),
     random_inputs=lambda rng: {"structure": rng.choice(["BCC", "FCC"]), "a": rng.uniform(0.28e-9, 0.6e-9)},
     templates=["Burgers vector: structure = FCC, a = 0.3615 nm", "Burgers vector: structure = BCC, a = 0.2866 nm"],
     examples=[{"inputs": {"structure": "FCC", "a": 0.3615e-9}, "outputs": {"b_nm": 0.3615 / SQ2}},
               {"inputs": {"structure": "BCC", "a": 0.2866e-9}, "outputs": {"b_nm": 0.2866 * SQ3 / 2}}],
     questions=[{"q": "Burgers vector: structure = FCC, a = 0.3615 nm", "outputs": {"b_nm": 0.3615 / SQ2}},
                {"q": "Burgers vector: structure = FCC", "outcome": "NEEDS_INPUT"}])


# ---------------------------------------------------------------------------------------- space groups
_SG = {"SC": (221, "Pm-3m"), "BCC": (229, "Im-3m"), "FCC": (225, "Fm-3m"), "HCP": (194, "P6_3/mmc"), "diamond": (227, "Fd-3m"),
       "rocksalt": (225, "Fm-3m"), "CsCl": (221, "Pm-3m"), "zincblende": (216, "F-43m"), "fluorite": (225, "Fm-3m")}


def _system(n):
    for top, name in ((2, "triclinic"), (15, "monoclinic"), (74, "orthorhombic"), (142, "tetragonal"), (167, "trigonal"),
                      (194, "hexagonal"), (230, "cubic")):
        if n <= top:
            return name


def space_group(structure):
    n, sym = _SG[structure]
    return {"number": n, "symbol": sym, "crystal_system": _system(n)}


def ref_space_group(structure):
    import spglib
    at = _ase_ceramic(structure, 5.0) if structure in _CERAMIC else _ase_cell(structure, 4.0)
    sg = spglib.get_spacegroup((at.get_cell()[:], at.get_scaled_positions(), at.get_atomic_numbers()), symprec=1e-5)
    sym, num = sg.split(" (")
    return {"number": int(num.rstrip(")")), "symbol": sym}


_add("space_group",
     name="Space group of a common crystal structure", topic="crystal structures / symmetry",
     purpose="The space-group number, Hermann-Mauguin symbol and crystal system of a common metallic or ionic structure.",
     inputs=[_choice("structure", list(_SG), "structure (" + ", ".join(_SG) + ")")],
     outputs=[_o("number", "number", "space-group number (International Tables)"), {"symbol": "symbol", "kind": "text", "meaning": "Hermann-Mauguin symbol"},
              {"symbol": "crystal_system", "kind": "text", "meaning": "crystal system"}],
     primary="number", conditions=[],
     method="the symmetry of the ideal structure (International Tables numbering)", latex=r"\text{e.g. FCC: } Fm\bar{3}m\ (225)",
     run=space_group,
     parse=lambda q: _pairs(q, r"space group", {"structure": "structure"}, words={"structure"}),
     reference=ref_space_group,
     wrong=_safe(lambda structure: {"number": 225, "symbol": "Fm-3m", "crystal_system": "cubic"}),
     random_inputs=lambda rng: {"structure": rng.choice(list(_SG))},
     templates=["Space group: structure = diamond", "Space group: structure = HCP"],
     examples=[{"inputs": {"structure": "diamond"}, "outputs": {"number": 227, "symbol": "Fd-3m"}},
               {"inputs": {"structure": "HCP"}, "outputs": {"number": 194, "crystal_system": "hexagonal"}},
               {"inputs": {"structure": "zincblende"}, "outputs": {"number": 216}}],
     questions=[{"q": "Space group: structure = BCC", "outputs": {"number": 229, "symbol": "Im-3m"}},
                {"q": "Space group: structure = quasicrystal", "outcome": "INVALID_INPUT"}])

# ================================================================================================ defects and diffusion
def vacancy_concentration(rho, A, Qv, T):
    _pos("the density, molar mass, formation energy and temperature", rho, A, Qv, T)
    N = NA * rho / A
    f = math.exp(-Qv / (KB * T))
    return _out(N=N, Nv=N * f, fraction=f, log10_Nv=math.log10(N) - Qv / (KB * T) / math.log(10), N_per_cm3=N * 1e-6)


def ref_vacancy_concentration(rho, A, Qv, T):
    """Equilibrium from the configurational free energy G(n) = n Qv - kT ln C(N, n), minimised exactly with
    digamma functions (no Stirling approximation): psi(N - n + 1) - psi(n + 1) = Qv/kT, then N n/(N - n)."""
    from scipy.constants import Avogadro, Boltzmann
    from scipy.optimize import brentq
    from scipy.special import digamma
    N = Avogadro * rho / A
    beta_q = Qv / (Boltzmann * T)
    n = brentq(lambda n: digamma(N - n + 1) - digamma(n + 1) - beta_q, 1e-3, N / 2, xtol=1e-300, rtol=8.9e-16, maxiter=500)
    nv = N * n / (N - n)
    return {"N_per_cm3": N * 1e-6, "fraction": nv / N, "log10_Nv": math.log10(nv)}


def _rand_vacancy(rng):
    T = rng.uniform(300, 1500)
    return {"rho": rng.uniform(2000, 20000), "A": rng.uniform(0.02, 0.2), "T": T, "Qv": rng.uniform(5, 25) * KB * T}


_add("vacancy_concentration",
     name="Equilibrium vacancy concentration", topic="defects / point defects",
     purpose="The number of atomic sites per volume and the equilibrium number of vacancies N_v = N exp(-Q_v/kT) of a metal.",
     inputs=[_phys("rho", "density", "density"), _phys("A", "molar_mass", "molar mass"),
             _phys("Qv", "energy", "vacancy formation energy (per vacancy, e.g. eV)"), _phys("T", "temperature", "temperature")],
     outputs=[_o("N", "number_density", "atomic sites per volume"), _o("Nv", "number_density", "vacancies per volume"),
              _o("fraction", "number", "vacancy fraction Nv/N"), _o("log10_Nv", "number", "log10 of Nv in m^-3"),
              _o("N_per_cm3", "number", "atomic sites per cm^3")],
     primary="log10_Nv", conditions=["all inputs > 0"],
     method="N = N_A rho / A; N_v = N exp(-Q_v / (k T))", latex=r"N_v = N\exp\!\left(-\frac{Q_v}{kT}\right),\quad N = \frac{N_A\rho}{A}",
     run=vacancy_concentration,
     parse=lambda q: _pairs(q, r"vacancy concentration", {"rho": "rho", "density": "rho", "A": "A", "Qv": "Qv", "T": "T"},
                            phys={"rho", "A", "Qv", "T"}),
     reference=ref_vacancy_concentration,
     wrong=_safe(lambda rho, A, Qv, T: (lambda N, f: _out(N=N, Nv=N * f, fraction=f, log10_Nv=math.log10(N * f), N_per_cm3=N * 1e-6))(
         NA * rho / A, math.exp(-Qv / (2 * KB * T)))),
     random_inputs=_rand_vacancy,
     templates=["Vacancy concentration: rho = 8.4 g/cm^3, A = 63.5 g/mol, Qv = 0.9 eV, T = 1000 K",
                "Vacancy concentration: rho = 8.4 g/cm^3, A = 63.5 g/mol, Qv = 0.9 eV, T = 727 degC"],
     examples=[{"inputs": {"rho": 8400.0, "A": 0.0635, "Qv": 0.9 * QE, "T": 1000.0},
                "outputs": {"fraction": math.exp(-0.9 * QE / (KB * 1000)), "N_per_cm3": NA * 8400 / 0.0635 * 1e-6}}],
     questions=[{"q": "Vacancy concentration: rho = 8.4 g/cm^3, A = 63.5 g/mol, Qv = 0.9 eV, T = 1000 K",
                 "outputs": {"fraction": math.exp(-0.9 * QE / (KB * 1000))}},
                {"q": "Vacancy concentration: rho = 8.4 g/cm^3, A = 63.5 g/mol, Qv = 0.9 eV", "outcome": "NEEDS_INPUT"}])


def _linfit(x, y):
    """Least-squares line y = b0 + b1 x (plain Python, centred sums): (b0, b1, r^2)."""
    n = len(x)
    if n != len(y):
        raise ValueError("the two lists must have the same length")
    if n < 2:
        raise ValueError("give at least two points")
    mx, my = sum(x) / n, sum(y) / n
    sxx = sum((a - mx) ** 2 for a in x)
    if sxx <= 1e-24 * max(1.0, max(abs(a) for a in x)) ** 2:
        raise ValueError("the x values must not all be equal")
    sxy = sum((a - mx) * (b - my) for a, b in zip(x, y))
    syy = sum((b - my) ** 2 for b in y)
    b1 = sxy / sxx
    r2 = 1.0 if syy == 0 else sxy * sxy / (sxx * syy)
    return my - b1 * mx, b1, r2


def vacancy_energy_fit(T, fraction):
    if len(T) != len(fraction):
        raise ValueError("T and fraction must have the same length")
    for t in T:
        _pos("each temperature", t)
    for f in fraction:
        _frac01("each vacancy fraction", f)
    b0, b1, r2 = _linfit([1 / t for t in T], [math.log(f) for f in fraction])
    if b1 >= 0:
        raise ValueError("the vacancy fraction must increase with temperature")
    Q = -b1 * KB
    return _out(Qv_eV=Q / QE, Qv=Q, prefactor=math.exp(b0), r_squared=r2)


def ref_vacancy_energy_fit(T, fraction):
    import numpy as np
    from scipy.stats import linregress
    from scipy.constants import Boltzmann, electron_volt
    r = linregress(1 / np.array(T), np.log(np.array(fraction)))
    out = {"Qv_eV": -r.slope * Boltzmann / electron_volt, "prefactor": math.exp(r.intercept)}
    if len(T) > 2:
        out["r_squared"] = r.rvalue ** 2
    return out


def _rand_arrhenius_points(rng, q_over_k, pre, tlo, thi, noise):
    n = rng.randint(2, 7)
    T = sorted(round(rng.uniform(tlo, thi), 1) for _ in range(n))
    while len(set(T)) < n:
        T = sorted(round(rng.uniform(tlo, thi), 1) for _ in range(n))
    y = [pre * math.exp(-q_over_k / t) * math.exp(rng.gauss(0, noise)) for t in T]
    return T, [float(f"{v:.6g}") for v in y]


def _rand_vac_fit(rng):
    T, f = _rand_arrhenius_points(rng, rng.uniform(0.6, 2.0) * QE / KB, rng.uniform(1, 20), 600, 1400, 0.05)
    return {"T": T, "fraction": [min(v, 0.5) for v in f]}


_add("vacancy_formation_energy_fit",
     name="Vacancy formation energy from vacancy fractions", topic="defects / point defects",
     purpose="The vacancy formation energy Q_v and prefactor from vacancy fractions measured at two or more temperatures (least-squares Arrhenius line).",
     inputs=[_vec("T", "temperatures in kelvin"), _vec("fraction", "vacancy fractions N_v/N at those temperatures")],
     outputs=[_o("Qv_eV", "number", "vacancy formation energy in eV"), _o("Qv", "energy", "vacancy formation energy"),
              _o("prefactor", "number", "prefactor C of N_v/N = C exp(-Q_v/kT)"), _o("r_squared", "number", "r^2 of the fit")],
     primary="Qv_eV", conditions=["equal lengths, at least two distinct temperatures", "0 < fraction < 1", "fraction rising with T"],
     method="least-squares line of ln(N_v/N) against 1/T: slope = -Q_v/k", latex=r"\ln\frac{N_v}{N} = \ln C - \frac{Q_v}{k}\frac1T",
     run=vacancy_energy_fit,
     parse=lambda q: _pairs(q, r"vacancy formation energy(?: fit)?", {"T": "T", "fraction": "fraction", "f": "fraction"}, vectors={"T", "fraction"}),
     reference=ref_vacancy_energy_fit,
     wrong=_safe(lambda T, fraction: (lambda o: dict(o, Qv_eV=o["Qv_eV"] / 2, Qv=o["Qv"] / 2))(vacancy_energy_fit(T, fraction))),
     random_inputs=_rand_vac_fit,
     templates=["Vacancy formation energy: T = [800, 1000], fraction = [1.2e-6, 3.4e-5]"],
     examples=[{"inputs": {"T": [800.0, 1000.0], "fraction": [1.2e-6, 3.4e-5]},
                "outputs": {"Qv_eV": math.log(3.4e-5 / 1.2e-6) * KB / (1 / 800 - 1 / 1000) / QE}}],
     questions=[{"q": "Vacancy formation energy: T = [800, 1000], fraction = [1.2e-6, 3.4e-5]",
                 "outputs": {"Qv_eV": math.log(3.4e-5 / 1.2e-6) * KB / (1 / 800 - 1 / 1000) / QE}},
                {"q": "Vacancy formation energy: T = [800, 1000], fraction = [3.4e-5, 1.2e-6]", "outcome": "INVALID_INPUT"},
                {"q": "Vacancy formation energy: T = [800, 1000]", "outcome": "NEEDS_INPUT"}])


# ---------------------------------------------------------------------------------------- Fick's laws
def fick_first(D, C1, C2, x1, x2):
    _pos("the diffusion coefficient", D)
    if x1 == x2:
        raise ValueError("the two positions must differ")
    J = -D * (C2 - C1) / (x2 - x1)
    return _out(J=J, J_per_hour=J * 3600)


def ref_fick_first(D, C1, C2, x1, x2):
    import numpy as np
    from scipy.integrate import solve_bvp
    x = np.linspace(x1, x2, 11) if x2 > x1 else np.linspace(x2, x1, 11)
    ca, cb = (C1, C2) if x2 > x1 else (C2, C1)
    sol = solve_bvp(lambda s, y: np.vstack([y[1], np.zeros_like(y[0])]),
                    lambda ya, yb: np.array([ya[0] - ca, yb[0] - cb]), x, np.zeros((2, x.size)), tol=1e-12)
    J = -D * float(sol.sol(x[0])[1])
    return {"J": J, "J_per_hour": J * 3600}


_add("fick_first_law_flux",
     name="Steady-state diffusion flux (Fick's first law)", topic="diffusion / steady state",
     purpose="The steady-state diffusion flux between two positions with known concentrations: J = -D (C2 - C1)/(x2 - x1).",
     inputs=[_phys("D", "diffusion_coefficient", "diffusion coefficient"), _phys("C1", "mass_concentration", "concentration at x1"),
             _phys("C2", "mass_concentration", "concentration at x2"), _phys("x1", "length", "first position"), _phys("x2", "length", "second position")],
     outputs=[_o("J", "mass_flux", "diffusion flux (positive towards increasing x)"), _o("J_per_hour", "number", "flux in kg/(m^2 h)")],
     primary="J", conditions=["D > 0", "x1 != x2"],
     method="J = -D dC/dx with the linear steady-state profile", latex=r"J = -D\frac{dC}{dx} = -D\frac{C_2-C_1}{x_2-x_1}",
     run=fick_first,
     parse=lambda q: _pairs(q, r"fick'?s first law|steady[- ]state diffusion flux", {"D": "D", "C1": "C1", "C2": "C2", "x1": "x1", "x2": "x2"},
                            phys={"D", "C1", "C2", "x1", "x2"}),
     reference=ref_fick_first,
     wrong=_safe(lambda D, C1, C2, x1, x2: (lambda J: _out(J=J, J_per_hour=J * 3600))(D * (C2 - C1) / (x2 - x1))),
     random_inputs=lambda rng: {"D": _lu(rng, 1e-12, 1e-8), "C1": rng.uniform(0.5, 5), "C2": rng.uniform(5.5, 10),
                                "x1": rng.uniform(0, 2e-3), "x2": rng.uniform(3e-3, 1e-2)},
     templates=["Fick's first law: D = 3e-11 m^2/s, C1 = 1.2 kg/m^3, C2 = 0.8 kg/m^3, x1 = 5 mm, x2 = 10 mm"],
     examples=[{"inputs": {"D": 3e-11, "C1": 1.2, "C2": 0.8, "x1": 5e-3, "x2": 10e-3}, "outputs": {"J": 2.4e-9}}],
     questions=[{"q": "Fick's first law: D = 3e-11 m^2/s, C1 = 1.2 kg/m^3, C2 = 0.8 kg/m^3, x1 = 5 mm, x2 = 10 mm", "outputs": {"J": 2.4e-9}},
                {"q": "Fick's first law: D = 3e-11 m^2/s, C1 = 1.2 kg/m^3, C2 = 0.8 kg/m^3, x1 = 5 mm, x2 = 5 mm", "outcome": "INVALID_INPUT"}])


def _erf_solution(C0, Cs, x, D, t):
    return Cs - (Cs - C0) * math.erf(x / (2 * math.sqrt(D * t)))


def _green_profile(C0, Cs, x, D, t):
    """C(x, t) of the semi-infinite solid (C = C0 at t = 0, C = Cs at x = 0) by the method of images: the initial
    excess C0 - Cs spread by the heat kernel and its mirror image, integrated with scipy quadrature (no erf)."""
    from scipy.integrate import quad
    z = x / math.sqrt(4 * D * t)
    # exp(-(z-s)^2) - exp(-(z+s)^2) written without cancellation: 2 exp(-(z^2+s^2)) sinh(2zs) = exp(-(z-s)^2)(1 - exp(-4zs))
    f = lambda s: math.exp(-(z - s) ** 2) * -math.expm1(-4 * z * s) / math.sqrt(math.pi)    # noqa: E731
    I1, _ = quad(f, 0, z, epsabs=0, epsrel=2e-14, limit=200)
    I2, _ = quad(f, z, z + 40, epsabs=0, epsrel=2e-14, limit=200)
    return Cs + (C0 - Cs) * (I1 + I2)


def fick_second(C0, Cs, x, D, t):
    _pos("x, D and t", x, D, t)
    if Cs == C0:
        raise ValueError("the surface and initial concentrations must differ")
    z = x / (2 * math.sqrt(D * t))
    Cx = _erf_solution(C0, Cs, x, D, t)
    return _out(Cx=Cx, z=z, ratio=(Cx - C0) / (Cs - C0))


def ref_fick_second(C0, Cs, x, D, t):
    Cx = _green_profile(C0, Cs, x, D, t)
    return {"Cx": Cx, "ratio": (Cx - C0) / (Cs - C0)}


_CONC = ["wt%", "at%", "%", "kg/m^3", "kg/m3", "mol/m^3", "atoms/m^3", "m^-3", "ppm"]


def _rand_fick2(rng):
    while True:
        D, t = _lu(rng, 1e-13, 1e-9), _lu(rng, 60, 3e5)
        x = rng.uniform(0.05, 2.5) * 2 * math.sqrt(D * t)
        C0, Cs = rng.uniform(0, 0.5), rng.uniform(0.6, 1.5)
        if rng.random() < 0.3:
            C0, Cs = Cs, C0
        r = 1 - math.erf(x / (2 * math.sqrt(D * t)))
        if r > 0.01:
            return {"C0": C0, "Cs": Cs, "x": x, "D": D, "t": t}


_add("fick_second_law_concentration",
     name="Concentration in a semi-infinite solid (Fick's second law)", topic="diffusion / non-steady state",
     purpose="The concentration at depth x after time t in a semi-infinite solid with a constant surface concentration (carburizing, doping): C_x = C_s - (C_s - C_0) erf(x / 2 sqrt(Dt)).",
     inputs=[_numin("C0", "initial (bulk) concentration (wt%, at% or any one unit)"), _numin("Cs", "surface concentration (same unit)"),
             _phys("x", "length", "depth below the surface"), _phys("D", "diffusion_coefficient", "diffusion coefficient"), _phys("t", "time", "diffusion time")],
     outputs=[_o("Cx", "number", "concentration at depth x (unit of C0, Cs)"), _o("z", "number", "erf argument x / (2 sqrt(D t))"),
              _o("ratio", "number", "(Cx - C0)/(Cs - C0)")],
     primary="Cx", conditions=["x, D, t > 0", "Cs != C0", "constant D, surface concentration held constant"],
     method="the error-function solution of dC/dt = D d2C/dx2", latex=r"\frac{C_x - C_0}{C_s - C_0} = 1 - \operatorname{erf}\!\left(\frac{x}{2\sqrt{Dt}}\right)",
     run=fick_second,
     parse=lambda q: _pairs(q, r"fick'?s second law|diffusion concentration profile", {"C0": "C0", "Cs": "Cs", "x": "x", "D": "D", "t": "t"},
                            phys={"x", "D", "t"}, num_units={"C0": _CONC, "Cs": _CONC}),
     reference=ref_fick_second,
     wrong=_safe(lambda C0, Cs, x, D, t: (lambda Cx: _out(Cx=Cx, z=x / math.sqrt(D * t), ratio=(Cx - C0) / (Cs - C0)))(
         Cs - (Cs - C0) * math.erf(x / math.sqrt(D * t)))),
     random_inputs=_rand_fick2,
     templates=["Fick's second law: C0 = 0.25 wt%, Cs = 1.2 wt%, x = 0.5 mm, D = 1.6e-11 m^2/s, t = 7 h"],
     examples=[{"inputs": {"C0": 0.25, "Cs": 1.2, "x": 0.5e-3, "D": 1.6e-11, "t": 7 * 3600.0},
                "outputs": {"Cx": 1.2 - 0.95 * math.erf(0.5e-3 / (2 * math.sqrt(1.6e-11 * 25200)))}},
               {"inputs": {"C0": 0.0, "Cs": 1.0, "x": 2e-4, "D": 1e-10, "t": 100.0}, "outputs": {"Cx": 1 - math.erf(1.0)}}],
     questions=[{"q": "Fick's second law: C0 = 0.25 wt%, Cs = 1.2 wt%, x = 0.5 mm, D = 1.6e-11 m^2/s, t = 7 h",
                 "outputs": {"Cx": 1.2 - 0.95 * math.erf(0.5e-3 / (2 * math.sqrt(1.6e-11 * 25200)))}},
                {"q": "Fick's second law: C0 = 0.25 wt%, Cs = 0.25 wt%, x = 0.5 mm, D = 1.6e-11 m^2/s, t = 7 h", "outcome": "INVALID_INPUT"}])


def _inverse_erf(y):
    """z with erf(z) = y for 0 < y < 1, by bisection (plain Python) to the last bit."""
    if not 0 < y < 1:
        raise ValueError("the concentration must lie strictly between the initial and the surface values")
    lo, hi = 0.0, 6.0
    for _ in range(200):
        mid = (lo + hi) / 2
        if mid in (lo, hi):
            break
        if math.erf(mid) < y:
            lo = mid
        else:
            hi = mid
    return (lo + hi) / 2


def diffusion_time(C0, Cs, Cx, x, D):
    _pos("x and D", x, D)
    if Cs == C0:
        raise ValueError("the surface and initial concentrations must differ")
    y = (Cs - Cx) / (Cs - C0)
    if not 0.002 < y < 0.998:
        raise ValueError("the target concentration must lie strictly between C0 and Cs (and not within 0.2 % of either)")
    z = _inverse_erf(y)
    t = x * x / (4 * D * z * z)
    return _out(t=t, t_hours=t / 3600, z=z)


def ref_diffusion_time(C0, Cs, Cx, x, D):
    from scipy.optimize import brentq
    tau = x * x / D
    g = lambda lt: _green_profile(C0, Cs, x, D, tau * math.exp(lt)) - Cx                  # noqa: E731
    lt = brentq(g, math.log(1e-6), math.log(1e8), xtol=1e-14, rtol=8.9e-16, maxiter=300)
    return {"t_hours": tau * math.exp(lt) / 3600}


def _rand_ratio_case(rng):
    C0, Cs = rng.uniform(0, 0.5), rng.uniform(0.6, 1.5)
    if rng.random() < 0.3:
        C0, Cs = Cs, C0
    y = rng.uniform(0.05, 0.95)
    return C0, Cs, Cs - y * (Cs - C0)


_add("diffusion_time_to_concentration",
     name="Time to reach a concentration at a depth (Fick's second law)", topic="diffusion / non-steady state",
     purpose="The diffusion time for the concentration at depth x to reach C_x in a semi-infinite solid with constant surface concentration (carburizing time).",
     inputs=[_numin("C0", "initial concentration"), _numin("Cs", "surface concentration"), _numin("Cx", "target concentration at depth x"),
             _phys("x", "length", "depth"), _phys("D", "diffusion_coefficient", "diffusion coefficient")],
     outputs=[_o("t", "time", "diffusion time"), _o("t_hours", "number", "diffusion time in hours"), _o("z", "number", "erf argument")],
     primary="t_hours", conditions=["x, D > 0", "Cx strictly between C0 and Cs"],
     method="erf(z) = (Cs - Cx)/(Cs - C0) inverted by bisection; t = x^2/(4 D z^2)",
     latex=r"t = \frac{x^2}{4Dz^2},\quad \operatorname{erf}z = \frac{C_s-C_x}{C_s-C_0}",
     run=diffusion_time,
     parse=lambda q: _pairs(q, r"diffusion time|carburizing time", {"C0": "C0", "Cs": "Cs", "Cx": "Cx", "x": "x", "D": "D"},
                            phys={"x", "D"}, num_units={"C0": _CONC, "Cs": _CONC, "Cx": _CONC}),
     reference=ref_diffusion_time,
     wrong=_safe(lambda C0, Cs, Cx, x, D: (lambda z: _out(t=x * x / (D * z * z), t_hours=x * x / (D * z * z) / 3600, z=z))(
         _inverse_erf((Cs - Cx) / (Cs - C0)))),
     random_inputs=lambda rng: (lambda c: {"C0": c[0], "Cs": c[1], "Cx": c[2], "x": rng.uniform(1e-4, 3e-3), "D": _lu(rng, 1e-12, 1e-9)})(_rand_ratio_case(rng)),
     templates=["Diffusion time: C0 = 0.25 wt%, Cs = 1.2 wt%, Cx = 0.8 wt%, x = 0.5 mm, D = 1.6e-11 m^2/s"],
     examples=[{"inputs": {"C0": 0.0, "Cs": 1.0, "Cx": 1 - math.erf(0.5), "x": 1e-3, "D": 1e-10}, "outputs": {"t_hours": 1e-6 / 1e-10 / 3600}},
               {"inputs": {"C0": 0.25, "Cs": 1.2, "Cx": 0.8, "x": 0.5e-3, "D": 1.6e-11}, "outputs": {"z": _inverse_erf(0.4 / 0.95)}}],
     questions=[{"q": "Diffusion time: C0 = 0, Cs = 1, Cx = 0.4795001221869535, x = 1 mm, D = 1e-10 m^2/s", "outputs": {"t_hours": 1e4 / 3600}},
                {"q": "Diffusion time: C0 = 0.25 wt%, Cs = 1.2 wt%, Cx = 1.5 wt%, x = 0.5 mm, D = 1.6e-11 m^2/s", "outcome": "INVALID_INPUT"}])


def diffusion_depth(C0, Cs, Cx, D, t):
    _pos("D and t", D, t)
    if Cs == C0:
        raise ValueError("the surface and initial concentrations must differ")
    y = (Cs - Cx) / (Cs - C0)
    if not 0.002 < y < 0.998:
        raise ValueError("the target concentration must lie strictly between C0 and Cs (and not within 0.2 % of either)")
    z = _inverse_erf(y)
    x = 2 * z * math.sqrt(D * t)
    return _out(x=x, x_mm=x * 1e3, z=z)


def ref_diffusion_depth(C0, Cs, Cx, D, t):
    from scipy.optimize import brentq
    L = math.sqrt(D * t)
    x = brentq(lambda u: _green_profile(C0, Cs, u * L, D, t) - Cx, 1e-9, 12.0, xtol=1e-15, rtol=8.9e-16, maxiter=300) * L
    return {"x_mm": x * 1e3}


_add("diffusion_depth_of_concentration",
     name="Depth at which a concentration is reached (Fick's second law)", topic="diffusion / non-steady state",
     purpose="The depth at which the concentration equals C_x after diffusion time t in a semi-infinite solid with constant surface concentration (case depth).",
     inputs=[_numin("C0", "initial concentration"), _numin("Cs", "surface concentration"), _numin("Cx", "target concentration"),
             _phys("D", "diffusion_coefficient", "diffusion coefficient"), _phys("t", "time", "diffusion time")],
     outputs=[_o("x", "length", "depth"), _o("x_mm", "number", "depth in mm"), _o("z", "number", "erf argument")],
     primary="x_mm", conditions=["D, t > 0", "Cx strictly between C0 and Cs"],
     method="x = 2 z sqrt(D t) with erf(z) = (Cs - Cx)/(Cs - C0)", latex=r"x = 2z\sqrt{Dt}",
     run=diffusion_depth,
     parse=lambda q: _pairs(q, r"diffusion depth|case depth", {"C0": "C0", "Cs": "Cs", "Cx": "Cx", "D": "D", "t": "t"},
                            phys={"D", "t"}, num_units={"C0": _CONC, "Cs": _CONC, "Cx": _CONC}),
     reference=ref_diffusion_depth,
     wrong=_safe(lambda C0, Cs, Cx, D, t: (lambda z: _out(x=z * math.sqrt(D * t), x_mm=z * math.sqrt(D * t) * 1e3, z=z))(
         _inverse_erf((Cs - Cx) / (Cs - C0)))),
     random_inputs=lambda rng: (lambda c: {"C0": c[0], "Cs": c[1], "Cx": c[2], "D": _lu(rng, 1e-12, 1e-9), "t": _lu(rng, 600, 3e5)})(_rand_ratio_case(rng)),
     templates=["Diffusion depth: C0 = 0.2 wt%, Cs = 1.0 wt%, Cx = 0.6 wt%, D = 2e-11 m^2/s, t = 10 h"],
     examples=[{"inputs": {"C0": 0.0, "Cs": 1.0, "Cx": 1 - math.erf(0.5), "D": 1e-10, "t": 1e4}, "outputs": {"x_mm": 1.0}}],
     questions=[{"q": "Diffusion depth: C0 = 0.2 wt%, Cs = 1.0 wt%, Cx = 0.6 wt%, D = 2e-11 m^2/s, t = 10 h",
                 "outputs": {"x_mm": 2 * _inverse_erf(0.5) * math.sqrt(2e-11 * 36000) * 1e3}}])


def arrhenius_fit(T, D, T_target=None):
    if len(T) != len(D):
        raise ValueError("T and D must have the same length")
    for v in list(T) + list(D):
        _pos("every temperature and diffusion coefficient", v)
    b0, b1, r2 = _linfit([1 / t for t in T], [math.log(d) for d in D])
    if b1 >= 0:
        raise ValueError("D must increase with temperature (a positive activation energy)")
    Q = -b1 * RGAS
    out = dict(Q=Q, Q_kJmol=Q / 1000, D0=math.exp(b0), log10_D0=b0 / math.log(10), r_squared=r2)
    if T_target is not None:
        _pos("the target temperature", T_target)
        lnd = b0 + b1 / T_target
        out.update(D_target=math.exp(lnd), log10_D_target=lnd / math.log(10))
    return _out(**out)


def ref_arrhenius_fit(T, D, T_target=None):
    import numpy as np
    from scipy.constants import gas_constant
    from scipy.stats import linregress
    r = linregress(1 / np.array(T), np.log(np.array(D)))
    out = {"Q_kJmol": -r.slope * gas_constant / 1000, "log10_D0": r.intercept / math.log(10)}
    if T_target is not None:
        out["log10_D_target"] = (r.intercept + r.slope / T_target) / math.log(10)
    return out


def _rand_arr_fit(rng):
    T, D = _rand_arrhenius_points(rng, rng.uniform(80e3, 300e3) / RGAS, _lu(rng, 1e-6, 1e-3), 700, 1500, 0.05)
    x = {"T": T, "D": D}
    if rng.random() < 0.5:
        x["T_target"] = rng.uniform(600, 1600)
    return x


_add("diffusion_arrhenius_fit",
     name="Activation energy and pre-exponential factor of diffusion", topic="diffusion / temperature dependence",
     purpose="Q_d and D_0 of D = D_0 exp(-Q_d/RT) from diffusion coefficients at two or more temperatures, and D at another temperature.",
     inputs=[_vec("T", "temperatures in kelvin"), _vec("D", "diffusion coefficients in m^2/s at those temperatures"),
             _phys("T_target", "temperature", "temperature at which D is wanted (optional)", required=False)],
     outputs=[_o("Q", "molar_energy", "activation energy"), _o("Q_kJmol", "number", "activation energy in kJ/mol"),
              _o("D0", "diffusion_coefficient", "pre-exponential factor"), _o("log10_D0", "number", "log10 of D0 in m^2/s"),
              _o("r_squared", "number", "r^2 of the fit"), _o("D_target", "diffusion_coefficient", "D at T_target"),
              _o("log10_D_target", "number", "log10 of D at T_target")],
     primary="Q_kJmol", conditions=["equal lengths, at least two distinct temperatures", "D rising with T"],
     method="least-squares line of ln D against 1/T: slope = -Q_d/R, intercept = ln D_0",
     latex=r"\ln D = \ln D_0 - \frac{Q_d}{R}\frac{1}{T}",
     run=arrhenius_fit,
     parse=lambda q: _pairs(q, r"diffusion arrhenius fit|diffusion activation energy", {"T": "T", "D": "D", "T_target": "T_target"},
                            phys={"T_target"}, vectors={"T", "D"}),
     reference=ref_arrhenius_fit,
     wrong=_safe(lambda T, D, T_target=None: (lambda o: dict(o, Q_kJmol=o["Q_kJmol"] * math.log(10), Q=o["Q"] * math.log(10)))(
         arrhenius_fit(T, D, T_target))),
     random_inputs=_rand_arr_fit,
     templates=["Diffusion arrhenius fit: T = [773, 873], D = [5.8e-13, 5.3e-12]",
                "Diffusion arrhenius fit: T = [773, 873], D = [5.8e-13, 5.3e-12], T_target = 1000 K"],
     examples=[{"inputs": {"T": [773.0, 873.0], "D": [5.8e-13, 5.3e-12]},
                "outputs": {"Q_kJmol": RGAS * math.log(5.3e-12 / 5.8e-13) / (1 / 773 - 1 / 873) / 1000}}],
     questions=[{"q": "Diffusion arrhenius fit: T = [773, 873], D = [5.8e-13, 5.3e-12]",
                 "outputs": {"Q_kJmol": RGAS * math.log(5.3e-12 / 5.8e-13) / (1 / 773 - 1 / 873) / 1000}},
                {"q": "Diffusion arrhenius fit: T = [773, 873], D = [5.3e-12, 5.8e-13]", "outcome": "INVALID_INPUT"},
                {"q": "Diffusion arrhenius fit: T = [773, 873, 973], D = [5.8e-13, 5.3e-12]", "outcome": "INVALID_INPUT"}])


# ================================================================================================ mechanical properties
def true_stress_strain(stress, strain):
    if not strain > -1:
        raise ValueError("the engineering strain must exceed -1")
    sT = stress * (1 + strain)
    eT = math.log1p(strain)
    return _out(true_stress=sT, true_stress_MPa=sT / 1e6, true_strain=eT)


def ref_true_stress_strain(stress, strain):
    """True strain as the integral of dl/l (scipy quadrature); true stress from the force over the current area of a
    bar whose volume is conserved (A l = A0 l0)."""
    from scipy.integrate import quad
    eT, _ = quad(lambda L: 1 / L, 1.0, 1.0 + strain, epsabs=0, epsrel=2e-14)
    A0, L0 = 1.0, 1.0
    F = stress * A0
    A = A0 * L0 / (L0 * (1 + strain))
    return {"true_strain": eT, "true_stress_MPa": F / A / 1e6}


_add("true_stress_and_strain",
     name="True stress and true strain from engineering values", topic="mechanical properties / tensile test",
     purpose="True stress and true strain from the engineering stress and strain of a tensile test before necking (volume conserved).",
     inputs=[_phys("stress", "stress", "engineering stress"), _numin("strain", "engineering strain (dimensionless)")],
     outputs=[_o("true_stress", "stress", "true stress"), _o("true_stress_MPa", "number", "true stress in MPa"), _o("true_strain", "number", "true strain")],
     primary="true_strain", conditions=["strain > -1", "uniform deformation (before necking)"],
     method="sigma_T = sigma (1 + epsilon), epsilon_T = ln(1 + epsilon)", latex=r"\sigma_T = \sigma(1+\varepsilon),\quad \varepsilon_T = \ln(1+\varepsilon)",
     run=true_stress_strain,
     parse=lambda q: _pairs(q, r"true stress and strain|true stress", {"stress": "stress", "sigma": "stress", "strain": "strain", "epsilon": "strain"},
                            phys={"stress"}),
     reference=ref_true_stress_strain,
     wrong=_safe(lambda stress, strain: _out(true_stress=stress * (1 - strain), true_stress_MPa=stress * (1 - strain) / 1e6, true_strain=math.log(1 + strain) / (1 + strain))),
     random_inputs=lambda rng: {"stress": rng.uniform(50e6, 900e6), "strain": rng.uniform(0.01, 0.4)},
     templates=["True stress and strain: stress = 400 MPa, strain = 0.2"],
     examples=[{"inputs": {"stress": 400e6, "strain": 0.2}, "outputs": {"true_stress_MPa": 480.0, "true_strain": math.log(1.2)}}],
     questions=[{"q": "True stress and strain: stress = 400 MPa, strain = 0.2", "outputs": {"true_stress_MPa": 480.0, "true_strain": math.log(1.2)}},
                {"q": "True stress and strain: stress = 400 MPa, strain = -2", "outcome": "INVALID_INPUT"}])


def elastic_deformation(F, d0, l0, E, nu):
    _pos("F, d0, l0 and E", F, d0, l0, E)
    if not -1 < nu < 0.5:
        raise ValueError("Poisson's ratio must lie between -1 and 0.5")
    A0 = math.pi * d0 * d0 / 4
    s = F / A0
    ez = s / E
    ex = -nu * ez
    return _out(stress_MPa=s / 1e6, axial_strain=ez, elongation_mm=ez * l0 * 1e3, lateral_strain=ex, diameter_change_um=ex * d0 * 1e6)


def ref_elastic_deformation(F, d0, l0, E, nu):
    """Strains from the isotropic stiffness tensor built from the Lame constants, solved with numpy (C eps = sigma)."""
    import numpy as np
    lam = E * nu / ((1 + nu) * (1 - 2 * nu))
    mu = E / (2 * (1 + nu))
    C = np.zeros((6, 6))
    C[:3, :3] = lam
    C[np.arange(3), np.arange(3)] += 2 * mu
    C[3:, 3:] = np.eye(3) * mu
    s = F / (math.pi * (d0 / 2) ** 2)
    eps = np.linalg.solve(C, np.array([0, 0, s, 0, 0, 0.0]))
    return {"stress_MPa": s / 1e6, "axial_strain": float(eps[2]), "elongation_mm": float(eps[2]) * l0 * 1e3,
            "lateral_strain": float(eps[0]), "diameter_change_um": float(eps[0]) * d0 * 1e6}


_add("elastic_deformation_of_a_rod",
     name="Elastic elongation and diameter change of a rod in tension", topic="mechanical properties / elasticity",
     purpose="Stress, axial strain, elongation, lateral strain and diameter change of a cylindrical rod under an axial force (Hooke's law and Poisson's ratio).",
     inputs=[_phys("F", "force", "axial force (tension positive)"), _phys("d0", "length", "original diameter"), _phys("l0", "length", "original length"),
             _phys("E", "elastic_modulus", "Young's modulus"), _numin("nu", "Poisson's ratio")],
     outputs=[_o("stress_MPa", "number", "axial stress in MPa"), _o("axial_strain", "number", "axial strain"),
              _o("elongation_mm", "number", "elongation in mm"), _o("lateral_strain", "number", "lateral strain"),
              _o("diameter_change_um", "number", "change of diameter in micrometres (negative = contraction)")],
     primary="elongation_mm", conditions=["F, d0, l0, E > 0", "-1 < nu < 0.5", "elastic, isotropic"],
     method="sigma = F/(pi d0^2/4), epsilon_z = sigma/E, epsilon_x = -nu epsilon_z, dl = epsilon_z l0, dd = epsilon_x d0",
     latex=r"\varepsilon_z = \frac{F}{A_0E},\quad \varepsilon_x = -\nu\varepsilon_z",
     run=elastic_deformation,
     parse=lambda q: _pairs(q, r"elastic deformation(?: of a rod)?", {"F": "F", "d0": "d0", "l0": "l0", "E": "E", "nu": "nu"}, phys={"F", "d0", "l0", "E"}),
     reference=ref_elastic_deformation,
     wrong=_safe(lambda F, d0, l0, E, nu: (lambda s: _out(stress_MPa=s / 1e6, axial_strain=s / E, elongation_mm=s / E * l0 * 1e3,
                                                          lateral_strain=-nu * s / E, diameter_change_um=-nu * s / E * d0 * 1e6))(F / (math.pi * d0 * d0))),
     random_inputs=lambda rng: {"F": rng.uniform(1e3, 1e5), "d0": rng.uniform(5e-3, 30e-3), "l0": rng.uniform(0.05, 1), "E": rng.uniform(40e9, 400e9),
                                "nu": rng.uniform(0.2, 0.45)},
     templates=["Elastic deformation: F = 15 kN, d0 = 10 mm, l0 = 300 mm, E = 110 GPa, nu = 0.34"],
     examples=[{"inputs": {"F": 15000.0, "d0": 0.01, "l0": 0.3, "E": 110e9, "nu": 0.34},
                "outputs": {"elongation_mm": 15000 / (math.pi * 2.5e-5) / 110e9 * 300, "diameter_change_um": -0.34 * 15000 / (math.pi * 2.5e-5) / 110e9 * 1e4}}],
     questions=[{"q": "Elastic deformation: F = 15 kN, d0 = 10 mm, l0 = 300 mm, E = 110 GPa, nu = 0.34",
                 "outputs": {"elongation_mm": 15000 / (math.pi * 2.5e-5) / 110e9 * 300}},
                {"q": "Elastic deformation: F = 15 kN, d0 = 10 mm, l0 = 300 mm, E = 110 GPa, nu = 0.7", "outcome": "INVALID_INPUT"}])


def elastic_constants(E, nu):
    _pos("Young's modulus", E)
    if not -1 < nu < 0.5:
        raise ValueError("Poisson's ratio must lie between -1 and 0.5")
    G = E / (2 * (1 + nu))
    K = E / (3 * (1 - 2 * nu))
    lam = E * nu / ((1 + nu) * (1 - 2 * nu))
    return _out(G=G, G_GPa=G / 1e9, K=K, K_GPa=K / 1e9, lame_GPa=lam / 1e9)


def ref_elastic_constants(E, nu):
    """pymatgen's ElasticTensor of the isotropic solid whose compliance has 1/E and -nu/E (inverted with numpy)."""
    import numpy as np
    from pymatgen.analysis.elasticity import ElasticTensor
    S = np.zeros((6, 6))
    S[:3, :3] = -nu / E
    S[np.arange(3), np.arange(3)] = 1 / E
    S[3:, 3:] = np.eye(3) * 2 * (1 + nu) / E
    C = np.linalg.inv(S) / 1e9
    et = ElasticTensor.from_voigt(C)
    return {"G_GPa": float(et.g_voigt), "K_GPa": float(et.k_voigt), "lame_GPa": float(C[0, 1])}


_add("isotropic_elastic_constants",
     name="Shear modulus, bulk modulus and Lame constant from E and Poisson's ratio", topic="mechanical properties / elasticity",
     purpose="The shear modulus G, bulk modulus K and Lame constant lambda of an isotropic material from Young's modulus E and Poisson's ratio nu.",
     inputs=[_phys("E", "elastic_modulus", "Young's modulus"), _numin("nu", "Poisson's ratio")],
     outputs=[_o("G", "elastic_modulus", "shear modulus"), _o("G_GPa", "number", "shear modulus in GPa"), _o("K", "elastic_modulus", "bulk modulus"),
              _o("K_GPa", "number", "bulk modulus in GPa"), _o("lame_GPa", "number", "Lame's first constant in GPa")],
     primary="G_GPa", conditions=["E > 0", "-1 < nu < 0.5"],
     method="G = E/(2(1 + nu)), K = E/(3(1 - 2 nu)), lambda = E nu/((1 + nu)(1 - 2 nu))",
     latex=r"G = \frac{E}{2(1+\nu)},\quad K = \frac{E}{3(1-2\nu)}",
     run=elastic_constants,
     parse=lambda q: _pairs(q, r"elastic constants|isotropic elastic constants", {"E": "E", "nu": "nu"}, phys={"E"}),
     reference=ref_elastic_constants,
     wrong=_safe(lambda E, nu: _out(G=E / (2 * (1 - nu)), G_GPa=E / (2 * (1 - nu)) / 1e9, K=E / (3 * (1 - nu)), K_GPa=E / (3 * (1 - nu)) / 1e9,
                                    lame_GPa=E * nu / (1 + nu) / 1e9)),
     random_inputs=lambda rng: {"E": rng.uniform(1e9, 500e9), "nu": rng.uniform(0.05, 0.48)},
     templates=["Elastic constants: E = 200 GPa, nu = 0.3"],
     examples=[{"inputs": {"E": 200e9, "nu": 0.3}, "outputs": {"G_GPa": 200 / 2.6, "K_GPa": 200 / 1.2}}],
     questions=[{"q": "Elastic constants: E = 200 GPa, nu = 0.3", "outputs": {"G_GPa": 200 / 2.6, "K_GPa": 200 / 1.2}},
                {"q": "Elastic constants: E = 200 GPa, nu = 0.5", "outcome": "INVALID_INPUT"}])


def youngs_modulus_fit(strain, stress):
    if len(strain) != len(stress):
        raise ValueError("strain and stress must have the same length")
    b0, b1, r2 = _linfit(list(strain), list(stress))
    if b1 <= 0:
        raise ValueError("stress must rise with strain in the elastic region")
    b0 = 0.0 if abs(b0) <= 1e-12 * max(abs(v) for v in stress) else b0      # zero by the data: written as exactly 0
    return _out(E_GPa=b1 / 1000, E=b1 * 1e6, intercept_MPa=b0, r_squared=r2)


def ref_youngs_modulus_fit(strain, stress):
    from scipy.stats import linregress
    r = linregress(strain, stress)
    b0 = 0.0 if abs(r.intercept) <= 1e-12 * max(abs(v) for v in stress) else float(r.intercept)
    out = {"E_GPa": r.slope / 1000, "intercept_MPa": b0}
    if len(strain) > 2:
        out["r_squared"] = r.rvalue ** 2
    return out


def _rand_elastic(rng):
    n = rng.randint(3, 10)
    E = rng.uniform(40e3, 400e3)                                  # MPa
    eps = sorted(rng.uniform(0, 0.003) for _ in range(n))
    return {"strain": [round(e, 7) for e in eps], "stress": [round(E * e + rng.gauss(0, 2), 4) for e in eps]}


_add("youngs_modulus_from_data",
     name="Young's modulus from elastic stress-strain data", topic="mechanical properties / tensile test",
     purpose="Young's modulus as the slope of the least-squares line through stress-strain points of the elastic region.",
     inputs=[_vec("strain", "engineering strains (dimensionless)"), _vec("stress", "engineering stresses in MPa")],
     outputs=[_o("E_GPa", "number", "Young's modulus in GPa"), _o("E", "elastic_modulus", "Young's modulus"),
              _o("intercept_MPa", "number", "intercept of the line in MPa"), _o("r_squared", "number", "r^2 of the fit")],
     primary="E_GPa", conditions=["equal lengths, at least two distinct strains", "stress in MPa"],
     method="least-squares line stress = b0 + E strain", latex=r"E = \frac{\sum(\varepsilon_i-\bar\varepsilon)(\sigma_i-\bar\sigma)}{\sum(\varepsilon_i-\bar\varepsilon)^2}",
     run=youngs_modulus_fit,
     parse=lambda q: _pairs(q, r"young'?s modulus from data|elastic modulus from data", {"strain": "strain", "stress": "stress"}, vectors={"strain", "stress"}),
     reference=ref_youngs_modulus_fit,
     wrong=_safe(lambda strain, stress: (lambda E: _out(E_GPa=E / 1000, E=E * 1e6, intercept_MPa=0.0, r_squared=1.0))(stress[-1] / strain[-1])),
     random_inputs=_rand_elastic,
     templates=["Young's modulus from data: strain = [0, 0.001, 0.002, 0.003], stress = [0, 205, 411, 618]"],
     examples=[{"inputs": {"strain": [0, 0.001, 0.002], "stress": [0, 200, 400]}, "outputs": {"E_GPa": 200.0, "intercept_MPa": 0.0}}],
     questions=[{"q": "Young's modulus from data: strain = [0, 0.001, 0.002], stress = [0, 200, 400]", "outputs": {"E_GPa": 200.0}},
                {"q": "Young's modulus from data: strain = [0, 0.001, 0.002], stress = [0, 200]", "outcome": "INVALID_INPUT"}])


def offset_yield(strain, stress, E, offset=0.002):
    if len(strain) != len(stress) or len(strain) < 3:
        raise ValueError("strain and stress must be lists of the same length (at least 3 points)")
    if any(b <= a for a, b in zip(strain, strain[1:])):
        raise ValueError("strains must increase strictly")
    _pos("Young's modulus", E)
    if not 0 < offset < 0.05:
        raise ValueError("the offset must lie between 0 and 0.05")
    Em = E / 1e6                                                # MPa per unit strain
    g = [s - Em * (e - offset) for e, s in zip(strain, stress)]
    if g[0] <= 0:
        raise ValueError("the curve must start above the offset line")
    for i in range(len(g) - 1):
        if g[i] > 0 >= g[i + 1]:
            t = g[i] / (g[i] - g[i + 1])
            ey = strain[i] + t * (strain[i + 1] - strain[i])
            sy = stress[i] + t * (stress[i + 1] - stress[i])
            return _out(yield_MPa=sy, yield_strain=ey)
    raise ValueError("the curve never crosses the offset line: the data end before yielding")


def ref_offset_yield(strain, stress, E, offset=0.002):
    import numpy as np
    from scipy.interpolate import interp1d
    from scipy.optimize import brentq
    f = interp1d(strain, stress, kind="linear")
    g = lambda e: float(f(e)) - E / 1e6 * (e - offset)        # noqa: E731
    gs = np.array([g(e) for e in strain])
    i = int(np.argmax((gs[:-1] > 0) & (gs[1:] <= 0)))
    ey = brentq(g, strain[i], strain[i + 1], xtol=1e-16, rtol=8.9e-16)
    return {"yield_MPa": float(f(ey)), "yield_strain": ey}


def _rand_curve(rng):
    E = rng.uniform(60e3, 250e3)                                 # MPa
    sy = rng.uniform(150, 900)
    n = rng.randint(12, 30)
    eps = sorted({round(rng.uniform(0, 0.03), 6) for _ in range(n)} | {0.0})
    sig = [round(sy * math.tanh(E * e / sy) * (1 + 2 * e), 3) for e in eps]
    return {"strain": eps, "stress": sig, "E": E * 1e6}


_add("offset_yield_strength",
     name="0.2 % offset yield strength from a stress-strain curve", topic="mechanical properties / tensile test",
     purpose="The yield strength where the stress-strain curve (piecewise linear through the data) meets a line of slope E offset by 0.2 % strain (or another offset).",
     inputs=[_vec("strain", "engineering strains, increasing", lo=3), _vec("stress", "engineering stresses in MPa", lo=3),
             _phys("E", "elastic_modulus", "Young's modulus"), _numin("offset", "strain offset (default 0.002)", required=False)],
     outputs=[_o("yield_MPa", "number", "offset yield strength in MPa"), _o("yield_strain", "number", "strain at the intersection")],
     primary="yield_MPa", conditions=["strains strictly increasing", "the curve starts above the offset line and crosses it"],
     method="first crossing of the data polyline with sigma = E (epsilon - offset), solved on that segment",
     latex=r"\sigma(\varepsilon_y) = E(\varepsilon_y - 0.002)",
     run=offset_yield,
     parse=lambda q: _pairs(q, r"offset yield strength|0\.2 ?% offset yield", {"strain": "strain", "stress": "stress", "E": "E", "offset": "offset"},
                            phys={"E"}, vectors={"strain", "stress"}),
     reference=ref_offset_yield,
     wrong=_safe(lambda strain, stress, E, offset=0.002: offset_yield(strain, stress, E, offset / 10)),
     random_inputs=_rand_curve,
     templates=["Offset yield strength: strain = [0, 0.002, 0.004, 0.006, 0.008], stress = [0, 400, 520, 560, 580], E = 200 GPa"],
     examples=[{"inputs": {"strain": [0, 0.002, 0.004, 0.006, 0.008], "stress": [0, 400, 520, 560, 580], "E": 200e9},
                "outputs": {"yield_MPa": 520 + 40 / 3}}],
     questions=[{"q": "Offset yield strength: strain = [0, 0.002, 0.004, 0.006, 0.008], stress = [0, 400, 520, 560, 580], E = 200 GPa",
                 "outputs": {"yield_MPa": 520 + 40 / 3}},
                {"q": "Offset yield strength: strain = [0, 0.001, 0.002], stress = [0, 200, 400], E = 200 GPa", "outcome": "INVALID_INPUT"}])


def tensile_toughness(strain, stress):
    if len(strain) != len(stress) or len(strain) < 2:
        raise ValueError("strain and stress must be lists of the same length (at least 2 points)")
    if any(b < a for a, b in zip(strain, strain[1:])):
        raise ValueError("strains must not decrease")
    area = sum((strain[i + 1] - strain[i]) * (stress[i + 1] + stress[i]) / 2 for i in range(len(strain) - 1))
    return _out(toughness_MJ_m3=area, toughness=area * 1e6, uts_MPa=max(stress), fracture_strain=strain[-1])


def ref_tensile_toughness(strain, stress):
    import numpy as np
    from scipy.integrate import trapezoid
    return {"toughness_MJ_m3": float(trapezoid(stress, strain)), "uts_MPa": float(np.max(stress)), "fracture_strain": float(strain[-1])}


def _rand_toughness(rng):
    x = _rand_curve(rng)
    return {"strain": x["strain"], "stress": x["stress"]}


_add("tensile_toughness",
     name="Toughness (area under the stress-strain curve)", topic="mechanical properties / tensile test",
     purpose="The energy absorbed per unit volume up to fracture (trapezoidal area under the stress-strain data), the tensile strength and the fracture strain.",
     inputs=[_vec("strain", "engineering strains, non-decreasing"), _vec("stress", "engineering stresses in MPa")],
     outputs=[_o("toughness_MJ_m3", "number", "toughness in MJ/m^3"), _o("toughness", "energy_density", "toughness"),
              _o("uts_MPa", "number", "tensile strength (largest stress) in MPa"), _o("fracture_strain", "number", "strain at the last point")],
     primary="toughness_MJ_m3", conditions=["equal lengths", "strains non-decreasing", "stress in MPa"],
     method="trapezoidal rule over the data points", latex=r"U_T = \int_0^{\varepsilon_f}\sigma\,d\varepsilon",
     run=tensile_toughness,
     parse=lambda q: _pairs(q, r"tensile toughness|toughness from (?:a )?stress[- ]strain curve", {"strain": "strain", "stress": "stress"},
                            vectors={"strain", "stress"}),
     reference=ref_tensile_toughness,
     wrong=_safe(lambda strain, stress: _out(toughness_MJ_m3=max(stress) * strain[-1], toughness=max(stress) * strain[-1] * 1e6,
                                             uts_MPa=max(stress), fracture_strain=strain[-1])),
     random_inputs=_rand_toughness,
     templates=["Tensile toughness: strain = [0, 0.002, 0.05, 0.1, 0.15], stress = [0, 400, 500, 520, 480]"],
     examples=[{"inputs": {"strain": [0, 0.002, 0.05, 0.1, 0.15], "stress": [0, 400, 500, 520, 480]},
                "outputs": {"toughness_MJ_m3": 0.002 * 200 + 0.048 * 450 + 0.05 * 510 + 0.05 * 500, "uts_MPa": 520.0}}],
     questions=[{"q": "Tensile toughness: strain = [0, 0.002, 0.05, 0.1, 0.15], stress = [0, 400, 500, 520, 480]",
                 "outputs": {"toughness_MJ_m3": 0.002 * 200 + 0.048 * 450 + 0.05 * 510 + 0.05 * 500}},
                {"q": "Tensile toughness: strain = [0, 0.1, 0.05], stress = [0, 400, 500]", "outcome": "INVALID_INPUT"}])


def resilience(sy, E):
    _pos("the yield strength and Young's modulus", sy, E)
    U = sy * sy / (2 * E)
    return _out(Ur=U, Ur_MJ_m3=U / 1e6, yield_strain=sy / E)


def ref_resilience(sy, E):
    from scipy.integrate import quad
    U, _ = quad(lambda e: E * e, 0, sy / E, epsabs=0, epsrel=2e-14)       # area under the linear elastic curve
    return {"Ur_MJ_m3": U / 1e6, "yield_strain": sy / E}


_add("modulus_of_resilience",
     name="Modulus of resilience", topic="mechanical properties / tensile test",
     purpose="The elastic strain energy per unit volume stored up to yielding: U_r = sigma_y^2/(2E).",
     inputs=[_phys("sy", "stress", "yield strength"), _phys("E", "elastic_modulus", "Young's modulus")],
     outputs=[_o("Ur", "energy_density", "modulus of resilience"), _o("Ur_MJ_m3", "number", "modulus of resilience in MJ/m^3 (= MPa)"),
              _o("yield_strain", "number", "strain at yield")],
     primary="Ur_MJ_m3", conditions=["sy, E > 0", "linear elastic up to yield"],
     method="area under the linear elastic part of the curve", latex=r"U_r = \frac{\sigma_y^2}{2E}",
     run=resilience,
     parse=lambda q: _pairs(q, r"modulus of resilience|resilience", {"sy": "sy", "yield": "sy", "E": "E"}, phys={"sy", "E"}),
     reference=ref_resilience,
     wrong=_safe(lambda sy, E: _out(Ur=sy * sy / E, Ur_MJ_m3=sy * sy / E / 1e6, yield_strain=sy / E)),
     random_inputs=lambda rng: {"sy": rng.uniform(50e6, 1500e6), "E": rng.uniform(40e9, 400e9)},
     templates=["Modulus of resilience: sy = 415 MPa, E = 207 GPa"],
     examples=[{"inputs": {"sy": 415e6, "E": 207e9}, "outputs": {"Ur_MJ_m3": 415 ** 2 / (2 * 207e3)}}],
     questions=[{"q": "Modulus of resilience: sy = 415 MPa, E = 207 GPa", "outputs": {"Ur_MJ_m3": 415 ** 2 / (2 * 207e3)}},
                {"q": "Modulus of resilience: sy = 415 MPa", "outcome": "NEEDS_INPUT"}])


def hollomon_fit(true_strain, true_stress, strain_target=None):
    if len(true_strain) != len(true_stress):
        raise ValueError("the two lists must have the same length")
    for v in list(true_strain) + list(true_stress):
        _pos("every true strain and stress (plastic region)", v)
    b0, b1, r2 = _linfit([math.log(e) for e in true_strain], [math.log(s) for s in true_stress])
    out = dict(n=b1, K_MPa=math.exp(b0), r_squared=r2)
    if strain_target is not None:
        _pos("the target strain", strain_target)
        out["stress_target_MPa"] = math.exp(b0) * strain_target ** b1
    return _out(**out)


def ref_hollomon_fit(true_strain, true_stress, strain_target=None):
    import numpy as np
    from scipy.stats import linregress
    r = linregress(np.log(true_strain), np.log(true_stress))
    out = {"n": r.slope, "K_MPa": math.exp(r.intercept)}
    if strain_target is not None:
        out["stress_target_MPa"] = math.exp(r.intercept + r.slope * math.log(strain_target))
    return out


def _rand_hollomon(rng):
    n, K = rng.uniform(0.05, 0.5), rng.uniform(300, 1500)
    m = rng.randint(2, 8)
    eps = sorted({round(rng.uniform(0.01, 0.4), 5) for _ in range(m)})
    if len(eps) < 2:
        eps = [0.05, 0.2]
    x = {"true_strain": eps, "true_stress": [round(K * e ** n * math.exp(rng.gauss(0, 0.01)), 4) for e in eps]}
    if rng.random() < 0.5:
        x["strain_target"] = rng.uniform(0.02, 0.5)
    return x


_add("strain_hardening_exponent",
     name="Strain-hardening exponent and strength coefficient (Hollomon)", topic="mechanical properties / plasticity",
     purpose="The strain-hardening exponent n and strength coefficient K of sigma_T = K epsilon_T^n from true stress-strain points of the plastic region, and the stress at another strain.",
     inputs=[_vec("true_strain", "true plastic strains"), _vec("true_stress", "true stresses in MPa"),
             _numin("strain_target", "true strain at which the stress is wanted (optional)", required=False)],
     outputs=[_o("n", "number", "strain-hardening exponent"), _o("K_MPa", "number", "strength coefficient in MPa"),
              _o("r_squared", "number", "r^2 of the log-log fit"), _o("stress_target_MPa", "number", "true stress at the target strain in MPa")],
     primary="n", conditions=["equal lengths, at least two distinct strains, all values > 0"],
     method="least-squares line of ln sigma_T against ln epsilon_T", latex=r"\ln\sigma_T = \ln K + n\ln\varepsilon_T",
     run=hollomon_fit,
     parse=lambda q: _pairs(q, r"strain[- ]hardening exponent|hollomon fit", {"true_strain": "true_strain", "true_stress": "true_stress",
                                                                               "strain_target": "strain_target"}, vectors={"true_strain", "true_stress"}),
     reference=ref_hollomon_fit,
     wrong=_safe(lambda true_strain, true_stress, strain_target=None: (lambda o: dict(o, n=o["n"] * 2.302585092994046))(
         hollomon_fit(true_strain, true_stress, strain_target))),
     random_inputs=_rand_hollomon,
     templates=["Strain hardening exponent: true_strain = [0.1, 0.2], true_stress = [415, 470]"],
     examples=[{"inputs": {"true_strain": [0.1, 0.2], "true_stress": [415, 470]}, "outputs": {"n": math.log(470 / 415) / math.log(2)}}],
     questions=[{"q": "Strain hardening exponent: true_strain = [0.1, 0.2], true_stress = [415, 470]", "outputs": {"n": math.log(470 / 415) / math.log(2)}},
                {"q": "Strain hardening exponent: true_strain = [0.1, -0.2], true_stress = [415, 470]", "outcome": "INVALID_INPUT"}])


def hall_petch_fit(d_um, yield_MPa, d_target_um=None):
    if len(d_um) != len(yield_MPa):
        raise ValueError("the two lists must have the same length")
    for v in d_um:
        _pos("every grain diameter", v)
    b0, b1, r2 = _linfit([v ** -0.5 for v in d_um], list(yield_MPa))
    out = dict(sigma0_MPa=b0, k_MPa_sqrt_um=b1, k_MPa_sqrt_m=b1 * 1e-3, r_squared=r2)
    if d_target_um is not None:
        _pos("the target grain diameter", d_target_um)
        out["yield_target_MPa"] = b0 + b1 * d_target_um ** -0.5
    return _out(**out)


def ref_hall_petch_fit(d_um, yield_MPa, d_target_um=None):
    import numpy as np
    k, s0 = np.polyfit(np.array(d_um) ** -0.5, np.array(yield_MPa, float), 1)
    out = {"sigma0_MPa": float(s0), "k_MPa_sqrt_um": float(k)}
    if d_target_um is not None:
        out["yield_target_MPa"] = float(s0 + k * d_target_um ** -0.5)
    return out


def _rand_hall_petch(rng):
    s0, k = rng.uniform(20, 300), rng.uniform(100, 800)
    m = rng.randint(2, 6)
    d = sorted({round(_lu(rng, 0.5, 200), 3) for _ in range(m)})
    if len(d) < 2:
        d = [1.0, 50.0]
    x = {"d_um": d, "yield_MPa": [round(s0 + k / math.sqrt(v) + rng.gauss(0, 3), 3) for v in d]}
    if rng.random() < 0.5:
        x["d_target_um"] = _lu(rng, 0.5, 200)
    return x


_add("hall_petch",
     name="Hall-Petch grain-size strengthening", topic="mechanical properties / strengthening mechanisms",
     purpose="sigma_0 and k of sigma_y = sigma_0 + k d^(-1/2) from yield strengths at two or more grain sizes, and the yield strength at another grain size.",
     inputs=[_vec("d_um", "grain diameters in micrometres"), _vec("yield_MPa", "yield strengths in MPa"),
             _numin("d_target_um", "grain diameter in micrometres at which the yield strength is wanted (optional)", required=False)],
     outputs=[_o("sigma0_MPa", "number", "friction stress sigma_0 in MPa"), _o("k_MPa_sqrt_um", "number", "k in MPa um^0.5"),
              _o("k_MPa_sqrt_m", "number", "k in MPa m^0.5"), _o("r_squared", "number", "r^2 of the fit"),
              _o("yield_target_MPa", "number", "yield strength at the target grain size in MPa")],
     primary="k_MPa_sqrt_um", conditions=["equal lengths, at least two distinct grain sizes"],
     method="least-squares line of sigma_y against d^(-1/2)", latex=r"\sigma_y = \sigma_0 + k_y d^{-1/2}",
     run=hall_petch_fit,
     parse=lambda q: _pairs(q, r"hall[- ]petch(?: fit)?", {"d_um": "d_um", "d": "d_um", "yield_MPa": "yield_MPa", "sy": "yield_MPa",
                                                           "d_target_um": "d_target_um", "d_target": "d_target_um"},
                            vectors={"d_um", "yield_MPa"}, num_units={"d_target_um": ["um", "µm", "micrometres", "micrometers"]}),
     reference=ref_hall_petch_fit,
     wrong=_safe(lambda d_um, yield_MPa, d_target_um=None: (lambda b: _out(sigma0_MPa=b[0], k_MPa_sqrt_um=b[1], k_MPa_sqrt_m=b[1] * 1e-3, r_squared=b[2],
                                                                           **({} if d_target_um is None else {"yield_target_MPa": b[0] + b[1] / d_target_um})))(
         _linfit([1 / v for v in d_um], list(yield_MPa)))),
     random_inputs=_rand_hall_petch,
     templates=["Hall-Petch: d_um = [100, 4], yield_MPa = [150, 350], d_target_um = 25"],
     examples=[{"inputs": {"d_um": [100, 4], "yield_MPa": [150, 350]}, "outputs": {"k_MPa_sqrt_um": 200 / 0.4, "sigma0_MPa": 100.0}}],
     questions=[{"q": "Hall-Petch: d_um = [100, 4], yield_MPa = [150, 350], d_target_um = 25", "outputs": {"yield_target_MPa": 200.0}},
                {"q": "Hall-Petch: d_um = [100, 100], yield_MPa = [150, 350]", "outcome": "INVALID_INPUT"}])


def thermal_stress(E, alpha, dT, constraint="uniaxial", nu=None):
    _pos("E and alpha", E, alpha)
    f = 1.0
    if constraint != "uniaxial":
        if nu is None:
            raise ValueError("Poisson's ratio is needed for a biaxial or triaxial constraint")
        if not -1 < nu < 0.5:
            raise ValueError("Poisson's ratio must lie between -1 and 0.5")
        f = 1 / (1 - nu) if constraint == "biaxial" else 1 / (1 - 2 * nu)
    s = -E * alpha * dT * f
    return _out(stress=s, stress_MPa=s / 1e6)


def ref_thermal_stress(E, alpha, dT, constraint="uniaxial", nu=None):
    """The isotropic thermoelastic law eps_i = (sigma_i - nu (sigma_j + sigma_k))/E + alpha dT for the three normal
    directions, with eps = 0 where the body is held and sigma = 0 where it is free: a 3x3 linear system (numpy)."""
    import numpy as np
    v = 0.3 if nu is None else nu
    held = {"uniaxial": [2], "biaxial": [0, 1], "triaxial": [0, 1, 2]}[constraint]
    A, b = np.zeros((3, 3)), np.zeros(3)
    for i in range(3):
        if i in held:
            A[i] = [-v / E] * 3
            A[i, i] = 1 / E
            b[i] = -alpha * dT
        else:
            A[i, i] = 1.0
    sig = np.linalg.solve(A, b)
    return {"stress_MPa": float(sig[held[0]]) / 1e6}


_add("thermal_stress",
     name="Thermal stress in a constrained body", topic="mechanical properties / thermal stresses",
     purpose="The stress from a temperature change in a body held in one direction (rod between walls), two (thin film or plate) or three (fully constrained).",
     inputs=[_phys("E", "elastic_modulus", "Young's modulus"), _phys("alpha", "expansion_coefficient", "linear thermal expansion coefficient"),
             _phys("dT", "temperature_difference", "temperature change (positive = heating)"),
             _choice("constraint", ["uniaxial", "biaxial", "triaxial"], "constraint (uniaxial, biaxial, triaxial; default uniaxial)", required=False),
             _numin("nu", "Poisson's ratio (needed for biaxial and triaxial)", required=False)],
     outputs=[_o("stress", "stress", "thermal stress (negative = compression)"), _o("stress_MPa", "number", "thermal stress in MPa")],
     primary="stress_MPa", conditions=["E, alpha > 0", "-1 < nu < 0.5 when needed"],
     method="sigma = -E alpha dT (uniaxial), / (1 - nu) (biaxial), / (1 - 2 nu) (triaxial)",
     latex=r"\sigma = -E\alpha\Delta T,\quad -\frac{E\alpha\Delta T}{1-\nu},\quad -\frac{E\alpha\Delta T}{1-2\nu}",
     run=thermal_stress,
     parse=lambda q: _pairs(q, r"thermal stress", {"E": "E", "alpha": "alpha", "dT": "dT", "constraint": "constraint", "nu": "nu"},
                            phys={"E", "alpha", "dT"}, words={"constraint"}),
     reference=ref_thermal_stress,
     wrong=_safe(lambda E, alpha, dT, constraint="uniaxial", nu=None: (lambda s: _out(stress=s, stress_MPa=s / 1e6))(
         -E * alpha * dT * (1 if constraint == "uniaxial" else (1 / (1 + nu) if constraint == "biaxial" else 1 / (1 + 2 * nu))) * (0.5 if constraint == "uniaxial" else 1))),
     random_inputs=lambda rng: (lambda c: {"E": rng.uniform(40e9, 400e9), "alpha": rng.uniform(2e-6, 25e-6), "dT": rng.uniform(-300, 300),
                                           "constraint": c, **({} if c == "uniaxial" and rng.random() < 0.5 else {"nu": rng.uniform(0.15, 0.45)})})(
         rng.choice(["uniaxial", "biaxial", "triaxial"])),
     templates=["Thermal stress: E = 97 GPa, alpha = 20e-6 1/K, dT = 60 K", "Thermal stress: E = 70 GPa, alpha = 23e-6 1/K, dT = -100 K, constraint = biaxial, nu = 0.33"],
     examples=[{"inputs": {"E": 97e9, "alpha": 20e-6, "dT": 60.0}, "outputs": {"stress_MPa": -116.4}},
               {"inputs": {"E": 70e9, "alpha": 23e-6, "dT": -100.0, "constraint": "biaxial", "nu": 0.33}, "outputs": {"stress_MPa": 70e3 * 23e-6 * 100 / 0.67}}],
     questions=[{"q": "Thermal stress: E = 97 GPa, alpha = 20e-6 1/K, dT = 60 K", "outputs": {"stress_MPa": -116.4}},
                {"q": "Thermal stress: E = 70 GPa, alpha = 23e-6 1/K, dT = -100 K, constraint = biaxial", "outcome": "INVALID_INPUT"}])


def rule_of_mixtures(Ef, Em, Vf):
    _pos("the moduli", Ef, Em)
    _frac01("the fibre volume fraction", Vf)
    El = Ef * Vf + Em * (1 - Vf)
    Et = Ef * Em / (Vf * Em + (1 - Vf) * Ef)
    return _out(E_long_GPa=El / 1e9, E_trans_GPa=Et / 1e9, fiber_load_fraction=Ef * Vf / El, E_long=El, E_trans=Et)


def ref_rule_of_mixtures(Ef, Em, Vf):
    """Two bars of unit length: side by side (equal strain) and end to end (equal stress), each solved as a small
    stiffness-matrix assembly with numpy (nodal equilibrium K u = F)."""
    import numpy as np
    kf, km = Ef * Vf, Em * (1 - Vf)                              # parallel: areas Vf and 1 - Vf, length 1
    K = np.array([[kf + km]])
    u = np.linalg.solve(K, np.array([1.0]))[0]
    El = 1.0 / u                                                 # stress 1 over strain u (area 1, length 1)
    load_f = kf * u
    k1, k2 = Ef / Vf, Em / (1 - Vf)                              # series: lengths Vf and 1 - Vf, area 1
    Ks = np.array([[k1 + k2, -k2], [-k2, k2]])                   # nodes 1 and 2 (node 0 fixed)
    us = np.linalg.solve(Ks, np.array([0.0, 1.0]))
    Et = 1.0 / us[1]
    return {"E_long_GPa": El / 1e9, "E_trans_GPa": Et / 1e9, "fiber_load_fraction": load_f}


_add("composite_rule_of_mixtures",
     name="Elastic moduli of a continuous-fibre composite (rule of mixtures)", topic="composites / elastic moduli",
     purpose="The longitudinal (isostrain) and transverse (isostress) moduli of an aligned continuous-fibre composite and the fraction of the load carried by the fibres.",
     inputs=[_phys("Ef", "elastic_modulus", "fibre modulus"), _phys("Em", "elastic_modulus", "matrix modulus"), _numin("Vf", "fibre volume fraction (0 to 1)")],
     outputs=[_o("E_long_GPa", "number", "longitudinal modulus in GPa"), _o("E_trans_GPa", "number", "transverse modulus in GPa"),
              _o("fiber_load_fraction", "number", "fraction of a longitudinal load carried by the fibres"),
              _o("E_long", "elastic_modulus", "longitudinal modulus"), _o("E_trans", "elastic_modulus", "transverse modulus")],
     primary="E_long_GPa", conditions=["moduli > 0", "0 < Vf < 1", "perfect bonding"],
     method="E_l = Ef Vf + Em Vm (Voigt), 1/E_t = Vf/Ef + Vm/Em (Reuss), Ff/F = Ef Vf / E_l",
     latex=r"E_{cl} = E_mV_m + E_fV_f,\quad \frac{1}{E_{ct}} = \frac{V_m}{E_m}+\frac{V_f}{E_f}",
     run=rule_of_mixtures,
     parse=lambda q: _pairs(q, r"rule of mixtures|composite moduli", {"Ef": "Ef", "Em": "Em", "Vf": "Vf"}, phys={"Ef", "Em"}),
     reference=ref_rule_of_mixtures,
     wrong=_safe(lambda Ef, Em, Vf: (lambda El: _out(E_long_GPa=El / 1e9, E_trans_GPa=(Ef * (1 - Vf) + Em * Vf) / 1e9, fiber_load_fraction=Vf,
                                                     E_long=El, E_trans=Ef * (1 - Vf) + Em * Vf))(Ef * (1 - Vf) + Em * Vf)),
     random_inputs=lambda rng: {"Ef": rng.uniform(50e9, 500e9), "Em": rng.uniform(1e9, 80e9), "Vf": rng.uniform(0.1, 0.7)},
     templates=["Rule of mixtures: Ef = 69 GPa, Em = 3.4 GPa, Vf = 0.4"],
     examples=[{"inputs": {"Ef": 69e9, "Em": 3.4e9, "Vf": 0.4}, "outputs": {"E_long_GPa": 69 * 0.4 + 3.4 * 0.6, "E_trans_GPa": 69 * 3.4 / (0.4 * 3.4 + 0.6 * 69)}}],
     questions=[{"q": "Rule of mixtures: Ef = 69 GPa, Em = 3.4 GPa, Vf = 0.4", "outputs": {"E_long_GPa": 69 * 0.4 + 3.4 * 0.6}},
                {"q": "Rule of mixtures: Ef = 69 GPa, Em = 3.4 GPa, Vf = 1.4", "outcome": "INVALID_INPUT"}])


def critical_fiber_length(sigma_f, d, tau_c):
    _pos("the fibre strength, diameter and bond strength", sigma_f, d, tau_c)
    lc = sigma_f * d / (2 * tau_c)
    return _out(lc=lc, lc_mm=lc * 1e3, aspect_ratio=lc / d)


def ref_critical_fiber_length(sigma_f, d, tau_c):
    """Shear-lag force balance on a fibre: the axial force sigma(x) pi d^2/4 built up by the interface shear tau_c pi d
    over the length x from the end (integrated with scipy quadrature); the load reaches the fibre strength at mid-length
    when the fibre is lc long (root by brentq)."""
    from scipy.integrate import quad
    from scipy.optimize import brentq
    area = math.pi * d * d / 4
    force = lambda x: quad(lambda s: tau_c * math.pi * d, 0, x, epsabs=0, epsrel=2e-14)[0]      # noqa: E731
    half = brentq(lambda x: force(x) / area - sigma_f, 0, 1e6 * d, xtol=1e-18, rtol=8.9e-16)
    return {"lc_mm": 2 * half * 1e3, "aspect_ratio": 2 * half / d}


_add("critical_fiber_length",
     name="Critical fibre length", topic="composites / fibre reinforcement",
     purpose="The critical length l_c = sigma_f d / (2 tau_c) a fibre needs to be loaded to its strength through the fibre-matrix interface.",
     inputs=[_phys("sigma_f", "stress", "fibre tensile strength"), _phys("d", "length", "fibre diameter"),
             _phys("tau_c", "stress", "fibre-matrix bond strength (or matrix shear yield strength)")],
     outputs=[_o("lc", "length", "critical fibre length"), _o("lc_mm", "number", "critical length in mm"), _o("aspect_ratio", "number", "l_c/d")],
     primary="lc_mm", conditions=["all inputs > 0"],
     method="l_c = sigma_f d / (2 tau_c)", latex=r"l_c = \frac{\sigma_f^* d}{2\tau_c}",
     run=critical_fiber_length,
     parse=lambda q: _pairs(q, r"critical fib(?:er|re) length", {"sigma_f": "sigma_f", "d": "d", "tau_c": "tau_c"}, phys={"sigma_f", "d", "tau_c"}),
     reference=ref_critical_fiber_length,
     wrong=_safe(lambda sigma_f, d, tau_c: _out(lc=sigma_f * d / tau_c, lc_mm=sigma_f * d / tau_c * 1e3, aspect_ratio=sigma_f / tau_c)),
     random_inputs=lambda rng: {"sigma_f": rng.uniform(1e9, 6e9), "d": rng.uniform(5e-6, 20e-6), "tau_c": rng.uniform(5e6, 80e6)},
     templates=["Critical fiber length: sigma_f = 3.5 GPa, d = 10 um, tau_c = 20 MPa"],
     examples=[{"inputs": {"sigma_f": 3.5e9, "d": 10e-6, "tau_c": 20e6}, "outputs": {"lc_mm": 0.875}}],
     questions=[{"q": "Critical fiber length: sigma_f = 3.5 GPa, d = 10 um, tau_c = 20 MPa", "outputs": {"lc_mm": 0.875}},
                {"q": "Critical fibre length: sigma_f = 3.5 GPa, d = 10 um", "outcome": "NEEDS_INPUT"}])

# ================================================================================================ fracture and failure
def griffith_stress(E, gamma_s, a):
    _pos("E, the surface energy and the crack length", E, gamma_s, a)
    s = math.sqrt(2 * E * gamma_s / (math.pi * a))
    return _out(sigma_c=s, sigma_c_MPa=s / 1e6)


_GRIFFITH: dict = {}


def ref_griffith_stress(E, gamma_s, a):
    """Griffith's energy balance per unit thickness of a plate with a crack of length 2a, U(a) = 4 a gamma -
    pi sigma^2 a^2 / E; the critical stress makes dU/da = 0 (sympy differentiation and solution)."""
    if "f" not in _GRIFFITH:
        import sympy as sp
        s, A_, E_, g_ = sp.symbols("sigma a E gamma", positive=True)
        U = 4 * A_ * g_ - sp.pi * s ** 2 * A_ ** 2 / E_
        sol = sp.solve(sp.Eq(sp.diff(U, A_), 0), s)
        _GRIFFITH["f"] = sp.lambdify((E_, g_, A_), sol[0], "math")
    return {"sigma_c_MPa": _GRIFFITH["f"](E, gamma_s, a) / 1e6}


_add("griffith_critical_stress",
     name="Griffith critical stress of a brittle solid", topic="fracture / fracture mechanics",
     purpose="The stress at which a crack of length 2a (internal) or a (surface) propagates in a brittle solid: sigma_c = sqrt(2 E gamma_s / (pi a)).",
     inputs=[_phys("E", "elastic_modulus", "Young's modulus"), _phys("gamma_s", "surface_energy", "specific surface energy"),
             _phys("a", "length", "half-length of an internal crack (or length of a surface crack)")],
     outputs=[_o("sigma_c", "stress", "critical stress"), _o("sigma_c_MPa", "number", "critical stress in MPa")],
     primary="sigma_c_MPa", conditions=["all inputs > 0", "ideally brittle (no plastic work), plane stress"],
     method="Griffith energy balance", latex=r"\sigma_c = \sqrt{\frac{2E\gamma_s}{\pi a}}",
     run=griffith_stress,
     parse=lambda q: _pairs(q, r"griffith critical stress|griffith stress", {"E": "E", "gamma_s": "gamma_s", "gamma": "gamma_s", "a": "a"},
                            phys={"E", "gamma_s", "a"}),
     reference=ref_griffith_stress,
     wrong=_safe(lambda E, gamma_s, a: _out(sigma_c=math.sqrt(E * gamma_s / (math.pi * a)), sigma_c_MPa=math.sqrt(E * gamma_s / (math.pi * a)) / 1e6)),
     random_inputs=lambda rng: {"E": rng.uniform(50e9, 450e9), "gamma_s": rng.uniform(0.2, 3), "a": _lu(rng, 1e-7, 1e-3)},
     templates=["Griffith critical stress: E = 69 GPa, gamma_s = 0.3 J/m^2, a = 0.5 um"],
     examples=[{"inputs": {"E": 69e9, "gamma_s": 0.3, "a": 0.5e-6}, "outputs": {"sigma_c_MPa": math.sqrt(2 * 69e9 * 0.3 / (math.pi * 0.5e-6)) / 1e6}}],
     questions=[{"q": "Griffith critical stress: E = 69 GPa, gamma_s = 0.3 J/m^2, a = 0.5 um",
                 "outputs": {"sigma_c_MPa": math.sqrt(2 * 69e9 * 0.3 / (math.pi * 0.5e-6)) / 1e6}},
                {"q": "Griffith critical stress: E = 69 GPa, gamma_s = 0.3 J/m^2", "outcome": "NEEDS_INPUT"}])


def _westergaard_K(a):
    """Stress intensity of a crack of half-length a in an infinite plate under unit remote stress, from Westergaard's
    exact field sigma_yy(x) = x / sqrt(x^2 - a^2) ahead of the tip: K = sqrt(2 pi r) sigma_yy(a + r) as r -> 0
    (mpmath, 60 digits, r = 1e-40 a)."""
    import mpmath
    with mpmath.workdps(60):
        A = mpmath.mpf(a)
        r = A * mpmath.mpf("1e-40")
        x = A + r
        return float(mpmath.sqrt(2 * mpmath.pi * r) * x / mpmath.sqrt(x * x - A * A))


def fracture_stress(K_IC, a, Y=1.0):
    _pos("K_IC, the crack length and Y", K_IC, a, Y)
    s = K_IC / (Y * math.sqrt(math.pi * a))
    return _out(sigma_c=s, sigma_c_MPa=s / 1e6)


def critical_crack(K_IC, sigma, Y=1.0):
    _pos("K_IC, the stress and Y", K_IC, sigma, Y)
    a = (K_IC / (Y * sigma)) ** 2 / math.pi
    return _out(a_c=a, a_c_mm=a * 1e3)


_YIN = _numin("Y", "geometry factor Y (default 1)", required=False)
_KNAMES = {"K_IC": "K_IC", "KIC": "K_IC", "Kc": "K_IC", "a": "a", "Y": "Y", "sigma": "sigma"}

_add("fracture_critical_stress",
     name="Fracture stress from the fracture toughness", topic="fracture / fracture mechanics",
     purpose="The stress at which a crack of length a fails in a material of plane-strain fracture toughness K_IC: sigma_c = K_IC / (Y sqrt(pi a)).",
     inputs=[_phys("K_IC", "fracture_toughness", "plane-strain fracture toughness"), _phys("a", "length", "crack length (surface) or half-length (internal)"), _YIN],
     outputs=[_o("sigma_c", "stress", "critical stress"), _o("sigma_c_MPa", "number", "critical stress in MPa")],
     primary="sigma_c_MPa", conditions=["all inputs > 0", "linear-elastic fracture mechanics"],
     method="K_I = Y sigma sqrt(pi a) set equal to K_IC", latex=r"\sigma_c = \frac{K_{Ic}}{Y\sqrt{\pi a}}",
     run=fracture_stress,
     parse=lambda q: _pairs(q, r"fracture stress|critical stress from fracture toughness", _KNAMES, phys={"K_IC", "a"}),
     reference=lambda K_IC, a, Y=1.0: {"sigma_c_MPa": K_IC / (Y * _westergaard_K(a)) / 1e6},
     wrong=_safe(lambda K_IC, a, Y=1.0: _out(sigma_c=K_IC / (Y * math.sqrt(a)), sigma_c_MPa=K_IC / (Y * math.sqrt(a)) / 1e6)),
     random_inputs=lambda rng: {"K_IC": rng.uniform(0.5e6, 120e6), "a": _lu(rng, 1e-6, 2e-2), **({"Y": rng.uniform(0.8, 1.5)} if rng.random() < 0.6 else {})},
     templates=["Fracture stress: K_IC = 26 MPa*m^0.5, a = 2 mm, Y = 1.2"],
     examples=[{"inputs": {"K_IC": 26e6, "a": 2e-3, "Y": 1.2}, "outputs": {"sigma_c_MPa": 26 / (1.2 * math.sqrt(math.pi * 2e-3))}}],
     questions=[{"q": "Fracture stress: K_IC = 26 MPa*m^0.5, a = 2 mm, Y = 1.2", "outputs": {"sigma_c_MPa": 26 / (1.2 * math.sqrt(math.pi * 2e-3))}},
                {"q": "Fracture stress: K_IC = 26 MPa*m^0.5", "outcome": "NEEDS_INPUT"}])

_add("critical_crack_length",
     name="Critical crack length from the fracture toughness", topic="fracture / fracture mechanics",
     purpose="The largest crack a part can carry at stress sigma without fracture: a_c = (K_IC / (Y sigma))^2 / pi.",
     inputs=[_phys("K_IC", "fracture_toughness", "plane-strain fracture toughness"), _phys("sigma", "stress", "applied stress"), _YIN],
     outputs=[_o("a_c", "length", "critical crack length"), _o("a_c_mm", "number", "critical crack length in mm")],
     primary="a_c_mm", conditions=["all inputs > 0", "linear-elastic fracture mechanics"],
     method="K_I = Y sigma sqrt(pi a) set equal to K_IC", latex=r"a_c = \frac{1}{\pi}\left(\frac{K_{Ic}}{Y\sigma}\right)^2",
     run=critical_crack,
     parse=lambda q: _pairs(q, r"critical crack length", _KNAMES, phys={"K_IC", "sigma"}),
     reference=lambda K_IC, sigma, Y=1.0: {"a_c_mm": (K_IC / (Y * sigma * _westergaard_K(1.0))) ** 2 * 1e3},   # K grows as sqrt(a)
     wrong=_safe(lambda K_IC, sigma, Y=1.0: _out(a_c=(K_IC / (Y * sigma)) ** 2, a_c_mm=(K_IC / (Y * sigma)) ** 2 * 1e3)),
     random_inputs=lambda rng: {"K_IC": rng.uniform(0.5e6, 120e6), "sigma": rng.uniform(20e6, 1500e6), **({"Y": rng.uniform(0.8, 1.5)} if rng.random() < 0.6 else {})},
     templates=["Critical crack length: K_IC = 55 MPa*m^0.5, sigma = 1000 MPa, Y = 1"],
     examples=[{"inputs": {"K_IC": 55e6, "sigma": 1000e6, "Y": 1.0}, "outputs": {"a_c_mm": 0.055 ** 2 / math.pi * 1e3}}],
     questions=[{"q": "Critical crack length: K_IC = 55 MPa*m^0.5, sigma = 1000 MPa", "outputs": {"a_c_mm": 0.055 ** 2 / math.pi * 1e3}},
                {"q": "Critical crack length: K_IC = 55 MPa*m^0.5, sigma = -10 MPa", "outcome": "INVALID_INPUT"}])


def weibull_failure(sigma, sigma0, m, V_ratio=1.0):
    _pos("the stresses, the Weibull modulus and the volume ratio", sigma, sigma0, m, V_ratio)
    x = V_ratio * (sigma / sigma0) ** m
    return _out(P_failure=-math.expm1(-x), P_survival=math.exp(-x))


def ref_weibull_failure(sigma, sigma0, m, V_ratio=1.0):
    from scipy.stats import weibull_min
    dist = weibull_min(m, scale=sigma0 * V_ratio ** (-1 / m))
    return {"P_failure": float(dist.cdf(sigma)), "P_survival": float(dist.sf(sigma))}


def _rand_weibull(rng):
    m, s0 = rng.uniform(3, 25), rng.uniform(50e6, 800e6)
    V = rng.choice([1.0, rng.uniform(0.1, 20)])
    x = _lu(rng, 0.01, 3)
    return {"sigma": s0 * (x / V) ** (1 / m), "sigma0": s0, "m": m, "V_ratio": V}


_add("weibull_failure_probability",
     name="Weibull probability of failure of a brittle part", topic="fracture / statistics of brittle fracture",
     purpose="The probability that a brittle (ceramic) part fails at stress sigma, from the Weibull modulus m and characteristic strength sigma_0, with an optional volume ratio V/V0.",
     inputs=[_phys("sigma", "stress", "applied stress"), _phys("sigma0", "stress", "characteristic strength (63.2 % failure at V0)"),
             _numin("m", "Weibull modulus"), _numin("V_ratio", "stressed volume over the reference volume V/V0 (default 1)", required=False)],
     outputs=[_o("P_failure", "number", "probability of failure"), _o("P_survival", "number", "probability of survival")],
     primary="P_failure", conditions=["all inputs > 0"],
     method="P_f = 1 - exp(-(V/V0)(sigma/sigma0)^m)", latex=r"P_f = 1 - \exp\!\left[-\frac{V}{V_0}\left(\frac{\sigma}{\sigma_0}\right)^m\right]",
     run=weibull_failure,
     parse=lambda q: _pairs(q, r"weibull failure probability|weibull probability of failure", {"sigma": "sigma", "sigma0": "sigma0", "m": "m", "V_ratio": "V_ratio"},
                            phys={"sigma", "sigma0"}),
     reference=ref_weibull_failure,
     wrong=_safe(lambda sigma, sigma0, m, V_ratio=1.0: (lambda x: _out(P_failure=-math.expm1(-x), P_survival=math.exp(-x)))(V_ratio * (sigma / sigma0) * m)),
     random_inputs=_rand_weibull,
     templates=["Weibull failure probability: sigma = 300 MPa, sigma0 = 400 MPa, m = 10"],
     examples=[{"inputs": {"sigma": 300e6, "sigma0": 400e6, "m": 10.0}, "outputs": {"P_failure": 1 - math.exp(-(0.75 ** 10))}},
               {"inputs": {"sigma": 400e6, "sigma0": 400e6, "m": 5.0, "V_ratio": 2.0}, "outputs": {"P_failure": 1 - math.exp(-2)}}],
     questions=[{"q": "Weibull failure probability: sigma = 300 MPa, sigma0 = 400 MPa, m = 10", "outputs": {"P_failure": 1 - math.exp(-(0.75 ** 10))}},
                {"q": "Weibull failure probability: sigma = 300 MPa, sigma0 = 400 MPa, m = -2", "outcome": "INVALID_INPUT"}])


def weibull_stress(P_failure, sigma0, m, V_ratio=1.0):
    _pos("sigma0, m and the volume ratio", sigma0, m, V_ratio)
    _frac01("the failure probability", P_failure)
    s = sigma0 * (-math.log1p(-P_failure) / V_ratio) ** (1 / m)
    return _out(sigma=s, sigma_MPa=s / 1e6)


def ref_weibull_stress(P_failure, sigma0, m, V_ratio=1.0):
    from scipy.stats import weibull_min
    return {"sigma_MPa": float(weibull_min(m, scale=sigma0 * V_ratio ** (-1 / m)).ppf(P_failure)) / 1e6}


_add("weibull_design_stress",
     name="Design stress for a target failure probability (Weibull)", topic="fracture / statistics of brittle fracture",
     purpose="The stress at which a brittle part reaches a given failure probability, from the Weibull modulus and characteristic strength (and V/V0).",
     inputs=[_numin("P_failure", "allowed probability of failure (0 to 1)"), _phys("sigma0", "stress", "characteristic strength"),
             _numin("m", "Weibull modulus"), _numin("V_ratio", "V/V0 (default 1)", required=False)],
     outputs=[_o("sigma", "stress", "design stress"), _o("sigma_MPa", "number", "design stress in MPa")],
     primary="sigma_MPa", conditions=["0 < P_failure < 1", "sigma0, m, V/V0 > 0"],
     method="sigma = sigma_0 (-ln(1 - P_f) V0/V)^(1/m)", latex=r"\sigma = \sigma_0\left[-\frac{V_0}{V}\ln(1-P_f)\right]^{1/m}",
     run=weibull_stress,
     parse=lambda q: _pairs(q, r"weibull design stress", {"P_failure": "P_failure", "Pf": "P_failure", "sigma0": "sigma0", "m": "m", "V_ratio": "V_ratio"},
                            phys={"sigma0"}),
     reference=ref_weibull_stress,
     wrong=_safe(lambda P_failure, sigma0, m, V_ratio=1.0: _out(sigma=sigma0 * (P_failure / V_ratio) ** (1 / m), sigma_MPa=sigma0 * (P_failure / V_ratio) ** (1 / m) / 1e6)),
     random_inputs=lambda rng: {"P_failure": _lu(rng, 1e-5, 0.9), "sigma0": rng.uniform(50e6, 800e6), "m": rng.uniform(3, 25),
                                **({"V_ratio": rng.uniform(0.1, 20)} if rng.random() < 0.5 else {})},
     templates=["Weibull design stress: P_failure = 0.001, sigma0 = 400 MPa, m = 10"],
     examples=[{"inputs": {"P_failure": 0.001, "sigma0": 400e6, "m": 10.0}, "outputs": {"sigma_MPa": 400 * (-math.log(0.999)) ** 0.1}}],
     questions=[{"q": "Weibull design stress: P_failure = 0.001, sigma0 = 400 MPa, m = 10", "outputs": {"sigma_MPa": 400 * (-math.log(0.999)) ** 0.1}},
                {"q": "Weibull design stress: P_failure = 1.5, sigma0 = 400 MPa, m = 10", "outcome": "INVALID_INPUT"}])


def weibull_fit(strengths):
    if len(strengths) < 3:
        raise ValueError("give at least three strengths")
    for v in strengths:
        _pos("every strength", v)
    s = sorted(strengths)
    n = len(s)
    if s[0] == s[-1]:
        raise ValueError("the strengths must not all be equal")
    F = [(i + 1 - 0.3) / (n + 0.4) for i in range(n)]
    b0, b1, r2 = _linfit([math.log(v) for v in s], [math.log(-math.log1p(-f)) for f in F])
    return _out(m=b1, sigma0=math.exp(-b0 / b1), r_squared=r2)


def ref_weibull_fit(strengths):
    import numpy as np
    from scipy.stats import linregress
    s = np.sort(np.array(strengths, float))
    n = len(s)
    F = (np.arange(1, n + 1) - 0.3) / (n + 0.4)
    r = linregress(np.log(s), np.log(-np.log1p(-F)))
    return {"m": r.slope, "sigma0": math.exp(-r.intercept / r.slope), "r_squared": r.rvalue ** 2}


_add("weibull_modulus_fit",
     name="Weibull modulus from a set of measured strengths", topic="fracture / statistics of brittle fracture",
     purpose="The Weibull modulus m and characteristic strength sigma_0 from measured fracture strengths, by median-rank regression (Bernard's ranks).",
     inputs=[_vec("strengths", "fracture strengths (any one unit, e.g. MPa)", lo=3)],
     outputs=[_o("m", "number", "Weibull modulus"), _o("sigma0", "number", "characteristic strength (unit of the data)"), _o("r_squared", "number", "r^2 of the fit")],
     primary="m", conditions=["at least three strengths, not all equal, all > 0"],
     method="sort; F_i = (i - 0.3)/(n + 0.4); least-squares line of ln(-ln(1 - F_i)) against ln(sigma_i): slope m, intercept -m ln(sigma_0)",
     latex=r"\ln\ln\frac{1}{1-F_i} = m\ln\sigma_i - m\ln\sigma_0",
     run=weibull_fit,
     parse=lambda q: _pairs(q, r"weibull modulus(?: fit)?", {"strengths": "strengths", "strength": "strengths"}, vectors={"strengths"}),
     reference=ref_weibull_fit,
     wrong=_safe(lambda strengths: (lambda s, n: (lambda b: _out(m=b[1], sigma0=math.exp(-b[0] / b[1]), r_squared=b[2]))(
         _linfit([math.log(v) for v in s], [math.log(-math.log1p(-(i + 1) / (n + 1))) for i in range(n)])))(sorted(strengths), len(strengths))),
     random_inputs=lambda rng: {"strengths": [round(rng.weibullvariate(400, rng.uniform(5, 20)), 3) for _ in range(rng.randint(5, 30))]},
     templates=["Weibull modulus: strengths = [310, 342, 365, 380, 395, 410, 432]"],
     examples=[{"inputs": {"strengths": [310, 342, 365, 380, 395, 410, 432]},
                "outputs": {"m": _linfit([math.log(v) for v in [310, 342, 365, 380, 395, 410, 432]],
                                         [math.log(-math.log1p(-(i + 0.7) / 7.4)) for i in range(7)])[1]}}],
     questions=[{"q": "Weibull modulus: strengths = [310, 342, 365, 380, 395, 410, 432]",
                 "outputs": {"m": _linfit([math.log(v) for v in [310, 342, 365, 380, 395, 410, 432]],
                                          [math.log(-math.log1p(-(i + 0.7) / 7.4)) for i in range(7)])[1]}},
                {"q": "Weibull modulus: strengths = [310, 342]", "outcome": "INVALID_INPUT"}])


# ---------------------------------------------------------------------------------------- creep and viscoelasticity
def _solve3(A, b):
    M = [list(r) + [v] for r, v in zip(A, b)]
    n = 3
    for c in range(n):
        p = max(range(c, n), key=lambda r: abs(M[r][c]))
        if abs(M[p][c]) < 1e-300:
            raise ValueError("singular")
        M[c], M[p] = M[p], M[c]
        for r in range(c + 1, n):
            f = M[r][c] / M[c][c]
            for k in range(c, n + 1):
                M[r][k] -= f * M[c][k]
    x = [0.0] * n
    for r in reversed(range(n)):
        x[r] = (M[r][n] - sum(M[r][k] * x[k] for k in range(r + 1, n))) / M[r][r]
    return x


def creep_fit(stress_MPa, T, rate):
    n = len(stress_MPa)
    if not (n == len(T) == len(rate)) or n < 3:
        raise ValueError("stress, T and rate must be lists of the same length (at least 3 points)")
    for v in list(stress_MPa) + list(T) + list(rate):
        _pos("every stress, temperature and creep rate", v)
    X = [[1.0, math.log(s), -1 / (RGAS * t)] for s, t in zip(stress_MPa, T)]
    y = [math.log(r_) for r_ in rate]
    # centred columns: the normal equations of the two slopes, then the intercept
    m1 = sum(r_[1] for r_ in X) / n
    m2 = sum(r_[2] for r_ in X) / n
    my = sum(y) / n
    a11 = sum((r_[1] - m1) ** 2 for r_ in X)
    a22 = sum((r_[2] - m2) ** 2 for r_ in X)
    a12 = sum((r_[1] - m1) * (r_[2] - m2) for r_ in X)
    det = a11 * a22 - a12 * a12
    if a11 <= 0 or a22 <= 0 or det <= 1e-8 * a11 * a22:
        raise ValueError("the stresses and temperatures must both vary, and not in proportion")
    b1 = sum((r_[1] - m1) * (v - my) for r_, v in zip(X, y))
    b2 = sum((r_[2] - m2) * (v - my) for r_, v in zip(X, y))
    nexp = (b1 * a22 - b2 * a12) / det
    Q = (a11 * b2 - a12 * b1) / det
    # ln A = mean(y) - n mean(ln sigma) - Q mean(-1/RT) loses digits to cancellation: it is not reported
    return _out(n=nexp, Q_kJmol=Q / 1000)


def ref_creep_fit(stress_MPa, T, rate):
    import numpy as np
    from scipy.constants import gas_constant
    X = np.column_stack([np.ones(len(T)), np.log(stress_MPa), -1 / (gas_constant * np.array(T, float))])
    c, *_ = np.linalg.lstsq(X, np.log(rate), rcond=None)
    return {"n": c[1], "Q_kJmol": c[2] / 1000}


def _rand_creep(rng):
    nexp, Q, lnA = rng.uniform(2, 8), rng.uniform(100e3, 400e3), rng.uniform(-10, 10)
    k = rng.randint(4, 9)
    S = [round(rng.uniform(20, 300), 3) for _ in range(k)]
    T = [round(rng.uniform(700, 1300), 2) for _ in range(k)]
    R = [float(f"{math.exp(lnA + nexp * math.log(s) - Q / (RGAS * t) + rng.gauss(0, 0.05)):.6g}") for s, t in zip(S, T)]
    return {"stress_MPa": S, "T": T, "rate": R}


_add("creep_parameters_fit",
     name="Stress exponent and activation energy of steady-state creep", topic="mechanical properties / creep",
     purpose="n and Q_c of the power-law creep model rate = A sigma^n exp(-Q_c/RT) from steady-state creep rates at several stresses and temperatures (least squares).",
     inputs=[_vec("stress_MPa", "stresses in MPa", lo=3), _vec("T", "temperatures in kelvin", lo=3), _vec("rate", "steady-state creep rates in 1/s", lo=3)],
     outputs=[_o("n", "number", "stress exponent"), _o("Q_kJmol", "number", "creep activation energy in kJ/mol")],
     primary="n", conditions=["equal lengths (at least 3)", "stress and temperature both vary independently", "all values > 0"],
     method="least squares of ln(rate) = ln A + n ln(sigma) - Q_c/(R T)", latex=r"\dot\varepsilon_s = A\sigma^n e^{-Q_c/RT}",
     run=creep_fit,
     parse=lambda q: _pairs(q, r"creep parameters(?: fit)?|power[- ]law creep fit", {"stress_MPa": "stress_MPa", "stress": "stress_MPa", "T": "T", "rate": "rate"},
                            vectors={"stress_MPa", "T", "rate"}),
     reference=ref_creep_fit,
     wrong=_safe(lambda stress_MPa, T, rate: (lambda o: dict(o, Q_kJmol=o["Q_kJmol"] / RGAS * 1000 / 1000, n=o["n"] + 1))(creep_fit(stress_MPa, T, rate))),
     random_inputs=_rand_creep,
     templates=["Creep parameters: stress_MPa = [100, 200, 100], T = [900, 900, 1000], rate = [1e-7, 1.6e-6, 2.5e-6]"],
     examples=[{"inputs": {"stress_MPa": [100, 200, 100], "T": [900, 900, 1000], "rate": [1e-7, 1.6e-6, 2.5e-6]},
                "outputs": {"n": 4.0, "Q_kJmol": RGAS * math.log(25) / (1 / 900 - 1 / 1000) / 1000}}],
     questions=[{"q": "Creep parameters: stress_MPa = [100, 200, 100], T = [900, 900, 1000], rate = [1e-7, 1.6e-6, 2.5e-6]",
                 "outputs": {"n": 4.0}},
                {"q": "Creep parameters: stress_MPa = [100, 200, 300], T = [900, 900, 900], rate = [1e-7, 1.6e-6, 2.5e-6]", "outcome": "INVALID_INPUT"}])


def maxwell_relaxation(sigma0, E, eta, t):
    _pos("the stress, modulus and viscosity", sigma0, E, eta)
    if t < 0:
        raise ValueError("the time must not be negative")
    tau = eta / E
    r = math.exp(-t / tau)
    return _out(sigma_t=sigma0 * r, sigma_t_MPa=sigma0 * r / 1e6, tau=tau, ratio=r)


def ref_maxwell_relaxation(sigma0, E, eta, t):
    """The Maxwell element (spring E and dashpot eta in series) held at constant strain: d sigma/dt = -(E/eta) sigma,
    integrated with scipy DOP853."""
    from scipy.integrate import solve_ivp
    k = E / eta
    sol = solve_ivp(lambda s, y: [-k * y[0]], (0, t), [1.0], method="DOP853", rtol=1e-13, atol=1e-300)
    r = float(sol.y[0, -1])
    return {"sigma_t_MPa": sigma0 * r / 1e6, "ratio": r, "tau": eta / E}


_add("maxwell_stress_relaxation",
     name="Stress relaxation of a Maxwell viscoelastic solid", topic="polymers / viscoelasticity",
     purpose="The stress remaining after time t in a polymer held at constant strain, modelled as a Maxwell element: sigma(t) = sigma_0 exp(-t E/eta).",
     inputs=[_phys("sigma0", "stress", "initial stress"), _phys("E", "elastic_modulus", "spring modulus"), _phys("eta", "dynamic_viscosity", "dashpot viscosity"),
             _phys("t", "time", "time held")],
     outputs=[_o("sigma_t", "stress", "stress at time t"), _o("sigma_t_MPa", "number", "stress at time t in MPa"), _o("tau", "time", "relaxation time eta/E"),
              _o("ratio", "number", "sigma(t)/sigma_0")],
     primary="sigma_t_MPa", conditions=["sigma0, E, eta > 0", "t >= 0"],
     method="sigma(t) = sigma_0 exp(-t/tau), tau = eta/E", latex=r"\sigma(t) = \sigma_0 e^{-t/\tau},\quad \tau = \frac{\eta}{E}",
     run=maxwell_relaxation,
     parse=lambda q: _pairs(q, r"maxwell stress relaxation|stress relaxation", {"sigma0": "sigma0", "E": "E", "eta": "eta", "t": "t"}, phys={"sigma0", "E", "eta", "t"}),
     reference=ref_maxwell_relaxation,
     wrong=_safe(lambda sigma0, E, eta, t: (lambda r: _out(sigma_t=sigma0 * r, sigma_t_MPa=sigma0 * r / 1e6, tau=E / eta, ratio=r))(math.exp(-t * eta / E / (eta / E) ** 2))),
     random_inputs=lambda rng: (lambda E, eta: {"sigma0": rng.uniform(1e6, 50e6), "E": E, "eta": eta, "t": eta / E * rng.uniform(0.02, 6)})(
         rng.uniform(1e6, 3e9), _lu(rng, 1e9, 1e14)),
     templates=["Maxwell stress relaxation: sigma0 = 10 MPa, E = 1 GPa, eta = 1e12 Pa*s, t = 1000 s"],
     examples=[{"inputs": {"sigma0": 10e6, "E": 1e9, "eta": 1e12, "t": 1000.0}, "outputs": {"sigma_t_MPa": 10 * math.exp(-1), "tau": 1000.0}}],
     questions=[{"q": "Maxwell stress relaxation: sigma0 = 10 MPa, E = 1 GPa, eta = 1e12 Pa*s, t = 1000 s", "outputs": {"sigma_t_MPa": 10 * math.exp(-1)}},
                {"q": "Maxwell stress relaxation: sigma0 = 10 MPa, E = 1 GPa, eta = 1e12 Pa*s", "outcome": "NEEDS_INPUT"}])


def kelvin_voigt_creep(sigma0, E, eta, t):
    _pos("the stress, modulus and viscosity", sigma0, E, eta)
    if t < 0:
        raise ValueError("the time must not be negative")
    tau = eta / E
    frac = -math.expm1(-t / tau)
    return _out(strain=sigma0 / E * frac, fraction_of_elastic=frac, tau=tau)


def ref_kelvin_voigt_creep(sigma0, E, eta, t):
    """The Kelvin-Voigt element (spring and dashpot in parallel) under constant stress: eta d eps/dt + E eps = sigma_0,
    integrated with scipy DOP853."""
    from scipy.integrate import solve_ivp
    e_el = sigma0 / E
    sol = solve_ivp(lambda s, y: [(1.0 - y[0]) * E / eta], (0, t), [0.0], method="DOP853", rtol=1e-13, atol=1e-16)   # y = eps/(sigma0/E)
    frac = float(sol.y[0, -1])
    return {"strain": frac * e_el, "fraction_of_elastic": frac}


_add("kelvin_voigt_creep",
     name="Creep strain of a Kelvin-Voigt viscoelastic solid", topic="polymers / viscoelasticity",
     purpose="The strain after time t under constant stress for a Kelvin-Voigt (spring and dashpot in parallel) material: eps(t) = (sigma_0/E)(1 - exp(-t E/eta)).",
     inputs=[_phys("sigma0", "stress", "applied stress"), _phys("E", "elastic_modulus", "spring modulus"), _phys("eta", "dynamic_viscosity", "dashpot viscosity"),
             _phys("t", "time", "time under load")],
     outputs=[_o("strain", "number", "creep strain at time t"), _o("fraction_of_elastic", "number", "strain over the final strain sigma_0/E"),
              _o("tau", "time", "retardation time eta/E")],
     primary="strain", conditions=["sigma0, E, eta > 0", "t >= 0"],
     method="eps(t) = (sigma_0/E)(1 - exp(-t/tau)), tau = eta/E", latex=r"\varepsilon(t) = \frac{\sigma_0}{E}\left(1-e^{-t/\tau}\right)",
     run=kelvin_voigt_creep,
     parse=lambda q: _pairs(q, r"kelvin[- ]voigt creep", {"sigma0": "sigma0", "E": "E", "eta": "eta", "t": "t"}, phys={"sigma0", "E", "eta", "t"}),
     reference=ref_kelvin_voigt_creep,
     wrong=_safe(lambda sigma0, E, eta, t: _out(strain=sigma0 / E * math.exp(-t * E / eta), fraction_of_elastic=math.exp(-t * E / eta), tau=eta / E)),
     random_inputs=lambda rng: (lambda E, eta: {"sigma0": rng.uniform(1e6, 50e6), "E": E, "eta": eta, "t": eta / E * rng.uniform(0.05, 5)})(
         rng.uniform(1e6, 3e9), _lu(rng, 1e9, 1e14)),
     templates=["Kelvin-Voigt creep: sigma0 = 5 MPa, E = 500 MPa, eta = 1e11 Pa*s, t = 200 s"],
     examples=[{"inputs": {"sigma0": 5e6, "E": 500e6, "eta": 1e11, "t": 200.0}, "outputs": {"strain": 0.01 * (1 - math.exp(-1))}}],
     questions=[{"q": "Kelvin-Voigt creep: sigma0 = 5 MPa, E = 500 MPa, eta = 1e11 Pa*s, t = 200 s", "outputs": {"strain": 0.01 * (1 - math.exp(-1))}}])


# ---------------------------------------------------------------------------------------- slip: Schmid's law
def _norm3(v):
    return math.sqrt(sum(x * x for x in v))


def _dot3(a, b):
    return sum(x * y for x, y in zip(a, b))


def schmid(load, plane, direction, sigma=None, tau_crss=None):
    for v, name in ((load, "the loading direction"), (plane, "the slip-plane normal"), (direction, "the slip direction")):
        if len(v) != 3 or _norm3(v) == 0:
            raise ValueError(f"{name} must be a nonzero 3-vector")
    if abs(_dot3(plane, direction)) > 1e-9 * _norm3(plane) * _norm3(direction):
        raise ValueError("the slip direction must lie in the slip plane (h u + k v + l w = 0)")
    cphi = _dot3(load, plane) / (_norm3(load) * _norm3(plane))
    clam = _dot3(load, direction) / (_norm3(load) * _norm3(direction))
    m = abs(cphi * clam)
    m = 0.0 if m < 1e-12 else m
    out = dict(schmid_factor=m, phi_deg=math.degrees(math.acos(max(-1.0, min(1.0, abs(cphi))))),
               lambda_deg=math.degrees(math.acos(max(-1.0, min(1.0, abs(clam))))))
    if sigma is not None:
        out["tau_R_MPa"] = sigma * m / 1e6
    if tau_crss is not None:
        _pos("the critical resolved shear stress", tau_crss)
        if m == 0:
            raise ValueError("the Schmid factor is 0: this system never yields under this load")
        out["yield_MPa"] = tau_crss / m / 1e6
    return _out(**out)


def ref_schmid(load, plane, direction, sigma=None, tau_crss=None):
    """The uniaxial stress tensor sigma n n^T resolved on the slip system: traction on the plane, projected on the
    slip direction (numpy)."""
    import numpy as np
    n = np.array(load, float) / np.linalg.norm(load)
    p = np.array(plane, float) / np.linalg.norm(plane)
    d = np.array(direction, float) / np.linalg.norm(direction)
    S = np.outer(n, n)
    m = abs(float((S @ p) @ d))
    m = 0.0 if m < 1e-12 else m
    out = {"schmid_factor": m}
    if sigma is not None:
        out["tau_R_MPa"] = sigma * m / 1e6
    if tau_crss is not None:
        out["yield_MPa"] = tau_crss / m / 1e6
    return out


def _rand_schmid(rng):
    while True:
        plane = [rng.randint(-2, 2) for _ in range(3)]
        if not any(plane):
            continue
        cand = [list(v) for v in _iproduct(range(-2, 3), repeat=3) if any(v) and _dot3(v, plane) == 0]
        if not cand:
            continue
        load = [rng.randint(-3, 3) for _ in range(3)]
        if not any(load):
            continue
        d = rng.choice(cand)
        x = {"load": load, "plane": plane, "direction": d}
        cphi = abs(_dot3(load, plane)) / (_norm3(load) * _norm3(plane))
        clam = abs(_dot3(load, d)) / (_norm3(load) * _norm3(d))
        if cphi * clam < 0.02:
            continue
        if rng.random() < 0.5:
            x["sigma"] = rng.uniform(10e6, 500e6)
        if rng.random() < 0.5:
            x["tau_crss"] = rng.uniform(1e6, 100e6)
        return x


_add("schmid_factor",
     name="Resolved shear stress and Schmid factor of a slip system", topic="mechanical properties / slip in single crystals",
     purpose="The Schmid factor cos(phi) cos(lambda) of a slip system (plane normal and slip direction) for a tensile load along a direction of a cubic crystal, the resolved shear stress and the stress to start slip.",
     inputs=[_vec("load", "loading direction [u v w]", 3, 3), _vec("plane", "slip-plane normal (h k l)", 3, 3), _vec("direction", "slip direction [u v w]", 3, 3),
             _phys("sigma", "stress", "applied tensile stress (optional)", required=False),
             _phys("tau_crss", "stress", "critical resolved shear stress (optional)", required=False)],
     outputs=[_o("schmid_factor", "number", "|cos(phi) cos(lambda)|"), _o("phi_deg", "number", "angle between load and plane normal"),
              _o("lambda_deg", "number", "angle between load and slip direction"), _o("tau_R_MPa", "number", "resolved shear stress in MPa"),
              _o("yield_MPa", "number", "tensile stress at which slip starts, in MPa")],
     primary="schmid_factor", conditions=["cubic crystal (indices as Cartesian directions)", "slip direction in the slip plane"],
     method="tau_R = sigma cos(phi) cos(lambda); sigma_y = tau_CRSS / (cos(phi) cos(lambda))",
     latex=r"\tau_R = \sigma\cos\phi\cos\lambda,\quad \sigma_y = \frac{\tau_{\mathrm{crss}}}{\cos\phi\cos\lambda}",
     run=schmid,
     parse=lambda q: _pairs(q, r"schmid factor|resolved shear stress", {"load": "load", "plane": "plane", "direction": "direction", "sigma": "sigma", "tau_crss": "tau_crss"},
                            phys={"sigma", "tau_crss"}, vectors={"load", "plane", "direction"}),
     reference=ref_schmid,
     wrong=_safe(lambda load, plane, direction, sigma=None, tau_crss=None: (lambda o: dict(o, schmid_factor=o["schmid_factor"] ** 0.5,
                                                                                         **({"tau_R_MPa": o["tau_R_MPa"] * 2} if "tau_R_MPa" in o else {}),
                                                                                         **({"yield_MPa": o["yield_MPa"] / 2} if "yield_MPa" in o else {})))(
         schmid(load, plane, direction, sigma, tau_crss))),
     random_inputs=_rand_schmid,
     templates=["Schmid factor: load = [0, 1, 0], plane = [1, 1, 0], direction = [-1, 1, 1], sigma = 52 MPa, tau_crss = 30 MPa"],
     examples=[{"inputs": {"load": [0, 1, 0], "plane": [1, 1, 0], "direction": [-1, 1, 1], "sigma": 52e6},
                "outputs": {"schmid_factor": 1 / math.sqrt(6), "tau_R_MPa": 52 / math.sqrt(6)}},
               {"inputs": {"load": [0, 0, 1], "plane": [1, 1, 1], "direction": [1, -1, 0]}, "outputs": {"schmid_factor": 0.0}}],
     questions=[{"q": "Schmid factor: load = [0, 1, 0], plane = [1, 1, 0], direction = [-1, 1, 1], sigma = 52 MPa",
                 "outputs": {"schmid_factor": 1 / math.sqrt(6), "tau_R_MPa": 52 / math.sqrt(6)}},
                {"q": "Schmid factor: load = [0, 1, 0], plane = [1, 1, 0], direction = [1, 1, 1]", "outcome": "INVALID_INPUT"},
                {"q": "Schmid factor: load = [0, 0, 1], plane = [1, 1, 1], direction = [1, -1, 0], tau_crss = 1 MPa", "outcome": "INVALID_INPUT"}])


def _slip_systems(structure):
    """{111}<110> (FCC) or {110}<111> (BCC), each system once (plane and direction up to sign)."""
    planes = [(1, 1, 1), (-1, 1, 1), (1, -1, 1), (1, 1, -1)] if structure == "FCC" else \
        [(1, 1, 0), (1, -1, 0), (1, 0, 1), (1, 0, -1), (0, 1, 1), (0, 1, -1)]
    dirs = [(1, -1, 0), (1, 0, -1), (0, 1, -1), (1, 1, 0), (1, 0, 1), (0, 1, 1)] if structure == "FCC" else \
        [(1, 1, 1), (-1, 1, 1), (1, -1, 1), (1, 1, -1)]
    return [(p, d) for p in planes for d in dirs if _dot3(p, d) == 0]


def max_schmid(structure, load, tau_crss=None):
    if len(load) != 3 or _norm3(load) == 0:
        raise ValueError("the loading direction must be a nonzero 3-vector")
    ms = [abs(_dot3(load, p) * _dot3(load, d)) / (_norm3(load) ** 2 * _norm3(p) * _norm3(d)) for p, d in _slip_systems(structure)]
    mx = max(ms)
    count = sum(1 for v in ms if v >= mx * (1 - 1e-9))
    out = dict(max_schmid_factor=mx, active_systems=count, systems=len(ms))
    if tau_crss is not None:
        _pos("the critical resolved shear stress", tau_crss)
        out["yield_MPa"] = tau_crss / mx / 1e6
    return _out(**out)


def ref_max_schmid(structure, load, tau_crss=None):
    import numpy as np
    pf, df = ((1, 1, 1), (1, 1, 0)) if structure == "FCC" else ((1, 1, 0), (1, 1, 1))

    def family(base):
        out = set()
        for perm in set(_iproduct(*[[x, -x] for x in base])):
            for p in {(perm[i], perm[j], perm[k]) for i, j, k in ((0, 1, 2), (0, 2, 1), (1, 0, 2), (1, 2, 0), (2, 0, 1), (2, 1, 0))}:
                v = np.array(p)
                if tuple(-v) not in out:
                    out.add(tuple(v))
        return [np.array(v, float) for v in out]
    n = np.array(load, float) / np.linalg.norm(load)
    S = np.outer(n, n)
    ms = []
    for p in family(pf):
        for d in family(df):
            if abs(p @ d) < 1e-12:
                ms.append(abs(float((S @ (p / np.linalg.norm(p))) @ (d / np.linalg.norm(d)))))
    mx = max(ms)
    out = {"max_schmid_factor": mx, "active_systems": int(sum(v >= mx * (1 - 1e-9) for v in ms)), "systems": len(ms)}
    if tau_crss is not None:
        out["yield_MPa"] = tau_crss / mx / 1e6
    return out


def _rand_maxschmid(rng):
    while True:
        load = [rng.randint(-4, 4) for _ in range(3)]
        if any(load):
            x = {"structure": rng.choice(["FCC", "BCC"]), "load": load}
            if rng.random() < 0.5:
                x["tau_crss"] = rng.uniform(1e6, 100e6)
            return x


_add("max_schmid_factor",
     name="Most favourable slip system of an FCC or BCC crystal", topic="mechanical properties / slip in single crystals",
     purpose="The largest Schmid factor over the 12 {111}<110> (FCC) or {110}<111> (BCC) slip systems for a tensile load direction, how many systems share it, and the yield stress from the CRSS.",
     inputs=[_struct_in(["FCC", "BCC"]), _vec("load", "loading direction [u v w]", 3, 3),
             _phys("tau_crss", "stress", "critical resolved shear stress (optional)", required=False)],
     outputs=[_o("max_schmid_factor", "number", "largest Schmid factor"), _o("active_systems", "number", "number of systems with that factor"),
              _o("systems", "number", "slip systems considered"), _o("yield_MPa", "number", "tensile yield stress in MPa")],
     primary="max_schmid_factor", conditions=["nonzero loading direction"],
     method="Schmid factor of each of the 12 systems; the maximum (ties within 1e-9 counted)",
     latex=r"m = \max_{\text{systems}}|\cos\phi\cos\lambda|",
     run=max_schmid,
     parse=lambda q: _pairs(q, r"maximum schmid factor|max schmid factor", {"structure": "structure", "load": "load", "tau_crss": "tau_crss"},
                            phys={"tau_crss"}, words={"structure"}, vectors={"load"}),
     reference=ref_max_schmid,
     wrong=_safe(lambda structure, load, tau_crss=None: max_schmid("BCC" if structure == "FCC" else "FCC", [load[0], load[1], 2 * load[2] + 1], tau_crss)),
     random_inputs=_rand_maxschmid,
     templates=["Maximum Schmid factor: structure = FCC, load = [0, 0, 1]", "Maximum Schmid factor: structure = BCC, load = [1, 2, 3], tau_crss = 30 MPa"],
     examples=[{"inputs": {"structure": "FCC", "load": [0, 0, 1]}, "outputs": {"max_schmid_factor": 1 / math.sqrt(6), "active_systems": 8}},
               {"inputs": {"structure": "FCC", "load": [1, 1, 1]}, "outputs": {"max_schmid_factor": 2 / (3 * math.sqrt(6)), "active_systems": 6}}],
     questions=[{"q": "Maximum Schmid factor: structure = FCC, load = [0, 0, 1]", "outputs": {"max_schmid_factor": 1 / math.sqrt(6), "active_systems": 8}},
                {"q": "Maximum Schmid factor: structure = FCC, load = [0, 0, 0]", "outcome": "INVALID_INPUT"}])

# ================================================================================================ phase diagrams
def _between(name, c, lo, hi, strict=False):
    a, b = min(lo, hi), max(lo, hi)
    if (strict and not a < c < b) or (not strict and not a <= c <= b):
        raise ValueError(f"{name} must lie between the two phase compositions")


def lever_rule(C0, C1, C2, mass=None):
    if C1 == C2:
        raise ValueError("the two phase compositions must differ")
    _between("the overall composition C0", C0, C1, C2)
    W1 = (C2 - C0) / (C2 - C1)
    W2 = (C0 - C1) / (C2 - C1)
    out = dict(W1=W1, W2=W2)
    if mass is not None:
        _pos("the mass", mass)
        out.update(m1=W1 * mass, m2=W2 * mass)
    return _out(**out)


def ref_lever_rule(C0, C1, C2, mass=None):
    """Mass balance solved with numpy: W1 + W2 = 1, W1 C1 + W2 C2 = C0."""
    import numpy as np
    W = np.linalg.solve(np.array([[1.0, 1.0], [C1, C2]]), np.array([1.0, C0]))
    W = [0.0 if abs(w) < 1e-13 else float(w) for w in W]
    out = {"W1": W[0], "W2": W[1]}
    if mass is not None:
        out.update(m1=W[0] * mass, m2=W[1] * mass)
    return out


def _rand_lever(rng):
    C1, C2 = sorted((round(rng.uniform(0, 100), 2), round(rng.uniform(0, 100), 2)))
    if C2 - C1 < 2:
        C2 = C1 + 5
    x = {"C0": round(rng.uniform(C1 + 0.5, C2 - 0.5), 3), "C1": C1, "C2": C2}
    if rng.random() < 0.5:
        x["mass"] = rng.uniform(0.1, 100)
    return x


_WT = ["wt%", "at%", "%", "wt.%", "mol%"]

_add("lever_rule",
     name="Phase fractions by the lever rule", topic="phase diagrams / two-phase regions",
     purpose="The mass fractions (and masses) of the two phases of an alloy of overall composition C0 in a two-phase region whose tie line ends at C1 and C2.",
     inputs=[_numin("C0", "overall composition (wt%)"), _numin("C1", "composition of phase 1 (wt%)"), _numin("C2", "composition of phase 2 (wt%)"),
             _phys("mass", "mass", "total mass of the alloy (optional)", required=False)],
     outputs=[_o("W1", "number", "mass fraction of phase 1"), _o("W2", "number", "mass fraction of phase 2"),
              _o("m1", "mass", "mass of phase 1"), _o("m2", "mass", "mass of phase 2")],
     primary="W1", conditions=["C1 != C2", "C0 between C1 and C2"],
     method="W1 = (C2 - C0)/(C2 - C1), W2 = (C0 - C1)/(C2 - C1)", latex=r"W_1 = \frac{C_2-C_0}{C_2-C_1},\quad W_2 = \frac{C_0-C_1}{C_2-C_1}",
     run=lever_rule,
     parse=lambda q: _pairs(q, r"lever rule", {"C0": "C0", "C1": "C1", "C2": "C2", "mass": "mass", "m": "mass"}, phys={"mass"},
                            num_units={"C0": _WT, "C1": _WT, "C2": _WT}),
     reference=ref_lever_rule,
     wrong=_safe(lambda C0, C1, C2, mass=None: (lambda W1: _out(W1=W1, W2=1 - W1, **({} if mass is None else {"m1": W1 * mass, "m2": (1 - W1) * mass})))(
         (C0 - C1) / (C2 - C1))),
     random_inputs=_rand_lever,
     templates=["Lever rule: C0 = 35 wt%, C1 = 31.5 wt%, C2 = 42.5 wt%", "Lever rule: C0 = 35, C1 = 31.5, C2 = 42.5, mass = 2 kg"],
     examples=[{"inputs": {"C0": 35.0, "C1": 31.5, "C2": 42.5}, "outputs": {"W1": 7.5 / 11, "W2": 3.5 / 11}},
               {"inputs": {"C0": 40.0, "C1": 20.0, "C2": 60.0, "mass": 3.0}, "outputs": {"W1": 0.5, "m2": 1.5}}],
     questions=[{"q": "Lever rule: C0 = 35 wt%, C1 = 31.5 wt%, C2 = 42.5 wt%", "outputs": {"W1": 7.5 / 11}},
                {"q": "Lever rule: C0 = 50 wt%, C1 = 31.5 wt%, C2 = 42.5 wt%", "outcome": "INVALID_INPUT"}])


def eutectic_constituents(C0, C_alpha, C_E, C_beta):
    if not C_alpha < C_E < C_beta:
        raise ValueError("compositions must satisfy C_alpha < C_E < C_beta")
    _between("C0", C0, C_alpha, C_beta, strict=True)
    if C0 <= C_E:
        primary, Wp = "alpha", (C_E - C0) / (C_E - C_alpha)
    else:
        primary, Wp = "beta", (C0 - C_E) / (C_beta - C_E)
    Wa = (C_beta - C0) / (C_beta - C_alpha)
    return {**_out(W_primary=Wp, W_eutectic=1 - Wp, W_alpha_total=Wa, W_beta_total=1 - Wa), "primary_phase": primary}


def ref_eutectic_constituents(C0, C_alpha, C_E, C_beta):
    """Two mass balances solved with numpy: just above the eutectic (primary phase + liquid of C_E) and just below it
    (alpha + beta)."""
    import numpy as np
    other = C_alpha if C0 <= C_E else C_beta
    Wp, _ = np.linalg.solve(np.array([[1.0, 1.0], [other, C_E]]), np.array([1.0, C0]))
    Wa, Wb = np.linalg.solve(np.array([[1.0, 1.0], [C_alpha, C_beta]]), np.array([1.0, C0]))
    snap = lambda w: 0.0 if abs(w) < 1e-13 else float(w)            # noqa: E731
    return {"W_primary": snap(Wp), "W_eutectic": snap(1 - Wp), "W_alpha_total": snap(Wa), "W_beta_total": snap(Wb),
            "primary_phase": "alpha" if C0 <= C_E else "beta"}


def _rand_eutectic(rng):
    ca = rng.uniform(1, 20)
    ce = rng.uniform(ca + 10, 80)
    cb = rng.uniform(ce + 5, 99)
    return {"C0": round(rng.uniform(ca + 0.5, cb - 0.5), 3), "C_alpha": round(ca, 2), "C_E": round(ce, 2), "C_beta": round(cb, 2)}


_add("eutectic_microconstituents",
     name="Primary and eutectic microconstituent fractions", topic="phase diagrams / eutectic systems",
     purpose="The fractions of primary phase and eutectic microconstituent, and of total alpha and beta, in a binary eutectic alloy cooled just below the eutectic temperature.",
     inputs=[_numin("C0", "alloy composition (wt%)"), _numin("C_alpha", "composition of alpha at the eutectic temperature"),
             _numin("C_E", "eutectic composition"), _numin("C_beta", "composition of beta at the eutectic temperature")],
     outputs=[_o("W_primary", "number", "fraction of primary (proeutectic) phase"), _o("W_eutectic", "number", "fraction of eutectic microconstituent"),
              _o("W_alpha_total", "number", "total fraction of alpha"), _o("W_beta_total", "number", "total fraction of beta"),
              {"symbol": "primary_phase", "kind": "text", "meaning": "the primary phase (alpha for hypoeutectic, beta for hypereutectic)"}],
     primary="W_primary", conditions=["C_alpha < C_E < C_beta", "C_alpha < C0 < C_beta"],
     method="lever rule on the tie line just above the eutectic (primary phase vs liquid of C_E) and just below it (alpha vs beta)",
     latex=r"W_{\alpha'} = \frac{C_E-C_0}{C_E-C_\alpha},\quad W_e = \frac{C_0-C_\alpha}{C_E-C_\alpha}",
     run=eutectic_constituents,
     parse=lambda q: _pairs(q, r"eutectic microconstituents", {"C0": "C0", "C_alpha": "C_alpha", "C_E": "C_E", "C_beta": "C_beta"},
                            num_units={"C0": _WT, "C_alpha": _WT, "C_E": _WT, "C_beta": _WT}),
     reference=ref_eutectic_constituents,
     wrong=_safe(lambda C0, C_alpha, C_E, C_beta: {**_out(W_primary=(C_E - C0) / (C_beta - C_alpha), W_eutectic=1 - (C_E - C0) / (C_beta - C_alpha),
                                                             W_alpha_total=(C0 - C_alpha) / (C_beta - C_alpha), W_beta_total=(C_beta - C0) / (C_beta - C_alpha)),
                                                    "primary_phase": "alpha"}),
     random_inputs=_rand_eutectic,
     templates=["Eutectic microconstituents: C0 = 25 wt%, C_alpha = 18.3 wt%, C_E = 61.9 wt%, C_beta = 97.8 wt%"],
     examples=[{"inputs": {"C0": 25.0, "C_alpha": 18.3, "C_E": 61.9, "C_beta": 97.8},
                "outputs": {"W_primary": 36.9 / 43.6, "W_alpha_total": 72.8 / 79.5}}],
     questions=[{"q": "Eutectic microconstituents: C0 = 25 wt%, C_alpha = 18.3 wt%, C_E = 61.9 wt%, C_beta = 97.8 wt%",
                 "outputs": {"W_primary": 36.9 / 43.6, "primary_phase": "alpha"}},
                {"q": "Eutectic microconstituents: C0 = 10 wt%, C_alpha = 18.3 wt%, C_E = 61.9 wt%, C_beta = 97.8 wt%", "outcome": "INVALID_INPUT"}])

_FE_A, _FE_E, _FE_C = 0.022, 0.76, 6.70


def iron_carbon(C0):
    if not _FE_A < C0 < 2.14:
        raise ValueError("a steel: the carbon content must lie between 0.022 and 2.14 wt%")
    if C0 <= _FE_E:
        pro, Wp = "ferrite", (_FE_E - C0) / (_FE_E - _FE_A)
    else:
        pro, Wp = "cementite", (C0 - _FE_E) / (_FE_C - _FE_E)
    Wf = (_FE_C - C0) / (_FE_C - _FE_A)
    return {**_out(W_proeutectoid=Wp, W_pearlite=1 - Wp, W_ferrite_total=Wf, W_cementite_total=1 - Wf), "proeutectoid_phase": pro}


def ref_iron_carbon(C0):
    import numpy as np
    other = _FE_A if C0 <= _FE_E else _FE_C
    Wp, _ = np.linalg.solve(np.array([[1.0, 1.0], [other, _FE_E]]), np.array([1.0, C0]))
    Wf, _ = np.linalg.solve(np.array([[1.0, 1.0], [_FE_A, _FE_C]]), np.array([1.0, C0]))
    snap = lambda w: 0.0 if abs(w) < 1e-13 else float(w)            # noqa: E731
    return {"W_proeutectoid": snap(Wp), "W_pearlite": snap(1 - Wp), "W_ferrite_total": snap(Wf), "W_cementite_total": snap(1 - Wf)}


_add("iron_carbon_microconstituents",
     name="Proeutectoid phase and pearlite in a plain-carbon steel", topic="phase diagrams / iron-carbon system",
     purpose="The fractions of proeutectoid ferrite or cementite and of pearlite, and of total ferrite and cementite, in a steel cooled slowly just below the eutectoid temperature.",
     inputs=[_numin("C0", "carbon content in wt% (0.022 to 2.14)")],
     outputs=[_o("W_proeutectoid", "number", "fraction of proeutectoid ferrite or cementite"), _o("W_pearlite", "number", "fraction of pearlite"),
              _o("W_ferrite_total", "number", "total fraction of ferrite"), _o("W_cementite_total", "number", "total fraction of cementite"),
              {"symbol": "proeutectoid_phase", "kind": "text", "meaning": "ferrite (hypoeutectoid) or cementite (hypereutectoid)"}],
     primary="W_pearlite", conditions=["0.022 < C0 < 2.14 wt% C"],
     method="lever rule with the eutectoid at 0.76 wt% C, ferrite 0.022 wt% C and cementite 6.70 wt% C",
     latex=r"W_p = \frac{C_0 - 0.022}{0.76 - 0.022}\ (\text{hypo}),\quad W_p = \frac{6.70 - C_0}{6.70 - 0.76}\ (\text{hyper})",
     run=iron_carbon,
     parse=lambda q: _pairs(q, r"iron[- ]carbon microconstituents|steel microconstituents", {"C0": "C0", "C": "C0"}, num_units={"C0": _WT}),
     reference=ref_iron_carbon,
     wrong=_safe(lambda C0: {**_out(W_proeutectoid=(_FE_E - C0) / _FE_E if C0 <= _FE_E else (C0 - _FE_E) / _FE_C,
                                    W_pearlite=1 - ((_FE_E - C0) / _FE_E if C0 <= _FE_E else (C0 - _FE_E) / _FE_C),
                                    W_ferrite_total=(_FE_C - C0) / _FE_C, W_cementite_total=C0 / _FE_C), "proeutectoid_phase": "ferrite"}),
     random_inputs=lambda rng: {"C0": round(rng.uniform(0.03, 2.1), 4)},
     templates=["Iron-carbon microconstituents: C0 = 0.35 wt%", "Steel microconstituents: C0 = 1.2"],
     examples=[{"inputs": {"C0": 0.35}, "outputs": {"W_pearlite": (0.35 - 0.022) / 0.738, "W_ferrite_total": 6.35 / 6.678}},
               {"inputs": {"C0": 1.2}, "outputs": {"W_proeutectoid": 0.44 / 5.94}}],
     questions=[{"q": "Iron-carbon microconstituents: C0 = 0.35 wt%", "outputs": {"W_pearlite": (0.35 - 0.022) / 0.738, "proeutectoid_phase": "ferrite"}},
                {"q": "Iron-carbon microconstituents: C0 = 3 wt%", "outcome": "INVALID_INPUT"}])


def gibbs_phase_rule(C, P, pressure="variable"):
    C, P = int(C), int(P)
    if C < 1 or P < 1:
        raise ValueError("the numbers of components and phases must be at least 1")
    F = C - P + (2 if pressure == "variable" else 1)
    if F < 0:
        raise ValueError("more phases than the phase rule allows in equilibrium")
    return {"F": F}


def ref_gibbs_phase_rule(C, P, pressure="variable"):
    """Degrees of freedom as unknowns minus independent equations: T (and p), C - 1 mole fractions per phase; the
    equality of each component's chemical potential across the phases, with generic smooth potentials whose Jacobian
    rank is computed (numpy) at a random state."""
    import numpy as np
    rng = np.random.default_rng(7)
    nT = 2 if pressure == "variable" else 1
    nvar = nT + P * (C - 1)
    rows = []
    for c in range(C):
        for p in range(1, P):
            row = np.zeros(nvar)
            row[:nT] = rng.normal(size=nT)                      # every potential depends on T (and p)
            for ph, sign in ((p, 1.0), (0, -1.0)):
                start = nT + ph * (C - 1)
                row[start:start + C - 1] += sign * rng.normal(size=C - 1)
            rows.append(row)
    rank = int(np.linalg.matrix_rank(np.array(rows))) if rows else 0
    F = nvar - rank
    if len(rows) > nvar:                                        # more equations than unknowns: no equilibrium
        F = nvar - len(rows)
    return {"F": F}


_add("gibbs_phase_rule",
     name="Degrees of freedom by Gibbs' phase rule", topic="phase diagrams / phase rule",
     purpose="The number of degrees of freedom F = C - P + 2 (or + 1 at fixed pressure) of a system of C components and P phases in equilibrium.",
     inputs=[_intin("C", "number of components", 1, 20), _intin("P", "number of phases", 1, 25),
             _choice("pressure", ["variable", "constant"], "pressure variable or held constant (default variable)", required=False)],
     outputs=[_o("F", "number", "degrees of freedom")],
     primary="F", conditions=["F >= 0"],
     method="F = C - P + 2 (variable pressure) or C - P + 1 (constant pressure, condensed systems)", latex=r"F = C - P + 2",
     run=gibbs_phase_rule,
     parse=lambda q: _pairs(q, r"gibbs phase rule|phase rule", {"C": "C", "P": "P", "pressure": "pressure"}, words={"pressure"}),
     reference=ref_gibbs_phase_rule,
     wrong=_safe(lambda C, P, pressure="variable": {"F": C + P - (2 if pressure == "variable" else 1)}),
     random_inputs=lambda rng: (lambda C, pr: {"C": C, "P": rng.randint(1, C + (2 if pr == "variable" else 1)), "pressure": pr})(
         rng.randint(1, 6), rng.choice(["variable", "constant"])),
     templates=["Gibbs phase rule: C = 2, P = 3, pressure = constant", "Phase rule: C = 1, P = 3"],
     examples=[{"inputs": {"C": 1, "P": 3}, "outputs": {"F": 0}}, {"inputs": {"C": 2, "P": 2, "pressure": "constant"}, "outputs": {"F": 1}},
               {"inputs": {"C": 2, "P": 3, "pressure": "constant"}, "outputs": {"F": 0}}],
     questions=[{"q": "Gibbs phase rule: C = 2, P = 3, pressure = constant", "outputs": {"F": 0}},
                {"q": "Gibbs phase rule: C = 1, P = 4", "outcome": "INVALID_INPUT"}])


def nucleation(gamma, dHf, Tm, T):
    _pos("gamma, the latent heat and the melting temperature", gamma, dHf, Tm)
    if not 0 < T < Tm:
        raise ValueError("the temperature must lie below the melting point (undercooled liquid)")
    dGv = -dHf * (Tm - T) / Tm
    r = -2 * gamma / dGv
    G = 16 * math.pi * gamma ** 3 / (3 * dGv * dGv)
    return _out(undercooling=Tm - T, dGv_MJ_m3=dGv / 1e6, r_star=r, r_star_nm=r * 1e9, dG_star=G, dG_star_eV=G / QE)


_NUCL: dict = {}


def ref_nucleation(gamma, dHf, Tm, T):
    """Homogeneous nucleation: maximise dG(r) = (4/3) pi r^3 dGv + 4 pi r^2 gamma symbolically (sympy)."""
    if "f" not in _NUCL:
        import sympy as sp
        r, g, dg = sp.symbols("r gamma dGv", real=True)
        G = sp.Rational(4, 3) * sp.pi * r ** 3 * dg + 4 * sp.pi * r ** 2 * g
        roots = [s for s in sp.solve(sp.diff(G, r), r) if s != 0]
        _NUCL["f"] = sp.lambdify((g, dg), (roots[0], G.subs(r, roots[0])), "math")
    from scipy.constants import electron_volt
    dGv = -dHf * (Tm - T) / Tm
    rs, Gs = _NUCL["f"](gamma, dGv)
    return {"r_star_nm": rs * 1e9, "dG_star_eV": Gs / electron_volt, "dGv_MJ_m3": dGv / 1e6}


_add("homogeneous_nucleation",
     name="Critical nucleus radius and activation energy (homogeneous nucleation)", topic="phase transformations / nucleation",
     purpose="The critical radius r* and the energy barrier dG* for homogeneous nucleation of a solid from its undercooled melt.",
     inputs=[_phys("gamma", "surface_energy", "solid-liquid interfacial energy"), _phys("dHf", "energy_density", "latent heat of fusion per volume"),
             _phys("Tm", "temperature", "melting temperature"), _phys("T", "temperature", "temperature of the undercooled liquid")],
     outputs=[_o("undercooling", "temperature_difference", "undercooling Tm - T"), _o("dGv_MJ_m3", "number", "volume free-energy change in MJ/m^3"),
              _o("r_star", "length", "critical radius"), _o("r_star_nm", "number", "critical radius in nm"),
              _o("dG_star", "energy", "activation free energy"), _o("dG_star_eV", "number", "activation free energy in eV")],
     primary="r_star_nm", conditions=["gamma, dHf, Tm > 0", "0 < T < Tm"],
     method="dGv = -dHf (Tm - T)/Tm; r* = -2 gamma/dGv; dG* = 16 pi gamma^3/(3 dGv^2)",
     latex=r"r^* = -\frac{2\gamma}{\Delta G_v},\quad \Delta G^* = \frac{16\pi\gamma^3}{3\Delta G_v^2},\quad \Delta G_v = -\frac{\Delta H_f\,\Delta T}{T_m}",
     run=nucleation,
     parse=lambda q: _pairs(q, r"homogeneous nucleation|critical nucleus", {"gamma": "gamma", "dHf": "dHf", "Tm": "Tm", "T": "T"}, phys={"gamma", "dHf", "Tm", "T"}),
     reference=ref_nucleation,
     wrong=_safe(lambda gamma, dHf, Tm, T: (lambda dGv: _out(undercooling=Tm - T, dGv_MJ_m3=dGv / 1e6, r_star=-gamma / dGv, r_star_nm=-gamma / dGv * 1e9,
                                                             dG_star=4 * math.pi * gamma ** 3 / (3 * dGv * dGv), dG_star_eV=4 * math.pi * gamma ** 3 / (3 * dGv * dGv) / QE))(
         -dHf * (Tm - T) / Tm)),
     random_inputs=lambda rng: (lambda Tm: {"gamma": rng.uniform(0.05, 0.4), "dHf": rng.uniform(0.5e9, 2.5e9), "Tm": Tm, "T": Tm - rng.uniform(5, 0.4 * Tm)})(
         rng.uniform(500, 2000)),
     templates=["Homogeneous nucleation: gamma = 0.132 J/m^2, dHf = 1.16e9 J/m^3, Tm = 1064 degC, T = 834 degC"],
     examples=[{"inputs": {"gamma": 0.132, "dHf": 1.16e9, "Tm": 1337.15, "T": 1107.15},
                "outputs": {"r_star_nm": 2 * 0.132 * 1337.15 / (1.16e9 * 230) * 1e9}}],
     questions=[{"q": "Homogeneous nucleation: gamma = 0.132 J/m^2, dHf = 1.16e9 J/m^3, Tm = 1064 degC, T = 834 degC",
                 "outputs": {"r_star_nm": 2 * 0.132 * 1337.15 / (1.16e9 * 230) * 1e9}},
                {"q": "Homogeneous nucleation: gamma = 0.132 J/m^2, dHf = 1.16e9 J/m^3, Tm = 1064 degC, T = 1100 degC", "outcome": "INVALID_INPUT"}])


def avrami_fit(t, y, y_target=None):
    if len(t) != len(y):
        raise ValueError("t and y must have the same length")
    for v in t:
        _pos("every time", v)
    for v in y:
        _frac01("every transformed fraction", v)
    b0, n, r2 = _linfit([math.log(v) for v in t], [math.log(-math.log1p(-v)) for v in y])
    if n <= 0:
        raise ValueError("the transformed fraction must increase with time")
    k = math.exp(b0)
    out = dict(n=n, k=k, log10_k=b0 / math.log(10), t_half=(math.log(2) / k) ** (1 / n), r_squared=r2)
    if y_target is not None:
        _frac01("the target fraction", y_target)
        out["t_target"] = (-math.log1p(-y_target) / k) ** (1 / n)
    return _out(**out)


def ref_avrami_fit(t, y, y_target=None):
    import numpy as np
    from scipy.stats import linregress
    r = linregress(np.log(t), np.log(-np.log1p(-np.array(y))))
    k = math.exp(r.intercept)
    out = {"n": r.slope, "log10_k": r.intercept / math.log(10), "t_half": (math.log(2) / k) ** (1 / r.slope)}
    if y_target is not None:
        out["t_target"] = (-math.log1p(-y_target) / k) ** (1 / r.slope)
    return out


def _rand_avrami(rng):
    n, tau = rng.uniform(1, 4), _lu(rng, 10, 1e4)
    m = rng.randint(2, 7)
    ts = sorted({round(tau * rng.uniform(0.3, 2.0), 4) for _ in range(m)})
    if len(ts) < 2:
        ts = [tau * 0.5, tau * 1.5]
    ys = [min(0.995, max(0.005, 1 - math.exp(-((v / tau) ** n) * math.exp(rng.gauss(0, 0.03))))) for v in ts]
    x = {"t": ts, "y": [round(v, 6) for v in ys]}
    if rng.random() < 0.5:
        x["y_target"] = rng.uniform(0.05, 0.95)
    return x


_add("avrami_kinetics",
     name="Avrami (JMAK) transformation kinetics", topic="phase transformations / kinetics",
     purpose="The Avrami exponent n and rate constant k of y = 1 - exp(-k t^n) from transformed fractions at two or more times, the half-transformation time and the time for a target fraction.",
     inputs=[_vec("t", "times (any one unit, e.g. s)"), _vec("y", "transformed fractions (0 to 1)"),
             _numin("y_target", "fraction whose time is wanted (optional)", required=False)],
     outputs=[_o("n", "number", "Avrami exponent"), _o("k", "number", "rate constant (time unit^-n)"), _o("log10_k", "number", "log10 of k"),
              _o("t_half", "number", "time to 50 % transformation (time unit of t)"), _o("r_squared", "number", "r^2 of the fit"),
              _o("t_target", "number", "time to the target fraction")],
     primary="n", conditions=["equal lengths, at least two distinct times", "0 < y < 1, rising with t"],
     method="least-squares line of ln(-ln(1 - y)) against ln t: slope n, intercept ln k",
     latex=r"y = 1 - e^{-kt^n},\quad \ln\ln\frac{1}{1-y} = \ln k + n\ln t",
     run=avrami_fit,
     parse=lambda q: _pairs(q, r"avrami kinetics|avrami fit|jmak fit", {"t": "t", "y": "y", "y_target": "y_target"}, vectors={"t", "y"}),
     reference=ref_avrami_fit,
     wrong=_safe(lambda t, y, y_target=None: (lambda b: _out(n=b[1], k=math.exp(b[0]), log10_k=b[0] / math.log(10),
                                                            t_half=(0.5 / math.exp(b[0])) ** (1 / b[1]), r_squared=b[2],
                                                            **({} if y_target is None else {"t_target": (y_target / math.exp(b[0])) ** (1 / b[1])})))(
         _linfit([math.log(v) for v in t], [math.log(v) for v in y]))),
     random_inputs=_rand_avrami,
     templates=["Avrami kinetics: t = [50, 100], y = [0.2, 0.8]", "Avrami kinetics: t = [50, 100], y = [0.2, 0.8], y_target = 0.95"],
     examples=[{"inputs": {"t": [50, 100], "y": [0.2, 0.8]},
                "outputs": {"n": (math.log(-math.log(0.2)) - math.log(-math.log(0.8))) / math.log(2)}}],
     questions=[{"q": "Avrami kinetics: t = [50, 100], y = [0.2, 0.8]", "outputs": {"n": (math.log(-math.log(0.2)) - math.log(-math.log(0.8))) / math.log(2)}},
                {"q": "Avrami kinetics: t = [50, 100], y = [0.2, 1.2]", "outcome": "INVALID_INPUT"}])


# ================================================================================================ compositions
_EL = re.compile(r"([A-Z][a-z]?)(\d*\.?\d*)")


def _formula_counts(formula: str) -> dict:
    """Element counts of a chemical formula such as C2H3Cl or C6H10O5 or (CH2)2O (plain Python)."""
    f = str(formula).replace(" ", "")
    if not f or not re.fullmatch(r"[A-Za-z0-9().]+", f):
        raise ValueError("write the formula with element symbols, counts and parentheses only (e.g. C2H3Cl)")

    def parse(s, i):
        counts = {}
        while i < len(s):
            if s[i] == "(":
                inner, i = parse(s, i + 1)
                m = re.match(r"\d+", s[i:])
                mult = int(m.group()) if m else 1
                i += len(m.group()) if m else 0
                for k, v in inner.items():
                    counts[k] = counts.get(k, 0) + v * mult
            elif s[i] == ")":
                return counts, i + 1
            else:
                m = _EL.match(s, i)
                if not m or m.end() == i:
                    raise ValueError(f"cannot read the formula at '{s[i:]}'")
                el, num = m.group(1), m.group(2)
                if el not in ATOMIC_WEIGHT:
                    raise ValueError(f"unknown element {el}")
                counts[el] = counts.get(el, 0) + (float(num) if num else 1)
                i = m.end()
        return counts, i
    counts, i = parse(f, 0)
    if i != len(f) or not counts:
        raise ValueError("unbalanced parentheses in the formula")
    return counts


def _formula_mass(formula):
    return sum(ATOMIC_WEIGHT[k] * v for k, v in _formula_counts(formula).items())


def wt_to_at(element1, element2, wt1):
    for e in (element1, element2):
        if e not in ATOMIC_WEIGHT:
            raise ValueError(f"unknown element {e}")
    if element1 == element2:
        raise ValueError("the two elements must differ")
    if not 0 <= wt1 <= 100:
        raise ValueError("the weight percent must lie between 0 and 100")
    A1, A2 = ATOMIC_WEIGHT[element1], ATOMIC_WEIGHT[element2]
    at1 = wt1 * A2 / (wt1 * A2 + (100 - wt1) * A1) * 100
    return _out(at1=at1, at2=100 - at1)


def at_to_wt(element1, element2, at1):
    for e in (element1, element2):
        if e not in ATOMIC_WEIGHT:
            raise ValueError(f"unknown element {e}")
    if element1 == element2:
        raise ValueError("the two elements must differ")
    if not 0 <= at1 <= 100:
        raise ValueError("the atomic percent must lie between 0 and 100")
    A1, A2 = ATOMIC_WEIGHT[element1], ATOMIC_WEIGHT[element2]
    wt1 = at1 * A1 / (at1 * A1 + (100 - at1) * A2) * 100
    return _out(wt1=wt1, wt2=100 - wt1)


def ref_wt_to_at(element1, element2, wt1):
    from pymatgen.core import Composition
    c = Composition.from_weight_dict({element1: wt1, element2: 100 - wt1}) if 0 < wt1 < 100 else Composition({element1 if wt1 else element2: 1})
    a1 = c.get_atomic_fraction(element1) * 100
    return {"at1": a1, "at2": 100 - a1}


def ref_at_to_wt(element1, element2, at1):
    from pymatgen.core import Composition
    c = Composition({element1: at1, element2: 100 - at1})
    w1 = c.get_wt_fraction(element1) * 100
    return {"wt1": float(w1), "wt2": 100 - float(w1)}


_ELS = [e for e in ATOMIC_WEIGHT if e not in ("Tc", "Pm", "Po", "At", "Rn", "Fr", "Ra", "Ac", "Np", "Pu")]


def _rand_pair(rng):
    a = rng.choice(_ELS)
    b = rng.choice([e for e in _ELS if e != a])
    return a, b


_ELN = {"element1": "element1", "element2": "element2", "el1": "element1", "el2": "element2"}

_add("weight_to_atomic_percent",
     name="Weight percent to atomic percent (binary alloy)", topic="phase diagrams / composition",
     purpose="Convert the composition of a binary alloy from weight percent to atomic percent.",
     inputs=[_text("element1", "symbol of element 1"), _text("element2", "symbol of element 2"), _numin("wt1", "weight percent of element 1")],
     outputs=[_o("at1", "number", "atomic percent of element 1"), _o("at2", "number", "atomic percent of element 2")],
     primary="at1", conditions=["known elements, different", "0 <= wt1 <= 100"],
     method="C1' = C1 A2 / (C1 A2 + C2 A1) x 100", latex=r"C_1' = \frac{C_1A_2}{C_1A_2 + C_2A_1}\times100",
     run=wt_to_at,
     parse=lambda q: _pairs(q, r"weight percent to atomic percent|wt% to at%", {**_ELN, "wt1": "wt1"}, texts={"element1", "element2"}, num_units={"wt1": _WT}),
     reference=ref_wt_to_at,
     wrong=_safe(lambda element1, element2, wt1: (lambda A1, A2: _out(at1=wt1 * A1 / (wt1 * A1 + (100 - wt1) * A2) * 100,
                                                                     at2=100 - wt1 * A1 / (wt1 * A1 + (100 - wt1) * A2) * 100))(ATOMIC_WEIGHT[element1], ATOMIC_WEIGHT[element2])),
     random_inputs=lambda rng: (lambda p: {"element1": p[0], "element2": p[1], "wt1": round(rng.uniform(1, 99), 3)})(_rand_pair(rng)),
     templates=["Weight percent to atomic percent: element1 = Cu, element2 = Ni, wt1 = 60"],
     examples=[{"inputs": {"element1": "Cu", "element2": "Ni", "wt1": 60.0}, "outputs": {"at1": 60 * 58.6934 / (60 * 58.6934 + 40 * 63.546) * 100}}],
     questions=[{"q": "Weight percent to atomic percent: element1 = Cu, element2 = Ni, wt1 = 60", "outputs": {"at1": 60 * 58.6934 / (60 * 58.6934 + 40 * 63.546) * 100}},
                {"q": "Weight percent to atomic percent: element1 = Cu, element2 = Xx, wt1 = 60", "outcome": "INVALID_INPUT"}])

_add("atomic_to_weight_percent",
     name="Atomic percent to weight percent (binary alloy)", topic="phase diagrams / composition",
     purpose="Convert the composition of a binary alloy from atomic percent to weight percent.",
     inputs=[_text("element1", "symbol of element 1"), _text("element2", "symbol of element 2"), _numin("at1", "atomic percent of element 1")],
     outputs=[_o("wt1", "number", "weight percent of element 1"), _o("wt2", "number", "weight percent of element 2")],
     primary="wt1", conditions=["known elements, different", "0 <= at1 <= 100"],
     method="C1 = C1' A1 / (C1' A1 + C2' A2) x 100", latex=r"C_1 = \frac{C_1'A_1}{C_1'A_1 + C_2'A_2}\times100",
     run=at_to_wt,
     parse=lambda q: _pairs(q, r"atomic percent to weight percent|at% to wt%", {**_ELN, "at1": "at1"}, texts={"element1", "element2"}, num_units={"at1": _WT}),
     reference=ref_at_to_wt,
     wrong=_safe(lambda element1, element2, at1: (lambda A1, A2: _out(wt1=at1 * A2 / (at1 * A2 + (100 - at1) * A1) * 100,
                                                                     wt2=100 - at1 * A2 / (at1 * A2 + (100 - at1) * A1) * 100))(ATOMIC_WEIGHT[element1], ATOMIC_WEIGHT[element2])),
     random_inputs=lambda rng: (lambda p: {"element1": p[0], "element2": p[1], "at1": round(rng.uniform(1, 99), 3)})(_rand_pair(rng)),
     templates=["Atomic percent to weight percent: element1 = Al, element2 = Cu, at1 = 90"],
     examples=[{"inputs": {"element1": "Al", "element2": "Cu", "at1": 90.0}, "outputs": {"wt1": 90 * 26.9815386 / (90 * 26.9815386 + 10 * 63.546) * 100}}],
     questions=[{"q": "Atomic percent to weight percent: element1 = Al, element2 = Cu, at1 = 90", "outputs": {"wt1": 90 * 26.9815386 / (90 * 26.9815386 + 10 * 63.546) * 100}}])


# ================================================================================================ polymers
def mw_averages(M, fraction, basis="number"):
    if len(M) != len(fraction):
        raise ValueError("M and fraction must have the same length")
    for v in M:
        _pos("every molecular weight", v)
    if any(f < 0 for f in fraction) or sum(fraction) <= 0:
        raise ValueError("fractions must be non-negative and not all zero")
    s = sum(fraction)
    f = [v / s for v in fraction]                               # normalised
    if basis == "number":
        Mn = sum(x * m for x, m in zip(f, M))
        Mw = sum(x * m * m for x, m in zip(f, M)) / Mn
    else:
        Mw = sum(w * m for w, m in zip(f, M))
        Mn = 1 / sum(w / m for w, m in zip(f, M))
    return _out(Mn=Mn, Mw=Mw, PDI=Mw / Mn)


def ref_mw_averages(M, fraction, basis="number"):
    """The other route for each basis: number fractions converted to weight fractions (and back), then scipy's
    weighted harmonic mean and numpy's weighted mean."""
    import numpy as np
    from scipy.stats import hmean
    M, f = np.array(M, float), np.array(fraction, float)
    f = f / f.sum()
    w = f * M / np.sum(f * M) if basis == "number" else f
    keep = w > 0
    Mn = float(hmean(M[keep], weights=w[keep]))
    Mw = float(np.average(M, weights=w))
    return {"Mn": Mn, "Mw": Mw, "PDI": Mw / Mn}


def _rand_mw(rng):
    k = rng.randint(3, 10)
    M = sorted(round(_lu(rng, 5e3, 5e5), 1) for _ in range(k))
    return {"M": M, "fraction": [round(rng.uniform(0.01, 1), 4) for _ in range(k)], "basis": rng.choice(["number", "weight"])}


_add("molecular_weight_averages",
     name="Number- and weight-average molecular weight and dispersity", topic="polymers / molecular weight",
     purpose="Mn, Mw and the dispersity Mw/Mn of a polymer from a molecular-weight distribution given as number fractions or weight fractions per size range.",
     inputs=[_vec("M", "mean molecular weight of each size range in g/mol", lo=1), _vec("fraction", "number or weight fraction of each range (normalised)", lo=1),
             _choice("basis", ["number", "weight"], "whether the fractions are number (x_i) or weight (w_i) fractions (default number)", required=False)],
     outputs=[_o("Mn", "number", "number-average molecular weight in g/mol"), _o("Mw", "number", "weight-average molecular weight in g/mol"),
              _o("PDI", "number", "dispersity (polydispersity index) Mw/Mn")],
     primary="Mw", conditions=["equal lengths", "M > 0", "fractions >= 0, not all zero (they are normalised)"],
     method="number basis: Mn = sum x_i M_i, Mw = sum x_i M_i^2 / Mn; weight basis: Mw = sum w_i M_i, Mn = 1/sum(w_i/M_i)",
     latex=r"\bar M_n = \sum x_iM_i,\quad \bar M_w = \sum w_iM_i,\quad \mathrm{PDI} = \bar M_w/\bar M_n",
     run=mw_averages,
     parse=lambda q: _pairs(q, r"molecular weight averages|polymer molecular weight", {"M": "M", "fraction": "fraction", "x": "fraction", "w": "fraction", "basis": "basis"},
                            vectors={"M", "fraction"}, words={"basis"}),
     reference=ref_mw_averages,
     wrong=_safe(lambda M, fraction, basis="number": (lambda o: dict(o, Mw=o["Mn"], Mn=o["Mw"], PDI=o["Mn"] / o["Mw"]))(
         mw_averages(M, fraction, "weight" if basis == "number" else "number"))),
     random_inputs=_rand_mw,
     templates=["Molecular weight averages: M = [10000, 20000, 30000], fraction = [0.2, 0.5, 0.3], basis = number"],
     examples=[{"inputs": {"M": [10000, 20000, 30000], "fraction": [0.2, 0.5, 0.3], "basis": "number"},
                "outputs": {"Mn": 21000.0, "Mw": (0.2e8 + 0.5 * 4e8 + 0.3 * 9e8) / 21000}},
               {"inputs": {"M": [10000, 20000], "fraction": [0.5, 0.5], "basis": "weight"}, "outputs": {"Mw": 15000.0, "Mn": 1 / (0.5e-4 + 0.25e-4)}}],
     questions=[{"q": "Molecular weight averages: M = [10000, 20000, 30000], fraction = [0.2, 0.5, 0.3], basis = number", "outputs": {"Mn": 21000.0}},
                {"q": "Molecular weight averages: M = [10000, 20000, 30000], fraction = [0.2, 0.5]", "outcome": "INVALID_INPUT"}])


def degree_of_polymerization(formula, Mn):
    _pos("the number-average molecular weight", Mn)
    m = _formula_mass(formula)
    return _out(repeat_unit_g_mol=m, DP=Mn * 1000 / m)


def ref_degree_of_polymerization(formula, Mn):
    from pymatgen.core import Composition
    m = float(Composition(formula).weight)
    return {"repeat_unit_g_mol": m, "DP": Mn * 1000 / m}


_REPEAT = ["C2H4", "C3H6", "C2H3Cl", "C8H8", "C2F4", "C5H8O2", "C4H6", "C2H4O", "C10H8O4", "C6H11NO", "C12H22N2O2", "CH2O",
           "C2H6OSi", "C3H3N", "C4H6O2", "C2H2Cl2", "C6H10O5", "C16H14O3", "(CH2)2O", "C4H8"]

_add("degree_of_polymerization",
     name="Repeat-unit molecular weight and degree of polymerization", topic="polymers / molecular weight",
     purpose="The molecular weight of a repeat unit from its formula and the number-average degree of polymerization DP = Mn / m.",
     inputs=[_text("formula", "repeat-unit formula, e.g. C2H3Cl"), _phys("Mn", "molar_mass", "number-average molecular weight")],
     outputs=[_o("repeat_unit_g_mol", "number", "repeat-unit molecular weight in g/mol"), _o("DP", "number", "degree of polymerization")],
     primary="DP", conditions=["a valid formula", "Mn > 0"],
     method="m = sum of atomic weights of the repeat unit; DP = Mn/m", latex=r"\mathrm{DP} = \frac{\bar M_n}{m}",
     run=degree_of_polymerization,
     parse=lambda q: _pairs(q, r"degree of polymeri[sz]ation", {"formula": "formula", "repeat_unit": "formula", "Mn": "Mn"}, phys={"Mn"}, texts={"formula"}),
     reference=ref_degree_of_polymerization,
     wrong=_safe(lambda formula, Mn: (lambda m: _out(repeat_unit_g_mol=m, DP=Mn * 1000 / m))(sum(_formula_counts(formula).values()) * 12.0)),
     random_inputs=lambda rng: {"formula": rng.choice(_REPEAT), "Mn": _lu(rng, 5, 2000)},
     templates=["Degree of polymerization: formula = C2H3Cl, Mn = 106000 g/mol"],
     examples=[{"inputs": {"formula": "C2H3Cl", "Mn": 106.0}, "outputs": {"repeat_unit_g_mol": 2 * 12.0107 + 3 * 1.00794 + 35.453,
                                                                        "DP": 106000 / (2 * 12.0107 + 3 * 1.00794 + 35.453)}}],
     questions=[{"q": "Degree of polymerization: formula = C2H3Cl, Mn = 106000 g/mol", "outputs": {"DP": 106000 / (2 * 12.0107 + 3 * 1.00794 + 35.453)}},
                {"q": "Degree of polymerization: formula = C2Xq3, Mn = 106000 g/mol", "outcome": "INVALID_INPUT"}])


def copolymer_dp(formula1, formula2, f1, Mn):
    _frac01("the mole fraction of repeat unit 1", f1, closed=True)
    _pos("Mn", Mn)
    m1, m2 = _formula_mass(formula1), _formula_mass(formula2)
    m = f1 * m1 + (1 - f1) * m2
    return _out(m_avg_g_mol=m, DP=Mn * 1000 / m)


def ref_copolymer_dp(formula1, formula2, f1, Mn):
    from pymatgen.core import Composition
    m = f1 * float(Composition(formula1).weight) + (1 - f1) * float(Composition(formula2).weight)
    return {"m_avg_g_mol": m, "DP": Mn * 1000 / m}


_add("copolymer_degree_of_polymerization",
     name="Average repeat-unit weight and degree of polymerization of a copolymer", topic="polymers / copolymers",
     purpose="The average repeat-unit molecular weight of a copolymer from the two repeat-unit formulas and the mole fraction of the first, and its degree of polymerization.",
     inputs=[_text("formula1", "formula of repeat unit 1"), _text("formula2", "formula of repeat unit 2"),
             _numin("f1", "mole fraction of repeat unit 1 (0 to 1)"), _phys("Mn", "molar_mass", "number-average molecular weight")],
     outputs=[_o("m_avg_g_mol", "number", "average repeat-unit molecular weight in g/mol"), _o("DP", "number", "degree of polymerization")],
     primary="DP", conditions=["valid formulas", "0 <= f1 <= 1", "Mn > 0"],
     method="m = f1 m1 + (1 - f1) m2; DP = Mn/m", latex=r"\bar m = \sum f_jm_j,\quad \mathrm{DP} = \frac{\bar M_n}{\bar m}",
     run=copolymer_dp,
     parse=lambda q: _pairs(q, r"copolymer degree of polymeri[sz]ation", {"formula1": "formula1", "formula2": "formula2", "f1": "f1", "Mn": "Mn"},
                            phys={"Mn"}, texts={"formula1", "formula2"}),
     reference=ref_copolymer_dp,
     wrong=_safe(lambda formula1, formula2, f1, Mn: (lambda m: _out(m_avg_g_mol=m, DP=Mn * 1000 / m))(
         (1 - f1) * _formula_mass(formula1) + f1 * _formula_mass(formula2))),
     random_inputs=lambda rng: (lambda a, b: {"formula1": a, "formula2": b, "f1": rng.uniform(0.05, 0.95), "Mn": _lu(rng, 5, 2000)})(
         *rng.sample(_REPEAT, 2)),
     templates=["Copolymer degree of polymerization: formula1 = C4H6, formula2 = C8H8, f1 = 0.75, Mn = 350000 g/mol"],
     examples=[{"inputs": {"formula1": "C4H6", "formula2": "C8H8", "f1": 0.75, "Mn": 350.0},
                "outputs": {"m_avg_g_mol": 0.75 * (4 * 12.0107 + 6 * 1.00794) + 0.25 * (8 * 12.0107 + 8 * 1.00794)}}],
     questions=[{"q": "Copolymer degree of polymerization: formula1 = C4H6, formula2 = C8H8, f1 = 0.75, Mn = 350000 g/mol",
                 "outputs": {"m_avg_g_mol": 0.75 * (4 * 12.0107 + 6 * 1.00794) + 0.25 * (8 * 12.0107 + 8 * 1.00794)}}])


def crystallinity(rho_s, rho_c, rho_a):
    _pos("the densities", rho_s, rho_c, rho_a)
    if not rho_a < rho_s < rho_c:
        raise ValueError("the sample density must lie between the amorphous and the crystalline densities")
    pc = rho_c * (rho_s - rho_a) / (rho_s * (rho_c - rho_a)) * 100
    return _out(crystallinity_percent=pc)


def ref_crystallinity(rho_s, rho_c, rho_a):
    """Mass and volume balance of 1 kg of sample solved with numpy: w_c + w_a = 1, w_c/rho_c + w_a/rho_a = 1/rho_s."""
    import numpy as np
    w = np.linalg.solve(np.array([[1.0, 1.0], [1 / rho_c, 1 / rho_a]]), np.array([1.0, 1 / rho_s]))
    return {"crystallinity_percent": float(w[0]) * 100}


_add("polymer_crystallinity",
     name="Percent crystallinity of a polymer from densities", topic="polymers / crystallinity",
     purpose="The mass percent crystallinity of a semicrystalline polymer from its density and the densities of the fully crystalline and fully amorphous polymer.",
     inputs=[_phys("rho_s", "density", "density of the sample"), _phys("rho_c", "density", "density of the totally crystalline polymer"),
             _phys("rho_a", "density", "density of the totally amorphous polymer")],
     outputs=[_o("crystallinity_percent", "number", "percent crystallinity (by mass)")],
     primary="crystallinity_percent", conditions=["rho_a < rho_s < rho_c"],
     method="%cryst = rho_c (rho_s - rho_a) / (rho_s (rho_c - rho_a)) x 100",
     latex=r"\%\,\text{cryst} = \frac{\rho_c(\rho_s-\rho_a)}{\rho_s(\rho_c-\rho_a)}\times100",
     run=crystallinity,
     parse=lambda q: _pairs(q, r"polymer crystallinity|percent crystallinity", {"rho_s": "rho_s", "rho_c": "rho_c", "rho_a": "rho_a"}, phys={"rho_s", "rho_c", "rho_a"}),
     reference=ref_crystallinity,
     wrong=_safe(lambda rho_s, rho_c, rho_a: _out(crystallinity_percent=(rho_s - rho_a) / (rho_c - rho_a) * 100)),
     random_inputs=lambda rng: (lambda a, c: {"rho_a": a, "rho_c": c, "rho_s": rng.uniform(a + 0.02 * (c - a), c - 0.02 * (c - a))})(
         rng.uniform(800, 1400), rng.uniform(1450, 2300)),
     templates=["Polymer crystallinity: rho_s = 0.925 g/cm^3, rho_c = 1.00 g/cm^3, rho_a = 0.87 g/cm^3"],
     examples=[{"inputs": {"rho_s": 925.0, "rho_c": 1000.0, "rho_a": 870.0}, "outputs": {"crystallinity_percent": 1000 * 55 / (925 * 130) * 100}}],
     questions=[{"q": "Polymer crystallinity: rho_s = 0.925 g/cm^3, rho_c = 1.00 g/cm^3, rho_a = 0.87 g/cm^3",
                 "outputs": {"crystallinity_percent": 1000 * 55 / (925 * 130) * 100}},
                {"q": "Polymer crystallinity: rho_s = 1.2 g/cm^3, rho_c = 1.00 g/cm^3, rho_a = 0.87 g/cm^3", "outcome": "INVALID_INPUT"}])


def rubber_elasticity(rho, Mc, T, stretch):
    _pos("the density, Mc, temperature and stretch", rho, Mc, T, stretch)
    G = rho * RGAS * T / Mc
    s = G * (stretch - stretch ** -2)
    return _out(G_MPa=G / 1e6, stress_MPa=s / 1e6, true_stress_MPa=s * stretch / 1e6)


_RUBBER: dict = {}


def ref_rubber_elasticity(rho, Mc, T, stretch):
    """Affine network: strain energy W = (G/2)(l^2 + 2/l - 3) per unit undeformed volume; the engineering stress is
    dW/dl (sympy). G = n k T with n = rho N_A / Mc network strands per volume (scipy constants)."""
    if "f" not in _RUBBER:
        import sympy as sp
        lam, G_ = sp.symbols("lambda G", positive=True)
        W = G_ / 2 * (lam ** 2 + 2 / lam - 3)
        _RUBBER["f"] = sp.lambdify((G_, lam), sp.diff(W, lam), "math")
    from scipy.constants import Avogadro, Boltzmann
    G = rho * Avogadro / Mc * Boltzmann * T
    s = _RUBBER["f"](G, stretch)
    return {"G_MPa": G / 1e6, "stress_MPa": s / 1e6, "true_stress_MPa": s * stretch / 1e6}


_add("rubber_elasticity",
     name="Stress in a stretched rubber (affine network)", topic="polymers / elastomers",
     purpose="The shear modulus G = rho R T / Mc of a crosslinked rubber and its engineering and true stress at a uniaxial stretch ratio lambda.",
     inputs=[_phys("rho", "density", "density of the rubber"), _phys("Mc", "molar_mass", "molecular weight between crosslinks"),
             _phys("T", "temperature", "temperature"), _numin("stretch", "stretch ratio lambda = L/L0")],
     outputs=[_o("G_MPa", "number", "shear modulus in MPa"), _o("stress_MPa", "number", "engineering stress in MPa"),
              _o("true_stress_MPa", "number", "true stress in MPa")],
     primary="stress_MPa", conditions=["all inputs > 0", "ideal affine network, incompressible"],
     method="G = rho R T / Mc; sigma = G (lambda - 1/lambda^2)", latex=r"\sigma = \frac{\rho RT}{M_c}\left(\lambda - \frac{1}{\lambda^2}\right)",
     run=rubber_elasticity,
     parse=lambda q: _pairs(q, r"rubber elasticity|rubber stress", {"rho": "rho", "Mc": "Mc", "T": "T", "stretch": "stretch", "lambda": "stretch"},
                            phys={"rho", "Mc", "T"}),
     reference=ref_rubber_elasticity,
     wrong=_safe(lambda rho, Mc, T, stretch: (lambda G: _out(G_MPa=G / 1e6, stress_MPa=G * (stretch - 1 / stretch) / 1e6, true_stress_MPa=G * (stretch - 1 / stretch) * stretch / 1e6))(
         rho * RGAS * T / Mc)),
     random_inputs=lambda rng: {"rho": rng.uniform(900, 1300), "Mc": rng.uniform(1, 30), "T": rng.uniform(250, 400), "stretch": rng.choice([rng.uniform(0.5, 0.95), rng.uniform(1.1, 5)])},
     templates=["Rubber elasticity: rho = 0.95 g/cm^3, Mc = 5000 g/mol, T = 300 K, stretch = 2"],
     examples=[{"inputs": {"rho": 950.0, "Mc": 5.0, "T": 300.0, "stretch": 2.0}, "outputs": {"stress_MPa": 950 * RGAS * 300 / 5 * 1.75 / 1e6}}],
     questions=[{"q": "Rubber elasticity: rho = 0.95 g/cm^3, Mc = 5000 g/mol, T = 300 K, stretch = 2", "outputs": {"stress_MPa": 950 * RGAS * 300 / 5 * 1.75 / 1e6}},
                {"q": "Rubber elasticity: rho = 0.95 g/cm^3, Mc = 5000 g/mol, T = 300 K, stretch = -2", "outcome": "INVALID_INPUT"}])


def chain_dimensions(n, l, theta=109.47):
    n = int(n)
    if n < 1:
        raise ValueError("the number of bonds must be at least 1")
    _pos("the bond length", l)
    if not 90 < theta < 180:
        raise ValueError("the bond angle must lie between 90 and 180 degrees")
    c = -_cos_deg(theta)                                       # correlation of successive bonds
    if abs(c) < 1e-15:
        r2 = n * l * l
    else:
        r2 = n * l * l * ((1 + c) / (1 - c) - 2 * c * (1 - c ** n) / (n * (1 - c) ** 2))
    return _out(r_rms=math.sqrt(r2), r_rms_nm=math.sqrt(r2) * 1e9, C_n=r2 / (n * l * l),
                contour_nm=n * l * math.sin(math.radians(theta) / 2) * 1e9)


def ref_chain_dimensions(n, l, theta=109.47):
    """<r^2> = sum over all bond pairs of <b_i . b_j> = l^2 c^|i-j| (freely rotating chain), summed with numpy."""
    import numpy as np
    c = -math.cos(math.radians(theta))
    k = np.arange(1, n)
    r2 = l * l * (n + 2 * float(np.sum((n - k) * c ** k)))
    return {"r_rms_nm": math.sqrt(r2) * 1e9, "C_n": r2 / (n * l * l), "contour_nm": n * l * math.sin(math.radians(theta) / 2) * 1e9}


_add("polymer_chain_dimensions",
     name="End-to-end distance of a freely rotating chain", topic="polymers / chain conformation",
     purpose="The root-mean-square end-to-end distance and characteristic ratio of a polymer chain of n bonds of length l with fixed bond angle and free rotation, and its all-trans contour length.",
     inputs=[_intin("n", "number of backbone bonds", 1, 200000), _phys("l", "length", "bond length"),
             _numin("theta", "bond angle in degrees (default 109.47, tetrahedral)", required=False)],
     outputs=[_o("r_rms", "length", "rms end-to-end distance"), _o("r_rms_nm", "number", "rms end-to-end distance in nm"),
              _o("C_n", "number", "characteristic ratio <r^2>/(n l^2)"), _o("contour_nm", "number", "all-trans contour length in nm")],
     primary="r_rms_nm", conditions=["n >= 1", "l > 0", "90 < theta < 180 degrees"],
     method="exact finite-chain sum <r^2> = n l^2 [(1 + c)/(1 - c) - 2c(1 - c^n)/(n(1 - c)^2)], c = -cos(theta)",
     latex=r"\langle r^2\rangle = nl^2\left[\frac{1+c}{1-c} - \frac{2c(1-c^n)}{n(1-c)^2}\right],\ c = -\cos\theta",
     run=chain_dimensions,
     parse=lambda q: _pairs(q, r"polymer chain dimensions|end[- ]to[- ]end distance", {"n": "n", "l": "l", "theta": "theta"}, phys={"l"},
                            num_units={"theta": ["deg", "degrees", "°"]}),
     reference=ref_chain_dimensions,
     wrong=_safe(lambda n, l, theta=109.47: _out(r_rms=math.sqrt(n) * l, r_rms_nm=math.sqrt(n) * l * 1e9, C_n=1.0, contour_nm=n * l * 1e9)),
     random_inputs=lambda rng: {"n": rng.randint(2, 5000), "l": rng.uniform(0.1e-9, 0.2e-9), "theta": rng.uniform(100, 130)},
     templates=["Polymer chain dimensions: n = 10000, l = 0.154 nm, theta = 109.5"],
     examples=[{"inputs": {"n": 1, "l": 0.154e-9}, "outputs": {"r_rms_nm": 0.154}},
               {"inputs": {"n": 2, "l": 0.154e-9, "theta": 120.0}, "outputs": {"r_rms_nm": 0.154 * SQ3}}],
     questions=[{"q": "Polymer chain dimensions: n = 2, l = 0.154 nm, theta = 120", "outputs": {"r_rms_nm": 0.154 * SQ3}},
                {"q": "Polymer chain dimensions: n = 0, l = 0.154 nm", "outcome": "INVALID_INPUT"}])


# ================================================================================================ electronic and thermal
def _nc(m_ratio, T):
    return 2 * (2 * math.pi * m_ratio * ME * KB * T / (HP * HP)) ** 1.5


def intrinsic_carriers(Eg, T, me=1.0, mh=1.0):
    _pos("the band gap, temperature and effective masses", Eg, T, me, mh)
    Nc, Nv = _nc(me, T), _nc(mh, T)
    lnni = 0.5 * math.log(Nc * Nv) - Eg / (2 * KB * T)
    return _out(Nc=Nc, Nv=Nv, ni=math.exp(lnni), log10_ni=lnni / math.log(10), log10_Nc=math.log10(Nc),
                Ei_offset_meV=0.75 * KB * T * math.log(mh / me) / QE * 1000)


def ref_intrinsic_carriers(Eg, T, me=1.0, mh=1.0):
    """Electrons and holes counted by integrating the parabolic densities of states times the Boltzmann occupation
    (scipy quadrature) with the Fermi level at midgap; n p = ni^2 for any Fermi level. The intrinsic level is where
    n = p (brentq on the quadrature counts)."""
    from scipy.constants import Boltzmann, electron_mass, electron_volt, hbar
    from scipy.integrate import quad
    from scipy.optimize import brentq
    kT = Boltzmann * T

    def count(m, offset):                                      # carriers with the band edge `offset` (J) from E_F
        pref = (1 / (2 * math.pi ** 2)) * (2 * m * electron_mass / hbar ** 2) ** 1.5 * kT ** 1.5
        I, _ = quad(lambda x: math.sqrt(x) * math.exp(-x), 0, math.inf, epsabs=0, epsrel=2e-14)
        return pref * I * math.exp(-offset / kT)
    n, p = count(me, Eg / 2), count(mh, Eg / 2)
    ln_ratio = brentq(lambda d: math.log(count(me, Eg / 2 - d)) - math.log(count(mh, Eg / 2 + d)), -Eg / 2, Eg / 2, xtol=1e-30, rtol=8.9e-16)
    return {"log10_ni": 0.5 * math.log10(n * p), "log10_Nc": math.log10(count(me, 0.0)), "Ei_offset_meV": ln_ratio / electron_volt * 1000}


_add("intrinsic_carrier_concentration",
     name="Intrinsic carrier concentration of a semiconductor", topic="electronic properties / semiconductors",
     purpose="The effective densities of states Nc and Nv and the intrinsic carrier concentration ni = sqrt(Nc Nv) exp(-Eg/2kT) from the band gap, temperature and effective masses.",
     inputs=[_phys("Eg", "energy", "band gap"), _phys("T", "temperature", "temperature"),
             _numin("me", "electron density-of-states effective mass / m0 (default 1)", required=False),
             _numin("mh", "hole density-of-states effective mass / m0 (default 1)", required=False)],
     outputs=[_o("Nc", "number_density", "effective density of states, conduction band"), _o("Nv", "number_density", "effective density of states, valence band"),
              _o("ni", "number_density", "intrinsic carrier concentration"), _o("log10_ni", "number", "log10 of ni in m^-3"),
              _o("log10_Nc", "number", "log10 of Nc in m^-3"), _o("Ei_offset_meV", "number", "intrinsic Fermi level above midgap in meV")],
     primary="log10_ni", conditions=["all inputs > 0", "non-degenerate (Boltzmann) statistics, parabolic bands"],
     method="Nc = 2(2 pi me* k T/h^2)^(3/2), ni = sqrt(Nc Nv) exp(-Eg/(2kT)), Ei - midgap = (3/4) k T ln(mh*/me*)",
     latex=r"n_i = \sqrt{N_cN_v}\,e^{-E_g/2kT},\quad N_c = 2\left(\frac{2\pi m_e^*kT}{h^2}\right)^{3/2}",
     run=intrinsic_carriers,
     parse=lambda q: _pairs(q, r"intrinsic carrier concentration", {"Eg": "Eg", "T": "T", "me": "me", "mh": "mh"}, phys={"Eg", "T"}),
     reference=ref_intrinsic_carriers,
     wrong=_safe(lambda Eg, T, me=1.0, mh=1.0: (lambda o: dict(o, log10_ni=o["log10_ni"] - Eg / (2 * KB * T) / math.log(10)))(intrinsic_carriers(Eg, T, me, mh))),
     random_inputs=lambda rng: {"Eg": rng.uniform(0.2, 3.5) * QE, "T": rng.uniform(150, 800), "me": rng.uniform(0.05, 1.5), "mh": rng.uniform(0.05, 1.5)},
     templates=["Intrinsic carrier concentration: Eg = 1.12 eV, T = 300 K, me = 1.08, mh = 0.81"],
     examples=[{"inputs": {"Eg": 1.12 * QE, "T": 300.0, "me": 1.08, "mh": 0.81},
                "outputs": {"log10_ni": (0.5 * math.log(_nc(1.08, 300) * _nc(0.81, 300)) - 1.12 * QE / (2 * KB * 300)) / math.log(10)}}],
     questions=[{"q": "Intrinsic carrier concentration: Eg = 1.12 eV, T = 300 K, me = 1.08, mh = 0.81",
                 "outputs": {"log10_ni": (0.5 * math.log(_nc(1.08, 300) * _nc(0.81, 300)) - 1.12 * QE / (2 * KB * 300)) / math.log(10)}},
                {"q": "Intrinsic carrier concentration: Eg = 1.12 eV", "outcome": "NEEDS_INPUT"}])


def band_gap_fit(T, sigma):
    if len(T) != len(sigma):
        raise ValueError("T and sigma must have the same length")
    for v in list(T) + list(sigma):
        _pos("every temperature and conductivity", v)
    b0, b1, r2 = _linfit([1 / t for t in T], [math.log(s) for s in sigma])
    if b1 >= 0:
        raise ValueError("an intrinsic conductivity rises with temperature")
    return _out(Eg_eV=-2 * b1 * KB / QE, prefactor=math.exp(b0), r_squared=r2)


def ref_band_gap_fit(T, sigma):
    import numpy as np
    from scipy.constants import Boltzmann, electron_volt
    from scipy.stats import linregress
    r = linregress(1 / np.array(T, float), np.log(np.array(sigma, float)))
    return {"Eg_eV": -2 * r.slope * Boltzmann / electron_volt, "prefactor": math.exp(r.intercept)}


def _rand_bandgap(rng):
    T, s = _rand_arrhenius_points(rng, rng.uniform(0.3, 2.5) * QE / (2 * KB), _lu(rng, 1e3, 1e7), 300, 900, 0.03)
    return {"T": T, "sigma": s}


_add("band_gap_from_conductivity",
     name="Band gap from intrinsic conductivities at several temperatures", topic="electronic properties / semiconductors",
     purpose="The band gap of an intrinsic semiconductor from its conductivity at two or more temperatures, with sigma = C exp(-Eg/2kT) (the weak temperature dependence of the prefactor neglected).",
     inputs=[_vec("T", "temperatures in kelvin"), _vec("sigma", "conductivities (any one unit) at those temperatures")],
     outputs=[_o("Eg_eV", "number", "band gap in eV"), _o("prefactor", "number", "prefactor C (unit of sigma)"), _o("r_squared", "number", "r^2 of the fit")],
     primary="Eg_eV", conditions=["equal lengths, at least two distinct temperatures", "sigma rising with T"],
     method="least-squares line of ln sigma against 1/T: slope = -Eg/(2k)", latex=r"\ln\sigma = \ln C - \frac{E_g}{2k}\frac{1}{T}",
     run=band_gap_fit,
     parse=lambda q: _pairs(q, r"band gap from conductivity|band gap fit", {"T": "T", "sigma": "sigma"}, vectors={"T", "sigma"}),
     reference=ref_band_gap_fit,
     wrong=_safe(lambda T, sigma: (lambda o: dict(o, Eg_eV=o["Eg_eV"] / 2))(band_gap_fit(T, sigma))),
     random_inputs=_rand_bandgap,
     templates=["Band gap from conductivity: T = [300, 400], sigma = [2.2, 63]"],
     examples=[{"inputs": {"T": [300, 400], "sigma": [2.2, 63]}, "outputs": {"Eg_eV": 2 * KB * math.log(63 / 2.2) / (1 / 300 - 1 / 400) / QE}}],
     questions=[{"q": "Band gap from conductivity: T = [300, 400], sigma = [2.2, 63]", "outputs": {"Eg_eV": 2 * KB * math.log(63 / 2.2) / (1 / 300 - 1 / 400) / QE}},
                {"q": "Band gap from conductivity: T = [300, 400], sigma = [63, 2.2]", "outcome": "INVALID_INPUT"}])

_LORENZ = math.pi ** 2 / 3 * (KB / QE) ** 2


def wiedemann_franz(T, sigma=None, rho=None):
    _pos("the temperature", T)
    if (sigma is None) == (rho is None):
        raise ValueError("give exactly one of the conductivity sigma and the resistivity rho")
    s = sigma if sigma is not None else 1 / rho if rho > 0 else -1
    _pos("the conductivity", s)
    return _out(kappa=_LORENZ * s * T, lorenz=_LORENZ)


def ref_wiedemann_franz(T, sigma=None, rho=None):
    """The Lorenz number from the Sommerfeld expansion of the free-electron transport integrals:
    L = (k/e)^2 * int x^2 (-df/dx) dx / int (-df/dx) dx over all x (scipy quadrature), f the Fermi function."""
    from scipy.constants import Boltzmann, elementary_charge
    from scipy.integrate import quad
    w = lambda x: math.exp(-abs(x)) / (1 + math.exp(-abs(x))) ** 2           # noqa: E731  -df/dx, even in x
    num, _ = quad(lambda x: 2 * x * x * w(x), 0, math.inf, epsabs=0, epsrel=2e-14)
    den, _ = quad(lambda x: 2 * w(x), 0, math.inf, epsabs=0, epsrel=2e-14)
    L = (Boltzmann / elementary_charge) ** 2 * num / den
    s = sigma if sigma is not None else 1 / rho
    return {"kappa": L * s * T, "lorenz": L}


_add("wiedemann_franz",
     name="Electronic thermal conductivity of a metal (Wiedemann-Franz law)", topic="thermal properties / thermal conductivity",
     purpose="The electronic thermal conductivity kappa = L sigma T of a metal from its electrical conductivity (or resistivity) and temperature, with the Sommerfeld value of the Lorenz number.",
     inputs=[_phys("T", "temperature", "temperature"), _phys("sigma", "electrical_conductivity", "electrical conductivity (or give rho)", required=False),
             _phys("rho", "resistivity", "electrical resistivity (or give sigma)", required=False)],
     outputs=[_o("kappa", "thermal_conductivity", "thermal conductivity"), _o("lorenz", "number", "Lorenz number in W ohm/K^2")],
     primary="kappa", conditions=["T > 0", "exactly one of sigma, rho, positive", "degenerate free-electron metal"],
     method="kappa = L sigma T, L = (pi^2/3)(k/e)^2 = 2.44e-8 W ohm/K^2", latex=r"\kappa = L\sigma T,\quad L = \frac{\pi^2}{3}\left(\frac{k}{e}\right)^2",
     run=wiedemann_franz,
     parse=lambda q: _pairs(q, r"wiedemann[- ]franz", {"T": "T", "sigma": "sigma", "rho": "rho"}, phys={"T", "sigma", "rho"}),
     reference=ref_wiedemann_franz,
     wrong=_safe(lambda T, sigma=None, rho=None: (lambda s: _out(kappa=3 / 2 * (KB / QE) ** 2 * s * T, lorenz=3 / 2 * (KB / QE) ** 2))(
         sigma if sigma is not None else 1 / rho)),
     random_inputs=lambda rng: {"T": rng.uniform(50, 1200), **({"sigma": _lu(rng, 1e6, 7e7)} if rng.random() < 0.5 else {"rho": _lu(rng, 1.5e-8, 1e-6)})},
     templates=["Wiedemann-Franz: T = 300 K, sigma = 5.96e7 S/m", "Wiedemann-Franz: T = 300 K, rho = 1.68e-8 ohm*m"],
     examples=[{"inputs": {"T": 300.0, "sigma": 5.96e7}, "outputs": {"kappa": _LORENZ * 5.96e7 * 300}}],
     questions=[{"q": "Wiedemann-Franz: T = 300 K, sigma = 5.96e7 S/m", "outputs": {"kappa": _LORENZ * 5.96e7 * 300}},
                {"q": "Wiedemann-Franz: T = 300 K, sigma = 5.96e7 S/m, rho = 1.68e-8 ohm*m", "outcome": "INVALID_INPUT"}])


def debye_heat_capacity(theta_D, T):
    _pos("the Debye temperature and the temperature", theta_D, T)
    import mpmath
    xD = theta_D / T
    if xD > 300:
        raise ValueError("T is below theta_D/300: use the low-temperature T^3 law")
    with mpmath.workdps(30):
        I = mpmath.quad(lambda x: x ** 4 * mpmath.exp(-x) / (1 - mpmath.exp(-x)) ** 2, [0, min(xD, 1), xD] if xD > 1 else [0, xD])
    Cv = 9 * RGAS * (T / theta_D) ** 3 * float(I)
    return _out(Cv=Cv, Cv_over_3R=Cv / (3 * RGAS))


def ref_debye_heat_capacity(theta_D, T):
    from scipy.constants import gas_constant
    from scipy.integrate import quad
    xD = theta_D / T
    f = lambda x: x ** 4 * math.exp(-x) / (-math.expm1(-x)) ** 2 if x > 0 else 0.0     # noqa: E731
    I, _ = quad(f, 0, xD, epsabs=0, epsrel=2e-14, limit=200)
    Cv = 9 * gas_constant * (T / theta_D) ** 3 * I
    return {"Cv": Cv, "Cv_over_3R": Cv / (3 * gas_constant)}


_add("debye_heat_capacity",
     name="Molar heat capacity of a solid (Debye model)", topic="thermal properties / heat capacity",
     purpose="The constant-volume molar heat capacity of a monatomic solid from the Debye model at temperature T.",
     inputs=[_phys("theta_D", "temperature", "Debye temperature"), _phys("T", "temperature", "temperature")],
     outputs=[_o("Cv", "molar_heat_capacity", "molar heat capacity at constant volume"), _o("Cv_over_3R", "number", "Cv / 3R (1 at high T)")],
     primary="Cv", conditions=["theta_D, T > 0", "T >= theta_D/300"],
     method="Cv = 9R (T/theta_D)^3 int_0^(theta_D/T) x^4 e^x/(e^x - 1)^2 dx (mpmath quadrature, 30 digits)",
     latex=r"C_v = 9R\left(\frac{T}{\Theta_D}\right)^3\int_0^{\Theta_D/T}\frac{x^4e^x}{(e^x-1)^2}\,dx",
     run=debye_heat_capacity,
     parse=lambda q: _pairs(q, r"debye heat capacity", {"theta_D": "theta_D", "T": "T"}, phys={"theta_D", "T"}),
     reference=ref_debye_heat_capacity,
     wrong=_safe(lambda theta_D, T: (lambda o: dict(o, Cv=o["Cv"] / 3, Cv_over_3R=o["Cv_over_3R"] / 3))(debye_heat_capacity(theta_D, T))),
     random_inputs=lambda rng: {"theta_D": rng.uniform(100, 2000), "T": rng.uniform(5, 1500)},
     templates=["Debye heat capacity: theta_D = 343 K, T = 300 K"],
     examples=[{"inputs": {"theta_D": 343.0, "T": 1e6}, "outputs": {"Cv_over_3R": 1 - (343e-6) ** 2 / 20 + (343e-6) ** 4 / 560}}],
     questions=[{"q": "Debye heat capacity: theta_D = 343 K, T = 1000000 K", "outputs": {"Cv_over_3R": 1 - (343e-6) ** 2 / 20 + (343e-6) ** 4 / 560}},
                {"q": "Debye heat capacity: theta_D = 343 K", "outcome": "NEEDS_INPUT"}])


def einstein_heat_capacity(theta_E, T):
    _pos("the Einstein temperature and the temperature", theta_E, T)
    x = theta_E / T
    if x > 700:
        raise ValueError("T is far below theta_E: the heat capacity underflows to 0")
    Cv = 3 * RGAS * x * x * math.exp(-x) / (-math.expm1(-x)) ** 2
    return _out(Cv=Cv, Cv_over_3R=Cv / (3 * RGAS))


def ref_einstein_heat_capacity(theta_E, T):
    """dU/dT of the Einstein energy U = 3 R theta_E / (exp(theta_E/T) - 1) per mole, by mpmath's high-precision
    numerical differentiation."""
    import mpmath
    with mpmath.workdps(40):
        U = lambda t: 3 * mpmath.mpf(RGAS) * theta_E / mpmath.expm1(theta_E / t)    # noqa: E731
        Cv = float(mpmath.diff(U, mpmath.mpf(T)))
    return {"Cv": Cv, "Cv_over_3R": Cv / (3 * RGAS)}


_add("einstein_heat_capacity",
     name="Molar heat capacity of a solid (Einstein model)", topic="thermal properties / heat capacity",
     purpose="The constant-volume molar heat capacity of a solid from the Einstein model (all atoms vibrating at one frequency).",
     inputs=[_phys("theta_E", "temperature", "Einstein temperature"), _phys("T", "temperature", "temperature")],
     outputs=[_o("Cv", "molar_heat_capacity", "molar heat capacity at constant volume"), _o("Cv_over_3R", "number", "Cv / 3R")],
     primary="Cv", conditions=["theta_E, T > 0"],
     method="Cv = 3R x^2 e^x/(e^x - 1)^2, x = theta_E/T", latex=r"C_v = 3R\left(\frac{\Theta_E}{T}\right)^2\frac{e^{\Theta_E/T}}{(e^{\Theta_E/T}-1)^2}",
     run=einstein_heat_capacity,
     parse=lambda q: _pairs(q, r"einstein heat capacity", {"theta_E": "theta_E", "T": "T"}, phys={"theta_E", "T"}),
     reference=ref_einstein_heat_capacity,
     wrong=_safe(lambda theta_E, T: (lambda x: _out(Cv=3 * RGAS * x * math.exp(-x) / (-math.expm1(-x)), Cv_over_3R=x * math.exp(-x) / (-math.expm1(-x))))(theta_E / T)),
     random_inputs=lambda rng: {"theta_E": rng.uniform(100, 1500), "T": rng.uniform(20, 1500)},
     templates=["Einstein heat capacity: theta_E = 240 K, T = 300 K"],
     examples=[{"inputs": {"theta_E": 300.0, "T": 300.0}, "outputs": {"Cv_over_3R": math.e / (math.e - 1) ** 2}}],
     questions=[{"q": "Einstein heat capacity: theta_E = 300 K, T = 300 K", "outputs": {"Cv_over_3R": math.e / (math.e - 1) ** 2}}])


def fermi_energy(rho, A, valence):
    _pos("the density, molar mass and valence", rho, A, valence)
    n = valence * NA * rho / A
    EF = HBAR * HBAR / (2 * ME) * (3 * math.pi * math.pi * n) ** (2 / 3)
    return _out(n=n, log10_n=math.log10(n), EF=EF, EF_eV=EF / QE, TF=EF / KB, vF=math.sqrt(2 * EF / ME))


def ref_fermi_energy(rho, A, valence):
    """E_F as the energy that fills the free-electron density of states g(E) = (1/2 pi^2)(2m/hbar^2)^(3/2) sqrt(E)
    with n electrons per volume (scipy quadrature inside brentq)."""
    from scipy.constants import Avogadro, Boltzmann, electron_mass, electron_volt, hbar
    from scipy.integrate import quad
    from scipy.optimize import brentq
    n = valence * Avogadro * rho / A
    pref = (1 / (2 * math.pi ** 2)) * (2 * electron_mass / hbar ** 2) ** 1.5
    filled = lambda E: pref * quad(lambda e: math.sqrt(e), 0, E, epsabs=0, epsrel=2e-14)[0]   # noqa: E731
    E = brentq(lambda e_ev: filled(e_ev * electron_volt) / n - 1, 0.01, 100, xtol=1e-15, rtol=8.9e-16) * electron_volt
    return {"log10_n": math.log10(n), "EF_eV": E / electron_volt, "TF": E / Boltzmann, "vF": math.sqrt(2 * E / electron_mass)}


_add("fermi_energy_free_electron",
     name="Fermi energy of a free-electron metal", topic="electronic properties / metals",
     purpose="The conduction-electron density, Fermi energy, Fermi temperature and Fermi velocity of a metal from its density, molar mass and number of valence electrons per atom.",
     inputs=[_phys("rho", "density", "density"), _phys("A", "molar_mass", "molar mass"), _numin("valence", "valence electrons per atom")],
     outputs=[_o("n", "number_density", "conduction-electron density"), _o("log10_n", "number", "log10 of n in m^-3"), _o("EF", "energy", "Fermi energy"),
              _o("EF_eV", "number", "Fermi energy in eV"), _o("TF", "temperature", "Fermi temperature"), _o("vF", "speed", "Fermi velocity")],
     primary="EF_eV", conditions=["all inputs > 0", "free-electron model"],
     method="n = z N_A rho/A; E_F = (hbar^2/2m)(3 pi^2 n)^(2/3)", latex=r"E_F = \frac{\hbar^2}{2m}\left(3\pi^2n\right)^{2/3}",
     run=fermi_energy,
     parse=lambda q: _pairs(q, r"fermi energy", {"rho": "rho", "A": "A", "valence": "valence", "z": "valence"}, phys={"rho", "A"}),
     reference=ref_fermi_energy,
     wrong=_safe(lambda rho, A, valence: (lambda o: dict(o, EF_eV=o["EF_eV"] * (3 * math.pi * math.pi) ** (1 / 3), TF=o["TF"] * (3 * math.pi * math.pi) ** (1 / 3),
                                                         vF=o["vF"] * (3 * math.pi * math.pi) ** (1 / 6)))(fermi_energy(rho, A, valence))),
     random_inputs=lambda rng: {"rho": rng.uniform(500, 22000), "A": rng.uniform(0.006, 0.21), "valence": rng.choice([1, 1, 2, 3, 4])},
     templates=["Fermi energy: rho = 8.96 g/cm^3, A = 63.55 g/mol, valence = 1"],
     examples=[{"inputs": {"rho": 8960.0, "A": 0.06355, "valence": 1},
                "outputs": {"EF_eV": HBAR ** 2 / (2 * ME) * (3 * math.pi ** 2 * NA * 8960 / 0.06355) ** (2 / 3) / QE}}],
     questions=[{"q": "Fermi energy: rho = 8.96 g/cm^3, A = 63.55 g/mol, valence = 1",
                 "outputs": {"EF_eV": HBAR ** 2 / (2 * ME) * (3 * math.pi ** 2 * NA * 8960 / 0.06355) ** (2 / 3) / QE}},
                {"q": "Fermi energy: rho = 8.96 g/cm^3, A = 63.55 g/mol", "outcome": "NEEDS_INPUT"}])


# ================================================================================================ definitional additions
# (owner, 2026-10-23: a capability that passes every gate ships; a weaker reference is documented). Arrhenius
# diffusivity, Larson-Miller parameter, linear and volumetric thermal expansion, extrinsic carrier concentrations,
# p-n junction built-in potential.

def arrhenius_diffusivity(D0, Q, T):
    _pos("D0, Q and T", D0, Q, T)
    x = Q / (RGAS * T)
    return _out(D=D0 * math.exp(-x), log10_D=math.log10(D0) - x / math.log(10), Q_over_RT=x)


def ref_arrhenius_diffusivity(D0, Q, T):
    """ln D integrated from the infinite-temperature limit ln D0 along d(ln D)/d(1/T) = -Q/R (scipy quadrature in
    beta = 1/T), with R from scipy.constants."""
    from scipy.constants import gas_constant
    from scipy.integrate import quad
    I, _ = quad(lambda b: -Q / gas_constant, 0.0, 1.0 / T, epsabs=0, epsrel=1e-13)
    return {"log10_D": (math.log(D0) + I) / math.log(10), "Q_over_RT": -I}


_add("arrhenius_diffusivity",
     name="Diffusion coefficient at a temperature (Arrhenius)", topic="diffusion / temperature dependence",
     purpose="The diffusion coefficient D = D0 exp(-Q_d/RT) at temperature T from the pre-exponential factor and the activation energy.",
     inputs=[_phys("D0", "diffusion_coefficient", "pre-exponential factor"), _phys("Q", "molar_energy", "activation energy per mole"),
             _phys("T", "temperature", "absolute temperature")],
     outputs=[_o("D", "diffusion_coefficient", "diffusion coefficient"), _o("log10_D", "number", "log10 of D in m^2/s"),
              _o("Q_over_RT", "number", "Q_d/(R T)")],
     primary="log10_D", conditions=["D0, Q, T > 0"],
     method="D = D0 exp(-Q_d/(R T)), R = N_A k", latex=r"D = D_0\exp\!\left(-\frac{Q_d}{RT}\right)",
     run=arrhenius_diffusivity,
     parse=lambda q: _pairs(q, r"arrhenius diffusivity|diffusion coefficient at (?:a )?temperature", {"D0": "D0", "Q": "Q", "Qd": "Q", "T": "T"},
                            phys={"D0", "Q", "T"}),
     reference=ref_arrhenius_diffusivity,
     wrong=_safe(lambda D0, Q, T: (lambda x: _out(D=D0 * 10 ** (-x), log10_D=math.log10(D0) - x, Q_over_RT=x))(Q / (RGAS * T))),
     random_inputs=lambda rng: {"D0": _lu(rng, 1e-7, 1e-3), "Q": rng.uniform(50e3, 400e3), "T": rng.uniform(400, 1800)},
     templates=["Arrhenius diffusivity: D0 = 2.3e-5 m^2/s, Q = 148 kJ/mol, T = 773 K"],
     examples=[{"inputs": {"D0": 2.3e-5, "Q": 148e3, "T": 773.0}, "outputs": {"D": 2.3e-5 * math.exp(-148e3 / (RGAS * 773.0))}},
               {"inputs": {"D0": 1e-4, "Q": RGAS * 1000 * math.log(10), "T": 1000.0}, "outputs": {"log10_D": -5.0}}],
     questions=[{"q": "Arrhenius diffusivity: D0 = 2.3e-5 m^2/s, Q = 148 kJ/mol, T = 773 K",
                 "outputs": {"log10_D": math.log10(2.3e-5 * math.exp(-148e3 / (RGAS * 773.0)))}},
                {"q": "Arrhenius diffusivity: D0 = 2.3e-5 m^2/s, Q = 148 kJ/mol, T = -10 K", "outcome": "INVALID_INPUT"},
                {"q": "Arrhenius diffusivity: D0 = 2.3e-5 m^2/s, Q = 148 kJ/mol", "outcome": "NEEDS_INPUT"}])


def larson_miller(T, t_r, C=20.0, T2=None):
    _pos("the temperature and the rupture time", T, t_r)
    if not (math.isfinite(C) and 5 <= C <= 50):
        raise ValueError("the Larson-Miller constant C must lie between 5 and 50 (typically 20)")
    h = t_r / 3600.0
    lmp = T * (C + math.log10(h))
    out = dict(LMP=lmp, LMP_thousands=lmp / 1000, log10_t_hours=math.log10(h))
    if T2 is not None:
        _pos("the second temperature", T2)
        l2 = lmp / T2 - C
        if l2 > 12:
            raise ValueError("the rupture time at T2 exceeds 1e12 hours")
        out.update(t_r2_hours=10 ** l2, log10_t2_hours=l2)
    return _out(**out)


def ref_larson_miller(T, t_r, C=20.0, T2=None):
    """The parameter in exact decimal arithmetic (Python's decimal module, 40 digits); the rupture time at T2 by
    scipy brentq on the equal-parameter condition."""
    from decimal import Decimal, getcontext
    getcontext().prec = 40
    h = Decimal(repr(t_r)) / Decimal(3600)
    lmp = Decimal(repr(T)) * (Decimal(repr(C)) + h.log10())
    out = {"LMP": float(lmp), "log10_t_hours": float(h.log10())}
    if T2 is not None:
        from scipy.optimize import brentq
        l2 = brentq(lambda y: T2 * (C + y) - float(lmp), -60, 60, xtol=1e-14, rtol=8.9e-16)
        out["log10_t2_hours"] = l2
    return out


def _rand_lmp(rng):
    x = {"T": rng.uniform(700, 1300), "t_r": _lu(rng, 10, 1e5) * 3600, "C": rng.choice([20.0, 20.0, rng.uniform(15, 25)])}
    if rng.random() < 0.5:
        lmp = x["T"] * (x["C"] + math.log10(x["t_r"] / 3600))
        x["T2"] = lmp / (x["C"] + rng.uniform(0.5, 6))
    return x


_add("larson_miller_parameter",
     name="Larson-Miller parameter (creep rupture)", topic="mechanical properties / creep",
     purpose="The Larson-Miller parameter LMP = T (C + log10 t_r) from a rupture time at a temperature, and the rupture time at another temperature for the same parameter (same stress).",
     inputs=[_phys("T", "temperature", "absolute temperature"), _phys("t_r", "time", "rupture time (hours, h ...)"),
             _numin("C", "Larson-Miller constant (default 20, with t_r in hours)", required=False),
             _phys("T2", "temperature", "another temperature at the same stress (optional)", required=False)],
     outputs=[_o("LMP", "number", "Larson-Miller parameter (K, t in hours)"), _o("LMP_thousands", "number", "LMP / 1000"),
              _o("log10_t_hours", "number", "log10 of the rupture time in hours"),
              _o("t_r2_hours", "number", "rupture time at T2 in hours"), _o("log10_t2_hours", "number", "log10 of the rupture time at T2")],
     primary="LMP", conditions=["T, t_r > 0", "5 <= C <= 50", "time in hours inside the logarithm"],
     method="LMP = T (C + log10 t_r[h]); at T2: log10 t_2 = LMP/T2 - C", latex=r"\mathrm{LMP} = T\left(C + \log_{10} t_r\right)",
     run=larson_miller,
     parse=lambda q: _pairs(q, r"larson[- ]miller(?: parameter)?", {"T": "T", "t_r": "t_r", "tr": "t_r", "C": "C", "T2": "T2"},
                            phys={"T", "t_r", "T2"}),
     reference=ref_larson_miller,
     wrong=_safe(lambda T, t_r, C=20.0, T2=None: (lambda o: dict(o, LMP=_r(T * (C + math.log(t_r / 3600))), LMP_thousands=_r(T * (C + math.log(t_r / 3600)) / 1000)))(
         larson_miller(T, t_r, C, T2))),
     random_inputs=_rand_lmp,
     templates=["Larson-Miller parameter: T = 1073 K, t_r = 1000 h", "Larson-Miller parameter: T = 1073 K, t_r = 1000 h, C = 20, T2 = 1000 K"],
     examples=[{"inputs": {"T": 1073.0, "t_r": 1000 * 3600.0}, "outputs": {"LMP": 1073 * 23.0}},
               {"inputs": {"T": 1000.0, "t_r": 100 * 3600.0, "C": 20.0, "T2": 1100.0}, "outputs": {"LMP": 22000.0, "t_r2_hours": 1.0}}],
     questions=[{"q": "Larson-Miller parameter: T = 1073 K, t_r = 1000 h", "outputs": {"LMP": 1073 * 23.0}},
                {"q": "Larson-Miller parameter: T = 1000 K, t_r = 100 h, C = 20, T2 = 1100 K", "outputs": {"t_r2_hours": 1.0}},
                {"q": "Larson-Miller parameter: T = 1000 K, t_r = 100 h, C = 80", "outcome": "INVALID_INPUT"},
                {"q": "Larson-Miller parameter: T = 1073 K", "outcome": "NEEDS_INPUT"}])


def linear_expansion(alpha, L0, dT):
    _pos("alpha and L0", alpha, L0)
    if not math.isfinite(dT):
        raise ValueError("the temperature change must be a number")
    dL = alpha * L0 * dT
    if L0 + dL <= 0:
        raise ValueError("the length change exceeds the length (outside the linear range)")
    return _out(dL=dL, dL_mm=dL * 1e3, L=L0 + dL, strain=alpha * dT)


def ref_linear_expansion(alpha, L0, dT):
    """The thermal strain integrated over the temperature change (scipy quadrature of alpha dT) applied to the length
    of a one-element bar whose free end moves (numpy nodal solve with zero stiffness load)."""
    from scipy.integrate import quad
    eps, _ = quad(lambda t: alpha, 0.0, dT, epsabs=0, epsrel=1e-13) if dT else (0.0, 0.0)
    return {"dL_mm": eps * L0 * 1e3, "strain": eps, "L": L0 * (1 + eps)}


_add("linear_thermal_expansion",
     name="Linear thermal expansion", topic="thermal properties / thermal expansion",
     purpose="The length change dL = alpha L0 dT and the new length of a bar heated or cooled freely by dT.",
     inputs=[_phys("alpha", "expansion_coefficient", "linear thermal expansion coefficient"), _phys("L0", "length", "initial length"),
             _phys("dT", "temperature_difference", "temperature change (positive = heating)")],
     outputs=[_o("dL", "length", "length change"), _o("dL_mm", "number", "length change in mm"), _o("L", "length", "new length"),
              _o("strain", "number", "thermal strain alpha dT")],
     primary="dL_mm", conditions=["alpha, L0 > 0", "alpha constant over the range (linear, small strain)"],
     method="dL = alpha L0 dT, L = L0 + dL", latex=r"\Delta L = \alpha L_0\,\Delta T",
     run=linear_expansion,
     parse=lambda q: _pairs(q, r"linear thermal expansion", {"alpha": "alpha", "L0": "L0", "dT": "dT"}, phys={"alpha", "L0", "dT"}),
     reference=ref_linear_expansion,
     wrong=_safe(lambda alpha, L0, dT: (lambda dL: _out(dL=dL, dL_mm=dL * 1e3, L=L0 + dL, strain=alpha * (dT + 273.15)))(alpha * L0 * (dT + 273.15))),
     random_inputs=lambda rng: {"alpha": rng.uniform(1e-6, 2.5e-5), "L0": _lu(rng, 1e-3, 100), "dT": rng.choice([-1, 1]) * rng.uniform(1, 400)},
     templates=["Linear thermal expansion: alpha = 17e-6 1/K, L0 = 0.5 m, dT = 100 K"],
     examples=[{"inputs": {"alpha": 17e-6, "L0": 0.5, "dT": 100.0}, "outputs": {"dL_mm": 0.85}},
               {"inputs": {"alpha": 23.6e-6, "L0": 10.0, "dT": -50.0}, "outputs": {"dL_mm": -11.8}}],
     questions=[{"q": "Linear thermal expansion: alpha = 17e-6 1/K, L0 = 0.5 m, dT = 100 K", "outputs": {"dL_mm": 0.85}},
                {"q": "Linear thermal expansion: alpha = 17e-6 1/K, L0 = -0.5 m, dT = 100 K", "outcome": "INVALID_INPUT"},
                {"q": "Linear thermal expansion: alpha = 17e-6 1/K, L0 = 0.5 m", "outcome": "NEEDS_INPUT"}])


def volumetric_expansion(alpha, V0, dT):
    _pos("alpha and V0", alpha, V0)
    if not math.isfinite(dT):
        raise ValueError("the temperature change must be a number")
    s = alpha * dT
    if s <= -1:
        raise ValueError("the strain alpha dT must exceed -1")
    beta = 3 * alpha
    dV = beta * V0 * dT
    dV_exact = V0 * s * (3 + 3 * s + s * s)                    # V0 ((1 + s)^3 - 1) without cancellation
    return _out(beta=beta, dV=dV, dV_cm3=dV * 1e6, V=V0 + dV, dV_exact=dV_exact, dV_exact_cm3=dV_exact * 1e6)


def ref_volumetric_expansion(alpha, V0, dT):
    """The volume of the freely expanded body as the determinant of its cell (numpy), a cube of side V0^(1/3) scaled
    by 1 + alpha dT along each axis; the linearised change from the derivative dV/dT at dT = 0 (sympy)."""
    import numpy as np
    import sympy as sp
    a = V0 ** (1 / 3)
    s = alpha * dT
    cell0 = np.eye(3) * a
    cell = cell0 @ np.diag([1 + s] * 3)
    dV_exact = float(np.linalg.det(cell) - np.linalg.det(cell0))
    t = sp.Symbol("t")
    dVdT = sp.diff(sp.Float(V0, 30) * (1 + sp.Float(alpha, 30) * t) ** 3, t).subs(t, 0)
    dV = float(dVdT) * dT
    return {"dV_cm3": dV * 1e6, "beta": float(dVdT) / V0, "dV_exact_cm3": dV_exact * 1e6}


_add("volumetric_thermal_expansion",
     name="Volumetric thermal expansion", topic="thermal properties / thermal expansion",
     purpose="The volume change dV = beta V0 dT of an isotropic solid with beta = 3 alpha (linearised), and the exact change V0((1 + alpha dT)^3 - 1).",
     inputs=[_phys("alpha", "expansion_coefficient", "LINEAR thermal expansion coefficient"), _phys("V0", "volume", "initial volume"),
             _phys("dT", "temperature_difference", "temperature change (positive = heating)")],
     outputs=[_o("beta", "expansion_coefficient", "volumetric coefficient 3 alpha"), _o("dV", "volume", "linearised volume change beta V0 dT"),
              _o("dV_cm3", "number", "linearised volume change in cm^3"), _o("V", "volume", "new volume (linearised)"),
              _o("dV_exact", "volume", "exact volume change V0((1 + alpha dT)^3 - 1)"), _o("dV_exact_cm3", "number", "exact volume change in cm^3")],
     primary="dV_cm3", conditions=["alpha, V0 > 0", "isotropic solid, alpha constant over the range"],
     method="beta = 3 alpha, dV = beta V0 dT; exact: V0((1 + alpha dT)^3 - 1)", latex=r"\Delta V = \beta V_0\,\Delta T,\quad \beta = 3\alpha",
     run=volumetric_expansion,
     parse=lambda q: _pairs(q, r"volumetric thermal expansion|volume thermal expansion", {"alpha": "alpha", "V0": "V0", "dT": "dT"},
                            phys={"alpha", "V0", "dT"}),
     reference=ref_volumetric_expansion,
     wrong=_safe(lambda alpha, V0, dT: (lambda o: dict(o, beta=_r(alpha), dV=_r(alpha * V0 * dT), dV_cm3=_r(alpha * V0 * dT * 1e6),
                                                       V=_r(V0 + alpha * V0 * dT)))(volumetric_expansion(alpha, V0, dT))),
     random_inputs=lambda rng: {"alpha": rng.uniform(1e-6, 2.5e-5), "V0": _lu(rng, 1e-6, 1.0), "dT": rng.choice([-1, 1]) * rng.uniform(1, 400)},
     templates=["Volumetric thermal expansion: alpha = 17e-6 1/K, V0 = 1000 cm^3, dT = 100 K"],
     examples=[{"inputs": {"alpha": 17e-6, "V0": 1e-3, "dT": 100.0}, "outputs": {"dV_cm3": 5.1, "dV_exact_cm3": 1000 * ((1 + 17e-4) ** 3 - 1)}},
               {"inputs": {"alpha": 1e-5, "V0": 1.0, "dT": -10.0}, "outputs": {"beta": 3e-5, "dV": -3e-4}}],
     questions=[{"q": "Volumetric thermal expansion: alpha = 17e-6 1/K, V0 = 1000 cm^3, dT = 100 K", "outputs": {"dV_cm3": 5.1}},
                {"q": "Volumetric thermal expansion: alpha = 17e-6 1/K, V0 = 0 cm^3, dT = 100 K", "outcome": "INVALID_INPUT"},
                {"q": "Volumetric thermal expansion: alpha = 17e-6 1/K, dT = 100 K", "outcome": "NEEDS_INPUT"}])


def _majority(Nd, Na, ni):
    D = Nd - Na
    s = math.sqrt(0.25 * D * D + ni * ni)
    if D >= 0:
        n = 0.5 * D + s
        return n, ni * ni / n
    p = -0.5 * D + s
    return ni * ni / p, p


def extrinsic_carriers(Nd, Na, ni, T=None):
    if not (Nd >= 0 and Na >= 0 and math.isfinite(Nd) and math.isfinite(Na)):
        raise ValueError("the donor and acceptor concentrations must be zero or positive")
    _pos("the intrinsic concentration", ni)
    n, p = _majority(Nd, Na, ni)
    out = dict(n=n, p=p, log10_n=math.log10(n), log10_p=math.log10(p),
               carrier_type="n-type" if n > p else ("p-type" if p > n else "intrinsic"))
    if T is not None:
        _pos("the temperature", T)
        out["EF_minus_Ei_eV"] = KB * T / QE * math.log(n / ni)
    return _out(**out)


def ref_extrinsic_carriers(Nd, Na, ni, T=None):
    """Charge neutrality n - p = Nd - Na with the mass-action law n p = ni^2, solved for ln(n/ni) by scipy brentq
    (complete ionisation, non-degenerate)."""
    from scipy.constants import Boltzmann, electron_volt
    from scipy.optimize import brentq
    D = (Nd - Na) / ni
    b = math.log(abs(D) + 1) + 2
    u = brentq(lambda x: 2 * math.sinh(x) - D, -b, b, xtol=1e-15, rtol=8.9e-16, maxiter=2000)
    n, p = ni * math.exp(u), ni * math.exp(-u)
    out = {"log10_n": math.log10(n), "log10_p": math.log10(p)}
    if T is not None:
        out["EF_minus_Ei_eV"] = Boltzmann * T / electron_volt * u
    return out


def _rand_extr(rng):
    ni = _lu(rng, 1e12, 1e20)
    big = _lu(rng, 1e19, 1e25)
    a, b = big, big * rng.uniform(0.05, 0.9)
    Nd, Na = (a, b) if rng.random() < 0.5 else (b, a)
    x = {"Nd": Nd, "Na": Na, "ni": ni}
    if rng.random() < 0.6:
        x["T"] = rng.uniform(200, 600)
    return x


_add("extrinsic_carrier_concentration",
     name="Electron and hole concentrations of a doped semiconductor", topic="electronic properties / semiconductors",
     purpose="The electron and hole concentrations of a semiconductor with donor and acceptor concentrations Nd and Na (complete ionisation), from charge neutrality and n p = ni^2, and the Fermi level relative to the intrinsic level.",
     inputs=[_phys("Nd", "number_density", "donor concentration"), _phys("Na", "number_density", "acceptor concentration"),
             _phys("ni", "number_density", "intrinsic carrier concentration"),
             _phys("T", "temperature", "temperature (optional, for the Fermi level)", required=False)],
     outputs=[_o("n", "number_density", "electron concentration"), _o("p", "number_density", "hole concentration"),
              _o("log10_n", "number", "log10 of n in m^-3"), _o("log10_p", "number", "log10 of p in m^-3"),
              {"symbol": "carrier_type", "kind": "number", "meaning": "n-type, p-type or intrinsic"},
              _o("EF_minus_Ei_eV", "number", "Fermi level above the intrinsic level in eV (when T is given)")],
     primary="log10_n", conditions=["Nd, Na >= 0", "ni > 0", "complete ionisation, non-degenerate statistics"],
     method="n - p = Nd - Na, n p = ni^2: majority = |Nd - Na|/2 + sqrt((Nd - Na)^2/4 + ni^2), minority = ni^2/majority; E_F - E_i = kT ln(n/ni)",
     latex=r"n = \frac{N_d - N_a}{2} + \sqrt{\left(\frac{N_d - N_a}{2}\right)^2 + n_i^2},\quad p = \frac{n_i^2}{n}",
     run=extrinsic_carriers,
     parse=lambda q: _pairs(q, r"extrinsic carrier concentrations?|doped semiconductor carrier concentrations?",
                            {"Nd": "Nd", "Na": "Na", "ni": "ni", "T": "T"}, phys={"Nd", "Na", "ni", "T"}),
     reference=ref_extrinsic_carriers,
     wrong=_safe(lambda Nd, Na, ni, T=None: (lambda o: dict(o, log10_n=_r(math.log10(Nd if Nd >= Na else ni * ni / Na)),
                                                             log10_p=_r(math.log10(Na if Na > Nd else ni * ni / Nd))))(extrinsic_carriers(Nd, Na, ni, T))),
     random_inputs=_rand_extr,
     templates=["Extrinsic carrier concentration: Nd = 1e22 m^-3, Na = 2e21 m^-3, ni = 1e16 m^-3",
                "Extrinsic carrier concentration: Nd = 1e22 m^-3, Na = 2e21 m^-3, ni = 1e16 m^-3, T = 300 K"],
     examples=[{"inputs": {"Nd": 1e22, "Na": 2e21, "ni": 1e16}, "outputs": {"n": 4e21 + math.sqrt(16e42 + 1e32), "carrier_type": "n-type"}},
               {"inputs": {"Nd": 0.0, "Na": 3e22, "ni": 4e22}, "outputs": {"p": 1.5e22 + math.sqrt(2.25e44 + 16e44)}}],
     questions=[{"q": "Extrinsic carrier concentration: Nd = 1e22 m^-3, Na = 2e21 m^-3, ni = 1e16 m^-3",
                 "outputs": {"n": 4e21 + math.sqrt(16e42 + 1e32)}},
                {"q": "Extrinsic carrier concentration: Nd = 1e22 m^-3, Na = 2e21 m^-3, ni = 0 m^-3", "outcome": "INVALID_INPUT"},
                {"q": "Extrinsic carrier concentration: Nd = 1e22 m^-3, Na = 2e21 m^-3", "outcome": "NEEDS_INPUT"}])

EPS0 = 8.8541878188e-12


def built_in_potential(Na, Nd, ni, T, eps_r=None):
    _pos("Na, Nd, ni and T", Na, Nd, ni, T)
    nn, _pn = _majority(Nd, 0.0, ni)
    _np, pp = _majority(0.0, Na, ni)
    vt = KB * T / QE
    Vbi = vt * math.log(nn * pp / (ni * ni))
    out = dict(V_bi=Vbi, V_bi_approx=vt * (math.log(Na / ni) + math.log(Nd / ni)), thermal_voltage=vt)
    if eps_r is not None:
        _pos("the relative permittivity", eps_r)
        if Vbi <= 0:
            raise ValueError("no depletion region for a non-positive built-in potential")
        W = math.sqrt(2 * eps_r * EPS0 * Vbi / QE * (1 / Na + 1 / Nd))
        out.update(W=W, W_um=W * 1e6, x_n_um=W * Na / (Na + Nd) * 1e6, x_p_um=W * Nd / (Na + Nd) * 1e6)
    return _out(**out)


def ref_built_in_potential(Na, Nd, ni, T, eps_r=None):
    """The Fermi level on each side from charge neutrality (scipy brentq on n - p = net doping, n p = ni^2); the
    built-in potential is the difference of the two Fermi levels. The depletion width: the n-side width x_n at which
    the two parabolic potential drops of the depletion approximation add up to V_bi (brentq), x_p = x_n Nd/Na."""
    from scipy.constants import Boltzmann, e, epsilon_0
    from scipy.optimize import brentq
    vt = Boltzmann * T / e

    def level(D):                                              # (E_F - E_i)/kT for net doping D
        b = math.log(abs(D / ni) + 1) + 2
        return brentq(lambda x: 2 * math.sinh(x) - D / ni, -b, b, xtol=1e-15, rtol=8.9e-16, maxiter=2000)
    Vbi = vt * (level(Nd) - level(-Na))
    out = {"V_bi": Vbi}
    if eps_r is not None:
        eps = eps_r * epsilon_0
        drop = lambda xn: e * Nd * xn * xn / (2 * eps) + e * Na * (xn * Nd / Na) ** 2 / (2 * eps) - Vbi   # noqa: E731
        xn = brentq(drop, 0.0, 1.0, xtol=1e-30, rtol=8.9e-16, maxiter=2000)
        out.update(W_um=(xn + xn * Nd / Na) * 1e6, x_n_um=xn * 1e6)
    return out


def _rand_vbi(rng):
    ni = _lu(rng, 1e12, 1e19)
    x = {"Na": _lu(rng, 1e20, 1e25), "Nd": _lu(rng, 1e20, 1e25), "ni": ni, "T": rng.uniform(200, 500)}
    if rng.random() < 0.6:
        x["eps_r"] = rng.uniform(4, 16)
    return x


_add("pn_junction_built_in_potential",
     name="Built-in potential of a p-n junction", topic="electronic properties / semiconductor devices",
     purpose="The built-in potential V_bi = (kT/q) ln(n_n p_p/ni^2) of an abrupt p-n junction (majority concentrations from charge neutrality; V_bi ~ (kT/q) ln(Na Nd/ni^2) when Na, Nd >> ni), and with a permittivity the zero-bias depletion width.",
     inputs=[_phys("Na", "number_density", "acceptor concentration on the p side"), _phys("Nd", "number_density", "donor concentration on the n side"),
             _phys("ni", "number_density", "intrinsic carrier concentration"), _phys("T", "temperature", "temperature"),
             _numin("eps_r", "relative permittivity of the semiconductor (optional, for the depletion width)", required=False)],
     outputs=[_o("V_bi", "voltage", "built-in potential"), _o("V_bi_approx", "voltage", "(kT/q) ln(Na Nd/ni^2)"),
              _o("thermal_voltage", "voltage", "kT/q"), _o("W", "length", "zero-bias depletion width"), _o("W_um", "number", "depletion width in um"),
              _o("x_n_um", "number", "depletion width on the n side in um"), _o("x_p_um", "number", "depletion width on the p side in um")],
     primary="V_bi", conditions=["Na, Nd, ni, T > 0", "abrupt junction, complete ionisation, non-degenerate", "depletion approximation for W"],
     method="V_bi = (kT/q) ln(n_n p_p/ni^2); W = sqrt(2 eps V_bi (1/Na + 1/Nd)/q), x_n = W Na/(Na + Nd)",
     latex=r"V_{bi} = \frac{kT}{q}\ln\frac{N_aN_d}{n_i^2},\quad W = \sqrt{\frac{2\varepsilon V_{bi}}{q}\left(\frac{1}{N_a} + \frac{1}{N_d}\right)}",
     run=built_in_potential,
     parse=lambda q: _pairs(q, r"(?:p-?n junction )?built[- ]in potential", {"Na": "Na", "Nd": "Nd", "ni": "ni", "T": "T", "eps_r": "eps_r"},
                            phys={"Na", "Nd", "ni", "T"}),
     reference=ref_built_in_potential,
     wrong=_safe(lambda Na, Nd, ni, T, eps_r=None: (lambda o: dict(o, V_bi=_r(KB * T / QE * math.log(Na * Nd / ni))))(built_in_potential(Na, Nd, ni, T, eps_r))),
     random_inputs=_rand_vbi,
     templates=["Built-in potential: Na = 1e23 m^-3, Nd = 1e22 m^-3, ni = 1e16 m^-3, T = 300 K",
                "Built-in potential: Na = 1e23 m^-3, Nd = 1e22 m^-3, ni = 1e16 m^-3, T = 300 K, eps_r = 11.7"],
     examples=[{"inputs": {"Na": 1e23, "Nd": 1e22, "ni": 1e16, "T": 300.0},
                "outputs": {"V_bi_approx": KB * 300 / QE * math.log(1e45 / 1e32)}},
               {"inputs": {"Na": 1e23, "Nd": 1e23, "ni": 1e16, "T": 300.0, "eps_r": 11.7},
                "outputs": {"x_n_um": 0.5 * math.sqrt(2 * 11.7 * EPS0 * (KB * 300 / QE * math.log(1e46 / 1e32)) / QE * 2e-23) * 1e6}}],
     questions=[{"q": "Built-in potential: Na = 1e23 m^-3, Nd = 1e22 m^-3, ni = 1e16 m^-3, T = 300 K",
                 "outputs": {"V_bi_approx": KB * 300 / QE * math.log(1e45 / 1e32)}},
                {"q": "Built-in potential: Na = 1e23 m^-3, Nd = 1e22 m^-3, ni = 1e16 m^-3, T = 0 K", "outcome": "INVALID_INPUT"},
                {"q": "Built-in potential: Na = 1e23 m^-3, Nd = 1e22 m^-3, T = 300 K", "outcome": "NEEDS_INPUT"}])

# ==== END OF PROCEDURES
