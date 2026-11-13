# materials-science: the Materials Science program, user guide

**materials-science-ai999x 1.6.0**, Rust build: command line, desktop app and browser. **Creator / engineer:** Ibnul Jaif Farabi · **Company:** AI999X · support@ai999x.com

Materials Science covers crystal structures and diffraction, defects and diffusion, mechanical properties, phase diagrams, polymers, ceramics and composites, and electronic and thermal properties. This guide is about `materials-science`, the model as a self-contained program for Windows and Linux: 88 capabilities in 13 topics, answered on your own computer with no network, no Python and nothing else to install. The same compiled model runs in the AI999X desktop app and in the browser on the model's page (https://www.ai999x.com/models/materials-science-ai999x/); the Python package (`pip install materials-science-ai999x`) has its own guide. Every command output printed here is what the program printed.

## Contents

- 1. Get it: desktop app, command line, browser, Python
- 2. Quick start
- 3. Writing inputs
- 4. Outcomes and exit codes
- 5. Scripts and agents: batch and JSON
- 6. Figures
- 7. In a web page or Node.js: WebAssembly
- 8. Every capability

## 1. Get it

Version 1.5.0 of the AI999X desktop app and of the command-line bundle. Checksums for every file: https://downloads.ai999x.com/desktop/1.5.0/SHA256SUMS; the newest version is always under https://downloads.ai999x.com/desktop/latest/. Full instructions: https://www.ai999x.com/desktop-cli/

| what | system | download |
|---|---|---|
| Desktop app: Materials Science | Windows x86-64 | https://downloads.ai999x.com/desktop/1.5.0/ai999x-materials-science-1.5.0-windows-x86_64.zip |
| Desktop app: Materials Science | Linux x86-64 | https://downloads.ai999x.com/desktop/1.5.0/ai999x-materials-science-1.5.0-linux-x86_64.tar.gz |
| Desktop app: all models | Windows x86-64 | https://downloads.ai999x.com/desktop/1.5.0/ai999x-1.5.0-windows-x86_64.zip |
| Desktop app: all models | Linux x86-64 | https://downloads.ai999x.com/desktop/1.5.0/ai999x-1.5.0-linux-x86_64.tar.gz |
| Command line | Windows x86-64 | https://downloads.ai999x.com/desktop/1.5.0/ai999x-cli-1.5.0-windows-x86_64.zip |
| Command line | Linux x86-64 | https://downloads.ai999x.com/desktop/1.5.0/ai999x-cli-1.5.0-linux-x86_64.tar.gz |

### Desktop app

Materials Science in its own app: search, questions in words, input forms, live figures and explanations. Windows: Download ai999x-materials-science-1.5.0-windows-x86_64.zip, right-click it → Extract All, then open the folder and start ai999x-materials-science.exe. If Windows SmartScreen asks, choose More info → Run anyway. Linux:

```shell
curl -LO https://downloads.ai999x.com/desktop/1.5.0/ai999x-materials-science-1.5.0-linux-x86_64.tar.gz
curl -LO https://downloads.ai999x.com/desktop/1.5.0/SHA256SUMS
sha256sum -c --ignore-missing SHA256SUMS
tar xzf ai999x-materials-science-1.5.0-linux-x86_64.tar.gz
./ai999x-materials-science-1.5.0-linux-x86_64/ai999x-materials-science
```

It opens on this model: type a question or pick a capability, fill in the form and press Compute. The AI999X desktop app with every model works the same way (choose the model first). Windows: Download ai999x-1.5.0-windows-x86_64.zip, right-click it → Extract All, then open the folder and start ai999x.exe. If Windows SmartScreen asks, choose More info → Run anyway. Linux:

```shell
curl -LO https://downloads.ai999x.com/desktop/1.5.0/ai999x-1.5.0-linux-x86_64.tar.gz
curl -LO https://downloads.ai999x.com/desktop/1.5.0/SHA256SUMS
sha256sum -c --ignore-missing SHA256SUMS
tar xzf ai999x-1.5.0-linux-x86_64.tar.gz
./ai999x-1.5.0-linux-x86_64/ai999x
```

### Command line: Linux

The bundle holds one program per model; `materials-science` is this one. The checksum line should print OK.

```shell
curl -LO https://downloads.ai999x.com/desktop/1.5.0/ai999x-cli-1.5.0-linux-x86_64.tar.gz
curl -LO https://downloads.ai999x.com/desktop/1.5.0/SHA256SUMS
sha256sum -c --ignore-missing SHA256SUMS
mkdir -p ~/.local/share && tar xzf ai999x-cli-1.5.0-linux-x86_64.tar.gz -C ~/.local/share
echo 'export PATH="$HOME/.local/share/ai999x-cli-1.5.0-linux-x86_64:$PATH"' >> ~/.bashrc && source ~/.bashrc
materials-science --version
```

### Command line: Windows (PowerShell)

The fourth line should print True.

```powershell
$F = "ai999x-cli-1.5.0-windows-x86_64.zip"
Invoke-WebRequest "https://downloads.ai999x.com/desktop/1.5.0/$F" -OutFile $F
Invoke-WebRequest "https://downloads.ai999x.com/desktop/1.5.0/SHA256SUMS" -OutFile SHA256SUMS
(Get-FileHash $F -Algorithm SHA256).Hash.ToLower() -eq ((Select-String -SimpleMatch $F SHA256SUMS).Line -split ' ')[0]
Expand-Archive $F -DestinationPath "$env:LOCALAPPDATA\AI999X" -Force
$D = "$env:LOCALAPPDATA\AI999X\ai999x-cli-1.5.0-windows-x86_64"; $env:Path += ";$D"
[Environment]::SetEnvironmentVariable("Path", [Environment]::GetEnvironmentVariable("Path", "User") + ";$D", "User")
materials-science --version
```

### In your browser

Open https://www.ai999x.com/models/materials-science-ai999x/, choose a capability and press Compute in your browser: the same program runs on your device as WebAssembly; nothing is sent anywhere.

### Python

```shell
pip install materials-science-ai999x
```

## 2. Quick start

### 2.1 Check the install

Prints the model, its version and that this is the Rust build.

```shell
$ materials-science --version
materials-science-ai999x 1.6.0 (Rust build, ax-engine 1.6.0)
```

### 2.2 Ask in words

Write the question as you would say it: the program reads the quantities and their units, picks the capability and answers.

```shell
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

### 2.3 Find a capability

Search the capabilities by what you want to know; the first column is the id to pass to info and solve.

```shell
$ materials-science search "theoretical density of a metal"
theoretical_density                  Theoretical density of a metal  (crystal structures / density)
fermi_energy_free_electron           Fermi energy of a free-electron metal  (electronic properties / metals)
wiedemann_franz                      Electronic thermal conductivity of a metal (Wiedemann-Franz law)  (thermal properties / thermal conductivity)
planar_density                       Planar density of a crystal plane  (crystal structures / linear and planar density)
polymer_crystallinity                Percent crystallinity of a polymer from densities  (polymers / crystallinity)
lattice_parameter_from_density       Lattice parameter and atomic radius from the density  (crystal structures / density)
ceramic_density                      Lattice parameter and density of an ionic crystal  (ceramics / crystal structures)
linear_density                       Linear density along a crystal direction  (crystal structures / linear and planar density)
… 4 more
```

### 2.4 List the capabilities

Every capability the program answers: its id (what you pass to solve) and its name.

```shell
$ materials-science list
allowed_reflections_cubic                First allowed reflections of a cubic crystal
angle_between_directions                 Angle between two lattice directions in any crystal
angle_between_directions_cubic           Angle between two directions in a cubic crystal
angle_between_planes                     Angle between two lattice planes in any crystal
arrhenius_diffusivity                    Diffusion coefficient at a temperature (Arrhenius)
atomic_packing_factor                    Atomic packing factor and coordination number
atomic_to_weight_percent                 Atomic percent to weight percent (binary alloy)
avrami_kinetics                          Avrami (JMAK) transformation kinetics
… 80 more
```

### 2.5 What a capability needs

The equation or method, every input and output with its SI unit. Read this before solve: it tells you the input names.

```shell
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

### 2.6 Solve

Inputs as name=value, each with any unit of the right kind. The answer lists every output, the method used, each input converted to SI, and the conditions the answer assumes.

```shell
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

### 2.7 Machine-readable answer

--json prints the same answer as one line of JSON (values in SI): for scripts and other programs.

```shell
$ materials-science solve theoretical_density structure=FCC "A=63.55 g/mol" "R=0.128 nm" --json
{"outcome":"SOLVED","message":"","value":8.8952826897,"unit":"","solved_for":"rho_gcc","si":[["A",0.06355],["R",1.2800000000000002e-10]],"values":[["rho",8895.2826897,"kg/m^3"],["rho_gcc",8.8952826897,""],["a_nm",0.362038671968,""]],"target":"rho_gcc","phrase":"theoretical density","assumptions":["A > 0","R > 0"],"name":"Theoretical density of a metal","equation":"a from R (hard spheres), V_cell = a^3 (HCP: (3 sqrt(3)/2) a^2 c with c = sqrt(8/3) a), rho = n A/(V_cell N_A)"}
```

### 2.8 When an input is missing

No number is guessed: the answer names what is missing, shows a complete call, and the program exits with status 1.

```shell
$ materials-science solve theoretical_density structure=FCC "A=63.55 g/mol"
NEEDS_INPUT: theoretical_density needs R (atomic radius). Example: solve('theoretical_density', structure='FCC', A='0.06355 kg/mol', R='1.28e-10 m').
```

### 2.9 Many questions at once

One JSON request per line in, one JSON answer per line out, in order: for scripts, notebooks and agents (the program starts once). In PowerShell: Get-Content cases.jsonl | materials-science batch

`cases.jsonl`:

```json
{"key": "theoretical_density", "inputs": {"structure": "FCC", "A": "63.55 g/mol", "R": "0.128 nm"}}
{"key": "bragg_angle", "inputs": {"d": "0.2087 nm", "lam": "0.1542 nm"}}
```

```shell
$ materials-science batch < cases.jsonl
{"outcome":"SOLVED","message":"","value":8.8952826897,"unit":"","solved_for":"rho_gcc","si":[["A",0.06355],["R",1.2800000000000002e-10]],"values":[["rho",8895.2826897,"kg/m^3"],["rho_gcc",8.8952826897,""],["a_nm",0.362038671968,""]],"target":"rho_gcc","phrase":"theoretical density","assumptions":["A > 0","R > 0"],"name":"Theoretical density of a metal","equation":"a from R (hard spheres), V_cell = a^3 (HCP: (3 sqrt(3)/2) a^2 c with c = sqrt(8/3) a), rho = n A/(V_cell N_A)"}
{"outcome":"SOLVED","message":"","value":43.3609121898,"unit":"","solved_for":"two_theta_deg","si":[["d",2.0870000000000001e-10],["lam",1.5420000000000002e-10]],"values":[["theta_deg",21.6804560949,""],["two_theta_deg",43.3609121898,""]],"target":"two_theta_deg","phrase":"Bragg angle theta in degrees","assumptions":["d, lambda > 0","n lambda < 2 d"],"name":"Bragg angle of a reflection","equation":"theta = asin(n lambda / (2 d))"}
```

## 3. Writing inputs

Inputs are written `name=value`, one per argument; `materials-science info <capability>` lists the names. Quote an argument that holds a space or brackets (bash, zsh and PowerShell all accept double quotes). The answer always states each input as it was read, converted to SI, so a misread unit is visible at once.

| kind | example | how it is read |
|---|---|---|
| a number with a unit | `"R=0.128 nm"` | any unit of the same kind; converted to SI |
| a number in SI | `R=1.28e-10` | no unit means the SI unit shown by info |
| a choice | `structure=FCC` | one of the values info lists |
| a list or vector | `"v=[1, 2, 3]" or "v=1, 2, 3"` | brackets keep it a list |
| a matrix | `"A=[[1, 2], [3, 4]]"` | one list per row |
| true / false | `inclusive=true` | for yes/no options |

An equation can be solved for any of its quantities: give the others and add `--target <symbol>`. Without `--target`, the program solves for the quantity the capability is named after.

## 4. Outcomes and exit codes

The program never guesses a number: when it cannot answer, it says why and exits with status 1, so a script can tell an answer from a refusal without reading the text.

| outcome | meaning | exit status |
|---|---|---|
| SOLVED | the answer: every output with its unit, the method, the inputs in SI, the conditions assumed | 0 |
| NEEDS_INPUT | an input is missing; the message names it and shows a complete call | 1 |
| INVALID_INPUT | a value or unit of the wrong kind, an unknown input name, or a value outside the conditions | 1 |
| AMBIGUOUS | the inputs allow two different answers; say which one is meant | 1 |
| OUT_OF_SCOPE | the model has no capability for this, or the result would not be meaningful | 1 |
| (usage) | a command the program does not know: it prints its usage | 2 |

## 5. Scripts and agents: batch and JSON

`materials-science solve ... --json` prints one answer as one line of JSON. `materials-science batch` reads one request per line on standard input and writes one JSON answer per line, in order, starting the program once for any number of requests. A request is `{"key": "<capability>", "inputs": {"name": "value", ...}, "target": null}`. In PowerShell, pipe the file in: `Get-Content cases.jsonl | materials-science batch`. `materials-science catalog` prints every capability with its inputs as JSON, to build forms or tool definitions.

| answer field | meaning |
|---|---|
| outcome | one of the outcomes above |
| message | why there is no answer (empty when SOLVED) |
| value, unit | the main result in SI units |
| values | every output: [symbol, value, unit] (procedures) |
| target | the quantity solved for |
| si | each numeric input converted to SI: [symbol, value] |
| assumptions | the conditions the answer relies on |
| name, equation | the capability and its equation or method |

```python
import json, subprocess

reqs = [{"key": "<capability>", "inputs": {"name": "value"}}]
out = subprocess.run(["materials-science", "batch"], input="\n".join(map(json.dumps, reqs)),
                     capture_output=True, text=True, check=True).stdout
answers = [json.loads(line) for line in out.splitlines()]
```

## 6. Figures

`materials-science figures <capability> name=value ... --out DIR` draws the live figures for those inputs and writes each as an SVG file into DIR (an existing folder); without `--out` the SVG text comes back as JSON. The desktop app and the model page draw the same figures as soon as you press Compute.

## 7. In a web page or Node.js: WebAssembly

The same model is a WebAssembly module, `materials_science_ai999x.wasm` (https://www.ai999x.com/compute/rust/materials_science_ai999x.wasm), with a small loader, `ax-wasm.js` (https://www.ai999x.com/compute/rust/ax-wasm.js). It runs in a browser, a Web Worker or Node.js, with no server.

```html
<script src="ax-wasm.js"></script>
<script>
  AXWasm.load("materials_science_ai999x.wasm").then(function (m) {
    var a = m.solve({key: "<capability>", inputs: {name: "value"}});
    console.log(a.outcome, a.values || a.value);
    console.log(m.catalog().capabilities.length + " capabilities");
  });
</script>
```

## 8. Every capability

By topic. For each: what it does, what it needs and gives, when it applies, and a command with what the program printed for it. `materials-science info <id>` shows the same inputs and outputs in the terminal.

### Ceramics (2)

<a id="radius_ratio_coordination"></a>

#### Coordination number from the cation-anion radius ratio, `radius_ratio_coordination`

The expected coordination number and polyhedron of a cation from the ratio of the cation and anion radii (Pauling's radius-ratio rule).

$$
\frac{r_c}{r_a}:\ 0.155\ (3),\ 0.225\ (4),\ 0.414\ (6),\ 0.732\ (8)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `rc` | in | cation radius | m |
| `ra` | in | anion radius | m |
| `ratio` | out | rc/ra | - |
| `CN` | out | coordination number | - |
| `geometry` | out | coordination polyhedron | - |

**Applies when:** 0 < rc < ra.

```shell
$ materials-science solve radius_ratio_coordination "rc=0.102 nm" "ra=0.181 nm"
ratio = 0.5635359116
CN = 6
geometry = octahedral
  by: Coordination number from the cation-anion radius ratio  (critical ratios 0.155 (3), 0.225 (4), 0.414 (6), 0.732 (8): the largest one not above rc/ra)
  rc = 1.02e-10 m
  ra = 1.81e-10 m
  assumes: 0 < rc < ra
```

**What the worked example means:** This calculation determines the expected structural arrangement by dividing the cation radius by the anion radius to obtain the ratio 0.563536. Based on Pauling's radius-ratio rule, this value is compared against critical thresholds to predict the most stable coordination environment. The resulting coordination number of 6 suggests that the cation is surrounded by 6 anions in a octahedral polyhedron, which reflects the balance between electrostatic attraction and geometric packing constraints in the crystal lattice.

<a id="ceramic_density"></a>

#### Lattice parameter and density of an ionic crystal, `ceramic_density`

The cubic lattice parameter and theoretical density of a rock-salt, cesium-chloride, zinc-blende or fluorite crystal from its ionic radii and molar masses.

$$
\rho = \frac{n'\left(\sum A_C + \sum A_A\right)}{V_C N_A}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `prototype` | in | crystal structure (rocksalt, CsCl, zincblende, fluorite) | - |
| `rc` | in | cation radius | m |
| `ra` | in | anion radius | m |
| `Ac` | in | molar mass of the cation element | kg/mol |
| `Aa` | in | molar mass of the anion element | kg/mol |
| `a` | out | lattice parameter | m |
| `a_nm` | out | lattice parameter in nm | - |
| `rho` | out | density | kg/m^3 |
| `rho_gcc` | out | density in g/cm^3 | - |

**Applies when:** radii and molar masses > 0; cations and anions touch (the anions need not).

```shell
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

**What the worked example means:** This calculation determines the cubic lattice parameter and theoretical density for an ionic crystal based on its structural prototype. Using the cation radius of 0.102 nm and the anion radius of 0.181 nm, the model assumes direct contact between ions to derive a lattice parameter of 5.66e-10 m. This geometric value, combined with the molar masses of the constituent elements, yields a predicted density of 2140.77 kg/m^3, linking microscopic atomic dimensions to macroscopic material properties.

### Composites (2)

<a id="critical_fiber_length"></a>

#### Critical fibre length: `critical_fiber_length`

The critical length l_c = sigma_f d / (2 tau_c) a fibre needs to be loaded to its strength through the fibre-matrix interface.

$$
l_c = \frac{\sigma_f^* d}{2\tau_c}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `sigma_f` | in | fibre tensile strength | Pa |
| `d` | in | fibre diameter | m |
| `tau_c` | in | fibre-matrix bond strength (or matrix shear yield strength) | Pa |
| `lc` | out | critical fibre length | m |
| `lc_mm` | out | critical length in mm | - |
| `aspect_ratio` | out | l_c/d | - |

**Applies when:** all inputs > 0.

```shell
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

**What the worked example means:** This calculation determines the minimum length required for a fibre to be effectively loaded to its breaking strength, balancing tensile force against interfacial shear. Given a fibre tensile strength of 3.5 GPa and diameter of 10 um, with a bond strength of 20 MPa, the resulting critical length is 0.000875 m. This means fibres shorter than this length will pull out rather than break, while longer fibres can achieve their full strength potential.

<a id="composite_rule_of_mixtures"></a>

#### Elastic moduli of a continuous-fibre composite (rule of mixtures), `composite_rule_of_mixtures`

The longitudinal (isostrain) and transverse (isostress) moduli of an aligned continuous-fibre composite and the fraction of the load carried by the fibres.

$$
E_{cl} = E_mV_m + E_fV_f,\quad \frac{1}{E_{ct}} = \frac{V_m}{E_m}+\frac{V_f}{E_f}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Ef` | in | fibre modulus | Pa |
| `Em` | in | matrix modulus | Pa |
| `Vf` | in | fibre volume fraction (0 to 1) | - |
| `E_long_GPa` | out | longitudinal modulus in GPa | - |
| `E_trans_GPa` | out | transverse modulus in GPa | - |
| `fiber_load_fraction` | out | fraction of a longitudinal load carried by the fibres | - |
| `E_long` | out | longitudinal modulus | Pa |
| `E_trans` | out | transverse modulus | Pa |

**Applies when:** moduli > 0; 0 < Vf < 1; perfect bonding.

```shell
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

**What the worked example means:** This calculation determines the stiffness of a continuous-fibre composite in both principal directions using the rule of mixtures. Given a fibre modulus of 69 GPa and a matrix modulus of 3.4 GPa at a fibre volume fraction of 0.4, the longitudinal modulus results in 2.964e+10 Pa. The transverse modulus, calculated under isostress conditions, yields 5.48644e+09 Pa. Additionally, the fraction of the applied longitudinal load borne by the fibres is computed as 0.931174. These values indicate how the composite responds to stress along and across the fibre alignment.

### Crystal structures (16)

<a id="angle_between_directions_cubic"></a>

#### Angle between two directions in a cubic crystal, `angle_between_directions_cubic`

The angle between the directions [u1 v1 w1] and [u2 v2 w2] of a cubic crystal (also the angle between the planes with those indices).

$$
\cos\varphi = \frac{u_1u_2+v_1v_2+w_1w_2}{\sqrt{u_1^2+v_1^2+w_1^2}\sqrt{u_2^2+v_2^2+w_2^2}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `u1` | in | index u of direction 1 | - |
| `v1` | in | index v of direction 1 | - |
| `w1` | in | index w of direction 1 | - |
| `u2` | in | index u of direction 2 | - |
| `v2` | in | index v of direction 2 | - |
| `w2` | in | index w of direction 2 | - |
| `angle_deg` | out | angle in degrees | - |

**Applies when:** integer indices, neither direction zero.

```shell
$ materials-science solve angle_between_directions_cubic u1=1 v1=0 w1=0 u2=1 v2=1 w2=1
angle_deg = 54.73561032
  by: Angle between two directions in a cubic crystal  (cos(angle) = (u1 u2 + v1 v2 + w1 w2) / (|d1| |d2|), evaluated as atan2(|d1 x d2|, d1 . d2))
  assumes: integer indices, neither direction zero
