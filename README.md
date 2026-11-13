# materials-science-ai999x: offline, verified materials science solver with step-by-step answers

[![tests](https://github.com/AI999X/materials-science-ai999x/actions/workflows/ci.yml/badge.svg)](https://github.com/AI999X/materials-science-ai999x/actions/workflows/ci.yml) [![PyPI](https://img.shields.io/pypi/v/materials-science-ai999x)](https://pypi.org/project/materials-science-ai999x/) [![Hugging Face](https://img.shields.io/badge/%F0%9F%A4%97-Hugging%20Face-yellow)](https://huggingface.co/AI999X/materials-science-ai999x)

**[Desktop app & command line](https://www.ai999x.com/desktop-cli/)** · **[Compute in your browser](https://www.ai999x.com/models/materials-science-ai999x/#try)** · **[Install (Python)](https://pypi.org/project/materials-science-ai999x/)** · **[Guides](https://huggingface.co/AI999X/materials-science-ai999x/tree/main/docs)** · **[Evaluation](https://huggingface.co/AI999X/materials-science-ai999x/tree/main/evaluation)** · **[AI999X on Hugging Face](https://huggingface.co/AI999X)** · **[Website](https://ai999x.com)** · **[Support: Buy Me a Coffee](https://buymeacoffee.com/ibnuljaiffarabi)**

License: Apache-2.0 · Author: Ibnul Jaif Farabi (creator and engineer) · Organization: AI999X · Version 1.6.0

Field: **Physical Sciences** · Tasks: Question Answering

materials-science-ai999x is an offline, deterministic materials science solver written in Rust: a desktop app, a command-line program, a browser module and a Python package. It answers materials science questions with verified results from 88 certified capabilities, gives 352 documented answers identically on Windows and Linux, and refuses rather than guesses when an answer cannot be verified.

**materials-science-ai999x** reads a question in plain English (or a typed call), picks a certified materials science capability, computes the answer exactly, checks it, and returns the result with its steps, units, LaTeX and assumptions, or a refusal that says what is missing. It runs fully offline on any CPU, needs no GPU, no API key and no internet, and is built to give **zero wrong answers**: when it is not sure, it refuses instead of guessing.

[![Buy me a coffee](https://img.shields.io/badge/Buy%20me%20a%20coffee-support%20development-FFDD00?logo=buymeacoffee&logoColor=black)](https://buymeacoffee.com/ibnuljaiffarabi) ![License](https://img.shields.io/badge/license-Apache--2.0-blue) ![Python](https://img.shields.io/badge/python-3.10%20%7C%203.11%20%7C%203.12%20%7C%203.13%20%7C%203.14-blue) ![Offline](https://img.shields.io/badge/runs-offline%20on%20CPU-green) ![Rust](https://img.shields.io/badge/Rust-Windows%20%7C%20Linux%20%7C%20WebAssembly-orange)

**Run it your way.** materials-science-ai999x comes as the AI999X desktop app and as the command-line program `materials-science` for Windows and Linux, runs in your browser as WebAssembly, and installs as a Python package. The desktop app, the command line and the browser run the same compiled Rust program on your own device: offline, no Python needed.

## Get it

| Way to run it | For | Get it |
|---|---|---|
| **AI999X Materials Science desktop app** 1.6.0 | Windows, Linux. This model in its own app: search, ask in words, forms, live figures, explanations, offline | [Windows (x86-64)](https://downloads.ai999x.com/desktop/1.6.0/ai999x-materials-science-1.6.0-windows-x86_64.zip) · [Linux (x86-64)](https://downloads.ai999x.com/desktop/1.6.0/ai999x-materials-science-1.6.0-linux-x86_64.tar.gz) |
| **AI999X desktop app** 1.6.0 | Windows, Linux. Every AI999X model in one app: search, ask in words, forms, live figures, explanations, offline | [Windows (x86-64)](https://downloads.ai999x.com/desktop/1.6.0/ai999x-1.6.0-windows-x86_64.zip) · [Linux (x86-64)](https://downloads.ai999x.com/desktop/1.6.0/ai999x-1.6.0-linux-x86_64.tar.gz) |
| **Command line** (`materials-science`) | Windows, Linux. One program per model, offline, no Python | [Windows (x86-64)](https://downloads.ai999x.com/desktop/1.6.0/ai999x-cli-1.6.0-windows-x86_64.zip) · [Linux (x86-64)](https://downloads.ai999x.com/desktop/1.6.0/ai999x-cli-1.6.0-linux-x86_64.tar.gz) |
| **In your browser** | any device. Nothing to install | [Compute on the model page](https://www.ai999x.com/models/materials-science-ai999x/#try) |
| **Python** | Python 3.10+ | `pip install materials-science-ai999x` |

Checksums: [SHA256SUMS](https://downloads.ai999x.com/desktop/1.6.0/SHA256SUMS) · every version: [https://downloads.ai999x.com/desktop/latest/](https://downloads.ai999x.com/desktop/latest/) · install guide: [desktop & command line](https://www.ai999x.com/desktop-cli/)

## Quick start

### Desktop app

Download **AI999X Materials Science** for your system (above), unpack it and start it; type a question in words or pick a capability, fill in the form and press Compute. Figures and an explanation of the answer appear beside it. The AI999X desktop app has the same, with Materials Science among all AI999X models.

Linux, from a terminal:

```bash
curl -LO https://downloads.ai999x.com/desktop/1.6.0/ai999x-materials-science-1.6.0-linux-x86_64.tar.gz
curl -LO https://downloads.ai999x.com/desktop/1.6.0/SHA256SUMS
sha256sum -c --ignore-missing SHA256SUMS
tar xzf ai999x-materials-science-1.6.0-linux-x86_64.tar.gz
./ai999x-materials-science-1.6.0-linux-x86_64/ai999x-materials-science
```

### Command line

Linux:

```bash
curl -LO https://downloads.ai999x.com/desktop/1.6.0/ai999x-cli-1.6.0-linux-x86_64.tar.gz
curl -LO https://downloads.ai999x.com/desktop/1.6.0/SHA256SUMS
sha256sum -c --ignore-missing SHA256SUMS
mkdir -p ~/.local/share && tar xzf ai999x-cli-1.6.0-linux-x86_64.tar.gz -C ~/.local/share
echo 'export PATH="$HOME/.local/share/ai999x-cli-1.6.0-linux-x86_64:$PATH"' >> ~/.bashrc && source ~/.bashrc
materials-science --version
```

Windows (PowerShell):

```powershell
$F = "ai999x-cli-1.6.0-windows-x86_64.zip"
Invoke-WebRequest "https://downloads.ai999x.com/desktop/1.6.0/$F" -OutFile $F
Invoke-WebRequest "https://downloads.ai999x.com/desktop/1.6.0/SHA256SUMS" -OutFile SHA256SUMS
(Get-FileHash $F -Algorithm SHA256).Hash.ToLower() -eq ((Select-String -SimpleMatch $F SHA256SUMS).Line -split ' ')[0]
Expand-Archive $F -DestinationPath "$env:LOCALAPPDATA\AI999X" -Force
$D = "$env:LOCALAPPDATA\AI999X\ai999x-cli-1.6.0-windows-x86_64"; $env:Path += ";$D"
[Environment]::SetEnvironmentVariable("Path", [Environment]::GetEnvironmentVariable("Path", "User") + ";$D", "User")
materials-science --version
```

Every command below was run as shown; the text under it is what the program printed.

**Ask in words**: Write the question as you would say it: the program reads the quantities and their units, picks the capability and answers.

```console
$ materials-science ask "Theoretical density: structure = FCC, A = 63.55 g/mol, R = 0.128 nm"
Read as: Theoretical density of a metal
rho = 8895.28269 kg/m^3
rho_gcc = 8.89528269
a_nm = 0.362038672
  by: Theoretical density of a metal  (a from R (hard spheres), V_cell = a^3 (HCP: (3 sqrt(3)/2) a^2 c with c = sqrt(8/3) a), rho = n A/(V_cell N_A))
  A = 0.06355 kg/mol
  R = 1.28e-10 m
  structure = FCC
  assumes: A > 0; R > 0
```

**What a capability needs**: The equation or method, every input and output with its SI unit. Read this before solve: it tells you the input names.

```console
$ materials-science info theoretical_density
Theoretical density of a metal
  a from R (hard spheres), V_cell = a^3 (HCP: (3 sqrt(3)/2) a^2 c with c = sqrt(8/3) a), rho = n A/(V_cell N_A)
  in   structure crystal structure (SC, BCC, FCC, HCP, diamond) (choice) []
  in   A        molar mass (atomic weight) [kg/mol]
  in   R        atomic radius [m]
  out  rho      theoretical density [kg/m^3]
  out  rho_gcc  density in g/cm^3 []
  out  a_nm     lattice parameter a in nm []
```

**Solve**: Inputs as name=value, each with any unit of the right kind. The answer lists every output, the method used, each input converted to SI, and the conditions the answer assumes.

```console
$ materials-science solve theoretical_density structure=FCC "A=63.55 g/mol" "R=0.128 nm"
rho = 8895.28269 kg/m^3
rho_gcc = 8.89528269
a_nm = 0.362038672
  by: Theoretical density of a metal  (a from R (hard spheres), V_cell = a^3 (HCP: (3 sqrt(3)/2) a^2 c with c = sqrt(8/3) a), rho = n A/(V_cell N_A))
  A = 0.06355 kg/mol
  R = 1.28e-10 m
  structure = FCC
  assumes: A > 0; R > 0
```

**Many questions at once**: One JSON request per line in, one JSON answer per line out, in order: for scripts, notebooks and agents (the program starts once). In PowerShell: Get-Content cases.jsonl | materials-science batch

`cases.jsonl`:

```json
{"key": "theoretical_density", "inputs": {"structure": "FCC", "A": "63.55 g/mol", "R": "0.128 nm"}}
{"key": "bragg_angle", "inputs": {"d": "0.2087 nm", "lam": "0.1542 nm"}}
```

```console
$ materials-science batch < cases.jsonl
{"outcome":"SOLVED","message":"","value":8.8952826897,"unit":"","solved_for":"rho_gcc","si":[["A",0.06355],["R",1.2800000000000002e-10]],"values":[["rho",8895.2826897,"kg/m^3"],["rho_gcc",8.8952826897,""],["a_nm",0.362038671968,""]],"target":"rho_gcc","phrase":"theoretical density","assumptions":["A > 0","R > 0"],"name":"Theoretical density of a metal","equation":"a from R (hard spheres), V_cell = a^3 (HCP: (3 sqrt(3)/2) a^2 c with c = sqrt(8/3) a), rho = n A/(V_cell N_A)"}
{"outcome":"SOLVED","message":"","value":43.3609121898,"unit":"","solved_for":"two_theta_deg","si":[["d",2.0870000000000001e-10],["lam",1.5420000000000002e-10]],"values":[["theta_deg",21.6804560949,""],["two_theta_deg",43.3609121898,""]],"target":"two_theta_deg","phrase":"Bragg angle theta in degrees","assumptions":["d, lambda > 0","n lambda < 2 d"],"name":"Bragg angle of a reflection","equation":"theta = asin(n lambda / (2 d))"}
```

### In your browser

Open [Materials Science on ai999x.com](https://www.ai999x.com/models/materials-science-ai999x/#try), choose a capability and press **Compute**: the model runs on your own device as WebAssembly; nothing is sent anywhere. Change any input (any unit of the right kind) and the answer, its explanation and the figures follow.

In your own page: `materials_science_ai999x.wasm` and the loader `ax-wasm.js` (https://www.ai999x.com/compute/rust/ax-wasm.js).

### Python

```bash
pip install materials-science-ai999x
```

## Worked examples in every form

The same worked example from the command line, from Python and in the browser. Each command-line output is what the program printed.

### Coordination number from the cation-anion radius ratio

The expected coordination number and polyhedron of a cation from the ratio of the cation and anion radii (Pauling's radius-ratio rule).

Command line:

```console
$ materials-science solve radius_ratio_coordination "rc=0.102 nm" "ra=0.181 nm"
ratio = 0.5635359116
CN = 6
geometry = octahedral
  by: Coordination number from the cation-anion radius ratio  (critical ratios 0.155 (3), 0.225 (4), 0.414 (6), 0.732 (8): the largest one not above rc/ra)
  rc = 1.02e-10 m
  ra = 1.81e-10 m
  assumes: 0 < rc < ra
```

Python:

```python
import materials_science_ai999x as m

a = m.solve("radius_ratio_coordination", rc="0.102 nm", ra="0.181 nm")
print(a.values or (a.value, a.unit))
```

Browser: [open this capability on the model page](https://www.ai999x.com/models/materials-science-ai999x/#cap-radius_ratio_coordination) and press **Compute in your browser**.

_What this means:_ This calculation determines the expected structural arrangement by dividing the cation radius by the anion radius to obtain the ratio 0.563536. Based on Pauling's radius-ratio rule, this value is compared against critical thresholds to predict the most stable coordination environment. The resulting coordination number of 6 suggests that the cation is surrounded by 6 anions in a octahedral polyhedron, which reflects the balance between electrostatic attraction and geometric packing constraints in the crystal lattice.

### Lattice parameter and density of an ionic crystal

The cubic lattice parameter and theoretical density of a rock-salt, cesium-chloride, zinc-blende or fluorite crystal from its ionic radii and molar masses.

Command line:

```console
$ materials-science solve ceramic_density prototype=rocksalt "rc=0.102 nm" "ra=0.181 nm" "Ac=22.99 g/mol" "Aa=35.45 g/mol"
a = 5.66e-10 m
a_nm = 0.566
rho = 2140.769963 kg/m^3
rho_gcc = 2.140769963
  by: Lattice parameter and density of an ionic crystal  (cation-anion contact: rock salt a = 2(rc + ra), CsCl a = 2(rc + ra)/sqrt3, zinc blende and fluorite a = 4(rc + ra)/sqrt3; rho = n'(Ac + k Aa)/(a^3 N_A))
  rc = 1.02e-10 m
  ra = 1.81e-10 m
  Ac = 0.02299 kg/mol
  Aa = 0.03545 kg/mol
  prototype = rocksalt
  assumes: radii and molar masses > 0; cations and anions touch (the anions need not)
```

Python:

```python
import materials_science_ai999x as m

a = m.solve("ceramic_density", prototype="rocksalt", rc="0.102 nm", ra="0.181 nm", Ac="22.99 g/mol", Aa="35.45 g/mol")
print(a.values or (a.value, a.unit))
```

Browser: [open this capability on the model page](https://www.ai999x.com/models/materials-science-ai999x/#cap-ceramic_density) and press **Compute in your browser**.

_What this means:_ This calculation determines the cubic lattice parameter and theoretical density for an ionic crystal based on its structural prototype. Using the cation radius of 0.102 nm and the anion radius of 0.181 nm, the model assumes direct contact between ions to derive a lattice parameter of 5.66e-10 m. This geometric value, combined with the molar masses of the constituent elements, yields a predicted density of 2140.77 kg/m^3, linking microscopic atomic dimensions to macroscopic material properties.

### Critical fibre length

The critical length l_c = sigma_f d / (2 tau_c) a fibre needs to be loaded to its strength through the fibre-matrix interface.

Command line:

```console
$ materials-science solve critical_fiber_length "sigma_f=3.5 GPa" "d=10 um" "tau_c=20 MPa"
lc = 0.000875 m
lc_mm = 0.875
aspect_ratio = 87.5
  by: Critical fibre length  (l_c = sigma_f d / (2 tau_c))
  sigma_f = 3500000000 Pa
  d = 1e-05 m
  tau_c = 20000000 Pa
  assumes: all inputs > 0
```

Python:

```python
import materials_science_ai999x as m

a = m.solve("critical_fiber_length", sigma_f="3.5 GPa", d="10 um", tau_c="20 MPa")
print(a.values or (a.value, a.unit))
```

Browser: [open this capability on the model page](https://www.ai999x.com/models/materials-science-ai999x/#cap-critical_fiber_length) and press **Compute in your browser**.

_What this means:_ This calculation determines the minimum length required for a fibre to be effectively loaded to its breaking strength, balancing tensile force against interfacial shear. Given a fibre tensile strength of 3.5 GPa and diameter of 10 um, with a bond strength of 20 MPa, the resulting critical length is 0.000875 m. This means fibres shorter than this length will pull out rather than break, while longer fibres can achieve their full strength potential.

### Elastic moduli of a continuous-fibre composite (rule of mixtures)

The longitudinal (isostrain) and transverse (isostress) moduli of an aligned continuous-fibre composite and the fraction of the load carried by the fibres.

Command line:

```console
$ materials-science solve composite_rule_of_mixtures "Ef=69 GPa" "Em=3.4 GPa" Vf=0.4
E_long_GPa = 29.64
E_trans_GPa = 5.486435921
fiber_load_fraction = 0.9311740891
E_long = 2.964e+10 Pa
E_trans = 5486435921 Pa
  by: Elastic moduli of a continuous-fibre composite (rule of mixtures)  (E_l = Ef Vf + Em Vm (Voigt), 1/E_t = Vf/Ef + Vm/Em (Reuss), Ff/F = Ef Vf / E_l)
  Ef = 6.9e+10 Pa
  Em = 3400000000 Pa
  Vf = 0.4
  assumes: moduli > 0; 0 < Vf < 1; perfect bonding
```

Python:

```python
import materials_science_ai999x as m

a = m.solve("composite_rule_of_mixtures", Ef="69 GPa", Em="3.4 GPa", Vf="0.4")
print(a.values or (a.value, a.unit))
```

Browser: [open this capability on the model page](https://www.ai999x.com/models/materials-science-ai999x/#cap-composite_rule_of_mixtures) and press **Compute in your browser**.

_What this means:_ This calculation determines the stiffness of a continuous-fibre composite in both principal directions using the rule of mixtures. Given a fibre modulus of 69 GPa and a matrix modulus of 3.4 GPa at a fibre volume fraction of 0.4, the longitudinal modulus results in 2.964e+10 Pa. The transverse modulus, calculated under isostress conditions, yields 5.48644e+09 Pa. Additionally, the fraction of the applied longitudinal load borne by the fibres is computed as 0.931174. These values indicate how the composite responds to stress along and across the fibre alignment.

## Documentation

- **Rust user guide**: the command line, every capability with a command and what it printed: [PDF](https://huggingface.co/AI999X/materials-science-ai999x/blob/main/docs/materials_science_ai999x_rust_guide.pdf) · [Markdown](https://huggingface.co/AI999X/materials-science-ai999x/blob/main/docs/materials_science_ai999x_rust_guide.md)
- **Python guide**: every capability with a Python example: [PDF](https://huggingface.co/AI999X/materials-science-ai999x/blob/main/docs/materials_science_ai999x_capabilities.pdf) · [Markdown](https://huggingface.co/AI999X/materials-science-ai999x/blob/main/docs/materials_science_ai999x_capabilities.md)
- **Desktop app & command line**: downloads, install, ask and search, figures, offline use, checksums: https://www.ai999x.com/desktop-cli/
- **Model page**: compute in your browser, every capability explained: https://www.ai999x.com/models/materials-science-ai999x/

## Highlights

- **Verified, not generated.** Every answer is computed by a certified materials science capability and checked (substituted back, dimension-checked or compared with an independent computation) before it is returned.
- **Zero wrong answers by design.** Seven explicit outcomes (SOLVED, NEEDS_INPUT, AMBIGUOUS, INVALID_INPUT, OUT_OF_SCOPE, UNRECOGNIZED_FORMAT, INTERNAL_ERROR), never a silent guess. Every one of its 352 documented answers is reproduced exactly on every tested platform.
- **Plain-English questions and a guaranteed typed interface.** `ask()` reads documented question forms; `solve(capability_id, **inputs)` always solves a supported task with valid inputs.
- **Explains itself.** Steps, formulas in LaTeX, units, assumptions and the model used, ready for notebooks, tutoring apps and homework checkers.
- **Native and in the browser.** Every capability also computes natively in Rust: the desktop app and the `materials-science` command line for Windows and Linux, and WebAssembly on the model page, no Python, no network.
- **Small and portable.** Pure Python (sympy, mpmath); identical answers on Windows and Linux (Python 3.10, 3.11, 3.12, 3.13, 3.14).
- **Coverage:** crystal structures and diffraction, defects and diffusion, mechanical properties, phase diagrams, polymers, ceramics and composites, and electronic and thermal properties.

## Model overview

| Property | Value |
|---|---|
| Package | `materials-science-ai999x` 1.6.0 (import `materials_science_ai999x`) |
| Type | Deterministic neuro-symbolic materials science engine (rule-based reading + certified symbolic capabilities) |
| Capabilities | 88 certified procedures |
| Knowledge pack | version 1.0.1 |
| Golden answers | 352 documented answers, identical on every tested platform |
| Platforms tested | Windows (Python 3.13), Linux x86-64 (Python 3.10, 3.11, 3.12, 3.13, 3.14) |
| Runtime | CPU only, offline; no GPU, no network, no API key |
| License | Apache-2.0 |
| Creator / engineer | Ibnul Jaif Farabi |
| Organization | AI999X |

## Python quick start

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

The Python package also installs a Python command, `materials-science-ai999x "<question>"`; for the native program use `materials-science` (above). The outputs above were produced by running this exact code on materials-science-ai999x 1.6.0.

## How every answer is verified

1. **Reading**: the question is turned into typed quantities (value, unit, kind, role) and one asked quantity; every stated number must be used or the question is refused.
2. **Planning**: only certified capabilities may answer; a capability applies only under the conditions the wording states (for example a pendulum, a liquid at rest, boiling).
3. **Computing**: exact symbolic or high-precision numeric evaluation.
4. **Checking**: dimensions, substitution back into the defining equation, and agreement between independent readings; any disagreement becomes a refusal, never a number.

Capabilities are certified before they ship: each must reproduce hand-worked reference cases, reject a deliberately wrong variant (a control), and leave every benchmark answer unchanged.

## Python worked examples, with their steps

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

![Diffusion into a solid: concentration profiles over time, computed by materials-science-ai999x](https://huggingface.co/AI999X/materials-science-ai999x/resolve/main/figures/fick-second-law-concentration-profile.svg)

_Concentration profiles of a species diffusing into a solid from a surface held at Cs = 1.2 wt% (initial C0 = 0.25 wt%, D = 1.6000e-11 m²/s) after 1, 7 and 24 hours, each computed by the package at 51 depths to 2.5 mm. Marked: x = 0.5 mm, t = 7 h, C = 0.798785 wt%._

**X-ray diffraction pattern of an FCC crystal**

![X-ray diffraction pattern of an FCC crystal, computed by materials-science-ai999x](https://huggingface.co/AI999X/materials-science-ai999x/resolve/main/figures/cubic-reflection-angle-pattern.svg)

_Diffraction stick pattern of an FCC crystal with a = 3.615 Å under λ = 1.542 Å: every (hkl) up to 4 is placed at the 2θ the package returns and given the structure factor it computes (|F|²/f² = 16 for the 8 allowed reflections, 0 for the 14 absent ones, shown as short grey stems). First line: (111) at 2θ = 43.3583°._

**Stress–strain curve and the 0.2 % offset yield strength**

![Stress–strain curve and the 0.2 % offset yield strength, computed by materials-science-ai999x](https://huggingface.co/AI999X/materials-science-ai999x/resolve/main/figures/offset-yield-strength-curve.svg)

_An engineering stress–strain curve (the stated test points) and the line of slope E = 200 GPa offset by 0.2 % strain; the package finds their intersection, the offset yield strength. Marked: ε = 0.466667 %, σy = 533.333 MPa._

**Steel microconstituents across the eutectoid (lever rule)**

![Steel microconstituents across the eutectoid (lever rule), computed by materials-science-ai999x](https://huggingface.co/AI999X/materials-science-ai999x/resolve/main/figures/iron-carbon-microconstituents-bars.svg)

_Microconstituents of slowly cooled plain-carbon steel just below the eutectoid temperature, from the lever rule as computed by the package, as stacked bars at 10 carbon contents: pearlite at the bottom, the proeutectoid constituent on top (ferrite below the eutectoid composition, cementite above it). Marked: C0 = 0.35 wt%, pearlite fraction 0.444444 (proeutectoid ferrite 0.555556)._

**Coordination number from the cation-anion radius ratio**

![Coordination number from the cation-anion radius ratio, computed by materials-science-ai999x](https://huggingface.co/AI999X/materials-science-ai999x/resolve/main/figures/radius-ratio-coordination-cn-vs-rc.svg)

_CN (coordination number) against rc (cation radius), with ra = 1.81e-10 m. 31 points computed by materials-science-ai999x; marked: rc = 1.02e-10, CN = 6._

## Capabilities

88 certified procedures. A few of them:

- **Procedures:** 0.2 % offset yield strength from a stress-strain curve; Activation energy and pre-exponential factor of diffusion; Angle between two directions in a cubic crystal; Angle between two lattice directions in any crystal; Angle between two lattice planes in any crystal; Atomic packing factor and coordination number; Atomic percent to weight percent (binary alloy); Average repeat-unit weight and degree of polymerization of a copolymer; Avrami (JMAK) transformation kinetics; Band gap from intrinsic conductivities at several temperatures; Bragg angle of a reflection; Built-in potential of a p-n junction; Burgers vector of a perfect dislocation; Concentration in a semi-infinite solid (Fick's second law); Coordination number from the cation-anion radius ratio; Creep strain of a Kelvin-Voigt viscoelastic solid; Critical crack length from the fracture toughness; Critical fibre length; Critical nucleus radius and activation energy (homogeneous nucleation); Degrees of freedom by Gibbs' phase rule; Depth at which a concentration is reached (Fick's second law); Design stress for a target failure probability (Weibull); Diffraction angle of a cubic (h k l) reflection; Diffusion coefficient at a temperature (Arrhenius) …

`materials_science_ai999x.templates()` lists every question form `ask()` reads; the capability guide (PDF, every example executed) is included in this repository.

## Out of scope

- Open-ended explanation, essay writing or tutoring dialogue: it is a solver, not a chatbot.
- Questions outside its certified materials science capabilities, and any question whose stated numbers it cannot all use: it refuses with the reason instead.
- High-stakes engineering, medical or legal decisions without independent review.

## Limitations

- It answers what its certified capabilities cover (crystal structures and diffraction, defects and diffusion, mechanical properties, phase diagrams, polymers, ceramics and composites, and electronic and thermal properties); anything else is refused with a reason.
- `ask()` reads documented question forms and common phrasings; unusual wording may be refused; use `solve()` for guaranteed results.
- Results are as correct as the certified relations and their stated conditions; a certified relation is a checked model, not a claim about every physical situation.

## Citation

```bibtex
@software{materials_science_ai999x_1_6_0,
  title  = {materials-science-ai999x: an offline, verified materials science solver},
  author = {Farabi, Ibnul Jaif}, organization = {AI999X},
  year   = {2026}, version = {1.6.0},
  url    = {https://huggingface.co/AI999X/materials-science-ai999x}
}
```

## Contact

Questions, bug reports and requests: open a discussion on https://huggingface.co/AI999X/materials-science-ai999x/discussions or visit https://ai999x.com.

## Support the project

materials-science-ai999x is free and open source. If it helps you, you can support further development on [Buy Me a Coffee](https://buymeacoffee.com/ibnuljaiffarabi).

Created and engineered by **Ibnul Jaif Farabi** · **[AI999X](https://huggingface.co/AI999X)** · [ai999x.com](https://ai999x.com)

## Source and releases

This repository holds the installable source of each release, exactly as published on [PyPI](https://pypi.org/project/materials-science-ai999x/) (`pip install .` builds it). Releases are tagged `v1.6.0`; the [GitHub release](https://github.com/AI999X/materials-science-ai999x/releases/tag/v1.6.0) carries the wheel, the desktop app and the command-line program for Windows and Linux, and their checksums.

## Build the program from source

`rust/` holds the Rust source of the `materials-science` command-line program and browser module: the AI999X engine (`rust/ax-engine`), this model (`rust/materials-science`) and its data (`rust/models/materials_science/pack.json`). With Rust 1.80 or later:

```bash
cd rust
cargo build --release -p materials-science-ai999x
./target/release/materials-science --help
cargo test -p materials-science-ai999x
```
