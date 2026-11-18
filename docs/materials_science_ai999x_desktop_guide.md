# Materials Science: the desktop app, user guide

**Materials Science**, desktop app 1.6.5 for Windows and Linux. **Creator / engineer:** Ibnul Jaif Farabi · **Company:** AI999X · support@ai999x.com

Materials Science covers crystal structures and diffraction, defects and diffusion, mechanical properties, phase diagrams, polymers, ceramics and composites, and electronic and thermal properties. Its desktop app holds all 88 calculations in 13 topics: pick one, change its inputs and the answer, a live scene and charts follow at once. It runs on your own computer with no network connection and nothing else to install. The pictures in this guide were taken in this app.

## Contents

- 1. Get it and start it
- 2. The window
- 3. Compute a calculation
- 4. Ask in words
- 5. The live scene and the charts
- 6. Light and dark
- 7. Every calculation
- 8. Other ways to use it

## 1. Get it and start it

| system | download |
|---|---|
| Windows x86-64 | https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/ai999x-materials-science-1.6.5-windows-x86_64.zip |
| Linux x86-64 | https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/ai999x-materials-science-1.6.5-linux-x86_64.tar.gz |

Windows: Download ai999x-materials-science-1.6.5-windows-x86_64.zip, right-click it → Extract All, then open the folder and start ai999x-materials-science.exe. If Windows SmartScreen asks, choose More info → Run anyway.

Linux:

```shell
curl -LO https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/ai999x-materials-science-1.6.5-linux-x86_64.tar.gz
curl -LO https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/SHA256SUMS
sha256sum -c --ignore-missing SHA256SUMS
tar xzf ai999x-materials-science-1.6.5-linux-x86_64.tar.gz
./ai999x-materials-science-1.6.5-linux-x86_64/ai999x-materials-science
```

Checksums of every download: https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/SHA256SUMS. Install steps for every app: https://www.ai999x.com/desktop-cli/

## 2. The window