```

**What the worked example means:** This calculation determines the angular separation between two specific directions within a cubic crystal lattice. By utilizing the dot product of the direction indices and their respective magnitudes, the method computes the cosine of the angle, which is then converted to degrees. The result, 54.7356, represents the precise geometric orientation difference between the first direction defined by indices 1, 0, and 0 and the second direction defined by 1, 1, and 1. This value is critical for understanding crystallographic relationships and anisotropic properties in materials science.

<a id="angle_between_directions"></a>

#### Angle between two lattice directions in any crystal, `angle_between_directions`

The angle between the directions [u1 v1 w1] and [u2 v2 w2] of a crystal with any cell (a, b, c, alpha, beta, gamma).

$$
\cos\varphi = \frac{\mathbf d_1^{\mathsf T}G\,\mathbf d_2}{\sqrt{\mathbf d_1^{\mathsf T}G\,\mathbf d_1}\sqrt{\mathbf d_2^{\mathsf T}G\,\mathbf d_2}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `a` | in | cell edge a | m |
| `b` | in | cell edge b | m |
| `c` | in | cell edge c | m |
| `alpha` | in | cell angle alpha in degrees (between b and c) | - |
| `beta` | in | cell angle beta in degrees (between a and c) | - |
| `gamma` | in | cell angle gamma in degrees (between a and b) | - |
| `u1` | in | index u of direction 1 | - |
| `v1` | in | index v of direction 1 | - |
| `w1` | in | index w of direction 1 | - |
| `u2` | in | index u of direction 2 | - |
| `v2` | in | index v of direction 2 | - |
| `w2` | in | index w of direction 2 | - |
| `angle_deg` | out | angle in degrees | - |

**Applies when:** a valid cell; neither direction zero.

```shell
$ materials-science solve angle_between_directions "a=0.3 nm" "b=0.3 nm" "c=0.5 nm" alpha=90 beta=90 gamma=120 u1=1 v1=0 w1=0 u2=0 v2=1 w2=0
angle_deg = 120
  by: Angle between two lattice directions in any crystal  (the metric tensor G: cos(angle) = d1 G d2 / sqrt((d1 G d1)(d2 G d2)))
  a = 3e-10 m
  b = 3e-10 m
  c = 5e-10 m
  alpha = 90
  beta = 90
  gamma = 120
  assumes: a valid cell; neither direction zero
```

**What the worked example means:** This calculation determines the geometric angle between two specific crystallographic directions, defined by their Miller indices, within a unit cell characterized by edge lengths and interaxial angles. By employing the metric tensor to account for non-orthogonal axes and varying edge sizes, the method yields an accurate angular relationship. The resulting angle is expressed as 120, reflecting how the chosen vectors orient relative to one another in the given crystal structure.

<a id="angle_between_planes"></a>

#### Angle between two lattice planes in any crystal, `angle_between_planes`

The angle between the normals of the planes (h1 k1 l1) and (h2 k2 l2) of a crystal with any cell.

$$
\cos\varphi = \frac{\mathbf h_1^{\mathsf T}G^{-1}\mathbf h_2}{\sqrt{\mathbf h_1^{\mathsf T}G^{-1}\mathbf h_1}\sqrt{\mathbf h_2^{\mathsf T}G^{-1}\mathbf h_2}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `a` | in | cell edge a | m |
| `b` | in | cell edge b | m |
| `c` | in | cell edge c | m |
| `alpha` | in | cell angle alpha in degrees (between b and c) | - |
| `beta` | in | cell angle beta in degrees (between a and c) | - |
| `gamma` | in | cell angle gamma in degrees (between a and b) | - |
| `h1` | in | Miller index h of plane 1 | - |
| `k1` | in | Miller index k of plane 1 | - |
| `l1` | in | Miller index l of plane 1 | - |
| `h2` | in | Miller index h of plane 2 | - |
| `k2` | in | Miller index k of plane 2 | - |
| `l2` | in | Miller index l of plane 2 | - |
| `angle_deg` | out | angle between the plane normals in degrees | - |

**Applies when:** a valid cell; neither plane (0 0 0).

```shell
$ materials-science solve angle_between_planes "a=0.4 nm" "b=0.4 nm" "c=0.4 nm" alpha=90 beta=90 gamma=90 h1=1 k1=1 l1=1 h2=1 k2=1 l2=-1
angle_deg = 70.52877937
  by: Angle between two lattice planes in any crystal  (the reciprocal metric tensor G* = G^-1: cos(angle) = h1 G* h2 / sqrt((h1 G* h1)(h2 G* h2)))
  a = 4e-10 m
  b = 4e-10 m
  c = 4e-10 m
  alpha = 90
  beta = 90
  gamma = 90
  assumes: a valid cell; neither plane (0 0 0)
```

**What the worked example means:** This calculation determines the geometric angle between two specific crystallographic planes within a unit cell defined by edges 0.4 nm, 0.4 nm, and 0.4 nm with interaxial angles 90, 90, and 90. By applying the reciprocal metric tensor to the Miller indices, the method computes the orientation relationship between the plane normals. The resulting angular separation is 70.5288, which quantifies how these atomic layers intersect relative to each other in the lattice structure.

<a id="atomic_packing_factor"></a>

#### Atomic packing factor and coordination number, `atomic_packing_factor`

The fraction of a crystal's volume filled by hard-sphere atoms, the coordination number and the atoms per conventional cell of SC, BCC, FCC, HCP or diamond cubic.

$$
\mathrm{APF} = \frac{n\,\tfrac43\pi R^3}{V_{\mathrm{cell}}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `structure` | in | crystal structure (SC, BCC, FCC, HCP, diamond) | - |
| `apf` | out | atomic packing factor | - |
| `coordination` | out | coordination number | - |
| `atoms_per_cell` | out | atoms per conventional cell (HCP: the hexagonal prism) | - |

**Applies when:** hard spheres touching along the close-packed direction; ideal c/a for HCP.

```shell
$ materials-science solve atomic_packing_factor structure=FCC
apf = 0.7404804897
coordination = 12
atoms_per_cell = 4
  by: Atomic packing factor and coordination number  (APF = n (4/3) pi R^3 / V_cell with the cell edge written in R)
  structure = FCC
  assumes: hard spheres touching along the close-packed direction; ideal c/a for HCP
```

**What the worked example means:** This calculation determines the fraction of unit cell volume occupied by atoms, yielding an atomic packing factor of 0.74048. It also identifies the coordination number as 12 and the number of atoms per conventional cell as 4, reflecting how densely the structure packs spheres under ideal hard-sphere assumptions.

<a id="miller_bravais_to_three_index"></a>

#### Four-index (Miller-Bravais) to three-index direction, `miller_bravais_to_three_index`

Convert a Miller-Bravais direction [u v t w] of a hexagonal crystal to the three-index form [U V W].

$$
U = u - t,\ V = v - t,\ W = w
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `u` | in | index along a1 | - |
| `v` | in | index along a2 | - |
| `t` | in | index along a3 (= -(u + v)) | - |
| `w` | in | index along c | - |
| `U` | out | three-index component U | - |
| `V` | out | three-index component V | - |
| `W` | out | three-index component W | - |
| `indices` | out | [U V W] | - |

**Applies when:** integer indices; t = -(u + v).

```shell
$ materials-science solve miller_bravais_to_three_index u=2 v=-1 t=-1 w=0
U = 1
V = 0
W = 0
indices = [1 0 0]
  by: Four-index (Miller-Bravais) to three-index direction  (U = u - t, V = v - t, W = w, reduced to the smallest integers)
  assumes: integer indices; t = -(u + v)
```

**What the worked example means:** This calculation transforms a Miller-Bravais direction defined by indices along the basal and vertical axes into its equivalent three-index representation. By subtracting the third basal index from the first two and retaining the vertical index, the system produces the components 1, 0, and 0, which together form the reduced vector [1 0 0]. This conversion allows hexagonal crystallographic data to be compared with standard lattice calculations that rely on three-dimensional coordinate systems.

<a id="lattice_parameter_from_density"></a>

#### Lattice parameter and atomic radius from the density, `lattice_parameter_from_density`

The lattice parameter a and hard-sphere atomic radius R of a crystal of known structure, density and molar mass.

$$
a = \left(\frac{nA}{\rho N_A}\right)^{1/3}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `structure` | in | crystal structure (SC, BCC, FCC, HCP, diamond) | - |
| `rho` | in | measured density | kg/m^3 |
| `A` | in | molar mass | kg/mol |
| `a` | out | lattice parameter a | m |
| `a_nm` | out | lattice parameter a in nm | - |
| `R_nm` | out | atomic radius in nm | - |

**Applies when:** rho > 0; A > 0.

```shell
$ materials-science solve lattice_parameter_from_density structure=FCC "rho=8.96 g/cm^3" "A=63.55 g/mol"
a = 3.611649068e-10 m
a_nm = 0.3611649068
R_nm = 0.1276910774
  by: Lattice parameter and atomic radius from the density  (V_cell = n A / (rho N_A); a = V_cell^(1/3) (HCP with ideal c/a); R from the touching condition)
  rho = 8960 kg/m^3
  A = 0.06355 kg/mol
  structure = FCC
  assumes: rho > 0; A > 0
```

**What the worked example means:** This calculation determines the unit cell edge length and atomic radius for a crystal with known structure, molar mass, and density. Rearranging the theoretical density equation gives the volume per atom and then the lattice parameter as 3.61165e-10 m meters, which converts to 0.361165 nanometers. The atomic radius is then estimated using geometric packing rules specific to the crystal type, yielding 0.127691 nanometers. These values allow researchers to infer microscopic structural details from macroscopic measurements like weight and composition.

<a id="cubic_lattice_parameter"></a>

#### Lattice parameter from the atomic radius: `cubic_lattice_parameter`

The cubic lattice parameter a of an SC, BCC, FCC or diamond-cubic crystal of hard-sphere atoms of radius R.

$$
a_{\mathrm{SC}} = 2R,\ a_{\mathrm{BCC}} = \frac{4R}{\sqrt3},\ a_{\mathrm{FCC}} = 2\sqrt2\,R,\ a_{\mathrm{dia}} = \frac{8R}{\sqrt3}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `structure` | in | crystal structure (SC, BCC, FCC, diamond) | - |
| `R` | in | atomic radius | m |
| `a` | out | lattice parameter | m |
| `a_nm` | out | lattice parameter in nm | - |
| `nearest_neighbour_nm` | out | nearest-neighbour distance 2R in nm | - |

**Applies when:** R > 0.

```shell
$ materials-science solve cubic_lattice_parameter structure=FCC "R=0.1278 nm"
a = 3.614729865e-10 m
a_nm = 0.3614729865
nearest_neighbour_nm = 0.2556
  by: Lattice parameter from the atomic radius  (atoms touch along the close-packed direction: SC a = 2R, BCC a = 4R/sqrt(3), FCC a = 2 sqrt(2) R, diamond a = 8R/sqrt(3))
  R = 1.278e-10 m
  structure = FCC
  assumes: R > 0
```

In the worked example, structure = FCC, R = 0.1278 nm go through the equation above and give a = 3.614729865e-10 m, a_nm = 0.3614729865, nearest_neighbour_nm = 0.2556. It applies only when R > 0: outside that it says what is wrong instead of returning a number.

<a id="linear_density"></a>

#### Linear density along a crystal direction: `linear_density`

The number of atoms per unit length along the [u v w] direction of an SC, BCC or FCC crystal, and the linear packing fraction.

$$
\mathrm{LD} = \frac{\text{atoms centred on the line}}{\text{length of the line}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `structure` | in | crystal structure (SC, BCC, FCC) | - |
| `u` | in | direction index u | - |
| `v` | in | direction index v | - |
| `w` | in | direction index w | - |
| `a` | in | lattice parameter | m |
| `LD_per_nm` | out | atoms per nm along [uvw] | - |
| `LD` | out | atoms per metre along [uvw] | 1/m |
| `repeat_nm` | out | repeat distance along [uvw] in nm | - |
| `LPF` | out | linear packing fraction (hard spheres) | - |

**Applies when:** a > 0; integer indices between -4 and 4, not all zero.

```shell
$ materials-science solve linear_density structure=FCC u=1 v=1 w=0 "a=0.3615 nm"
LD_per_nm = 3.912070712
LD = 3912070712 1/m
repeat_nm = 0.2556191014
LPF = 1
  by: Linear density along a crystal direction  (the shortest lattice translation along [uvw]: |[uvw]| a, halved when the half vector is a lattice vector (BCC all odd, FCC two odd))
  a = 3.615e-10 m
  structure = FCC
  assumes: a > 0; integer indices between -4 and 4, not all zero
```

**What the worked example means:** This calculation determines the number of atoms per unit length along a specific crystallographic direction defined by indices u, v, and w. Using the lattice parameter a for the given structure, it computes the shortest translation vector to find the repeat distance, which is 0.255619. The resulting linear density is 3.91207 atoms per nanometre, or 3.91207e+09 1/m per metre. This metric indicates how closely packed atoms are along that line, influencing slip systems and material anisotropy.

<a id="miller_indices_from_intercepts"></a>

#### Miller indices of a plane from its intercepts, `miller_indices_from_intercepts`

The Miller indices (h k l) of a plane cutting the axes at x, y, z (in units of the cell edges; inf for a plane parallel to an axis).

$$
(hkl) \propto \left(\tfrac1x, \tfrac1y, \tfrac1z\right)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `x` | in | intercept on the a axis (number, fraction or inf) | - |
| `y` | in | intercept on the b axis | - |
| `z` | in | intercept on the c axis | - |
| `h` | out | Miller index h | - |
| `k` | out | Miller index k | - |
| `l` | out | Miller index l | - |
| `indices` | out | (h k l) | - |

**Applies when:** no intercept 0 (move the origin); at least one finite intercept.

```shell
$ materials-science solve miller_indices_from_intercepts x=1 y=1/2 z=inf
h = 1
k = 2
l = 0
indices = (1 2 0)
  by: Miller indices of a plane from its intercepts  (take reciprocals of the intercepts, clear fractions, reduce to the smallest integers (signs kept))
  x = 1
  y = 1/2
  z = inf
  assumes: no intercept 0 (move the origin); at least one finite intercept
```

**What the worked example means:** This calculation determines the Miller indices (h k l) of a crystallographic plane based on its intercepts with the unit cell axes. Taking the reciprocals of the input intercepts 1, 1/2, and inf, clearing any fractions, and reducing to the smallest integers while preserving signs gives the indices 1, 2, and 0. These values describe the orientation of the plane relative to the crystal lattice.

<a id="planar_density"></a>

#### Planar density of a crystal plane: `planar_density`

The number of atoms per unit area on the (h k l) plane of an SC, BCC or FCC crystal, and the planar packing fraction.

$$
\mathrm{PD} = n_V\,d_{\mathrm{planes}},\quad \mathrm{PPF} = \mathrm{PD}\,\pi R^2
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `structure` | in | crystal structure (SC, BCC, FCC) | - |
| `h` | in | Miller index h | - |
| `k` | in | Miller index k | - |
| `l` | in | Miller index l | - |
| `a` | in | lattice parameter | m |
| `PD_per_nm2` | out | atoms per nm^2 on the plane | - |
| `PD` | out | atoms per m^2 on the plane | 1/m^2 |
| `PPF` | out | planar packing fraction (hard spheres) | - |

**Applies when:** a > 0; integer indices between -3 and 3, not all zero.

```shell
$ materials-science solve planar_density structure=FCC h=1 k=1 l=1 "a=0.4 nm"
PD_per_nm2 = 14.43375673
PD = 1.443375673e+19 1/m^2
PPF = 0.9068996821
  by: Planar density of a crystal plane  (PD = (atoms per volume) x (spacing of the lattice planes parallel to (hkl)); the lattice-plane spacing is a/sqrt(N), halved when (hkl) is not a reciprocal-lattice vector (BCC h+k+l odd, FCC mixed))
  a = 4e-10 m
  structure = FCC
  assumes: a > 0; integer indices between -3 and 3, not all zero
```

**What the worked example means:** This calculation determines the number of atoms centered on the (h k l) plane per unit area, denoted as 1.44338e+19 1/m^2, along with the planar packing fraction 0.9069. The result depends on the lattice parameter 0.4 nm and the specific atomic arrangement of the FCC structure. A higher planar density indicates more tightly packed atoms on that specific crystallographic plane, which often correlates with slip systems in deformation mechanisms.

<a id="space_group"></a>

#### Space group of a common crystal structure: `space_group`

The space-group number, Hermann-Mauguin symbol and crystal system of a common metallic or ionic structure.

$$
\text{e.g. FCC: } Fm\bar{3}m\ (225)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `structure` | in | structure (SC, BCC, FCC, HCP, diamond, rocksalt, CsCl, zincblende, fluorite) | - |
| `number` | out | space-group number (International Tables) | - |
| `symbol` | out | Hermann-Mauguin symbol | - |
| `crystal_system` | out | crystal system | - |

```shell
$ materials-science solve space_group structure=BCC
number = 229
symbol = Im-3m
crystal_system = cubic
  by: Space group of a common crystal structure  (the symmetry of the ideal structure (International Tables numbering))
  structure = BCC
```

**What the worked example means:** This calculation identifies the specific symmetry classification for a given ideal crystal structure. By mapping the structural type BCC to its standard International Tables entry, it determines the unique space-group number 229 and the corresponding Hermann-Mauguin symbol Im-3m. The result also specifies the broader crystal system cubic that governs the lattice geometry, providing essential data for predicting physical properties and diffraction patterns.

<a id="theoretical_density"></a>

#### Theoretical density of a metal: `theoretical_density`

The density of a crystal from its structure, the atomic radius and the molar mass: rho = n A / (V_cell N_A).

$$
\rho = \frac{n A}{V_{\mathrm{cell}} N_A}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `structure` | in | crystal structure (SC, BCC, FCC, HCP, diamond) | - |
| `A` | in | molar mass (atomic weight) | kg/mol |
| `R` | in | atomic radius | m |
| `rho` | out | theoretical density | kg/m^3 |
| `rho_gcc` | out | density in g/cm^3 | - |
| `a_nm` | out | lattice parameter a in nm | - |

**Applies when:** A > 0; R > 0.

```shell
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

**What the worked example means:** This calculation determines the theoretical density of a crystal by assuming atoms are hard spheres packed in a specific structure. Using an atomic radius of 0.128 nm and a molar mass of 63.55 g/mol, the model computes a unit cell volume based on the chosen structure, such as face-centered cubic or hexagonal close-packed. The resulting theoretical density is 8895.28 kg/m^3, which converts to 8.89528 in grams per cubic centimeter, while the derived lattice parameter is 0.362039. This value represents the maximum possible density for a perfect crystal without defects.

<a id="miller_bravais_direction"></a>

#### Three-index to four-index (Miller-Bravais) direction, `miller_bravais_direction`

Convert a hexagonal direction [U V W] (axes a1, a2, c) to the Miller-Bravais form [u v t w] (axes a1, a2, a3, c).

$$
u = \tfrac13(2U-V),\ v = \tfrac13(2V-U),\ t = -(u+v),\ w = W
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `U` | in | index along a1 | - |
| `V` | in | index along a2 | - |
| `W` | in | index along c | - |
| `u` | out | Miller-Bravais index u | - |
| `v` | out | Miller-Bravais index v | - |
| `t` | out | Miller-Bravais index t | - |
| `w` | out | Miller-Bravais index w | - |
| `indices` | out | [u v t w] | - |

**Applies when:** integer indices, not all zero.

```shell
$ materials-science solve miller_bravais_direction U=1 V=0 W=0
u = 2
v = -1
t = -1
w = 0
indices = [2 -1 -1 0]
  by: Three-index to four-index (Miller-Bravais) direction  (u = (2U - V)/3, v = (2V - U)/3, t = -(u + v), w = W, reduced to the smallest integers)
  assumes: integer indices, not all zero
```

**What the worked example means:** This calculation transforms a hexagonal crystallographic direction given in three-index notation into the four-index Miller-Bravais system. Using the input values 1, 0, and 0, the algorithm computes the new indices as 2, -1, -1, and 0. The resulting set [2 -1 -1 0] allows for easier identification of symmetry-equivalent directions within the basal plane, ensuring that geometrically related vectors share similar index patterns.

<a id="hcp_cell_density"></a>

#### Volume and density of a hexagonal close-packed cell, `hcp_cell_density`

