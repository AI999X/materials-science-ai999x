# materials-science-ai999x — offline, verified materials science solver with step-by-step answers

[![tests](https://github.com/AI999X/materials-science-ai999x/actions/workflows/ci.yml/badge.svg)](https://github.com/AI999X/materials-science-ai999x/actions/workflows/ci.yml) [![PyPI](https://img.shields.io/pypi/v/materials-science-ai999x)](https://pypi.org/project/materials-science-ai999x/) [![Hugging Face](https://img.shields.io/badge/%F0%9F%A4%97-Hugging%20Face-yellow)](https://huggingface.co/AI999X/materials-science-ai999x)

**[Install](https://pypi.org/project/materials-science-ai999x/)** · **[Capability guide](https://huggingface.co/AI999X/materials-science-ai999x/tree/main/docs)** · **[Evaluation](https://huggingface.co/AI999X/materials-science-ai999x/tree/main/evaluation)** · **[AI999X on Hugging Face](https://huggingface.co/AI999X)** · **[Website](https://ai999x.com)** · **[Support: Buy Me a Coffee](https://buymeacoffee.com/ibnuljaiffarabi)**

License: Apache-2.0 · Author: Ibnul Jaif Farabi (creator and engineer) · Organization: AI999X · Version 1.0.2

Field: **Physical Sciences** · Tasks: Question Answering, Token Classification, Text Classification, Sentence Similarity, Feature Extraction

materials-science-ai999x is an offline, deterministic Python solver that answers materials science questions with verified results from 87 certified capabilities, gives 345 documented answers identically on Windows and Linux, and refuses rather than guesses when an answer cannot be verified.

**materials-science-ai999x** is a compact, deterministic materials science engine for Python. It reads a question in plain English (or a typed call), picks a certified materials science capability, computes the answer exactly, checks it, and returns the result with its steps, units, LaTeX and assumptions — or an honest refusal that says what is missing. It runs fully offline on any CPU, needs no GPU, no API key and no internet, and is built to give **zero wrong answers**: when it is not sure, it refuses instead of guessing.

[![Buy me a coffee](https://img.shields.io/badge/Buy%20me%20a%20coffee-support%20development-FFDD00?logo=buymeacoffee&logoColor=black)](https://buymeacoffee.com/ibnuljaiffarabi) ![License](https://img.shields.io/badge/license-Apache--2.0-blue) ![Python](https://img.shields.io/badge/python-3.10%20%7C%203.11%20%7C%203.12%20%7C%203.13%20%7C%203.14-blue) ![Offline](https://img.shields.io/badge/runs-offline%20on%20CPU-green)

## Highlights

- **Verified, not generated.** Every answer is computed by a certified materials science capability and checked (substituted back, dimension-checked or compared with an independent computation) before it is returned.
- **Zero wrong answers by design.** Seven explicit outcomes — SOLVED, NEEDS_INPUT, AMBIGUOUS, INVALID_INPUT, OUT_OF_SCOPE, UNRECOGNIZED_FORMAT, INTERNAL_ERROR — never a silent guess. Every one of its 345 documented answers is reproduced exactly on every tested platform.
- **Plain-English questions and a guaranteed typed interface.** `ask()` reads documented question forms; `solve(capability_id, **inputs)` always solves a supported task with valid inputs.
- **Explains itself.** Steps, formulas in LaTeX, units, assumptions and the model used, ready for notebooks, tutoring apps and homework checkers.
- **Small and portable.** Pure Python (sympy, mpmath); identical answers on Windows and Linux (Python 3.10, 3.11, 3.12, 3.13, 3.14).
- **Coverage:** crystal structures and diffraction, defects and diffusion, mechanical properties, phase diagrams, polymers, ceramics and composites, and electronic and thermal properties.

## Model overview

| Property | Value |
|---|---|
| Package | `materials-science-ai999x` 1.0.2 (import `materials_science_ai999x`) |
| Type | Deterministic neuro-symbolic materials science engine (rule-based reading + certified symbolic capabilities) |
| Capabilities | 87 certified procedures |
| Knowledge pack | version 1.0.1 |
| Golden answers | 345 documented answers, identical on every tested platform |
| Platforms tested | Windows (Python 3.13), Linux x86-64 (Python 3.10, 3.11, 3.12, 3.13, 3.14) |
| Runtime | CPU only, offline; no GPU, no network, no API key |
| License | Apache-2.0 |
| Creator / engineer | Ibnul Jaif Farabi |
| Organization | AI999X |

## Quickstart

```bash
pip install materials-science-ai999x
```

```python
import materials_science_ai999x as materials_science

materials_science.ask("Theoretical density: structure = FCC, A = 63.55 g/mol, R = 0.128 nm").show()
# -> SOLVED — rho = 8895.28, rho_gcc = 8.89528, a_nm = 0.362039  (theoretical_density)
materials_science.ask("Bragg angle: d = 0.2087 nm, lambda = 0.1542 nm").show()
# -> SOLVED — theta_deg = 21.6805, two_theta_deg = 43.3609  (bragg_angle)
materials_science.ask("Fick's second law: C0 = 0.25 wt%, Cs = 1.2 wt%, x = 0.5 mm, D = 1.6e-11 m^2/s, t = 7 h").show()
# -> SOLVED — Cx = 0.798785, z = 0.393713, ratio = 0.577668  (fick_second_law_concentration)
materials_science.ask("Iron-carbon microconstituents: C0 = 0.35 wt%").show()
# -> SOLVED — W_proeutectoid = 0.555556, W_pearlite = 0.444444, W_ferrite_total = 0.950883, W_cementite_total = 0.0491165  (iron_carbon_microconstituents)
materials_science.solve("planar_density", structure="BCC", h=1, k=1, l=0, a="0.2866 nm").show()
# -> SOLVED — PD_per_nm2 = 17.2172, PD = 1.72172e+19, PPF = 0.833041  (planar_density)
materials_science.ask("Who created you?").message
# -> materials-science-ai999x was created and engineered by Ibnul Jaif Farabi at AI999X.
```

Command line: `materials-science-ai999x "<question>"`, `materials-science-ai999x --capabilities`. The outputs above were produced by running this exact code on materials-science-ai999x 1.0.2.

## Find the right capability, and draw it

`materials_science.search()` finds which certified capability a question is about — across **every** AI999X model, so a question for another model points you to it. It runs **[ai999x-encoder](https://huggingface.co/AI999X/ai999x-encoder)**, AI999X's own capability encoder (4.7M parameters, a 4.9 MB ONNX model, offline; the root `model.safetensors` of this repository): `pip install "materials-science-ai999x[search]"`. `ask(question, suggest=True)` adds the same pointers to a question it does not answer; an answer itself never changes.

```python
import materials_science_ai999x as materials_science

materials_science.search("Solve x^2 - 5x + 6 = 0", k=2)
# -> Roots of a quadratic equation (math-foundations-ai999x, 0.63); Roots of a cubic equation (math-foundations-ai999x, 0.59)
```

## How every answer is verified

1. **Reading** — the question is turned into typed quantities (value, unit, kind, role) and one asked quantity; every stated number must be used or the question is refused.
2. **Planning** — only certified capabilities may answer; a capability applies only under the conditions the wording states (for example a pendulum, a liquid at rest, boiling).
3. **Computing** — exact symbolic or high-precision numeric evaluation.
4. **Checking** — dimensions, substitution back into the defining equation, and agreement between independent readings; any disagreement becomes a refusal, never a number.

Capabilities are certified before they ship: each must reproduce hand-worked reference cases, reject a deliberately wrong variant (a control), and leave every benchmark answer unchanged.

## Worked examples

Each question was put to `materials-science-ai999x`; the answer is the one it returned, with the steps it took.

**Theoretical density of a metal** · _Crystal structures_

> Theoretical density: structure = FCC, A = 63.55 g/mol, R = 0.128 nm

→ **rho = 8895.28269 kg/m^3, rho_gcc = 8.89528269, a_nm = 0.362038672**

<details><summary>Steps</summary>

- Theoretical density of a metal: structure = FCC, A = 63.55 g/mol, R = 0.128 nm → rho = 8895.28269 kg/m^3, rho_gcc = 8.89528269, a_nm = 0.362038672

</details>

**Bragg angle of a reflection** · _Diffraction_

> Bragg angle: d = 0.2087 nm, lambda = 0.1542 nm

→ **theta_deg = 21.68045609, two_theta_deg = 43.36091219**

<details><summary>Steps</summary>

- Bragg angle of a reflection: d = 0.2087 nm, lam = 0.1542 nm → theta_deg = 21.68045609, two_theta_deg = 43.36091219

</details>

**Concentration in a semi-infinite solid (Fick's second law)** · _Diffusion_

> Fick's second law: C0 = 0.25 wt%, Cs = 1.2 wt%, x = 0.5 mm, D = 1.6e-11 m^2/s, t = 7 h

→ **Cx = 0.7987846097, z = 0.3937129927, ratio = 0.5776680102**

<details><summary>Steps</summary>

- Concentration in a semi-infinite solid (Fick's second law): C0 = 0.25, Cs = 1.2, x = 0.5 mm, D = 1.6e-11 m^2/s, t = 7 h → Cx = 0.7987846097, z = 0.3937129927, ratio = 0.5776680102

</details>

**Proeutectoid phase and pearlite in a plain-carbon steel** · _Phase diagrams_

> Iron-carbon microconstituents: C0 = 0.35 wt%

→ **W_proeutectoid = 0.5555555556, W_pearlite = 0.4444444444, W_ferrite_total = 0.9508834981, W_cementite_total = 0.04911650195, proeutectoid_phase = ferrite**

<details><summary>Steps</summary>

- Proeutectoid phase and pearlite in a plain-carbon steel: C0 = 0.35 → W_proeutectoid = 0.5555555556, W_pearlite = 0.4444444444, W_ferrite_total = 0.9508834981, W_cementite_total = 0.04911650195, proeutectoid_phase = ferrite

</details>

**Planar density of a crystal plane** · _Crystal structures_

`m.solve("planar_density", structure="BCC", h=1, k=1, l=0, a="0.2866 nm")`

→ **PD_per_nm2 = 17.21720402, PD = 1.721720402e+19 1/m^2, PPF = 0.8330405509**

<details><summary>Steps</summary>

- Planar density of a crystal plane: structure = BCC, h = 1, k = 1, l = 0, a = 0.2866 nm → PD_per_nm2 = 17.21720402, PD = 1.721720402e+19 1/m^2, PPF = 0.8330405509

</details>

**Lattice parameter and density of an ionic crystal** · _Ceramics_

> Ceramic density: prototype = rocksalt, rc = 0.102 nm, ra = 0.181 nm, Ac = 22.99 g/mol, Aa = 35.45 g/mol

→ **a = 5.66e-10 m, a_nm = 0.566, rho = 2140.769963 kg/m^3, rho_gcc = 2.140769963**

<details><summary>Steps</summary>

- Lattice parameter and density of an ionic crystal: prototype = rocksalt, rc = 0.102 nm, ra = 0.181 nm, Ac = 22.99 g/mol, Aa = 35.45 g/mol → a = 5.66e-10 m, a_nm = 0.566, rho = 2140.769963 kg/m^3, rho_gcc = 2.140769963

</details>

## Figures

Every point was computed by `materials-science-ai999x`.

**Diffusion into a solid: concentration profiles over time**

![Diffusion into a solid: concentration profiles over time — computed by materials-science-ai999x](https://huggingface.co/AI999X/materials-science-ai999x/resolve/main/figures/fick-second-law-concentration-profile.svg)

_Concentration profiles of a species diffusing into a solid from a surface held at Cs = 1.2 wt% (initial C0 = 0.25 wt%, D = 1.6000e-11 m²/s) after 1, 7 and 24 hours, each computed by the package at 51 depths to 2.5 mm. Marked: x = 0.5 mm, t = 7 h, C = 0.798785 wt%._

**X-ray diffraction pattern of an FCC crystal**

![X-ray diffraction pattern of an FCC crystal — computed by materials-science-ai999x](https://huggingface.co/AI999X/materials-science-ai999x/resolve/main/figures/cubic-reflection-angle-pattern.svg)

_Diffraction stick pattern of an FCC crystal with a = 3.615 Å under λ = 1.542 Å: every (hkl) up to 4 is placed at the 2θ the package returns and given the structure factor it computes (|F|²/f² = 16 for the 8 allowed reflections, 0 for the 14 absent ones, shown as short grey stems). First line: (111) at 2θ = 43.3583°._

**Stress–strain curve and the 0.2 % offset yield strength**

![Stress–strain curve and the 0.2 % offset yield strength — computed by materials-science-ai999x](https://huggingface.co/AI999X/materials-science-ai999x/resolve/main/figures/offset-yield-strength-curve.svg)

_An engineering stress–strain curve (the stated test points) and the line of slope E = 200 GPa offset by 0.2 % strain; the package finds their intersection, the offset yield strength. Marked: ε = 0.466667 %, σy = 533.333 MPa._

**Steel microconstituents across the eutectoid (lever rule)**

![Steel microconstituents across the eutectoid (lever rule) — computed by materials-science-ai999x](https://huggingface.co/AI999X/materials-science-ai999x/resolve/main/figures/iron-carbon-microconstituents-bars.svg)

_Microconstituents of slowly cooled plain-carbon steel just below the eutectoid temperature, from the lever rule as computed by the package, as stacked bars at 10 carbon contents: pearlite at the bottom, the proeutectoid constituent on top (ferrite below the eutectoid composition, cementite above it). Marked: C0 = 0.35 wt%, pearlite fraction 0.444444 (proeutectoid ferrite 0.555556)._

**Coordination number from the cation-anion radius ratio**

![Coordination number from the cation-anion radius ratio — computed by materials-science-ai999x](https://huggingface.co/AI999X/materials-science-ai999x/resolve/main/figures/radius-ratio-coordination-cn-vs-rc.svg)

_CN (coordination number) against rc (cation radius), with ra = 1.81e-10 m. 31 points computed by materials-science-ai999x; marked: rc = 1.02e-10, CN = 6._

## Capabilities

87 certified procedures. A few of them:

- **Procedures:** 0.2 % offset yield strength from a stress-strain curve; Activation energy and pre-exponential factor of diffusion; Angle between two directions in a cubic crystal; Angle between two lattice directions in any crystal; Angle between two lattice planes in any crystal; Atomic packing factor and coordination number; Atomic percent to weight percent (binary alloy); Average repeat-unit weight and degree of polymerization of a copolymer; Avrami (JMAK) transformation kinetics; Band gap from intrinsic conductivities at several temperatures; Bragg angle of a reflection; Built-in potential of a p-n junction; Burgers vector of a perfect dislocation; Concentration in a semi-infinite solid (Fick's second law); Coordination number from the cation-anion radius ratio; Creep strain of a Kelvin-Voigt viscoelastic solid; Critical crack length from the fracture toughness; Critical fibre length; Critical nucleus radius and activation energy (homogeneous nucleation); Degrees of freedom by Gibbs' phase rule; Depth at which a concentration is reached (Fick's second law); Design stress for a target failure probability (Weibull); Diffraction angle of a cubic (h k l) reflection; Diffusion coefficient at a temperature (Arrhenius) …

`materials_science_ai999x.templates()` lists every question form `ask()` reads; the capability guide (PDF, every example executed) is included in this repository.

## Out of scope

- Open-ended explanation, essay writing or tutoring dialogue: it is a solver, not a chatbot.
- Questions outside its certified materials science capabilities, and any question whose stated numbers it cannot all use — it refuses with the reason instead.
- High-stakes engineering, medical or legal decisions without independent review.

## Limitations

- It answers what its certified capabilities cover (crystal structures and diffraction, defects and diffusion, mechanical properties, phase diagrams, polymers, ceramics and composites, and electronic and thermal properties); anything else is refused with a reason.
- `ask()` reads documented question forms and common phrasings; unusual wording may be refused — use `solve()` for guaranteed results.
- Results are as correct as the certified relations and their stated conditions; a certified relation is a checked model, not a claim about every physical situation.

## Citation

```bibtex
@software{materials_science_ai999x_1_0_2,
  title  = {materials-science-ai999x: an offline, verified materials science solver},
  author = {Farabi, Ibnul Jaif}, organization = {AI999X},
  year   = {2026}, version = {1.0.2},
  url    = {https://huggingface.co/AI999X/materials-science-ai999x}
}
```

## Contact

Questions, bug reports and requests: open a discussion on https://huggingface.co/AI999X/materials-science-ai999x/discussions or visit https://ai999x.com.

## Support the project

materials-science-ai999x is free and open source. If it helps you, you can support further development on [Buy Me a Coffee](https://buymeacoffee.com/ibnuljaiffarabi).

Created and engineered by **Ibnul Jaif Farabi** · **[AI999X](https://huggingface.co/AI999X)** · [ai999x.com](https://ai999x.com)

## Source and releases

This repository holds the installable source of each release, exactly as published on [PyPI](https://pypi.org/project/materials-science-ai999x/) (`pip install .` builds it). Releases are tagged `v<version>`; the wheel is attached to each [GitHub release](https://github.com/AI999X/materials-science-ai999x/releases). The optional ONNX models (formula graphs and, where measured, a decision model) are on [Hugging Face](https://huggingface.co/AI999X/materials-science-ai999x/tree/main/onnx).