![The Materials Science app, opened on Theoretical density of a metal with its worked example.](https://www.ai999x.com/?ax_shot=materials-science-ai999x/hero.webp)

*The Materials Science app, opened on Theoretical density of a metal with its worked example.*

On the left, a search box and the topics with their calculations. In the middle, the chosen calculation: its answer at the top, then the live scene (or its charts), what the answer means and the formula. On the right, its inputs, each with a slider where that makes sense, and the Compute and Reset buttons. At the bottom, a box for a question in words.

## 3. Compute a calculation

Open Crystal structures › Theoretical density of a metal (or type part of its name in the search box). It opens with a worked example filled in: “Theoretical density: structure = FCC, A = 63.55 g/mol, R = 0.128 nm” The answer: rho = 8895.28269 kg/m^3, rho_gcc = 8.89528269, a_nm = 0.362038672.

Change any input (type a number with its unit, or drag its slider) and the answer, the scene and the charts follow; Compute recomputes after typing, Reset brings back the worked example. An input outside the range the formula holds for is refused with the reason, never answered with a guessed number.

![Theoretical density of a metal: the live scene of the worked example.](https://www.ai999x.com/?ax_shot=materials-science-ai999x/scene-1.webp)

*Theoretical density of a metal: the live scene of the worked example.*

## 4. Ask in words

Type a question as you would say it, for example “Theoretical density: structure = FCC, A = 63.55 g/mol, R = 0.128 nm”. The app reads the quantities and their units, picks the calculation and answers; the command-line program reads it the same way and printed:

```text
Read as: Theoretical density of a metal
rho = 8895.28269 kg/m^3
rho_gcc = 8.89528269
```

When a question names a quantity the app cannot place, it says what is missing instead of guessing.

## 5. The live scene and the charts

Every calculation draws a scene from its answer: it moves as an input changes. The Charts button shows charts that vary one input around your values; Enlarge opens one in a large view. Scenes from other topics:

![Bragg angle of a reflection](https://www.ai999x.com/?ax_shot=materials-science-ai999x/scene-2.webp)

*Bragg angle of a reflection*

![Concentration in a semi-infinite solid (Fick's second law)](https://www.ai999x.com/?ax_shot=materials-science-ai999x/scene-3.webp)

*Concentration in a semi-infinite solid (Fick's second law)*

![Proeutectoid phase and pearlite in a plain-carbon steel](https://www.ai999x.com/?ax_shot=materials-science-ai999x/scene-4.webp)

*Proeutectoid phase and pearlite in a plain-carbon steel*

![Lattice parameter and density of an ionic crystal](https://www.ai999x.com/?ax_shot=materials-science-ai999x/scene-5.webp)

*Lattice parameter and density of an ionic crystal*

![Elastic moduli of a continuous-fibre composite (rule of mixtures)](https://www.ai999x.com/?ax_shot=materials-science-ai999x/scene-6.webp)

*Elastic moduli of a continuous-fibre composite (rule of mixtures)*

![Burgers vector of a perfect dislocation](https://www.ai999x.com/?ax_shot=materials-science-ai999x/scene-7.webp)

*Burgers vector of a perfect dislocation*

![Band gap from intrinsic conductivities at several temperatures](https://www.ai999x.com/?ax_shot=materials-science-ai999x/scene-8.webp)

*Band gap from intrinsic conductivities at several temperatures*

![Theoretical density of a metal: a chart, enlarged.](https://www.ai999x.com/?ax_shot=materials-science-ai999x/chart-1.webp)

*Theoretical density of a metal: a chart, enlarged.*

![Bragg angle of a reflection: a chart, enlarged.](https://www.ai999x.com/?ax_shot=materials-science-ai999x/chart-2.webp)

*Bragg angle of a reflection: a chart, enlarged.*

## 6. Light and dark

The app follows a light or a dark theme (the switch is in its top bar).

![The Materials Science app in the dark theme.](https://www.ai999x.com/?ax_shot=materials-science-ai999x/hero-dark.webp)

*The Materials Science app in the dark theme.*

## 7. Every calculation

88 calculations in 13 topics; each has its own entry on the model's page (https://www.ai999x.com/models/materials-science-ai999x/) with its formula, worked example and charts.

| topic | calculations |
|---|---|
| Ceramics | Coordination number from the cation-anion radius ratio, Lattice parameter and density of an ionic crystal |
| Composites | Critical fibre length, Elastic moduli of a continuous-fibre composite (rule of mixtures) |
| Crystal structures | Angle between two directions in a cubic crystal, Angle between two lattice directions in any crystal, Angle between two lattice planes in any crystal, Atomic packing factor and coordination number, Four-index (Miller-Bravais) to three-index direction, Lattice parameter and atomic radius from the density, Lattice parameter from the atomic radius, Linear density along a crystal direction, Miller indices of a plane from its intercepts, Planar density of a crystal plane, Space group of a common crystal structure, Theoretical density of a metal, Three-index to four-index (Miller-Bravais) direction, Volume and density of a hexagonal close-packed cell, Volume change of a BCC to FCC transformation, Volume of a general (triclinic) unit cell |
| Defects | Burgers vector of a perfect dislocation, Equilibrium vacancy concentration, Largest atom that fits an interstitial site, Vacancy formation energy from vacancy fractions |
| Diffraction | Bragg angle of a reflection, Diffraction angle of a cubic (h k l) reflection, First allowed reflections of a cubic crystal, Interplanar spacing from a Bragg reflection, Interplanar spacing in a cubic crystal, Interplanar spacing in a general (triclinic) lattice, Interplanar spacing in a hexagonal crystal, Interplanar spacing in a tetragonal crystal, Interplanar spacing in an orthorhombic crystal, Structure factor and extinction of a cubic reflection, X-ray wavelength from a Bragg reflection |
| Diffusion | Activation energy and pre-exponential factor of diffusion, Concentration in a semi-infinite solid (Fick's second law), Depth at which a concentration is reached (Fick's second law), Diffusion coefficient at a temperature (Arrhenius), Steady-state diffusion flux (Fick's first law), Time to reach a concentration at a depth (Fick's second law) |
| Electronic properties | Band gap from intrinsic conductivities at several temperatures, Built-in potential of a p-n junction, Electron and hole concentrations of a doped semiconductor, Fermi energy of a free-electron metal, Intrinsic carrier concentration of a semiconductor |
| Fracture | Critical crack length from the fracture toughness, Design stress for a target failure probability (Weibull), Fatigue crack growth life by Paris' law, Fracture stress from the fracture toughness, Griffith critical stress of a brittle solid, Weibull modulus from a set of measured strengths, Weibull probability of failure of a brittle part |
| Mechanical properties | 0.2 % offset yield strength from a stress-strain curve, Elastic elongation and diameter change of a rod in tension, Hall-Petch grain-size strengthening, Larson-Miller parameter (creep rupture), Modulus of resilience, Most favourable slip system of an FCC or BCC crystal, Resolved shear stress and Schmid factor of a slip system, Shear modulus, bulk modulus and Lame constant from E and Poisson's ratio, Strain-hardening exponent and strength coefficient (Hollomon), Stress exponent and activation energy of steady-state creep, Thermal stress in a constrained body, Toughness (area under the stress-strain curve), True stress and true strain from engineering values, Young's modulus from elastic stress-strain data |
| Phase diagrams | Atomic percent to weight percent (binary alloy), Degrees of freedom by Gibbs' phase rule, Phase fractions by the lever rule, Primary and eutectic microconstituent fractions, Proeutectoid phase and pearlite in a plain-carbon steel, Weight percent to atomic percent (binary alloy) |
| Phase transformations | Avrami (JMAK) transformation kinetics, Critical nucleus radius and activation energy (homogeneous nucleation) |
| Polymers | Average repeat-unit weight and degree of polymerization of a copolymer, Creep strain of a Kelvin-Voigt viscoelastic solid, End-to-end distance of a freely rotating chain, Number- and weight-average molecular weight and dispersity, Percent crystallinity of a polymer from densities, Repeat-unit molecular weight and degree of polymerization, Stress in a stretched rubber (affine network), Stress relaxation of a Maxwell viscoelastic solid |
| Thermal properties | Electronic thermal conductivity of a metal (Wiedemann-Franz law), Linear thermal expansion, Molar heat capacity of a solid (Debye model), Molar heat capacity of a solid (Einstein model), Volumetric thermal expansion |

## 8. Other ways to use it

The same engine runs on the command line (`materials-science`, its own guide), in Python (`pip install materials-science-ai999x`, `import materials_science_ai999x`, its own guide) and in the browser on https://www.ai999x.com/models/materials-science-ai999x/.

Licence: Apache-2.0 · AI999X · https://www.ai999x.com/models/materials-science-ai999x/