The volume of the HCP hexagonal prism (6 atoms), the density and the c/a ratio from a, c and the molar mass.

$$
V = \frac{3\sqrt3}{2}a^2c,\quad \rho = \frac{6A}{V N_A}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `a` | in | lattice parameter a | m |
| `c` | in | lattice parameter c | m |
| `A` | in | molar mass | kg/mol |
| `V` | out | volume of the hexagonal prism (6 atoms) | m^3 |
| `V_nm3` | out | prism volume in nm^3 | - |
| `rho` | out | density | kg/m^3 |
| `rho_gcc` | out | density in g/cm^3 | - |
| `c_over_a` | out | c/a ratio | - |

**Applies when:** a > 0; c > 0; A > 0.

```shell
$ materials-science solve hcp_cell_density "a=0.3209 nm" "c=0.5211 nm" "A=24.31 g/mol"
V = 1.39415928e-28 m^3
V_nm3 = 0.139415928
rho = 1737.292373 kg/m^3
rho_gcc = 1.737292373
c_over_a = 1.623870365
  by: Volume and density of a hexagonal close-packed cell  (V = (3 sqrt(3)/2) a^2 c, rho = 6 A/(V N_A))
  a = 3.209e-10 m
  c = 5.211e-10 m
  A = 0.02431 kg/mol
  assumes: a > 0; c > 0; A > 0
```

In the worked example, a = 0.3209 nm, c = 0.5211 nm, A = 24.31 g/mol go through the equation above and give V = 1.39415928e-28 m^3, V_nm3 = 0.139415928, rho = 1737.292373 kg/m^3, rho_gcc = 1.737292373. It applies only when a > 0; c > 0; A > 0: outside that it says what is wrong instead of returning a number.

<a id="polymorphic_volume_change"></a>

#### Volume change of a BCC to FCC transformation, `polymorphic_volume_change`

The percentage volume change when a metal transforms from BCC to FCC (as iron at 912 C), from the atomic radius in each structure.

$$
\frac{\Delta V}{V} = \frac{a_F^3/4 - a_B^3/2}{a_B^3/2}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `R_bcc` | in | atomic radius in the BCC form | m |
| `R_fcc` | in | atomic radius in the FCC form | m |
| `volume_change_percent` | out | volume change BCC -> FCC in percent (negative = contraction) | - |
| `V_atom_bcc_nm3` | out | volume per atom in BCC, nm^3 | - |
| `V_atom_fcc_nm3` | out | volume per atom in FCC, nm^3 | - |

**Applies when:** radii > 0.

```shell
$ materials-science solve polymorphic_volume_change "R_bcc=0.1258 nm" "R_fcc=0.1292 nm"
volume_change_percent = -0.4932571866
V_atom_bcc_nm3 = 0.01226055189
V_atom_fcc_nm3 = 0.01220007583
  by: Volume change of a BCC to FCC transformation  (volume per atom: BCC (4R/sqrt3)^3/2, FCC (2 sqrt2 R)^3/4)
  R_bcc = 1.258e-10 m
  R_fcc = 1.292e-10 m
  assumes: radii > 0
```

**What the worked example means:** This calculation determines the percentage volume change when a metal transforms from a body-centered cubic structure to a face-centered cubic structure, using atomic radii 0.1258 nm for the BCC phase and 0.1292 nm for the FCC phase. The resulting volume change is -0.493257, indicating whether the material expands or contracts during the transformation. This value helps predict dimensional stability and internal stresses in materials undergoing heat treatment.

<a id="unit_cell_volume"></a>

#### Volume of a general (triclinic) unit cell: `unit_cell_volume`

The volume of a unit cell from its six lattice parameters a, b, c, alpha, beta, gamma.

$$
V = abc\sqrt{1-\cos^2\alpha-\cos^2\beta-\cos^2\gamma+2\cos\alpha\cos\beta\cos\gamma}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `a` | in | cell edge a | m |
| `b` | in | cell edge b | m |
| `c` | in | cell edge c | m |
| `alpha` | in | cell angle alpha in degrees (between b and c) | - |
| `beta` | in | cell angle beta in degrees (between a and c) | - |
| `gamma` | in | cell angle gamma in degrees (between a and b) | - |
| `V` | out | cell volume | m^3 |
| `V_nm3` | out | cell volume in nm^3 | - |

**Applies when:** 0 < angles < 180; 1 - cos^2(alpha) - cos^2(beta) - cos^2(gamma) + 2 cos(alpha) cos(beta) cos(gamma) > 0.

```shell
$ materials-science solve unit_cell_volume "a=0.5 nm" "b=0.6 nm" "c=0.7 nm" alpha=90 beta=100 gamma=90
V = 2.068096281e-28 m^3
V_nm3 = 0.2068096281
  by: Volume of a general (triclinic) unit cell  (V = abc sqrt(1 - cos^2 alpha - cos^2 beta - cos^2 gamma + 2 cos alpha cos beta cos gamma))
  a = 5e-10 m
  b = 6e-10 m
  c = 7e-10 m
  alpha = 90
  beta = 100
  gamma = 90
  assumes: 0 < angles < 180; 1 - cos^2(alpha) - cos^2(beta) - cos^2(gamma) + 2 cos(alpha) cos(beta) cos(gamma) > 0
```

**What the worked example means:** This calculation determines the spatial extent of a general triclinic unit cell using its six lattice parameters. By combining the edge lengths and interaxial angles, the method yields the total volume as 2.0681e-28 m^3, which converts to 0.20681 for easier comparison with typical crystallographic data. This value represents the fundamental repeating volume within the crystalline structure.

### Defects (4)

<a id="burgers_vector"></a>

#### Burgers vector of a perfect dislocation: `burgers_vector`

The magnitude of the shortest lattice translation (the Burgers vector of a perfect dislocation) in an SC, BCC or FCC metal.

$$
b_{\mathrm{FCC}} = \frac{a}{\sqrt2},\ b_{\mathrm{BCC}} = \frac{a\sqrt3}{2},\ b_{\mathrm{SC}} = a
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `structure` | in | crystal structure (SC, BCC, FCC) | - |
| `a` | in | lattice parameter | m |
| `b` | out | Burgers vector magnitude | m |
| `b_nm` | out | Burgers vector in nm | - |
| `direction` | out | Burgers vector family | - |

**Applies when:** a > 0.

```shell
$ materials-science solve burgers_vector structure=FCC "a=0.3615 nm"
b = 2.556191014e-10 m
b_nm = 0.2556191014
direction = a/2<110>
  by: Burgers vector of a perfect dislocation  (shortest lattice translation: SC a<100>, BCC (a/2)<111> = a sqrt3/2, FCC (a/2)<110> = a/sqrt2)
  a = 3.615e-10 m
  structure = FCC
  assumes: a > 0
```

In the worked example, structure = FCC, a = 0.3615 nm go through the equation above and give b = 2.556191014e-10 m, b_nm = 0.2556191014, direction = a/2<110>. It applies only when a > 0: outside that it says what is wrong instead of returning a number.

<a id="vacancy_concentration"></a>

#### Equilibrium vacancy concentration: `vacancy_concentration`

The number of atomic sites per volume and the equilibrium number of vacancies N_v = N exp(-Q_v/kT) of a metal.

$$
N_v = N\exp\!\left(-\frac{Q_v}{kT}\right),\quad N = \frac{N_A\rho}{A}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `rho` | in | density | kg/m^3 |
| `A` | in | molar mass | kg/mol |
| `Qv` | in | vacancy formation energy (per vacancy, e.g. eV) | J |
| `T` | in | temperature | K |
| `N` | out | atomic sites per volume | 1/m^3 |
| `Nv` | out | vacancies per volume | 1/m^3 |
| `fraction` | out | vacancy fraction Nv/N | - |
| `log10_Nv` | out | log10 of Nv in m^-3 | - |
| `N_per_cm3` | out | atomic sites per cm^3 | - |

**Applies when:** all inputs > 0.

```shell
$ materials-science solve vacancy_concentration "rho=8.4 g/cm^3" "A=63.5 g/mol" "Qv=0.9 eV" "T=1000 K"
N = 7.966296438e+28 1/m^3
Nv = 2.319829658e+24 1/m^3
fraction = 2.912055402e-05
log10_Nv = 24.3654561
N_per_cm3 = 7.966296438e+22
  by: Equilibrium vacancy concentration  (N = N_A rho / A; N_v = N exp(-Q_v / (k T)))
  rho = 8400 kg/m^3
  A = 0.0635 kg/mol
  Qv = 1.441958971e-19 J
  T = 1000 K
  assumes: all inputs > 0
```

**What the worked example means:** This calculation determines the number of vacancies in a metal at thermal equilibrium. It first computes the total atomic sites per volume using the density and molar mass, yielding 7.9663e+28 1/m^3. Then, it applies the Boltzmann factor with the vacancy formation energy and temperature to find the vacancy concentration, resulting in 2.31983e+24 1/m^3. This value indicates how many atoms are missing from their lattice positions, which directly influences diffusion rates and material properties.

<a id="interstitial_site_size"></a>

#### Largest atom that fits an interstitial site, `interstitial_site_size`

The radius of the largest sphere that fits the octahedral or tetrahedral interstitial site of an FCC or BCC lattice of host atoms of radius R.

$$
r = d_{\mathrm{site}\to\mathrm{atom}} - R
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `structure` | in | crystal structure (FCC, BCC) | - |
| `site` | in | interstitial site (octahedral, tetrahedral) | - |
| `R` | in | radius of the host atoms | m |
| `r` | out | radius of the interstitial sphere | m |
| `r_nm` | out | interstitial radius in nm | - |
| `ratio` | out | r/R | - |

**Applies when:** R > 0.

```shell
$ materials-science solve interstitial_site_size structure=FCC site=octahedral "R=0.129 nm"
r = 5.343354955e-11 m
r_nm = 0.05343354955
ratio = 0.4142135624
  by: Largest atom that fits an interstitial site  (hard spheres: distance from the site centre to the nearest host atom minus R (FCC oct sqrt2-1, tet sqrt(3/2)-1; BCC oct 2/sqrt3-1, tet sqrt(5/3)-1))
  R = 1.29e-10 m
  structure = FCC
  site = octahedral
  assumes: R > 0
```

**What the worked example means:** This calculation determines the maximum radius of a sphere that can fit into the specified interstitial site within the crystal structure. Given a host atom radius of 0.129 nm, the resulting interstitial radius is 5.34335e-11 m, which corresponds to 0.0534335 when converted to nanometers. The ratio of the interstitial radius to the host radius is 0.414214, indicating how much space remains after accounting for the touching host atoms.

<a id="vacancy_formation_energy_fit"></a>

#### Vacancy formation energy from vacancy fractions, `vacancy_formation_energy_fit`

The vacancy formation energy Q_v and prefactor from vacancy fractions measured at two or more temperatures (least-squares Arrhenius line).

$$
\ln\frac{N_v}{N} = \ln C - \frac{Q_v}{k}\frac1T
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `T` | in | temperatures in kelvin | - |
| `fraction` | in | vacancy fractions N_v/N at those temperatures | - |
| `Qv_eV` | out | vacancy formation energy in eV | - |
| `Qv` | out | vacancy formation energy | J |
| `prefactor` | out | prefactor C of N_v/N = C exp(-Q_v/kT) | - |
| `r_squared` | out | r^2 of the fit | - |

**Applies when:** equal lengths, at least two distinct temperatures; 0 < fraction < 1; fraction rising with T.

```shell
$ materials-science solve vacancy_formation_energy_fit "T=[800, 1000]" "fraction=[1.2e-06, 3.4e-05]"
Qv_eV = 1.152667929
Qv = 1.846777623e-19 J
prefactor = 21.91137346
r_squared = 1
  by: Vacancy formation energy from vacancy fractions  (least-squares line of ln(N_v/N) against 1/T: slope = -Q_v/k)
  assumes: equal lengths, at least two distinct temperatures; 0 < fraction < 1; fraction rising with T
```

**What the worked example means:** This calculation determines the vacancy formation energy and prefactor by performing a least-squares linear fit of the natural logarithm of the vacancy fraction against the inverse of temperature. The slope of this Arrhenius line yields the formation energy, which is calculated as 1.15267, while the intercept provides the prefactor value of 21.9114. The quality of this linear relationship is indicated by an r-squared value of 1, confirming how well the experimental data aligns with theoretical expectations for equilibrium defect concentrations.

### Diffraction (11)

<a id="bragg_angle"></a>

#### Bragg angle of a reflection: `bragg_angle`

The diffraction angle theta (and 2-theta) of planes of spacing d for X-rays of wavelength lambda: n lambda = 2 d sin(theta).

$$
n\lambda = 2d\sin\theta
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `d` | in | interplanar spacing | m |
| `lam` | in | X-ray wavelength | m |
| `n` | in | order of the reflection (default 1) (optional) | - |
| `theta_deg` | out | Bragg angle theta in degrees | - |
| `two_theta_deg` | out | diffraction angle 2-theta in degrees | - |

**Applies when:** d, lambda > 0; n lambda < 2 d.

```shell
$ materials-science solve bragg_angle "d=0.2087 nm" "lam=0.1542 nm"
theta_deg = 21.68045609
two_theta_deg = 43.36091219
  by: Bragg angle of a reflection  (theta = asin(n lambda / (2 d)))
  d = 2.087e-10 m
  lam = 1.542e-10 m
  assumes: d, lambda > 0; n lambda < 2 d
```

**What the worked example means:** This calculation determines the angle at which X-rays constructively interfere with crystal planes. Using an interplanar spacing of 0.2087 nm and a wavelength of 0.1542 nm, the resulting Bragg angle is 21.6805. Consequently, the detector observes the peak at a two-theta position of 43.3609. This relationship confirms that larger spacings or shorter wavelengths shift the reflection to lower angles.

<a id="cubic_reflection_angle"></a>

#### Diffraction angle of a cubic (h k l) reflection, `cubic_reflection_angle`

The interplanar spacing and the diffraction angle 2-theta of the (h k l) reflection of a cubic crystal of lattice parameter a at wavelength lambda.

$$
\sin\theta = \frac{\lambda\sqrt{h^2+k^2+l^2}}{2a}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `a` | in | lattice parameter | m |
| `h` | in | Miller index h | - |
| `k` | in | Miller index k | - |
| `l` | in | Miller index l | - |
| `lam` | in | X-ray wavelength | m |
| `d_nm` | out | interplanar spacing in nm | - |
| `theta_deg` | out | Bragg angle in degrees | - |
| `two_theta_deg` | out | diffraction angle 2-theta in degrees | - |

**Applies when:** a, lambda > 0; lambda < 2 d.

```shell
$ materials-science solve cubic_reflection_angle "a=0.3615 nm" h=1 k=1 l=1 "lam=0.1542 nm"
d_nm = 0.2087121223
theta_deg = 21.67913311
two_theta_deg = 43.35826623
  by: Diffraction angle of a cubic (h k l) reflection  (d = a/sqrt(h^2 + k^2 + l^2), sin(theta) = lambda/(2 d) (first order))
  a = 3.615e-10 m
  lam = 1.542e-10 m
  assumes: a, lambda > 0; lambda < 2 d
```

**What the worked example means:** This calculation determines the geometric position of a specific reflection in an X-ray diffraction pattern for a cubic crystal. By combining the lattice parameter 0.3615 nm with the Miller indices h, k, and l, it first computes the interplanar spacing d as 0.208712. Using this spacing and the incident wavelength 0.1542 nm, it applies Bragg's law to find the Bragg angle theta as 21.6791, which corresponds to a measured detector angle of 43.3583. This result allows researchers to identify phases by matching calculated angles to experimental peaks.

<a id="allowed_reflections_cubic"></a>

#### First allowed reflections of a cubic crystal, `allowed_reflections_cubic`

The values of h^2 + k^2 + l^2 of the first allowed X-ray reflections of a monatomic SC, BCC, FCC or diamond-cubic crystal, in order of increasing angle.

$$
N = h^2+k^2+l^2,\quad \sin^2\theta = \frac{\lambda^2}{4a^2}N
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `structure` | in | crystal structure (SC, BCC, FCC, diamond) | - |
| `count` | in | how many reflections (1 to 15, default 6) (optional) | - |
| `N_values` | out | h^2 + k^2 + l^2 of the allowed reflections, ascending | - |
| `N_last` | out | the last of them | - |
| `N_first` | out | the first allowed reflection | - |

**Applies when:** 1 <= count <= 15.

```shell
$ materials-science solve allowed_reflections_cubic structure=FCC count=6
N_values = [3,4,8,11,12,16]
N_last = 16
N_first = 3
  by: First allowed reflections of a cubic crystal  (each N = h^2 + k^2 + l^2 is allowed when some (h k l) with that sum passes the extinction rules)
  structure = FCC
  assumes: 1 <= count <= 15
```

**What the worked example means:** This calculation determines the sequence of squared Miller index sums, denoted as N, that correspond to observable X-ray diffraction peaks for a cubic crystal with structure FCC. Applying extinction rules specific to the lattice type identifies which integer combinations of h, k, and l produce non-zero intensity. The resulting list [3, 4, 8, 11, 12, 16] represents these valid N values in ascending order, directly linking the crystal's atomic arrangement to its diffraction pattern.

<a id="bragg_spacing"></a>

#### Interplanar spacing from a Bragg reflection, `bragg_spacing`

The interplanar spacing d of planes that reflect X-rays of wavelength lambda at the Bragg angle theta in order n.

$$
d = \frac{n\lambda}{2\sin\theta}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `lam` | in | X-ray wavelength | m |
| `theta` | in | Bragg angle theta in degrees (half of 2-theta) | - |
| `n` | in | order of the reflection (default 1) (optional) | - |
| `d` | out | interplanar spacing | m |
| `d_nm` | out | interplanar spacing in nm | - |

**Applies when:** lambda > 0; 0 < theta < 90 degrees.

```shell
$ materials-science solve bragg_spacing "lam=0.1542 nm" theta=30
d = 1.542e-10 m
d_nm = 0.1542
  by: Interplanar spacing from a Bragg reflection  (d = n lambda / (2 sin(theta)))
  lam = 1.542e-10 m
  theta = 30
  assumes: lambda > 0; 0 < theta < 90 degrees
```

**What the worked example means:** The interplanar spacing is 1.542e-10 m (0.1542).

<a id="d_spacing_cubic"></a>

#### Interplanar spacing in a cubic crystal: `d_spacing_cubic`

The spacing of the (h k l) planes of a cubic lattice: d = a / sqrt(h^2 + k^2 + l^2).

