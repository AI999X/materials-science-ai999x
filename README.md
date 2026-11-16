# Materials Science (materials-science-ai999x)

[![tests](https://github.com/AI999X/materials-science-ai999x/actions/workflows/ci.yml/badge.svg)](https://github.com/AI999X/materials-science-ai999x/actions/workflows/ci.yml) [![PyPI](https://img.shields.io/pypi/v/materials-science-ai999x)](https://pypi.org/project/materials-science-ai999x/) [![Hugging Face](https://img.shields.io/badge/%F0%9F%A4%97-Hugging%20Face-yellow)](https://huggingface.co/AI999X/materials-science-ai999x)

Materials Science covers crystal structures and diffraction, defects and diffusion, mechanical properties, phase diagrams, polymers, ceramics and composites, and electronic and thermal properties: 88 calculations in 13 topics, each with its formula, its steps and its units, and a plain reason instead of a number when an input is missing or out of range. One compiled engine, four ways to use it: the desktop app, the command line, the browser and Python. Everything runs on your own computer, offline.

[![The Materials Science desktop app: Angle between two directions in a cubic crystal, with its worked example and live scene](https://www.ai999x.com/?ax_shot=materials-science-ai999x/hero.webp)](https://www.ai999x.com/models/materials-science-ai999x/)

## Four ways to use it

| | |
|---|---|
| **Desktop app** (Windows, Linux) | [Windows](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.4/ai999x-1.6.4-windows-x86_64.zip) · [Linux](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.4/ai999x-1.6.4-linux-x86_64.tar.gz) |
| **Command line** `materials-science` (Windows, Linux) | [Windows](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.4/ai999x-cli-1.6.4-windows-x86_64.zip) · [Linux](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.4/ai999x-cli-1.6.4-linux-x86_64.tar.gz) |
| **Browser** (any device) | [Try it on the model page](https://www.ai999x.com/models/materials-science-ai999x/#try) |
| **Python** 3.10+ | `pip install materials-science-ai999x`: on Windows and Linux (x86-64) `pip` installs the same compiled engine; on other systems (macOS, ARM) it installs a pure-Python build that gives the same answers |

## Ask in words: 15 examples

Each question below was asked on this release; the comment under it is the line it printed.

```python
import materials_science_ai999x as materials_science

a = materials_science.ask("Radius ratio: rc = 0.102 nm, ra = 0.181 nm")
print(a.outcome, a.values)
# SOLVED {'ratio': 0.563535911602, 'CN': 6, 'geometry': 'octahedral'}

a = materials_science.ask(
    "Critical fiber length: sigma_f = 3.5 GPa, d = 10 um, tau_c = 20 "
    "MPa"
)
print(a.outcome, a.values)
# SOLVED {'lc': 0.000875, 'lc_mm': 0.875, 'aspect_ratio': 87.5}

a = materials_science.ask(
    "Angle between cubic directions: u1 = 1, v1 = 0, w1 = 0, u2 = 1, "
    "v2 = 1, w2 = 1"
)
print(a.outcome, a.values)
# SOLVED {'angle_deg': 54.7356103172}

a = materials_science.ask(
    "Bragg angle: d = 0.2087 nm, lambda = 0.1542 nm"
)
print(a.outcome, a.values)
# SOLVED {'theta_deg': 21.6804560949, 'two_theta_deg': 43.3609121898}

a = materials_science.ask(
    "Critical crack length: K_IC = 55 MPa*m^0.5, sigma = 1000 MPa"
)
print(a.outcome, a.values)
# SOLVED {'a_c': 0.000962887405706, 'a_c_mm': 0.962887405706}

a = materials_science.ask(
    "Offset yield strength: strain = [0, 0.002, 0.004, 0.006, 0.008], "
    "stress = [0, 400, 520, 560, 580], E = 200 GPa"
)
print(a.outcome, a.values)
# SOLVED {'yield_MPa': 533.333333333, 'yield_strain': 0.00466666666667}

a = materials_science.ask(
    "Atomic percent to weight percent: element1 = Al, element2 = Cu, "
    "at1 = 90"
)
print(a.outcome, a.values)
# SOLVED {'wt1': 79.2590796884, 'wt2': 20.7409203116}

a = materials_science.ask(
    "Copolymer degree of polymerization: formula1 = C4H6, formula2 = "
    "C8H8, f1 = 0.75, Mn = 350000 g/mol"
)
print(a.outcome, a.values)
# SOLVED {'m_avg_g_mol': 66.60511, 'DP': 5254.85206766}

a = materials_science.ask(
    "Wiedemann-Franz: T = 300 K, sigma = 5.96e7 S/m"
)
print(a.outcome, a.values)
# SOLVED {'kappa': 436.809206222, 'lorenz': 2.44300450907e-08}

a = materials_science.ask(
    "Angle between lattice directions: a = 0.3 nm, b = 0.3 nm, c = 0.5 "
    "nm, alpha = 90, beta = 90, gamma = 120, u1 = 1, v1 = 0, w1 = 0, "
    "u2 = 0, v2 = 1, w2 = 0"
)
print(a.outcome, a.values)
# SOLVED {'angle_deg': 120.0}

a = materials_science.ask(
    "Weibull design stress: P_failure = 0.001, sigma0 = 400 MPa, m = "
    "10"
)
print(a.outcome, a.values)
# SOLVED {'sigma': 200484921.625, 'sigma_MPa': 200.484921625}

a = materials_science.ask(
    "Gibbs phase rule: C = 2, P = 3, pressure = constant"
)
print(a.outcome, a.values)
# SOLVED {'F': 0}

a = materials_science.ask(
    "Linear thermal expansion: alpha = 17e-6 1/K, L0 = 0.5 m, dT = 100 "
    "K"
)
print(a.outcome, a.values)
# SOLVED {'dL': 0.00085, 'dL_mm': 0.85, 'L': 0.50085, 'strain': 0.0017}

a = materials_science.ask(
    "Angle between planes: a = 0.4 nm, b = 0.4 nm, c = 0.4 nm, alpha = "
    "90, beta = 90, gamma = 90, h1 = 1, k1 = 1, l1 = 1, h2 = 1, k2 = "
    "1, l2 = -1"
)
print(a.outcome, a.values)
# SOLVED {'angle_deg': 70.5287793655}

a = materials_science.ask(
    "Allowed reflections: structure = FCC, count = 6"
)
print(a.outcome, a.values)
# SOLVED {'N_values': [3, 4, 8, 11, 12, 16], 'N_last': 16, 'N_first': 3}
```

## Screenshots

[![The Materials Science desktop app: Angle between two directions in a cubic crystal, with its worked example and live scene](https://www.ai999x.com/?ax_shot=materials-science-ai999x/hero-dark.webp)](https://www.ai999x.com/models/materials-science-ai999x/)

[![Elastic elongation and diameter change of a rod in tension: the live scene in the Materials Science desktop app](https://www.ai999x.com/?ax_shot=materials-science-ai999x/see-1.webp)](https://www.ai999x.com/models/materials-science-ai999x/)

*Elastic elongation and diameter change of a rod in tension*

[![Interplanar spacing in a cubic crystal: the live scene in the Materials Science desktop app](https://www.ai999x.com/?ax_shot=materials-science-ai999x/see-2.webp)](https://www.ai999x.com/models/materials-science-ai999x/)

*Interplanar spacing in a cubic crystal*

[![Number- and weight-average molecular weight and dispersity: the live scene in the Materials Science desktop app](https://www.ai999x.com/?ax_shot=materials-science-ai999x/see-3.webp)](https://www.ai999x.com/models/materials-science-ai999x/)

*Number- and weight-average molecular weight and dispersity*

Every calculation, with its formula, a worked example and its figures: **[Materials Science on ai999x.com](https://www.ai999x.com/models/materials-science-ai999x/)**. Install guides for the app and the command line: [https://www.ai999x.com/desktop-cli/](https://www.ai999x.com/desktop-cli/).

Version 1.6.4 · License: Apache-2.0 · Author: Ibnul Jaif Farabi · Organization: AI999X · Contact: support@ai999x.com · [Support the project](https://buymeacoffee.com/ibnuljaiffarabi)

*The Future Is Self-Improving.*

## Source and releases

This repository holds the installable source of each release, exactly as published on [PyPI](https://pypi.org/project/materials-science-ai999x/) (`pip install .` builds it). Releases are tagged `v1.6.4`; the [GitHub release](https://github.com/AI999X/materials-science-ai999x/releases/tag/v1.6.4) carries the wheel, the desktop app and the command-line program for Windows and Linux, and their checksums.

## Build the program from source

`rust/` holds the Rust source of the `materials-science` command-line program and browser module: the AI999X engine (`rust/ax-engine`), this model (`rust/materials-science`) and its data (`rust/models/materials_science/pack.json`). With Rust 1.80 or later:

```bash
cd rust
cargo build --release -p materials-science-ai999x
./target/release/materials-science --help
cargo test -p materials-science-ai999x
```