$$
d_{hkl} = \frac{a}{\sqrt{h^2+k^2+l^2}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `a` | in | lattice parameter | m |
| `h` | in | Miller index h | - |
| `k` | in | Miller index k | - |
| `l` | in | Miller index l | - |
| `d` | out | interplanar spacing | m |
| `d_nm` | out | interplanar spacing in nm | - |

**Applies when:** a > 0; integer indices, not all zero.

```shell
$ materials-science solve d_spacing_cubic "a=0.3615 nm" h=1 k=1 l=1
d = 2.087121223e-10 m
d_nm = 0.2087121223
  by: Interplanar spacing in a cubic crystal  (d = a / sqrt(h^2 + k^2 + l^2))
  a = 3.615e-10 m
  assumes: a > 0; integer indices, not all zero
```

**What the worked example means:** This calculation determines the perpendicular distance between parallel planes in a cubic crystal using the lattice parameter and Miller indices. Applying the formula d = a / sqrt(h^2 + k^2 + l^2) shows that for the given inputs, the resulting spacing is 2.08712e-10 m. This value is also expressed as 0.208712 in nanometers, providing a convenient scale for comparing with typical X-ray diffraction data.

<a id="d_spacing_triclinic"></a>

#### Interplanar spacing in a general (triclinic) lattice, `d_spacing_triclinic`

The spacing of the (h k l) planes of any lattice from its six cell parameters, through the reciprocal metric tensor.

$$
\frac{1}{d_{hkl}^2} = \mathbf{h}^{\mathsf T} G^{-1}\mathbf{h}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `a` | in | cell edge a | m |
| `b` | in | cell edge b | m |
| `c` | in | cell edge c | m |
| `alpha` | in | cell angle alpha in degrees (between b and c) | - |
| `beta` | in | cell angle beta in degrees (between a and c) | - |
| `gamma` | in | cell angle gamma in degrees (between a and b) | - |
| `h` | in | Miller index h | - |
| `k` | in | Miller index k | - |
| `l` | in | Miller index l | - |
| `d` | out | interplanar spacing | m |
| `d_nm` | out | interplanar spacing in nm | - |

**Applies when:** a valid cell; integer indices, not all zero.

```shell
$ materials-science solve d_spacing_triclinic "a=0.5 nm" "b=0.5 nm" "c=0.7 nm" alpha=90 beta=90 gamma=120 h=1 k=0 l=0
d = 4.330127019e-10 m
d_nm = 0.4330127019
  by: Interplanar spacing in a general (triclinic) lattice  (1/d^2 = h G* h^T with G* the inverse of the metric tensor G_ij = a_i . a_j)
  a = 5e-10 m
  b = 5e-10 m
  c = 7e-10 m
  alpha = 90
  beta = 90
  gamma = 120
  assumes: a valid cell; integer indices, not all zero
```

**What the worked example means:** This calculation determines the distance between parallel lattice planes defined by Miller indices h, k, and l. It uses the six cell parameters of a triclinic unit cell to construct the metric tensor and its inverse. The result, 4.33013e-10 m m, represents the physical separation of these planes in meters. This value is also expressed as 0.433013 nm for convenience in nanometer-scale analysis. The method accounts for arbitrary angles between cell edges, making it suitable for any crystal system.

<a id="d_spacing_hexagonal"></a>

#### Interplanar spacing in a hexagonal crystal, `d_spacing_hexagonal`

The spacing of the (h k l) planes of a hexagonal lattice: 1/d^2 = 4(h^2 + hk + k^2)/(3a^2) + l^2/c^2.

$$
\frac{1}{d^2} = \frac{4}{3}\frac{h^2+hk+k^2}{a^2}+\frac{l^2}{c^2}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `a` | in | lattice parameter a | m |
| `c` | in | lattice parameter c | m |
| `h` | in | Miller index h | - |
| `k` | in | Miller index k | - |
| `l` | in | Miller index l | - |
| `i` | in | Miller-Bravais index i = -(h + k) (optional, checked) (optional) | - |
| `d` | out | interplanar spacing | m |
| `d_nm` | out | interplanar spacing in nm | - |

**Applies when:** a, c > 0; integer indices, not all zero; i = -(h + k) when given.

```shell
$ materials-science solve d_spacing_hexagonal "a=0.3209 nm" "c=0.5211 nm" h=1 k=0 l=1
d = 2.452149781e-10 m
d_nm = 0.2452149781
  by: Interplanar spacing in a hexagonal crystal  (1/d^2 = 4(h^2 + hk + k^2)/(3a^2) + l^2/c^2)
  a = 3.209e-10 m
  c = 5.211e-10 m
  assumes: a, c > 0; integer indices, not all zero; i = -(h + k) when given
```

**What the worked example means:** This calculation determines the distance between parallel atomic planes in a hexagonal crystal using the provided lattice parameters and Miller indices. Applying the relationship between the reciprocal squared spacing and the geometric factors of the unit cell shows that the resulting interplanar spacing is 2.45215e-10 m meters. This value is often expressed as 0.245215 nanometers to facilitate comparison with standard diffraction data, allowing researchers to verify phase identification or refine structural models based on the measured peak positions.

<a id="d_spacing_tetragonal"></a>

#### Interplanar spacing in a tetragonal crystal, `d_spacing_tetragonal`

The spacing of the (h k l) planes of a tetragonal lattice: 1/d^2 = (h^2 + k^2)/a^2 + l^2/c^2.

$$
\frac{1}{d^2} = \frac{h^2+k^2}{a^2}+\frac{l^2}{c^2}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `a` | in | lattice parameter a | m |
| `c` | in | lattice parameter c | m |
| `h` | in | Miller index h | - |
| `k` | in | Miller index k | - |
| `l` | in | Miller index l | - |
| `d` | out | interplanar spacing | m |
| `d_nm` | out | interplanar spacing in nm | - |

**Applies when:** a, c > 0; integer indices, not all zero.

```shell
$ materials-science solve d_spacing_tetragonal "a=0.3 nm" "c=0.5 nm" h=1 k=0 l=1
d = 2.572478777e-10 m
d_nm = 0.2572478777
  by: Interplanar spacing in a tetragonal crystal  (1/d^2 = (h^2 + k^2)/a^2 + l^2/c^2)
  a = 3e-10 m
  c = 5e-10 m
  assumes: a, c > 0; integer indices, not all zero
```

**What the worked example means:** This calculation determines the distance between adjacent crystal planes defined by Miller indices h, k, and l in a tetragonal lattice. Using the lattice parameters a = 0.3 nm and c = 0.5 nm, the formula yields an interplanar spacing of d = 2.57248e-10 m. This value represents the periodicity of the atomic arrangement along that specific direction within the crystal structure.

<a id="d_spacing_orthorhombic"></a>

#### Interplanar spacing in an orthorhombic crystal, `d_spacing_orthorhombic`

The spacing of the (h k l) planes of an orthorhombic lattice: 1/d^2 = h^2/a^2 + k^2/b^2 + l^2/c^2.

$$
\frac{1}{d^2} = \frac{h^2}{a^2}+\frac{k^2}{b^2}+\frac{l^2}{c^2}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `a` | in | lattice parameter a | m |
| `b` | in | lattice parameter b | m |
| `c` | in | lattice parameter c | m |
| `h` | in | Miller index h | - |
| `k` | in | Miller index k | - |
| `l` | in | Miller index l | - |
| `d` | out | interplanar spacing | m |
| `d_nm` | out | interplanar spacing in nm | - |

**Applies when:** a, b, c > 0; integer indices, not all zero.

```shell
$ materials-science solve d_spacing_orthorhombic "a=0.4 nm" "b=0.5 nm" "c=0.6 nm" h=1 k=1 l=0
d = 3.123475238e-10 m
d_nm = 0.3123475238
  by: Interplanar spacing in an orthorhombic crystal  (1/d^2 = h^2/a^2 + k^2/b^2 + l^2/c^2)
  a = 4e-10 m
  b = 5e-10 m
  c = 6e-10 m
  assumes: a, b, c > 0; integer indices, not all zero
```

**What the worked example means:** This calculation determines the distance between parallel atomic planes in an orthorhombic crystal using the lattice parameters and Miller indices. The resulting interplanar spacing is 3.12348e-10 m meters or 0.312348 nanometers. This value is crucial for interpreting X-ray diffraction data to identify the material structure and verify lattice dimensions.

<a id="structure_factor_cubic"></a>

#### Structure factor and extinction of a cubic reflection, `structure_factor_cubic`

The structure factor |F|/f of the (h k l) reflection of a monatomic SC, BCC, FCC or diamond-cubic crystal, and whether the reflection is allowed.

$$
F_{hkl} = f\sum_j e^{2\pi i(hx_j+ky_j+lz_j)}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `structure` | in | crystal structure (SC, BCC, FCC, diamond) | - |
| `h` | in | Miller index h | - |
| `k` | in | Miller index k | - |
| `l` | in | Miller index l | - |
| `F_over_f` | out | \|F\| in units of the atomic scattering factor f | - |
| `F2_over_f2` | out | \|F\|^2 / f^2 (proportional to intensity) | - |
| `allowed` | out | whether the reflection appears | - |

**Applies when:** integer indices, not all zero.

```shell
$ materials-science solve structure_factor_cubic structure=FCC h=2 k=0 l=0
F_over_f = 4
F2_over_f2 = 16
allowed = true
  by: Structure factor and extinction of a cubic reflection  (extinction rules: BCC h+k+l even (F = 2f); FCC unmixed (F = 4f); diamond: all odd (4 sqrt(2) f), all even with h+k+l = 4m (8f), otherwise 0)
  structure = FCC
  assumes: integer indices, not all zero
```

**What the worked example means:** This calculation determines the structure factor for a specific set of Miller indices in a given crystal structure. By summing the scattering contributions from all atoms within the unit cell, it reveals whether the reflection is allowed or extinct due to destructive interference. The result provides the normalized structure factor magnitude as 4 and the squared intensity proportionality as 16, indicating how strong the diffraction spot would be if present.

<a id="bragg_wavelength"></a>

#### X-ray wavelength from a Bragg reflection: `bragg_wavelength`

The wavelength that reflects from planes of spacing d at the Bragg angle theta in order n.

$$
\lambda = \frac{2d\sin\theta}{n}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `d` | in | interplanar spacing | m |
| `theta` | in | Bragg angle theta in degrees (half of 2-theta) | - |
| `n` | in | order of the reflection (default 1) (optional) | - |
| `lam` | out | wavelength | m |
| `lam_nm` | out | wavelength in nm | - |

**Applies when:** d > 0; 0 < theta < 90 degrees.

```shell
$ materials-science solve bragg_wavelength "d=0.2 nm" theta=30
lam = 2e-10 m
lam_nm = 0.2
  by: X-ray wavelength from a Bragg reflection  (lambda = 2 d sin(theta) / n)
  d = 2e-10 m
  theta = 30
  assumes: d > 0; 0 < theta < 90 degrees
```

**What the worked example means:** The calculated wavelength is 2e-10 m, equivalent to 0.2.

### Diffusion (6)

<a id="diffusion_arrhenius_fit"></a>

#### Activation energy and pre-exponential factor of diffusion, `diffusion_arrhenius_fit`

Q_d and D_0 of D = D_0 exp(-Q_d/RT) from diffusion coefficients at two or more temperatures, and D at another temperature.

$$
\ln D = \ln D_0 - \frac{Q_d}{R}\frac{1}{T}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `T` | in | temperatures in kelvin | - |
| `D` | in | diffusion coefficients in m^2/s at those temperatures | - |
| `T_target` | in | temperature at which D is wanted (optional) (optional) | K |
| `Q` | out | activation energy | J/mol |
| `Q_kJmol` | out | activation energy in kJ/mol | - |
| `D0` | out | pre-exponential factor | m^2/s |
| `log10_D0` | out | log10 of D0 in m^2/s | - |
| `r_squared` | out | r^2 of the fit | - |
| `D_target` | out | D at T_target | m^2/s |
| `log10_D_target` | out | log10 of D at T_target | - |

**Applies when:** equal lengths, at least two distinct temperatures; D rising with T.

```shell
$ materials-science solve diffusion_arrhenius_fit "T=[773, 873]" "D=[5.8e-13, 5.3e-12]"
Q = 124136.1426 J/mol
Q_kJmol = 124.1361426
D0 = 0.0001417848903 m^2/s
log10_D0 = -3.848370049
r_squared = 1
  by: Activation energy and pre-exponential factor of diffusion  (least-squares line of ln D against 1/T: slope = -Q_d/R, intercept = ln D_0)
  assumes: equal lengths, at least two distinct temperatures; D rising with T
```

In the worked example, T = [773, 873], D = [5.8e-13, 5.3e-12] go through the equation above and give Q = 124136.1426 J/mol, Q_kJmol = 124.1361426, D0 = 0.0001417848903 m^2/s, log10_D0 = -3.848370049. It applies only when equal lengths, at least two distinct temperatures; D rising with T: outside that it says what is wrong instead of returning a number.

<a id="fick_second_law_concentration"></a>

#### Concentration in a semi-infinite solid (Fick's second law), `fick_second_law_concentration`

The concentration at depth x after time t in a semi-infinite solid with a constant surface concentration (carburizing, doping): C_x = C_s - (C_s - C_0) erf(x / 2 sqrt(Dt)).

$$
\frac{C_x - C_0}{C_s - C_0} = 1 - \operatorname{erf}\!\left(\frac{x}{2\sqrt{Dt}}\right)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `C0` | in | initial (bulk) concentration (wt%, at% or any one unit) | - |
| `Cs` | in | surface concentration (same unit) | - |
| `x` | in | depth below the surface | m |
| `D` | in | diffusion coefficient | m^2/s |
| `t` | in | diffusion time | s |
| `Cx` | out | concentration at depth x (unit of C0, Cs) | - |
| `z` | out | erf argument x / (2 sqrt(D t)) | - |
| `ratio` | out | (Cx - C0)/(Cs - C0) | - |

**Applies when:** x, D, t > 0; Cs != C0; constant D, surface concentration held constant.

```shell
$ materials-science solve fick_second_law_concentration C0=0.25 Cs=1.2 "x=0.5 mm" "D=1.6e-11 m^2/s" "t=7 h"
Cx = 0.7987846097
z = 0.3937129927
ratio = 0.5776680102
  by: Concentration in a semi-infinite solid (Fick's second law)  (the error-function solution of dC/dt = D d2C/dx2)
  C0 = 0.25
  Cs = 1.2
  x = 0.0005 m
  D = 1.6e-11 m^2/s
  t = 25200 s
  assumes: x, D, t > 0; Cs != C0; constant D, surface concentration held constant
```

**What the worked example means:** This calculation determines the concentration profile within a semi-infinite solid after diffusion for time 7 h. Using the error function solution to Fick's second law, the result shows that at depth 0.5 mm, the local concentration 0.798785 reflects the interplay between the constant surface concentration 1.2 and the initial uniform concentration 0.25. The dimensionless ratio 0.577668 indicates how far the concentration has equilibrated towards the surface value relative to the total possible change. This model assumes a constant diffusion coefficient 1.6e-11 m^2/s throughout the process.

<a id="diffusion_depth_of_concentration"></a>

#### Depth at which a concentration is reached (Fick's second law), `diffusion_depth_of_concentration`

The depth at which the concentration equals C_x after diffusion time t in a semi-infinite solid with constant surface concentration (case depth).

$$
x = 2z\sqrt{Dt}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `C0` | in | initial concentration | - |
| `Cs` | in | surface concentration | - |
| `Cx` | in | target concentration | - |
| `D` | in | diffusion coefficient | m^2/s |
| `t` | in | diffusion time | s |
| `x` | out | depth | m |
| `x_mm` | out | depth in mm | - |
| `z` | out | erf argument | - |

**Applies when:** D, t > 0; Cx strictly between C0 and Cs.

```shell
$ materials-science solve diffusion_depth_of_concentration C0=0.2 Cs=1 Cx=0.6 "D=2e-11 m^2/s" "t=10 h"
x = 0.0008093877002 m
x_mm = 0.8093877002
z = 0.4769362762
  by: Depth at which a concentration is reached (Fick's second law)  (x = 2 z sqrt(D t) with erf(z) = (Cs - Cx)/(Cs - C0))
  C0 = 0.2
  Cs = 1
  Cx = 0.6
  D = 2e-11 m^2/s
  t = 36000 s
  assumes: D, t > 0; Cx strictly between C0 and Cs
```

**What the worked example means:** This calculation determines how far a specific concentration penetrates into a material over time. Given an initial concentration of 0.2, a surface concentration of 1, and a target concentration of 0.6, with a diffusion coefficient of 2e-11 m^2/s acting for a duration of 10 h, the resulting depth is 0.000809388 m. This value represents the case depth, indicating where the material's properties change significantly due to diffusion.

<a id="arrhenius_diffusivity"></a>

#### Diffusion coefficient at a temperature (Arrhenius), `arrhenius_diffusivity`

The diffusion coefficient D = D0 exp(-Q_d/RT) at temperature T from the pre-exponential factor and the activation energy.

$$
D = D_0\exp\!\left(-\frac{Q_d}{RT}\right)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `D0` | in | pre-exponential factor | m^2/s |
| `Q` | in | activation energy per mole | J/mol |
| `T` | in | absolute temperature | K |
| `D` | out | diffusion coefficient | m^2/s |
| `log10_D` | out | log10 of D in m^2/s | - |
| `Q_over_RT` | out | Q_d/(R T) | - |

**Applies when:** D0, Q, T > 0.

```shell
$ materials-science solve arrhenius_diffusivity "D0=2.3e-05 m^2/s" "Q=148 kJ/mol" "T=773 K"
D = 2.296058635e-15 m^2/s
log10_D = -14.63901703
Q_over_RT = 23.02756604
  by: Diffusion coefficient at a temperature (Arrhenius)  (D = D0 exp(-Q_d/(R T)), R = N_A k)
  D0 = 2.3e-05 m^2/s
  Q = 148000 J/mol
  T = 773 K
  assumes: D0, Q, T > 0
```

**What the worked example means:** This calculation determines the diffusion coefficient, which measures how quickly atoms move through a solid material. It uses the Arrhenius equation, combining the pre-exponential factor 2.3e-05 m^2/s, which represents the maximum possible diffusion rate, with the activation energy 148 kJ/mol, which is the energy barrier atoms must overcome to jump between sites. These values are evaluated at the absolute temperature 773 K. The resulting diffusion coefficient is 2.29606e-15 m^2/s. A higher value indicates faster atomic movement, which is crucial for understanding processes like heat treatment or sintering.

<a id="fick_first_law_flux"></a>

#### Steady-state diffusion flux (Fick's first law), `fick_first_law_flux`

The steady-state diffusion flux between two positions with known concentrations: J = -D (C2 - C1)/(x2 - x1).

$$
J = -D\frac{dC}{dx} = -D\frac{C_2-C_1}{x_2-x_1}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `D` | in | diffusion coefficient | m^2/s |
| `C1` | in | concentration at x1 | kg/m^3 |
| `C2` | in | concentration at x2 | kg/m^3 |
| `x1` | in | first position | m |
| `x2` | in | second position | m |
| `J` | out | diffusion flux (positive towards increasing x) | kg/(m^2*s) |
| `J_per_hour` | out | flux in kg/(m^2 h) | - |

**Applies when:** D > 0; x1 != x2.

```shell
$ materials-science solve fick_first_law_flux "D=3e-11 m^2/s" "C1=1.2 kg/m^3" "C2=0.8 kg/m^3" "x1=5 mm" "x2=10 mm"
J = 2.4e-09 kg/(m^2*s)
J_per_hour = 8.64e-06
  by: Steady-state diffusion flux (Fick's first law)  (J = -D dC/dx with the linear steady-state profile)
  D = 3e-11 m^2/s
  C1 = 1.2 kg/m^3
  C2 = 0.8 kg/m^3
  x1 = 0.005 m
  x2 = 0.01 m
  assumes: D > 0; x1 != x2
```

**What the worked example means:** This calculation determines the steady-state diffusion flux using Fick's first law. By taking the diffusion coefficient, two concentrations, and their respective positions, it computes the rate of mass transfer through the material. The result, 2.4e-09 kg/(m^2*s), represents the flux in kilograms per square meter per second. This value indicates how much material moves across a unit area each second under the given gradient. For practical applications involving longer timeframes, the flux is also expressed as 8.64e-06 in kilograms per square meter per hour.

<a id="diffusion_time_to_concentration"></a>

#### Time to reach a concentration at a depth (Fick's second law), `diffusion_time_to_concentration`

The diffusion time for the concentration at depth x to reach C_x in a semi-infinite solid with constant surface concentration (carburizing time).

$$
t = \frac{x^2}{4Dz^2},\quad \operatorname{erf}z = \frac{C_s-C_x}{C_s-C_0}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `C0` | in | initial concentration | - |
| `Cs` | in | surface concentration | - |
| `Cx` | in | target concentration at depth x | - |
| `x` | in | depth | m |
| `D` | in | diffusion coefficient | m^2/s |
| `t` | out | diffusion time | s |
| `t_hours` | out | diffusion time in hours | - |
| `z` | out | erf argument | - |

**Applies when:** x, D > 0; Cx strictly between C0 and Cs.

```shell
$ materials-science solve diffusion_time_to_concentration C0=0 Cs=1 Cx=0.4795001222 "x=1 mm" "D=1e-10 m^2/s"
t = 10000 s
t_hours = 2.777777778
z = 0.5
  by: Time to reach a concentration at a depth (Fick's second law)  (erf(z) = (Cs - Cx)/(Cs - C0) inverted by bisection; t = x^2/(4 D z^2))
  C0 = 0
  Cs = 1
  Cx = 0.4795001222
  x = 0.001 m
  D = 1e-10 m^2/s
  assumes: x, D > 0; Cx strictly between C0 and Cs
```

**What the worked example means:** This calculation determines the time required for a diffusing species to reach a target concentration at a specific depth in a semi-infinite solid. Given an initial concentration of 0, a constant surface concentration of 1, and a desired concentration of 0.4795 at depth 1 mm, with a diffusion coefficient of 1e-10 m^2/s, the model computes the necessary duration. The result indicates that it takes 10000 s seconds, or approximately 2.77778 hours, for the concentration profile to develop such that the value at 1 mm equals 0.4795. This timing is essential for scheduling heat treatments like carburizing.

### Electronic properties (5)

<a id="band_gap_from_conductivity"></a>

#### Band gap from intrinsic conductivities at several temperatures, `band_gap_from_conductivity`

The band gap of an intrinsic semiconductor from its conductivity at two or more temperatures, with sigma = C exp(-Eg/2kT) (the weak temperature dependence of the prefactor neglected).

$$
\ln\sigma = \ln C - \frac{E_g}{2k}\frac{1}{T}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `T` | in | temperatures in kelvin | - |
| `sigma` | in | conductivities (any one unit) at those temperatures | - |
| `Eg_eV` | out | band gap in eV | - |
| `prefactor` | out | prefactor C (unit of sigma) | - |
| `r_squared` | out | r^2 of the fit | - |

**Applies when:** equal lengths, at least two distinct temperatures; sigma rising with T.

```shell
$ materials-science solve band_gap_from_conductivity "T=[300, 400]" "sigma=[2.2, 63]"
Eg_eV = 0.6938009484
prefactor = 1479429.095
r_squared = 1
  by: Band gap from intrinsic conductivities at several temperatures  (least-squares line of ln sigma against 1/T: slope = -Eg/(2k))
  assumes: equal lengths, at least two distinct temperatures; sigma rising with T
```

**What the worked example means:** This calculation determines the band gap energy for an intrinsic semiconductor by analyzing conductivity measurements taken at various temperatures. By plotting the natural logarithm of conductivity against the inverse of absolute temperature, a linear relationship is established where the slope corresponds to negative half the band gap divided by Boltzmann's constant. The resulting band gap value is 0.693801, derived from input temperatures and conductivities such as [300, 400] and [2.2, 63]. The goodness of this linear fit is indicated by the coefficient of determination, which is 1, while the pre-exponential factor governing the magnitude of conductivity is estimated as 1.47943e+06.

<a id="pn_junction_built_in_potential"></a>

#### Built-in potential of a p-n junction: `pn_junction_built_in_potential`

The built-in potential V_bi = (kT/q) ln(n_n p_p/ni^2) of an abrupt p-n junction (majority concentrations from charge neutrality; V_bi ~ (kT/q) ln(Na Nd/ni^2) when Na, Nd >> ni), and with a permittivity the zero-bias depletion width.

$$
V_{bi} = \frac{kT}{q}\ln\frac{N_aN_d}{n_i^2},\quad W = \sqrt{\frac{2\varepsilon V_{bi}}{q}\left(\frac{1}{N_a} + \frac{1}{N_d}\right)}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Na` | in | acceptor concentration on the p side | 1/m^3 |
| `Nd` | in | donor concentration on the n side | 1/m^3 |
| `ni` | in | intrinsic carrier concentration | 1/m^3 |
| `T` | in | temperature | K |
| `eps_r` | in | relative permittivity of the semiconductor (optional, for the depletion width) (optional) | - |
| `V_bi` | out | built-in potential | V |
| `V_bi_approx` | out | (kT/q) ln(Na Nd/ni^2) | V |
| `thermal_voltage` | out | kT/q | V |
| `W` | out | zero-bias depletion width | m |
| `W_um` | out | depletion width in um | - |
| `x_n_um` | out | depletion width on the n side in um | - |
| `x_p_um` | out | depletion width on the p side in um | - |

**Applies when:** Na, Nd, ni, T > 0; abrupt junction, complete ionisation, non-degenerate; depletion approximation for W.

```shell
$ materials-science solve pn_junction_built_in_potential "Na=1e+23 m^-3" "Nd=1e+22 m^-3" "ni=1e+16 m^-3" "T=300 K"
V_bi = 0.7738435813 V
V_bi_approx = 0.7738435813 V
thermal_voltage = 0.02585199979 V
  by: Built-in potential of a p-n junction  (V_bi = (kT/q) ln(n_n p_p/ni^2); W = sqrt(2 eps V_bi (1/Na + 1/Nd)/q), x_n = W Na/(Na + Nd))
  Na = 1e+23 1/m^3
  Nd = 1e+22 1/m^3
  ni = 1e+16 1/m^3
  T = 300 K
  assumes: Na, Nd, ni, T > 0; abrupt junction, complete ionisation, non-degenerate; depletion approximation for W
```

In the worked example, Na = 1e+23 m^-3, Nd = 1e+22 m^-3, ni = 1e+16 m^-3, T = 300 K go through the equation above and give V_bi = 0.7738435813 V, V_bi_approx = 0.7738435813 V, thermal_voltage = 0.02585199979 V. It applies only when Na, Nd, ni, T > 0; abrupt junction, complete ionisation, non-degenerate; depletion approximation for W: outside that it says what is wrong instead of returning a number.

<a id="extrinsic_carrier_concentration"></a>

#### Electron and hole concentrations of a doped semiconductor, `extrinsic_carrier_concentration`

The electron and hole concentrations of a semiconductor with donor and acceptor concentrations Nd and Na (complete ionisation), from charge neutrality and n p = ni^2, and the Fermi level relative to the intrinsic level.

$$
n = \frac{N_d - N_a}{2} + \sqrt{\left(\frac{N_d - N_a}{2}\right)^2 + n_i^2},\quad p = \frac{n_i^2}{n}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Nd` | in | donor concentration | 1/m^3 |
| `Na` | in | acceptor concentration | 1/m^3 |
| `ni` | in | intrinsic carrier concentration | 1/m^3 |
| `T` | in | temperature (optional, for the Fermi level) (optional) | K |
| `n` | out | electron concentration | 1/m^3 |
| `p` | out | hole concentration | 1/m^3 |
| `log10_n` | out | log10 of n in m^-3 | - |
| `log10_p` | out | log10 of p in m^-3 | - |
| `carrier_type` | out | n-type, p-type or intrinsic | - |
| `EF_minus_Ei_eV` | out | Fermi level above the intrinsic level in eV (when T is given) | - |

**Applies when:** Nd, Na >= 0; ni > 0; complete ionisation, non-degenerate statistics.

```shell
$ materials-science solve extrinsic_carrier_concentration "Nd=1e+22 m^-3" "Na=2e+21 m^-3" "ni=1e+16 m^-3"
n = 8e+21 1/m^3
p = 1.25e+10 1/m^3
log10_n = 21.90308999
log10_p = 10.09691001
carrier_type = n-type
  by: Electron and hole concentrations of a doped semiconductor  (n - p = Nd - Na, n p = ni^2: majority = |Nd - Na|/2 + sqrt((Nd - Na)^2/4 + ni^2), minority = ni^2/majority; E_F - E_i = kT ln(n/ni))
  Nd = 1e+22 1/m^3
  Na = 2e+21 1/m^3
  ni = 1e+16 1/m^3
  assumes: Nd, Na >= 0; ni > 0; complete ionisation, non-degenerate statistics
```

**What the worked example means:** This calculation determines the free electron and hole concentrations in a doped semiconductor using charge neutrality and the mass action law. Given donor and acceptor concentrations, the majority carrier density is derived from the net doping level combined with intrinsic effects, while the minority carrier density follows from the product of electrons and holes equaling the square of the intrinsic concentration. The result shows that the electron concentration is 8e+21 1/m^3 and the hole concentration is 1.25e+10 1/m^3, classifying the material as n-type.

<a id="fermi_energy_free_electron"></a>

#### Fermi energy of a free-electron metal: `fermi_energy_free_electron`

The conduction-electron density, Fermi energy, Fermi temperature and Fermi velocity of a metal from its density, molar mass and number of valence electrons per atom.

$$
E_F = \frac{\hbar^2}{2m}\left(3\pi^2n\right)^{2/3}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `rho` | in | density | kg/m^3 |
| `A` | in | molar mass | kg/mol |
| `valence` | in | valence electrons per atom | - |
| `n` | out | conduction-electron density | 1/m^3 |
| `log10_n` | out | log10 of n in m^-3 | - |
| `EF` | out | Fermi energy | J |
| `EF_eV` | out | Fermi energy in eV | - |
| `TF` | out | Fermi temperature | K |
| `vF` | out | Fermi velocity | m/s |

**Applies when:** all inputs > 0; free-electron model.

```shell
$ materials-science solve fermi_energy_free_electron "rho=8.96 g/cm^3" "A=63.55 g/mol" valence=1
n = 8.490697279e+28 1/m^3
log10_n = 28.92894336
EF = 1.128607421e-18 J
EF_eV = 7.044213463
TF = 81744.70278 K
vF = 1574134.91 m/s
  by: Fermi energy of a free-electron metal  (n = z N_A rho/A; E_F = (hbar^2/2m)(3 pi^2 n)^(2/3))
  rho = 8960 kg/m^3
  A = 0.06355 kg/mol
  valence = 1
  assumes: all inputs > 0; free-electron model
```

**What the worked example means:** This calculation determines the electronic properties of a metal using the free-electron model. It takes the material's density, molar mass, and valence electrons per atom to compute the conduction-electron density, which is found to be 8.4907e+28 1/m^3 per cubic meter. From this density, the Fermi energy is derived as 7.04421 electronvolts. This energy represents the highest occupied quantum state at absolute zero temperature. The associated Fermi temperature is 81744.7 K Kelvin, indicating the thermal energy required to significantly alter the electron distribution. Additionally, the Fermi velocity is calculated as 1.57413e+06 m/s meters per second, describing the speed of electrons at the Fermi surface.

<a id="intrinsic_carrier_concentration"></a>

#### Intrinsic carrier concentration of a semiconductor, `intrinsic_carrier_concentration`

The effective densities of states Nc and Nv and the intrinsic carrier concentration ni = sqrt(Nc Nv) exp(-Eg/2kT) from the band gap, temperature and effective masses.

$$
n_i = \sqrt{N_cN_v}\,e^{-E_g/2kT},\quad N_c = 2\left(\frac{2\pi m_e^*kT}{h^2}\right)^{3/2}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Eg` | in | band gap | J |
| `T` | in | temperature | K |
| `me` | in | electron density-of-states effective mass / m0 (default 1) (optional) | - |
| `mh` | in | hole density-of-states effective mass / m0 (default 1) (optional) | - |
| `Nc` | out | effective density of states, conduction band | 1/m^3 |
| `Nv` | out | effective density of states, valence band | 1/m^3 |
| `ni` | out | intrinsic carrier concentration | 1/m^3 |
| `log10_ni` | out | log10 of ni in m^-3 | - |
| `log10_Nc` | out | log10 of Nc in m^-3 | - |
| `Ei_offset_meV` | out | intrinsic Fermi level above midgap in meV | - |

**Applies when:** all inputs > 0; non-degenerate (Boltzmann) statistics, parabolic bands.

```shell
$ materials-science solve intrinsic_carrier_concentration "Eg=1.12 eV" "T=300 K" me=1.08 mh=0.81
Nc = 2.816486303e+25 1/m^3
Nv = 1.829361516e+25 1/m^3
ni = 8.880095679e+15 1/m^3
log10_ni = 15.94841765
log10_Nc = 25.44970764
Ei_offset_meV = -5.577867657
  by: Intrinsic carrier concentration of a semiconductor  (Nc = 2(2 pi me* k T/h^2)^(3/2), ni = sqrt(Nc Nv) exp(-Eg/(2kT)), Ei - midgap = (3/4) k T ln(mh*/me*))
  Eg = 1.79443783e-19 J
  T = 300 K
  me = 1.08
  mh = 0.81
  assumes: all inputs > 0; non-degenerate (Boltzmann) statistics, parabolic bands
```

**What the worked example means:** This calculation determines the effective densities of states for the conduction and valence bands, yielding Nc as 2.81649e+25 1/m^3 and Nv as 1.82936e+25 1/m^3 per cubic meter. These values are combined with the band gap to find the intrinsic carrier concentration ni, which equals 8.8801e+15 1/m^3 per cubic meter. The logarithmic base ten value of this concentration is 15.9484. Additionally, the position of the intrinsic Fermi level relative to the midgap is calculated as an offset of -5.57787 milli electron volts, reflecting the asymmetry between electron and hole effective masses.

### Fracture (7)

<a id="critical_crack_length"></a>

#### Critical crack length from the fracture toughness, `critical_crack_length`

The largest crack a part can carry at stress sigma without fracture: a_c = (K_IC / (Y sigma))^2 / pi.

$$
a_c = \frac{1}{\pi}\left(\frac{K_{Ic}}{Y\sigma}\right)^2
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `K_IC` | in | plane-strain fracture toughness | Pa*m^0.5 |
| `sigma` | in | applied stress | Pa |
| `Y` | in | geometry factor Y (default 1) (optional) | - |
| `a_c` | out | critical crack length | m |
| `a_c_mm` | out | critical crack length in mm | - |

**Applies when:** all inputs > 0; linear-elastic fracture mechanics.

```shell
$ materials-science solve critical_crack_length "K_IC=55 MPa*m^0.5" "sigma=1000 MPa"
a_c = 0.0009628874057 m
a_c_mm = 0.9628874057
  by: Critical crack length from the fracture toughness  (K_I = Y sigma sqrt(pi a) set equal to K_IC)
  K_IC = 55000000 Pa*m^0.5
  sigma = 1000000000 Pa
  assumes: all inputs > 0; linear-elastic fracture mechanics
```

**What the worked example means:** This calculation determines the maximum allowable crack size before fracture occurs. Setting the stress intensity factor equal to the material's plane-strain fracture toughness shows that a crack of length 0.000962887 m represents the threshold for failure. If any existing defect grows larger than 0.962887, the component will catastrophically break under the applied load of 1000 MPa. The result depends directly on the toughness 55 MPa*m^0.5 and inversely on the square of both the stress and the geometry factor.

<a id="weibull_design_stress"></a>

#### Design stress for a target failure probability (Weibull), `weibull_design_stress`

The stress at which a brittle part reaches a given failure probability, from the Weibull modulus and characteristic strength (and V/V0).

$$
\sigma = \sigma_0\left[-\frac{V_0}{V}\ln(1-P_f)\right]^{1/m}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `P_failure` | in | allowed probability of failure (0 to 1) | - |
| `sigma0` | in | characteristic strength | Pa |
| `m` | in | Weibull modulus | - |
| `V_ratio` | in | V/V0 (default 1) (optional) | - |
| `sigma` | out | design stress | Pa |
| `sigma_MPa` | out | design stress in MPa | - |

**Applies when:** 0 < P_failure < 1; sigma0, m, V/V0 > 0.

```shell
$ materials-science solve weibull_design_stress P_failure=0.001 "sigma0=400 MPa" m=10
sigma = 200484921.6 Pa
sigma_MPa = 200.4849216
  by: Design stress for a target failure probability (Weibull)  (sigma = sigma_0 (-ln(1 - P_f) V0/V)^(1/m))
  P_failure = 0.001
  sigma0 = 400000000 Pa
  m = 10
  assumes: 0 < P_failure < 1; sigma0, m, V/V0 > 0
```

**What the worked example means:** The calculated design stress is 2.00485e+08 Pa Pa, ensuring the brittle part meets the target failure probability of 0.001.

<a id="paris_law_fatigue_life"></a>

#### Fatigue crack growth life by Paris' law: `paris_law_fatigue_life`

The number of load cycles for a crack to grow from a0 to a final length af, or to the critical length a_c = (K_IC/(Y sigma_max))^2/pi at which it fractures, by integrating Paris' law da/dN = C (dK)^m with dK = Y dsigma sqrt(pi a).

$$
N_f = \int_{a_0}^{a_f}\frac{da}{C\,(Y\Delta\sigma\sqrt{\pi a})^m},\qquad a_c = \frac{1}{\pi}\left(\frac{K_{Ic}}{Y\sigma_{max}}\right)^2
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `C` | in | Paris coefficient C in (m/cycle)/(MPa sqrt(m))^m | - |
| `m` | in | Paris exponent m | - |
| `dsigma` | in | stress range of the cycle | Pa |
| `a0` | in | initial crack length (surface) or half-length (internal) | m |
| `Y` | in | geometry factor Y, constant (default 1) (optional) | - |
| `af` | in | final crack length (give af or K_IC) (optional) | m |
| `K_IC` | in | plane-strain fracture toughness, for the critical crack (give K_IC or af) (optional) | Pa*m^0.5 |
| `sigma_max` | in | peak stress of the cycle (with K_IC) (optional) | Pa |
| `N` | out | cycles to grow the crack from a0 to the final (or critical) length | - |
| `log10_N` | out | log10 of the life | - |
| `a_final` | out | final crack length used (af or a_c) | m |
| `a_final_mm` | out | final crack length in mm | - |
| `dK_initial` | out | stress intensity range at a0 in MPa sqrt(m) | - |
| `dK_final` | out | stress intensity range at the final crack in MPa sqrt(m) | - |
| `dadN_initial` | out | initial growth rate in m per cycle | - |
| `a_c` | out | critical crack length (with K_IC) | m |
| `a_c_mm` | out | critical crack length in mm (with K_IC) | - |

**Applies when:** C, m, dsigma, a0, Y > 0; 0.5 <= m <= 10; exactly one of af and K_IC (K_IC with sigma_max); final (or critical) crack longer than a0; Y constant over the growth; Paris (stable growth) regime.

```shell
$ materials-science solve paris_law_fatigue_life C=1e-12 m=3 "dsigma=200 MPa" "a0=1 mm" "af=10 mm"
N = 970793.0556
log10_N = 5.987126661
a_final = 0.01 m
a_final_mm = 10
dK_initial = 11.20998243
dK_final = 35.44907702
dadN_initial = 1.408687938e-09
  by: Fatigue crack growth life by Paris' law  (N = integral from a0 to af of da / (C (Y dsigma sqrt(pi a))^m) = (af^(1-m/2) - a0^(1-m/2)) / (C (Y dsigma sqrt(pi))^m (1 - m/2)); m = 2: ln(af/a0)/(C (Y dsigma)^2 pi))
  C = 1e-12
  m = 3
  dsigma = 200000000 Pa
  a0 = 0.001 m
  af = 0.01 m
  assumes: C, m, dsigma, a0, Y > 0; 0.5 <= m <= 10; exactly one of af and K_IC (K_IC with sigma_max); final (or critical) crack longer than a0; Y constant over the growth; Paris (stable growth) regime
```

In the worked example, C = 1e-12, m = 3, dsigma = 200 MPa, a0 = 1 mm, af = 10 mm go through the equation above and give N = 970793.0556, log10_N = 5.987126661, a_final = 0.01 m, a_final_mm = 10. It applies only when C, m, dsigma, a0, Y > 0; 0.5 <= m <= 10; exactly one of af and K_IC (K_IC with sigma_max); final (or critical) crack longer than a0; Y constant over the growth; Paris (stable growth) regime: outside that it says what is wrong instead of returning a number.

<a id="fracture_critical_stress"></a>

#### Fracture stress from the fracture toughness, `fracture_critical_stress`

The stress at which a crack of length a fails in a material of plane-strain fracture toughness K_IC: sigma_c = K_IC / (Y sqrt(pi a)).

$$
\sigma_c = \frac{K_{Ic}}{Y\sqrt{\pi a}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `K_IC` | in | plane-strain fracture toughness | Pa*m^0.5 |
| `a` | in | crack length (surface) or half-length (internal) | m |
| `Y` | in | geometry factor Y (default 1) (optional) | - |
| `sigma_c` | out | critical stress | Pa |
| `sigma_c_MPa` | out | critical stress in MPa | - |

**Applies when:** all inputs > 0; linear-elastic fracture mechanics.

```shell
$ materials-science solve fracture_critical_stress "K_IC=26 MPa*m^0.5" "a=2 mm" Y=1.2
sigma_c = 273339356.6 Pa
sigma_c_MPa = 273.3393566
  by: Fracture stress from the fracture toughness  (K_I = Y sigma sqrt(pi a) set equal to K_IC)
  K_IC = 26000000 Pa*m^0.5
  a = 0.002 m
  Y = 1.2
  assumes: all inputs > 0; linear-elastic fracture mechanics
```

**What the worked example means:** This calculation determines the maximum stress a material can withstand before a crack propagates catastrophically. Using the plane-strain fracture toughness of 26 MPa*m^0.5 and a geometry factor of 1.2, the model computes the critical stress as 2.73339e+08 Pa. This value represents the threshold where the energy release rate at the crack tip exceeds the material's resistance, leading to failure under linear elastic conditions.

<a id="griffith_critical_stress"></a>

#### Griffith critical stress of a brittle solid, `griffith_critical_stress`

The stress at which a crack of length 2a (internal) or a (surface) propagates in a brittle solid: sigma_c = sqrt(2 E gamma_s / (pi a)).

$$
\sigma_c = \sqrt{\frac{2E\gamma_s}{\pi a}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `E` | in | Young's modulus | Pa |
| `gamma_s` | in | specific surface energy | J/m^2 |
| `a` | in | half-length of an internal crack (or length of a surface crack) | m |
| `sigma_c` | out | critical stress | Pa |
| `sigma_c_MPa` | out | critical stress in MPa | - |

**Applies when:** all inputs > 0; ideally brittle (no plastic work), plane stress.

```shell
$ materials-science solve griffith_critical_stress "E=69 GPa" "gamma_s=0.3 J/m^2" "a=0.5 um"
sigma_c = 162345491.4 Pa
sigma_c_MPa = 162.3454914
  by: Griffith critical stress of a brittle solid  (Griffith energy balance)
  E = 6.9e+10 Pa
  gamma_s = 0.3 J/m^2
  a = 5e-07 m
  assumes: all inputs > 0; ideally brittle (no plastic work), plane stress
```

**What the worked example means:** This calculation determines the Griffith critical stress, the threshold at which a crack propagates spontaneously in a brittle material. It balances the release of elastic strain energy against the energy required to create new surfaces. Using a Young's modulus of 69 GPa, a specific surface energy of 0.3 J/m^2, and a characteristic crack dimension of 0.5 um, the resulting critical stress is 1.62345e+08 Pa. This value represents the maximum tensile stress the material can withstand before catastrophic failure occurs under plane stress conditions.

<a id="weibull_modulus_fit"></a>

#### Weibull modulus from a set of measured strengths, `weibull_modulus_fit`

The Weibull modulus m and characteristic strength sigma_0 from measured fracture strengths, by median-rank regression (Bernard's ranks).

$$
\ln\ln\frac{1}{1-F_i} = m\ln\sigma_i - m\ln\sigma_0
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `strengths` | in | fracture strengths (any one unit, e.g. MPa) | - |
| `m` | out | Weibull modulus | - |
| `sigma0` | out | characteristic strength (unit of the data) | - |
| `r_squared` | out | r^2 of the fit | - |

**Applies when:** at least three strengths, not all equal, all > 0.

```shell
$ materials-science solve weibull_modulus_fit "strengths=[310, 342, 365, 380, 395, 410, 432]"
m = 9.557482159
sigma0 = 394.7087405
r_squared = 0.9995661203
  by: Weibull modulus from a set of measured strengths  (sort; F_i = (i - 0.3)/(n + 0.4); least-squares line of ln(-ln(1 - F_i)) against ln(sigma_i): slope m, intercept -m ln(sigma_0))
  assumes: at least three strengths, not all equal, all > 0
```

**What the worked example means:** This calculation determines the statistical parameters describing fracture strength variability. It uses measured strengths to compute the Weibull modulus 9.55748, which indicates how consistent the material's strength is, and the characteristic strength 394.709, representing the stress level where failure probability reaches approximately sixty-three percent. The result includes the coefficient of determination 0.999566 to show how well the data aligns with the theoretical model.

<a id="weibull_failure_probability"></a>

#### Weibull probability of failure of a brittle part, `weibull_failure_probability`

The probability that a brittle (ceramic) part fails at stress sigma, from the Weibull modulus m and characteristic strength sigma_0, with an optional volume ratio V/V0.

$$
P_f = 1 - \exp\!\left[-\frac{V}{V_0}\left(\frac{\sigma}{\sigma_0}\right)^m\right]
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `sigma` | in | applied stress | Pa |
| `sigma0` | in | characteristic strength (63.2 % failure at V0) | Pa |
| `m` | in | Weibull modulus | - |
| `V_ratio` | in | stressed volume over the reference volume V/V0 (default 1) (optional) | - |
| `P_failure` | out | probability of failure | - |
| `P_survival` | out | probability of survival | - |

**Applies when:** all inputs > 0.

```shell
$ materials-science solve weibull_failure_probability "sigma=300 MPa" "sigma0=400 MPa" m=10
P_failure = 0.05475725807
P_survival = 0.9452427419
  by: Weibull probability of failure of a brittle part  (P_f = 1 - exp(-(V/V0)(sigma/sigma0)^m))
  sigma = 300000000 Pa
  sigma0 = 400000000 Pa
  m = 10
  assumes: all inputs > 0
```

**What the worked example means:** The probability of failure is 0.0547573, meaning there is a 0.945243 chance the part survives.

### Mechanical properties (14)

<a id="offset_yield_strength"></a>

#### 0.2 % offset yield strength from a stress-strain curve, `offset_yield_strength`

The yield strength where the stress-strain curve (piecewise linear through the data) meets a line of slope E offset by 0.2 % strain (or another offset).

$$
\sigma(\varepsilon_y) = E(\varepsilon_y - 0.002)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `strain` | in | engineering strains, increasing | - |
| `stress` | in | engineering stresses in MPa | - |
| `E` | in | Young's modulus | Pa |
| `offset` | in | strain offset (default 0.002) (optional) | - |
| `yield_MPa` | out | offset yield strength in MPa | - |
| `yield_strain` | out | strain at the intersection | - |

**Applies when:** strains strictly increasing; the curve starts above the offset line and crosses it.

```shell
$ materials-science solve offset_yield_strength "strain=[0, 0.002, 0.004, 0.006, 0.008]" "stress=[0, 400, 520, 560, 580]" "E=200 GPa"
yield_MPa = 533.3333333
yield_strain = 0.004666666667
  by: 0.2 % offset yield strength from a stress-strain curve  (first crossing of the data polyline with sigma = E (epsilon - offset), solved on that segment)
  E = 2e+11 Pa
  assumes: strains strictly increasing; the curve starts above the offset line and crosses it
```

**What the worked example means:** This calculation determines the stress level where permanent deformation begins, defined as the intersection of the material's piecewise linear stress-strain curve with a line parallel to the initial elastic slope but shifted by a specified strain offset. The resulting yield strength of 533.333 indicates the maximum stress the component can withstand before exceeding this conventional limit of plasticity, occurring at a strain of 0.00466667. This metric is essential for design safety when materials lack a distinct yield point.

<a id="elastic_deformation_of_a_rod"></a>

#### Elastic elongation and diameter change of a rod in tension, `elastic_deformation_of_a_rod`

Stress, axial strain, elongation, lateral strain and diameter change of a cylindrical rod under an axial force (Hooke's law and Poisson's ratio).

$$
\varepsilon_z = \frac{F}{A_0E},\quad \varepsilon_x = -\nu\varepsilon_z
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `F` | in | axial force (tension positive) | N |
| `d0` | in | original diameter | m |
| `l0` | in | original length | m |
| `E` | in | Young's modulus | Pa |
| `nu` | in | Poisson's ratio | - |
| `stress_MPa` | out | axial stress in MPa | - |
| `axial_strain` | out | axial strain | - |
| `elongation_mm` | out | elongation in mm | - |
| `lateral_strain` | out | lateral strain | - |
| `diameter_change_um` | out | change of diameter in micrometres (negative = contraction) | - |

**Applies when:** F, d0, l0, E > 0; -1 < nu < 0.5; elastic, isotropic.

```shell
$ materials-science solve elastic_deformation_of_a_rod "F=15 kN" "d0=10 mm" "l0=300 mm" "E=110 GPa" nu=0.34
stress_MPa = 190.9859317
axial_strain = 0.001736235743
elongation_mm = 0.5208707228
lateral_strain = -0.0005903201526
diameter_change_um = -5.903201526
  by: Elastic elongation and diameter change of a rod in tension  (sigma = F/(pi d0^2/4), epsilon_z = sigma/E, epsilon_x = -nu epsilon_z, dl = epsilon_z l0, dd = epsilon_x d0)
  F = 15000 N
  d0 = 0.01 m
  l0 = 0.3 m
  E = 1.1e+11 Pa
  nu = 0.34
  assumes: F, d0, l0, E > 0; -1 < nu < 0.5; elastic, isotropic
```

**What the worked example means:** This calculation determines the elastic deformation of a cylindrical rod under tension. Using the applied force 15 kN and original dimensions, it computes the axial stress as 190.986. Hooke's law yields an axial strain of 0.00173624, causing an elongation of 0.520871. Poisson's ratio links this to lateral contraction, giving a lateral strain of -0.00059032 and a diameter change of -5.9032. These values help assess dimensional stability within safe limits.

<a id="hall_petch"></a>

#### Hall-Petch grain-size strengthening: `hall_petch`

sigma_0 and k of sigma_y = sigma_0 + k d^(-1/2) from yield strengths at two or more grain sizes, and the yield strength at another grain size.

$$
\sigma_y = \sigma_0 + k_y d^{-1/2}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `d_um` | in | grain diameters in micrometres | - |
| `yield_MPa` | in | yield strengths in MPa | - |
| `d_target_um` | in | grain diameter in micrometres at which the yield strength is wanted (optional) (optional) | - |
| `sigma0_MPa` | out | friction stress sigma_0 in MPa | - |
| `k_MPa_sqrt_um` | out | k in MPa um^0.5 | - |
| `k_MPa_sqrt_m` | out | k in MPa m^0.5 | - |
| `r_squared` | out | r^2 of the fit | - |
| `yield_target_MPa` | out | yield strength at the target grain size in MPa | - |

**Applies when:** equal lengths, at least two distinct grain sizes.

```shell
$ materials-science solve hall_petch "d_um=[100, 4]" "yield_MPa=[150, 350]" d_target_um=25
sigma0_MPa = 100
k_MPa_sqrt_um = 500
k_MPa_sqrt_m = 0.5
r_squared = 1
yield_target_MPa = 200
  by: Hall-Petch grain-size strengthening  (least-squares line of sigma_y against d^(-1/2))
  d_target_um = 25
  assumes: equal lengths, at least two distinct grain sizes
```

**What the worked example means:** A least-squares linear regression of yield strength against the inverse square root of grain diameter determines the Hall-Petch parameters. The intercept yields the friction stress 100, representing the lattice resistance to dislocation motion, while the slope provides the strengthening coefficient 500, which quantifies how effectively grain boundaries impede slip. If a target grain size was specified, the predicted yield strength at that size is 200. The goodness of fit is indicated by an r-squared value of 1, confirming the linearity of the relationship within the measured range.

<a id="larson_miller_parameter"></a>

#### Larson-Miller parameter (creep rupture): `larson_miller_parameter`

The Larson-Miller parameter LMP = T (C + log10 t_r) from a rupture time at a temperature, and the rupture time at another temperature for the same parameter (same stress).

$$
\mathrm{LMP} = T\left(C + \log_{10} t_r\right)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `T` | in | absolute temperature | K |
| `t_r` | in | rupture time (hours, h ...) | s |
| `C` | in | Larson-Miller constant (default 20, with t_r in hours) (optional) | - |
| `T2` | in | another temperature at the same stress (optional) (optional) | K |
| `LMP` | out | Larson-Miller parameter (K, t in hours) | - |
| `LMP_thousands` | out | LMP / 1000 | - |
| `log10_t_hours` | out | log10 of the rupture time in hours | - |
| `t_r2_hours` | out | rupture time at T2 in hours | - |
| `log10_t2_hours` | out | log10 of the rupture time at T2 | - |

**Applies when:** T, t_r > 0; 5 <= C <= 50; time in hours inside the logarithm.

```shell
$ materials-science solve larson_miller_parameter "T=1073 K" "t_r=1000 h"
LMP = 24679
LMP_thousands = 24.679
log10_t_hours = 3
  by: Larson-Miller parameter (creep rupture)  (LMP = T (C + log10 t_r[h]); at T2: log10 t_2 = LMP/T2 - C)
  T = 1073 K
  t_r = 3600000 s
  assumes: T, t_r > 0; 5 <= C <= 50; time in hours inside the logarithm
```

**What the worked example means:** This calculation determines the Larson-Miller parameter, which combines absolute temperature and rupture time into a single value for creep life prediction. Using an input temperature of 1073 K and a rupture time of 1000 h, the resulting parameter is 24679. This value represents the thermal activation energy required for failure under constant stress, allowing engineers to extrapolate short-term test data to long-term service conditions for components like turbine blades.

<a id="modulus_of_resilience"></a>

#### Modulus of resilience: `modulus_of_resilience`

The elastic strain energy per unit volume stored up to yielding: U_r = sigma_y^2/(2E).

$$
U_r = \frac{\sigma_y^2}{2E}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `sy` | in | yield strength | Pa |
| `E` | in | Young's modulus | Pa |
| `Ur` | out | modulus of resilience | J/m^3 |
| `Ur_MJ_m3` | out | modulus of resilience in MJ/m^3 (= MPa) | - |
| `yield_strain` | out | strain at yield | - |

**Applies when:** sy, E > 0; linear elastic up to yield.

```shell
$ materials-science solve modulus_of_resilience "sy=415 MPa" "E=207 GPa"
Ur = 416002.4155 J/m^3
Ur_MJ_m3 = 0.4160024155
yield_strain = 0.002004830918
  by: Modulus of resilience  (area under the linear elastic part of the curve)
  sy = 415000000 Pa
  E = 2.07e+11 Pa
  assumes: sy, E > 0; linear elastic up to yield
```

**What the worked example means:** This calculation determines the elastic strain energy per unit volume stored in the material up to the point of yielding, using the formula derived from the area under the linear elastic portion of the stress-strain curve. The result is expressed as 416002 J/m^3 in joules per cubic meter, or equivalently 0.416002 megajoules per cubic meter. This value represents the maximum energy the material can absorb without undergoing permanent deformation, based on an input yield strength of 415 MPa and a Young's modulus of 207 GPa.

<a id="max_schmid_factor"></a>

#### Most favourable slip system of an FCC or BCC crystal, `max_schmid_factor`

The largest Schmid factor over the 12 {111}<110> (FCC) or {110}<111> (BCC) slip systems for a tensile load direction, how many systems share it, and the yield stress from the CRSS.

$$
m = \max_{\text{systems}}|\cos\phi\cos\lambda|
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `structure` | in | crystal structure (FCC, BCC) | - |
| `load` | in | loading direction [u v w] | - |
| `tau_crss` | in | critical resolved shear stress (optional) (optional) | Pa |
| `max_schmid_factor` | out | largest Schmid factor | - |
| `active_systems` | out | number of systems with that factor | - |
| `systems` | out | slip systems considered | - |
| `yield_MPa` | out | tensile yield stress in MPa | - |

**Applies when:** nonzero loading direction.

```shell
$ materials-science solve max_schmid_factor structure=FCC "load=[0, 0, 1]"
max_schmid_factor = 0.4082482905
active_systems = 8
systems = 12
  by: Most favourable slip system of an FCC or BCC crystal  (Schmid factor of each of the 12 systems; the maximum (ties within 1e-9 counted))
  structure = FCC
  assumes: nonzero loading direction
```

In the worked example, structure = FCC, load = [0, 0, 1] go through the equation above and give max_schmid_factor = 0.4082482905, active_systems = 8, systems = 12. It applies only when nonzero loading direction: outside that it says what is wrong instead of returning a number.

<a id="schmid_factor"></a>

#### Resolved shear stress and Schmid factor of a slip system, `schmid_factor`

The Schmid factor cos(phi) cos(lambda) of a slip system (plane normal and slip direction) for a tensile load along a direction of a cubic crystal, the resolved shear stress and the stress to start slip.

$$
\tau_R = \sigma\cos\phi\cos\lambda,\quad \sigma_y = \frac{\tau_{\mathrm{crss}}}{\cos\phi\cos\lambda}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `load` | in | loading direction [u v w] | - |
| `plane` | in | slip-plane normal (h k l) | - |
| `direction` | in | slip direction [u v w] | - |
| `sigma` | in | applied tensile stress (optional) (optional) | Pa |
| `tau_crss` | in | critical resolved shear stress (optional) (optional) | Pa |
| `schmid_factor` | out | \|cos(phi) cos(lambda)\| | - |
| `phi_deg` | out | angle between load and plane normal | - |
| `lambda_deg` | out | angle between load and slip direction | - |
| `tau_R_MPa` | out | resolved shear stress in MPa | - |
| `yield_MPa` | out | tensile stress at which slip starts, in MPa | - |

**Applies when:** cubic crystal (indices as Cartesian directions); slip direction in the slip plane.

```shell
$ materials-science solve schmid_factor "load=[0, 1, 0]" "plane=[1, 1, 0]" "direction=[-1, 1, 1]" "sigma=52 MPa"
schmid_factor = 0.4082482905
phi_deg = 45
lambda_deg = 54.73561032
tau_R_MPa = 21.2289111
  by: Resolved shear stress and Schmid factor of a slip system  (tau_R = sigma cos(phi) cos(lambda); sigma_y = tau_CRSS / (cos(phi) cos(lambda)))
  sigma = 52000000 Pa
  assumes: cubic crystal (indices as Cartesian directions); slip direction in the slip plane
```

In the worked example, load = [0, 1, 0], plane = [1, 1, 0], direction = [-1, 1, 1], sigma = 52 MPa go through the equation above and give schmid_factor = 0.4082482905, phi_deg = 45, lambda_deg = 54.73561032, tau_R_MPa = 21.2289111. It applies only when cubic crystal (indices as Cartesian directions); slip direction in the slip plane: outside that it says what is wrong instead of returning a number.

<a id="isotropic_elastic_constants"></a>

#### Shear modulus, bulk modulus and Lame constant from E and Poisson's ratio, `isotropic_elastic_constants`

The shear modulus G, bulk modulus K and Lame constant lambda of an isotropic material from Young's modulus E and Poisson's ratio nu.

$$
G = \frac{E}{2(1+\nu)},\quad K = \frac{E}{3(1-2\nu)}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `E` | in | Young's modulus | Pa |
| `nu` | in | Poisson's ratio | - |
| `G` | out | shear modulus | Pa |
| `G_GPa` | out | shear modulus in GPa | - |
| `K` | out | bulk modulus | Pa |
| `K_GPa` | out | bulk modulus in GPa | - |
| `lame_GPa` | out | Lame's first constant in GPa | - |

**Applies when:** E > 0; -1 < nu < 0.5.

```shell
$ materials-science solve isotropic_elastic_constants "E=200 GPa" nu=0.3
G = 7.692307692e+10 Pa
G_GPa = 76.92307692
K = 1.666666667e+11 Pa
K_GPa = 166.6666667
lame_GPa = 115.3846154
  by: Shear modulus, bulk modulus and Lame constant from E and Poisson's ratio  (G = E/(2(1 + nu)), K = E/(3(1 - 2 nu)), lambda = E nu/((1 + nu)(1 - 2 nu)))
  E = 2e+11 Pa
  nu = 0.3
  assumes: E > 0; -1 < nu < 0.5
```

**What the worked example means:** This calculation derives three key elastic moduli from the fundamental tensile properties of an isotropic material. Given a Young's modulus of 200 GPa and a Poisson's ratio of 0.3, it follows that the shear modulus, which measures resistance to shape change without volume change, is 76.9231 gigapascals. The bulk modulus, representing resistance to uniform compression or volume change, is calculated as 166.667 gigapascals. Additionally, Lamé's first constant, often used in stress-strain tensor formulations, is found to be 115.385 gigapascals. These values allow engineers to predict how the material will respond to complex loading scenarios beyond simple tension.

<a id="strain_hardening_exponent"></a>

#### Strain-hardening exponent and strength coefficient (Hollomon), `strain_hardening_exponent`

The strain-hardening exponent n and strength coefficient K of sigma_T = K epsilon_T^n from true stress-strain points of the plastic region, and the stress at another strain.

$$
\ln\sigma_T = \ln K + n\ln\varepsilon_T
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `true_strain` | in | true plastic strains | - |
| `true_stress` | in | true stresses in MPa | - |
| `strain_target` | in | true strain at which the stress is wanted (optional) (optional) | - |
| `n` | out | strain-hardening exponent | - |
| `K_MPa` | out | strength coefficient in MPa | - |
| `r_squared` | out | r^2 of the log-log fit | - |
| `stress_target_MPa` | out | true stress at the target strain in MPa | - |

**Applies when:** equal lengths, at least two distinct strains, all values > 0.

```shell
$ materials-science solve strain_hardening_exponent "true_strain=[0.1, 0.2]" "true_stress=[415, 470]"
n = 0.1795494203
K_MPa = 627.4765746
r_squared = 1
  by: Strain-hardening exponent and strength coefficient (Hollomon)  (least-squares line of ln sigma_T against ln epsilon_T)
  assumes: equal lengths, at least two distinct strains, all values > 0
```

**What the worked example means:** The fit yields a strain-hardening exponent of 0.179549 and a strength coefficient of 627.477, with a goodness of fit of 1.

<a id="creep_parameters_fit"></a>

#### Stress exponent and activation energy of steady-state creep, `creep_parameters_fit`

n and Q_c of the power-law creep model rate = A sigma^n exp(-Q_c/RT) from steady-state creep rates at several stresses and temperatures (least squares).

$$
\dot\varepsilon_s = A\sigma^n e^{-Q_c/RT}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `stress_MPa` | in | stresses in MPa | - |
| `T` | in | temperatures in kelvin | - |
| `rate` | in | steady-state creep rates in 1/s | - |
| `n` | out | stress exponent | - |
| `Q_kJmol` | out | creep activation energy in kJ/mol | - |

**Applies when:** equal lengths (at least 3); stress and temperature both vary independently; all values > 0.

```shell
$ materials-science solve creep_parameters_fit "stress_MPa=[100, 200, 100]" "T=[900, 900, 1000]" "rate=[1e-07, 1.6e-06, 2.5e-06]"
n = 4
Q_kJmol = 240.8690045
  by: Stress exponent and activation energy of steady-state creep  (least squares of ln(rate) = ln A + n ln(sigma) - Q_c/(R T))
  assumes: equal lengths (at least 3); stress and temperature both vary independently; all values > 0
```

**What the worked example means:** This calculation determines the stress exponent and creep activation energy by applying least squares regression to the logarithmic form of the power-law creep equation. It uses measured steady-state creep rates at various stresses and temperatures to find that the stress exponent equals 4 and the activation energy equals 240.869. These values characterize how sensitive the material's deformation rate is to changes in applied load and temperature under secondary creep conditions.

<a id="thermal_stress"></a>

#### Thermal stress in a constrained body: `thermal_stress`

The stress from a temperature change in a body held in one direction (rod between walls), two (thin film or plate) or three (fully constrained).

$$
\sigma = -E\alpha\Delta T,\quad -\frac{E\alpha\Delta T}{1-\nu},\quad -\frac{E\alpha\Delta T}{1-2\nu}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `E` | in | Young's modulus | Pa |
| `alpha` | in | linear thermal expansion coefficient | 1/K |
| `dT` | in | temperature change (positive = heating) | K |
| `constraint` | in | constraint (uniaxial, biaxial, triaxial; default uniaxial) (optional) | - |
| `nu` | in | Poisson's ratio (needed for biaxial and triaxial) (optional) | - |
| `stress` | out | thermal stress (negative = compression) | Pa |
| `stress_MPa` | out | thermal stress in MPa | - |

**Applies when:** E, alpha > 0; -1 < nu < 0.5 when needed.

```shell
$ materials-science solve thermal_stress "E=97 GPa" "alpha=2e-05 1/K" "dT=60 K"
stress = -116400000 Pa
stress_MPa = -116.4
  by: Thermal stress in a constrained body  (sigma = -E alpha dT (uniaxial), / (1 - nu) (biaxial), / (1 - 2 nu) (triaxial))
  E = 9.7e+10 Pa
  alpha = 2e-05 1/K
  dT = 60 K
  assumes: E, alpha > 0; -1 < nu < 0.5 when needed
```

**What the worked example means:** This calculation determines the internal stress generated in a constrained body due to a temperature change. Using the Young's modulus of 97 GPa, the linear thermal expansion coefficient of 2e-05 1/K, and a temperature change of 60 K, the resulting stress is computed as -1.164e+08 Pa. This value represents the mechanical load induced when the material attempts to expand or contract but is prevented from doing so by rigid constraints, with negative values indicating compression during heating.

<a id="tensile_toughness"></a>

#### Toughness (area under the stress-strain curve), `tensile_toughness`

The energy absorbed per unit volume up to fracture (trapezoidal area under the stress-strain data), the tensile strength and the fracture strain.

$$
U_T = \int_0^{\varepsilon_f}\sigma\,d\varepsilon
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `strain` | in | engineering strains, non-decreasing | - |
| `stress` | in | engineering stresses in MPa | - |
| `toughness_MJ_m3` | out | toughness in MJ/m^3 | - |
| `toughness` | out | toughness | J/m^3 |
| `uts_MPa` | out | tensile strength (largest stress) in MPa | - |
| `fracture_strain` | out | strain at the last point | - |

**Applies when:** equal lengths; strains non-decreasing; stress in MPa.

```shell
$ materials-science solve tensile_toughness "strain=[0, 0.002, 0.05, 0.1, 0.15]" "stress=[0, 400, 500, 520, 480]"
toughness_MJ_m3 = 72.5
toughness = 72500000 J/m^3
uts_MPa = 520
fracture_strain = 0.15
  by: Toughness (area under the stress-strain curve)  (trapezoidal rule over the data points)
  assumes: equal lengths; strains non-decreasing; stress in MPa
```

**What the worked example means:** Toughness represents the energy absorbed per unit volume by a material before it fractures. This value, 72.5, was determined by integrating the engineering stress against engineering strain using the trapezoidal rule over the provided data points. The calculation also identified the ultimate tensile strength as 520 and the fracture strain as 0.15, indicating the material's capacity to withstand impact without breaking.

<a id="true_stress_and_strain"></a>

#### True stress and true strain from engineering values, `true_stress_and_strain`

True stress and true strain from the engineering stress and strain of a tensile test before necking (volume conserved).

$$
\sigma_T = \sigma(1+\varepsilon),\quad \varepsilon_T = \ln(1+\varepsilon)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `stress` | in | engineering stress | Pa |
| `strain` | in | engineering strain (dimensionless) | - |
| `true_stress` | out | true stress | Pa |
| `true_stress_MPa` | out | true stress in MPa | - |
| `true_strain` | out | true strain | - |

**Applies when:** strain > -1; uniform deformation (before necking).

```shell
$ materials-science solve true_stress_and_strain "stress=400 MPa" strain=0.2
true_stress = 480000000 Pa
true_stress_MPa = 480
true_strain = 0.1823215568
  by: True stress and true strain from engineering values  (sigma_T = sigma (1 + epsilon), epsilon_T = ln(1 + epsilon))
  stress = 400000000 Pa
  strain = 0.2
  assumes: strain > -1; uniform deformation (before necking)
```

**What the worked example means:** This calculation converts the measured engineering stress of 400 MPa and engineering strain of 0.2 into their true counterparts. The resulting true stress is 4.8e+08 Pa and the true strain is 0.182322. This transformation accounts for the instantaneous change in cross-sectional area during uniform deformation, providing a more accurate representation of the material's state for plastic analysis before necking occurs.

<a id="youngs_modulus_from_data"></a>

#### Young's modulus from elastic stress-strain data, `youngs_modulus_from_data`

Young's modulus as the slope of the least-squares line through stress-strain points of the elastic region.

$$
E = \frac{\sum(\varepsilon_i-\bar\varepsilon)(\sigma_i-\bar\sigma)}{\sum(\varepsilon_i-\bar\varepsilon)^2}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `strain` | in | engineering strains (dimensionless) | - |
| `stress` | in | engineering stresses in MPa | - |
| `E_GPa` | out | Young's modulus in GPa | - |
| `E` | out | Young's modulus | Pa |
| `intercept_MPa` | out | intercept of the line in MPa | - |
| `r_squared` | out | r^2 of the fit | - |

**Applies when:** equal lengths, at least two distinct strains; stress in MPa.

```shell
$ materials-science solve youngs_modulus_from_data "strain=[0, 0.001, 0.002]" "stress=[0, 200, 400]"
E_GPa = 200
E = 2e+11 Pa
intercept_MPa = 0
r_squared = 1
  by: Young's modulus from elastic stress-strain data  (least-squares line stress = b0 + E strain)
  assumes: equal lengths, at least two distinct strains; stress in MPa
```

**What the worked example means:** This calculation determines the stiffness of the material by fitting a straight line to the engineering stress and strain measurements in the linear elastic region. The resulting Young's modulus is 200, which represents the ratio of stress to strain for this specific sample. The quality of this linear approximation is indicated by an r-squared value of 1, while the intercept of the fitted line is 0.

### Phase diagrams (6)

<a id="atomic_to_weight_percent"></a>

#### Atomic percent to weight percent (binary alloy), `atomic_to_weight_percent`

Convert the composition of a binary alloy from atomic percent to weight percent.

$$
C_1 = \frac{C_1'A_1}{C_1'A_1 + C_2'A_2}\times100
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `element1` | in | symbol of element 1 | - |
| `element2` | in | symbol of element 2 | - |
| `at1` | in | atomic percent of element 1 | - |
| `wt1` | out | weight percent of element 1 | - |
| `wt2` | out | weight percent of element 2 | - |

**Applies when:** known elements, different; 0 <= at1 <= 100.

```shell
$ materials-science solve atomic_to_weight_percent element1=Al element2=Cu at1=90
wt1 = 79.25907969
wt2 = 20.74092031
  by: Atomic percent to weight percent (binary alloy)  (C1 = C1' A1 / (C1' A1 + C2' A2) x 100)
  at1 = 90
  element1 = Al
  element2 = Cu
  assumes: known elements, different; 0 <= at1 <= 100
```

**What the worked example means:** This calculation converts the composition of a binary alloy from atomic percent to weight percent. Given that element one constitutes 90 atomic percent, the resulting mass fraction for element one is 79.2591 and for element two is 20.7409. This distinction is vital because properties like density depend on mass, whereas crystal structures are defined by atom counts.

<a id="gibbs_phase_rule"></a>

#### Degrees of freedom by Gibbs' phase rule: `gibbs_phase_rule`

The number of degrees of freedom F = C - P + 2 (or + 1 at fixed pressure) of a system of C components and P phases in equilibrium.

$$
F = C - P + 2
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `C` | in | number of components | - |
| `P` | in | number of phases | - |
| `pressure` | in | pressure variable or held constant (default variable) (optional) | - |
| `F` | out | degrees of freedom | - |

**Applies when:** F >= 0.

```shell
$ materials-science solve gibbs_phase_rule C=2 P=3 pressure=constant
F = 0
  by: Degrees of freedom by Gibbs' phase rule  (F = C - P + 2 (variable pressure) or C - P + 1 (constant pressure, condensed systems))
  pressure = constant
  assumes: F >= 0
```

**What the worked example means:** Gibbs' phase rule determines the number of independent variables that can be changed without altering the number of phases in equilibrium. Using the formula F equals C minus P plus two for variable pressure systems, the calculation yields 0 degrees of freedom based on 2 components and 3 phases. This result indicates how many intensive properties, such as temperature or composition, can be varied while maintaining the current phase balance.

<a id="lever_rule"></a>

#### Phase fractions by the lever rule: `lever_rule`

The mass fractions (and masses) of the two phases of an alloy of overall composition C0 in a two-phase region whose tie line ends at C1 and C2.

$$
W_1 = \frac{C_2-C_0}{C_2-C_1},\quad W_2 = \frac{C_0-C_1}{C_2-C_1}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `C0` | in | overall composition (wt%) | - |
| `C1` | in | composition of phase 1 (wt%) | - |
| `C2` | in | composition of phase 2 (wt%) | - |
| `mass` | in | total mass of the alloy (optional) (optional) | kg |
| `W1` | out | mass fraction of phase 1 | - |
| `W2` | out | mass fraction of phase 2 | - |
| `m1` | out | mass of phase 1 | kg |
| `m2` | out | mass of phase 2 | kg |

**Applies when:** C1 != C2; C0 between C1 and C2.

```shell
$ materials-science solve lever_rule C0=35 C1=31.5 C2=42.5
W1 = 0.6818181818
W2 = 0.3181818182
  by: Phase fractions by the lever rule  (W1 = (C2 - C0)/(C2 - C1), W2 = (C0 - C1)/(C2 - C1))
  C0 = 35
  C1 = 31.5
  C2 = 42.5
  assumes: C1 != C2; C0 between C1 and C2
```

**What the worked example means:** The mass fraction of phase one is 0.681818 and phase two is 0.318182, derived from the overall composition 35 within the tie line range defined by 31.5 and 42.5.

<a id="eutectic_microconstituents"></a>

#### Primary and eutectic microconstituent fractions, `eutectic_microconstituents`

The fractions of primary phase and eutectic microconstituent, and of total alpha and beta, in a binary eutectic alloy cooled just below the eutectic temperature.

$$
W_{\alpha'} = \frac{C_E-C_0}{C_E-C_\alpha},\quad W_e = \frac{C_0-C_\alpha}{C_E-C_\alpha}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `C0` | in | alloy composition (wt%) | - |
| `C_alpha` | in | composition of alpha at the eutectic temperature | - |
| `C_E` | in | eutectic composition | - |
| `C_beta` | in | composition of beta at the eutectic temperature | - |
| `W_primary` | out | fraction of primary (proeutectic) phase | - |
| `W_eutectic` | out | fraction of eutectic microconstituent | - |
| `W_alpha_total` | out | total fraction of alpha | - |
| `W_beta_total` | out | total fraction of beta | - |
| `primary_phase` | out | the primary phase (alpha for hypoeutectic, beta for hypereutectic) | - |

**Applies when:** C_alpha < C_E < C_beta; C_alpha < C0 < C_beta.

```shell
$ materials-science solve eutectic_microconstituents C0=25 C_alpha=18.3 C_E=61.9 C_beta=97.8
W_primary = 0.8463302752
W_eutectic = 0.1536697248
W_alpha_total = 0.9157232704
W_beta_total = 0.08427672956
primary_phase = alpha
  by: Primary and eutectic microconstituent fractions  (lever rule on the tie line just above the eutectic (primary phase vs liquid of C_E) and just below it (alpha vs beta))
  C0 = 25
  C_alpha = 18.3
  C_E = 61.9
  C_beta = 97.8
  assumes: C_alpha < C_E < C_beta; C_alpha < C0 < C_beta
```

**What the worked example means:** This calculation determines the proportion of primary phase versus eutectic structure in a binary alloy cooled just below the eutectic temperature. Using the lever rule on the tie line between the alpha composition and the eutectic composition shows that for an alloy with composition 25, the primary phase accounts for 0.84633 while the remaining 0.15367 forms the eutectic mixture. The total fractions of individual phases are also derived as 0.915723 for alpha and 0.0842767 for beta.

<a id="iron_carbon_microconstituents"></a>

#### Proeutectoid phase and pearlite in a plain-carbon steel, `iron_carbon_microconstituents`

The fractions of proeutectoid ferrite or cementite and of pearlite, and of total ferrite and cementite, in a steel cooled slowly just below the eutectoid temperature.

$$
W_p = \frac{C_0 - 0.022}{0.76 - 0.022}\ (\text{hypo}),\quad W_p = \frac{6.70 - C_0}{6.70 - 0.76}\ (\text{hyper})
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `C0` | in | carbon content in wt% (0.022 to 2.14) | - |
| `W_proeutectoid` | out | fraction of proeutectoid ferrite or cementite | - |
| `W_pearlite` | out | fraction of pearlite | - |
| `W_ferrite_total` | out | total fraction of ferrite | - |
| `W_cementite_total` | out | total fraction of cementite | - |
| `proeutectoid_phase` | out | ferrite (hypoeutectoid) or cementite (hypereutectoid) | - |

**Applies when:** 0.022 < C0 < 2.14 wt% C.

```shell
$ materials-science solve iron_carbon_microconstituents C0=0.35
W_proeutectoid = 0.5555555556
W_pearlite = 0.4444444444
W_ferrite_total = 0.9508834981
W_cementite_total = 0.04911650195
proeutectoid_phase = ferrite
  by: Proeutectoid phase and pearlite in a plain-carbon steel  (lever rule with the eutectoid at 0.76 wt% C, ferrite 0.022 wt% C and cementite 6.70 wt% C)
  C0 = 0.35
  assumes: 0.022 < C0 < 2.14 wt% C
```

**What the worked example means:** This calculation determines the microstructural constituents of a plain-carbon steel cooled slowly to just below the eutectoid temperature. Using the lever rule on the iron-carbon phase diagram, it computes the fraction of pearlite (0.444444) based on the input carbon content (0.35). It also identifies whether the proeutectoid phase is ferrite or cementite (ferrite) and calculates its fraction (0.555556), alongside the total amounts of ferrite (0.950883) and cementite (0.0491165). These values indicate the expected balance between soft ferrite and hard cementite phases in the final equilibrium structure.

<a id="weight_to_atomic_percent"></a>

#### Weight percent to atomic percent (binary alloy), `weight_to_atomic_percent`

Convert the composition of a binary alloy from weight percent to atomic percent.

$$
C_1' = \frac{C_1A_2}{C_1A_2 + C_2A_1}\times100
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `element1` | in | symbol of element 1 | - |
| `element2` | in | symbol of element 2 | - |
| `wt1` | in | weight percent of element 1 | - |
| `at1` | out | atomic percent of element 1 | - |
| `at2` | out | atomic percent of element 2 | - |

**Applies when:** known elements, different; 0 <= wt1 <= 100.

```shell
$ materials-science solve weight_to_atomic_percent element1=Cu element2=Ni wt1=60
at1 = 58.0792698
at2 = 41.9207302
  by: Weight percent to atomic percent (binary alloy)  (C1' = C1 A2 / (C1 A2 + C2 A1) x 100)
  wt1 = 60
  element1 = Cu
  element2 = Ni
  assumes: known elements, different; 0 <= wt1 <= 100
```

**What the worked example means:** This calculation converts the composition of a binary alloy from weight percent to atomic percent. Using the input weight percent of element1, which is 60, and the known atomic weights of element1 and element2, the model determines the fraction of atoms each element contributes to the total. The resulting atomic percent for element1 is 58.0793 and for element2 is 41.9207. This metric is essential for understanding crystal structures and defect chemistry where atom counts matter more than mass.

### Phase transformations (2)

<a id="avrami_kinetics"></a>

#### Avrami (JMAK) transformation kinetics: `avrami_kinetics`

The Avrami exponent n and rate constant k of y = 1 - exp(-k t^n) from transformed fractions at two or more times, the half-transformation time and the time for a target fraction.

$$
y = 1 - e^{-kt^n},\quad \ln\ln\frac{1}{1-y} = \ln k + n\ln t
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `t` | in | times (any one unit, e.g. s) | - |
| `y` | in | transformed fractions (0 to 1) | - |
| `y_target` | in | fraction whose time is wanted (optional) (optional) | - |
| `n` | out | Avrami exponent | - |
| `k` | out | rate constant (time unit^-n) | - |
| `log10_k` | out | log10 of k | - |
| `t_half` | out | time to 50 % transformation (time unit of t) | - |
| `r_squared` | out | r^2 of the fit | - |
| `t_target` | out | time to the target fraction | - |

**Applies when:** equal lengths, at least two distinct times; 0 < y < 1, rising with t.

```shell
$ materials-science solve avrami_kinetics "t=[50, 100]" "y=[0.2, 0.8]"
n = 2.850512903
k = 3.203674769e-06
log10_k = -5.494351579
t_half = 74.41407874
r_squared = 1
  by: Avrami (JMAK) transformation kinetics  (least-squares line of ln(-ln(1 - y)) against ln t: slope n, intercept ln k)
  assumes: equal lengths, at least two distinct times; 0 < y < 1, rising with t
```

**What the worked example means:** This calculation determines the kinetics of phase transformation by fitting the transformed fraction against time using the Johnson-Mehl-Avrami-Kolmogorov equation. The slope of the linearized plot provides the Avrami exponent, which is 2.85051, indicating the nature of nucleation and growth mechanisms. The intercept yields the rate constant, 3.20367e-06, governing the speed of the reaction. Based on these parameters, the time required for fifty percent transformation is calculated as 74.4141, offering insight into processing durations needed to achieve specific microstructural states.

<a id="homogeneous_nucleation"></a>

#### Critical nucleus radius and activation energy (homogeneous nucleation), `homogeneous_nucleation`

The critical radius r* and the energy barrier dG* for homogeneous nucleation of a solid from its undercooled melt.

$$
r^* = -\frac{2\gamma}{\Delta G_v},\quad \Delta G^* = \frac{16\pi\gamma^3}{3\Delta G_v^2},\quad \Delta G_v = -\frac{\Delta H_f\,\Delta T}{T_m}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `gamma` | in | solid-liquid interfacial energy | J/m^2 |
| `dHf` | in | latent heat of fusion per volume | J/m^3 |
| `Tm` | in | melting temperature | K |
| `T` | in | temperature of the undercooled liquid | K |
| `undercooling` | out | undercooling Tm - T | K |
| `dGv_MJ_m3` | out | volume free-energy change in MJ/m^3 | - |
| `r_star` | out | critical radius | m |
| `r_star_nm` | out | critical radius in nm | - |
| `dG_star` | out | activation free energy | J |
| `dG_star_eV` | out | activation free energy in eV | - |

**Applies when:** gamma, dHf, Tm > 0; 0 < T < Tm.

```shell
$ materials-science solve homogeneous_nucleation "gamma=0.132 J/m^2" "dHf=1160000000 J/m^3" "Tm=1064 degC" "T=834 degC"
undercooling = 230 K
dGv_MJ_m3 = -199.5288487
r_star = 1.323116942e-09 m
r_star_nm = 1.323116942
dG_star = 9.679635443e-19 J
dG_star_eV = 6.041553245
  by: Critical nucleus radius and activation energy (homogeneous nucleation)  (dGv = -dHf (Tm - T)/Tm; r* = -2 gamma/dGv; dG* = 16 pi gamma^3/(3 dGv^2))
  gamma = 0.132 J/m^2
  dHf = 1160000000 J/m^3
  Tm = 1337.15 K
  T = 1107.15 K
  assumes: gamma, dHf, Tm > 0; 0 < T < Tm
```

**What the worked example means:** This calculation determines the size a solid nucleus must reach to grow spontaneously in an undercooled liquid. Using the interfacial energy, latent heat of fusion, melting temperature, and current temperature, it computes the volume free-energy change and derives the critical radius and activation barrier. The resulting critical radius of 1.32312 nanometers indicates the minimum stable nucleus size, while the activation energy of 6.04155 electron volts represents the thermodynamic hurdle for nucleation.

### Polymers (8)

<a id="copolymer_degree_of_polymerization"></a>

#### Average repeat-unit weight and degree of polymerization of a copolymer, `copolymer_degree_of_polymerization`

The average repeat-unit molecular weight of a copolymer from the two repeat-unit formulas and the mole fraction of the first, and its degree of polymerization.

$$
\bar m = \sum f_jm_j,\quad \mathrm{DP} = \frac{\bar M_n}{\bar m}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `formula1` | in | formula of repeat unit 1 | - |
| `formula2` | in | formula of repeat unit 2 | - |
| `f1` | in | mole fraction of repeat unit 1 (0 to 1) | - |
| `Mn` | in | number-average molecular weight | kg/mol |
| `m_avg_g_mol` | out | average repeat-unit molecular weight in g/mol | - |
| `DP` | out | degree of polymerization | - |

**Applies when:** valid formulas; 0 <= f1 <= 1; Mn > 0.

```shell
$ materials-science solve copolymer_degree_of_polymerization formula1=C4H6 formula2=C8H8 f1=0.75 "Mn=350000 g/mol"
m_avg_g_mol = 66.60511
DP = 5254.852068
  by: Average repeat-unit weight and degree of polymerization of a copolymer  (m = f1 m1 + (1 - f1) m2; DP = Mn/m)
  f1 = 0.75
  Mn = 350 kg/mol
  formula1 = C4H6
  formula2 = C8H8
  assumes: valid formulas; 0 <= f1 <= 1; Mn > 0
```

**What the worked example means:** This calculation determines the average molecular weight of a single repeat unit in a copolymer using the mole fraction of the first monomer, 0.75, and the number-average molecular weight, 350000 g/mol. The resulting average repeat unit weight is 66.6051, which represents the mass contribution of one repeating segment. Dividing the total chain mass by this value yields the degree of polymerization, 5254.85, indicating the average number of repeat units per chain.

<a id="kelvin_voigt_creep"></a>

#### Creep strain of a Kelvin-Voigt viscoelastic solid, `kelvin_voigt_creep`

The strain after time t under constant stress for a Kelvin-Voigt (spring and dashpot in parallel) material: eps(t) = (sigma_0/E)(1 - exp(-t E/eta)).

$$
\varepsilon(t) = \frac{\sigma_0}{E}\left(1-e^{-t/\tau}\right)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `sigma0` | in | applied stress | Pa |
| `E` | in | spring modulus | Pa |
| `eta` | in | dashpot viscosity | Pa*s |
| `t` | in | time under load | s |
| `strain` | out | creep strain at time t | - |
| `fraction_of_elastic` | out | strain over the final strain sigma_0/E | - |
| `tau` | out | retardation time eta/E | s |

**Applies when:** sigma0, E, eta > 0; t >= 0.

```shell
$ materials-science solve kelvin_voigt_creep "sigma0=5 MPa" "E=500 MPa" "eta=1e+11 Pa*s" "t=200 s"
strain = 0.006321205588
fraction_of_elastic = 0.6321205588
tau = 200 s
  by: Creep strain of a Kelvin-Voigt viscoelastic solid  (eps(t) = (sigma_0/E)(1 - exp(-t/tau)), tau = eta/E)
  sigma0 = 5000000 Pa
  E = 500000000 Pa
  eta = 1e+11 Pa*s
  t = 200 s
  assumes: sigma0, E, eta > 0; t >= 0
```

**What the worked example means:** This calculation determines the viscoelastic response of a Kelvin-Voigt solid under constant stress. Given an applied stress of 5 MPa, a spring modulus of 500 MPa, and a dashpot viscosity of 1e+11 Pa*s, the model computes the strain after a duration of 200 s. The resulting creep strain is 0.00632121, which represents 0.632121 of the ultimate elastic strain. This behavior is characterized by the retardation time 200 s, indicating how quickly the material approaches its equilibrium deformation.

<a id="polymer_chain_dimensions"></a>

#### End-to-end distance of a freely rotating chain, `polymer_chain_dimensions`

The root-mean-square end-to-end distance and characteristic ratio of a polymer chain of n bonds of length l with fixed bond angle and free rotation, and its all-trans contour length.

$$
\langle r^2\rangle = nl^2\left[\frac{1+c}{1-c} - \frac{2c(1-c^n)}{n(1-c)^2}\right],\ c = -\cos\theta
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `n` | in | number of backbone bonds | - |
| `l` | in | bond length | m |
| `theta` | in | bond angle in degrees (default 109.47, tetrahedral) (optional) | - |
| `r_rms` | out | rms end-to-end distance | m |
| `r_rms_nm` | out | rms end-to-end distance in nm | - |
| `C_n` | out | characteristic ratio <r^2>/(n l^2) | - |
| `contour_nm` | out | all-trans contour length in nm | - |

**Applies when:** n >= 1; l > 0; 90 < theta < 180 degrees.

```shell
$ materials-science solve polymer_chain_dimensions n=2 "l=0.154 nm" theta=120
r_rms = 2.667358244e-10 m
r_rms_nm = 0.2667358244
C_n = 1.5
contour_nm = 0.2667358244
  by: End-to-end distance of a freely rotating chain  (exact finite-chain sum <r^2> = n l^2 [(1 + c)/(1 - c) - 2c(1 - c^n)/(n(1 - c)^2)], c = -cos(theta))
  l = 1.54e-10 m
  theta = 120
  assumes: n >= 1; l > 0; 90 < theta < 180 degrees
```

**What the worked example means:** This calculation determines the statistical size of a polymer chain based on its backbone geometry, specifically the number of bonds 2, bond length 0.154 nm, and bond angle 120. The result shows that the root-mean-square end-to-end distance is 2.66736e-10 m, which corresponds to 0.266736 nanometers. This value reflects the average spatial extent of the coil formed by freely rotating bonds with fixed angles, while the characteristic ratio 1.5 quantifies how much larger this real chain is compared to an ideal random walk. The all-trans contour length, representing the maximum possible extension, is calculated as 0.266736 nanometers.

<a id="molecular_weight_averages"></a>

#### Number- and weight-average molecular weight and dispersity, `molecular_weight_averages`

Mn, Mw and the dispersity Mw/Mn of a polymer from a molecular-weight distribution given as number fractions or weight fractions per size range.

$$
\bar M_n = \sum x_iM_i,\quad \bar M_w = \sum w_iM_i,\quad \mathrm{PDI} = \bar M_w/\bar M_n
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `M` | in | mean molecular weight of each size range in g/mol | - |
| `fraction` | in | number or weight fraction of each range (normalised) | - |
| `basis` | in | whether the fractions are number (x_i) or weight (w_i) fractions (default number) (optional) | - |
| `Mn` | out | number-average molecular weight in g/mol | - |
| `Mw` | out | weight-average molecular weight in g/mol | - |
| `PDI` | out | dispersity (polydispersity index) Mw/Mn | - |

**Applies when:** equal lengths; M > 0; fractions >= 0, not all zero (they are normalised).

```shell
$ materials-science solve molecular_weight_averages "M=[10000, 20000, 30000]" "fraction=[0.2, 0.5, 0.3]" basis=number
Mn = 21000
Mw = 23333.33333
PDI = 1.111111111
  by: Number- and weight-average molecular weight and dispersity  (number basis: Mn = sum x_i M_i, Mw = sum x_i M_i^2 / Mn; weight basis: Mw = sum w_i M_i, Mn = 1/sum(w_i/M_i))
  basis = number
  assumes: equal lengths; M > 0; fractions >= 0, not all zero (they are normalised)
```

**What the worked example means:** This calculation determines the average chain lengths of a polymer based on its molecular weight distribution. Using the provided mean weights for each size range and their corresponding fractions, it computes the number-average molecular weight as 21000 and the weight-average molecular weight as 23333.3. The ratio of these two values yields the dispersity, or polydispersity index, which is 1.11111. This metric indicates how broad the distribution of chain sizes is, directly influencing the material's mechanical strength and thermal behavior.

<a id="polymer_crystallinity"></a>

#### Percent crystallinity of a polymer from densities, `polymer_crystallinity`

The mass percent crystallinity of a semicrystalline polymer from its density and the densities of the fully crystalline and fully amorphous polymer.

$$
\%\,\text{cryst} = \frac{\rho_c(\rho_s-\rho_a)}{\rho_s(\rho_c-\rho_a)}\times100
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `rho_s` | in | density of the sample | kg/m^3 |
| `rho_c` | in | density of the totally crystalline polymer | kg/m^3 |
| `rho_a` | in | density of the totally amorphous polymer | kg/m^3 |
| `crystallinity_percent` | out | percent crystallinity (by mass) | - |

**Applies when:** rho_a < rho_s < rho_c.

```shell
$ materials-science solve polymer_crystallinity "rho_s=0.925 g/cm^3" "rho_c=1 g/cm^3" "rho_a=0.87 g/cm^3"
crystallinity_percent = 45.73804574
  by: Percent crystallinity of a polymer from densities  (%cryst = rho_c (rho_s - rho_a) / (rho_s (rho_c - rho_a)) x 100)
  rho_s = 925 kg/m^3
  rho_c = 1000 kg/m^3
  rho_a = 870 kg/m^3
  assumes: rho_a < rho_s < rho_c
```

In the worked example, rho_s = 0.925 g/cm^3, rho_c = 1 g/cm^3, rho_a = 0.87 g/cm^3 go through the equation above and give crystallinity_percent = 45.73804574. It applies only when rho_a < rho_s < rho_c: outside that it says what is wrong instead of returning a number.

<a id="degree_of_polymerization"></a>

#### Repeat-unit molecular weight and degree of polymerization, `degree_of_polymerization`

The molecular weight of a repeat unit from its formula and the number-average degree of polymerization DP = Mn / m.

$$
\mathrm{DP} = \frac{\bar M_n}{m}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `formula` | in | repeat-unit formula, e.g. C2H3Cl | - |
| `Mn` | in | number-average molecular weight | kg/mol |
| `repeat_unit_g_mol` | out | repeat-unit molecular weight in g/mol | - |
| `DP` | out | degree of polymerization | - |

**Applies when:** a valid formula; Mn > 0.

```shell
$ materials-science solve degree_of_polymerization formula=C2H3Cl "Mn=106000 g/mol"
repeat_unit_g_mol = 62.49822
DP = 1696.048303
  by: Repeat-unit molecular weight and degree of polymerization  (m = sum of atomic weights of the repeat unit; DP = Mn/m)
  Mn = 106 kg/mol
  formula = C2H3Cl
  assumes: a valid formula; Mn > 0
```

**What the worked example means:** This calculation determines the mass of a single repeating unit in a polymer chain, expressed as 62.4982, based on its chemical formula. Using this value and the provided number-average molecular weight of 106000 g/mol, the average number of these units per chain, known as the degree of polymerization, is calculated to be 1696.05. This result links the microscopic chemical structure to the macroscopic physical size of the polymer molecules.

<a id="rubber_elasticity"></a>

#### Stress in a stretched rubber (affine network), `rubber_elasticity`

The shear modulus G = rho R T / Mc of a crosslinked rubber and its engineering and true stress at a uniaxial stretch ratio lambda.

$$
\sigma = \frac{\rho RT}{M_c}\left(\lambda - \frac{1}{\lambda^2}\right)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `rho` | in | density of the rubber | kg/m^3 |
| `Mc` | in | molecular weight between crosslinks | kg/mol |
| `T` | in | temperature | K |
| `stretch` | in | stretch ratio lambda = L/L0 | - |
| `G_MPa` | out | shear modulus in MPa | - |
| `stress_MPa` | out | engineering stress in MPa | - |
| `true_stress_MPa` | out | true stress in MPa | - |

**Applies when:** all inputs > 0; ideal affine network, incompressible.

```shell
$ materials-science solve rubber_elasticity "rho=0.95 g/cm^3" "Mc=5000 g/mol" "T=300 K" stretch=2
G_MPa = 0.4739243692
stress_MPa = 0.8293676462
true_stress_MPa = 1.658735292
  by: Stress in a stretched rubber (affine network)  (G = rho R T / Mc; sigma = G (lambda - 1/lambda^2))
  rho = 950 kg/m^3
  Mc = 5 kg/mol
  T = 300 K
  stretch = 2
  assumes: all inputs > 0; ideal affine network, incompressible
```

**What the worked example means:** This calculation determines the mechanical response of an ideal crosslinked rubber network. Using the density, temperature, and molecular weight between crosslinks, it first computes the shear modulus as 0.473924. Then, applying the affine network theory to the specified stretch ratio, it yields an engineering stress of 0.829368 and a true stress of 1.65874, reflecting the material's resistance to deformation under these specific conditions.

<a id="maxwell_stress_relaxation"></a>

#### Stress relaxation of a Maxwell viscoelastic solid, `maxwell_stress_relaxation`

The stress remaining after time t in a polymer held at constant strain, modelled as a Maxwell element: sigma(t) = sigma_0 exp(-t E/eta).

$$
\sigma(t) = \sigma_0 e^{-t/\tau},\quad \tau = \frac{\eta}{E}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `sigma0` | in | initial stress | Pa |
| `E` | in | spring modulus | Pa |
| `eta` | in | dashpot viscosity | Pa*s |
| `t` | in | time held | s |
| `sigma_t` | out | stress at time t | Pa |
| `sigma_t_MPa` | out | stress at time t in MPa | - |
| `tau` | out | relaxation time eta/E | s |
| `ratio` | out | sigma(t)/sigma_0 | - |

**Applies when:** sigma0, E, eta > 0; t >= 0.

```shell
$ materials-science solve maxwell_stress_relaxation "sigma0=10 MPa" "E=1 GPa" "eta=1e+12 Pa*s" "t=1000 s"
sigma_t = 3678794.412 Pa
sigma_t_MPa = 3.678794412
tau = 1000 s
ratio = 0.3678794412
  by: Stress relaxation of a Maxwell viscoelastic solid  (sigma(t) = sigma_0 exp(-t/tau), tau = eta/E)
  sigma0 = 10000000 Pa
  E = 1000000000 Pa
  eta = 1e+12 Pa*s
  t = 1000 s
  assumes: sigma0, E, eta > 0; t >= 0
```

**What the worked example means:** This calculation determines the remaining stress in a Maxwell viscoelastic solid after a specific duration. Using the spring modulus and dashpot viscosity, the relaxation time is computed as 1000 s. The final stress value of 3.67879e+06 Pa indicates how much tension persists compared to the initial state, showing that the material has relaxed significantly if the ratio 0.367879 is low.

### Thermal properties (5)

<a id="wiedemann_franz"></a>

#### Electronic thermal conductivity of a metal (Wiedemann-Franz law), `wiedemann_franz`

The electronic thermal conductivity kappa = L sigma T of a metal from its electrical conductivity (or resistivity) and temperature, with the Sommerfeld value of the Lorenz number.

$$
\kappa = L\sigma T,\quad L = \frac{\pi^2}{3}\left(\frac{k}{e}\right)^2
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `T` | in | temperature | K |
| `sigma` | in | electrical conductivity (or give rho) (optional) | 1/(ohm*m) |
| `rho` | in | electrical resistivity (or give sigma) (optional) | ohm*m |
| `kappa` | out | thermal conductivity | W/(m*K) |
| `lorenz` | out | Lorenz number in W ohm/K^2 | - |

**Applies when:** T > 0; exactly one of sigma, rho, positive; degenerate free-electron metal.

```shell
$ materials-science solve wiedemann_franz "T=300 K" "sigma=59600000 S/m"
kappa = 436.8092062 W/(m*K)
lorenz = 2.443004509e-08
  by: Electronic thermal conductivity of a metal (Wiedemann-Franz law)  (kappa = L sigma T, L = (pi^2/3)(k/e)^2 = 2.44e-8 W ohm/K^2)
  T = 300 K
  sigma = 59600000 1/(ohm*m)
  assumes: T > 0; exactly one of sigma, rho, positive; degenerate free-electron metal
```

**What the worked example means:** This calculation determines the electronic thermal conductivity, denoted as kappa, for a metal by applying the Wiedemann-Franz law. The result, 436.809 W/(m*K), is derived from the input temperature 300 K and either the electrical conductivity or resistivity provided. The constant of proportionality used is the Sommerfeld value for the Lorenz number, which is 2.443e-08. This method assumes that heat is primarily transported by free electrons and provides an estimate when direct thermal measurements are impractical.

<a id="linear_thermal_expansion"></a>

#### Linear thermal expansion: `linear_thermal_expansion`

The length change dL = alpha L0 dT and the new length of a bar heated or cooled freely by dT.

$$
\Delta L = \alpha L_0\,\Delta T
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `alpha` | in | linear thermal expansion coefficient | 1/K |
| `L0` | in | initial length | m |
| `dT` | in | temperature change (positive = heating) | K |
| `dL` | out | length change | m |
| `dL_mm` | out | length change in mm | - |
| `L` | out | new length | m |
| `strain` | out | thermal strain alpha dT | - |

**Applies when:** alpha, L0 > 0; alpha constant over the range (linear, small strain).

```shell
$ materials-science solve linear_thermal_expansion "alpha=1.7e-05 1/K" "L0=0.5 m" "dT=100 K"
dL = 0.00085 m
dL_mm = 0.85
L = 0.50085 m
strain = 0.0017
  by: Linear thermal expansion  (dL = alpha L0 dT, L = L0 + dL)
  alpha = 1.7e-05 1/K
  L0 = 0.5 m
  dT = 100 K
  assumes: alpha, L0 > 0; alpha constant over the range (linear, small strain)
```

**What the worked example means:** This calculation determines the change in length and final size of a material when its temperature shifts. Applying the linear thermal expansion coefficient shows that the initial length of 0.5 m changes by 0.00085 m, resulting in a new length of 0.50085 m. The corresponding thermal strain is 0.0017. This helps predict how parts fit together under varying temperatures.

<a id="debye_heat_capacity"></a>

#### Molar heat capacity of a solid (Debye model), `debye_heat_capacity`

The constant-volume molar heat capacity of a monatomic solid from the Debye model at temperature T.

$$
C_v = 9R\left(\frac{T}{\Theta_D}\right)^3\int_0^{\Theta_D/T}\frac{x^4e^x}{(e^x-1)^2}\,dx
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `theta_D` | in | Debye temperature | K |
| `T` | in | temperature | K |
| `Cv` | out | molar heat capacity at constant volume | J/(mol*K) |
| `Cv_over_3R` | out | Cv / 3R (1 at high T) | - |

**Applies when:** theta_D, T > 0; T >= theta_D/300.

```shell
$ materials-science solve debye_heat_capacity "theta_D=343 K" "T=1000000 K"
Cv = 24.94338771 J/(mol*K)
Cv_over_3R = 0.9999999941
  by: Molar heat capacity of a solid (Debye model)  (Cv = 9R (T/theta_D)^3 int_0^(theta_D/T) x^4 e^x/(e^x - 1)^2 dx (mpmath quadrature, 30 digits))
  theta_D = 343 K
  T = 1000000 K
  assumes: theta_D, T > 0; T >= theta_D/300
```

**What the worked example means:** This calculation determines the constant-volume molar heat capacity of a monatomic solid using the Debye model. By integrating over the phonon spectrum defined by the Debye temperature, it accounts for quantum mechanical effects that become significant at low temperatures. The result indicates that the thermal energy storage capacity per mole is 24.9434 J/(mol*K). This value represents 1 of the maximum classical heat capacity predicted by the Dulong-Petit law, showing how closely the material approaches its high-temperature limit under the given conditions.

<a id="einstein_heat_capacity"></a>

#### Molar heat capacity of a solid (Einstein model), `einstein_heat_capacity`

The constant-volume molar heat capacity of a solid from the Einstein model (all atoms vibrating at one frequency).

$$
C_v = 3R\left(\frac{\Theta_E}{T}\right)^2\frac{e^{\Theta_E/T}}{(e^{\Theta_E/T}-1)^2}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `theta_E` | in | Einstein temperature | K |
| `T` | in | temperature | K |
| `Cv` | out | molar heat capacity at constant volume | J/(mol*K) |
| `Cv_over_3R` | out | Cv / 3R | - |

**Applies when:** theta_E, T > 0.

```shell
$ materials-science solve einstein_heat_capacity "theta_E=300 K" "T=300 K"
Cv = 22.96471855 J/(mol*K)
Cv_over_3R = 0.9206735942
  by: Molar heat capacity of a solid (Einstein model)  (Cv = 3R x^2 e^x/(e^x - 1)^2, x = theta_E/T)
  theta_E = 300 K
  T = 300 K
  assumes: theta_E, T > 0
```

**What the worked example means:** This calculation determines the constant-volume molar heat capacity of a solid using the Einstein model, where all atoms vibrate at a single characteristic frequency. Given an Einstein temperature of 300 K and a system temperature of 300 K, the resulting heat capacity is 22.9647 J/(mol*K). The dimensionless ratio of this capacity to the classical limit of three R is 0.920674, indicating how far the quantum behavior deviates from the high-temperature classical prediction.

<a id="volumetric_thermal_expansion"></a>

#### Volumetric thermal expansion: `volumetric_thermal_expansion`

The volume change dV = beta V0 dT of an isotropic solid with beta = 3 alpha (linearised), and the exact change V0((1 + alpha dT)^3 - 1).

$$
\Delta V = \beta V_0\,\Delta T,\quad \beta = 3\alpha
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `alpha` | in | LINEAR thermal expansion coefficient | 1/K |
| `V0` | in | initial volume | m^3 |
| `dT` | in | temperature change (positive = heating) | K |
| `beta` | out | volumetric coefficient 3 alpha | 1/K |
| `dV` | out | linearised volume change beta V0 dT | m^3 |
| `dV_cm3` | out | linearised volume change in cm^3 | - |
| `V` | out | new volume (linearised) | m^3 |
| `dV_exact` | out | exact volume change V0((1 + alpha dT)^3 - 1) | m^3 |
| `dV_exact_cm3` | out | exact volume change in cm^3 | - |

**Applies when:** alpha, V0 > 0; isotropic solid, alpha constant over the range.

```shell
$ materials-science solve volumetric_thermal_expansion "alpha=1.7e-05 1/K" "V0=1000 cm^3" "dT=100 K"
beta = 5.1e-05 1/K
dV = 5.1e-06 m^3
dV_cm3 = 5.1
V = 0.0010051 m^3
dV_exact = 5.108674913e-06 m^3
dV_exact_cm3 = 5.108674913
  by: Volumetric thermal expansion  (beta = 3 alpha, dV = beta V0 dT; exact: V0((1 + alpha dT)^3 - 1))
  alpha = 1.7e-05 1/K
  V0 = 0.001 m^3
  dT = 100 K
  assumes: alpha, V0 > 0; isotropic solid, alpha constant over the range
```

**What the worked example means:** This calculation determines how the volume of an isotropic solid changes due to temperature variations. Using the linear expansion coefficient 1.7e-05 1/K and initial volume 1000 cm^3, the model computes the volumetric coefficient as 5.1e-05 1/K. The linearized volume change is found to be 5.1e-06 m^3 cubic meters, while the exact calculation yields 5.10867e-06 m^3 cubic meters. These results help predict dimensional stability under thermal stress.

Questions and reports: support@ai999x.com. AI999X, The Future Is Self-Improving.
