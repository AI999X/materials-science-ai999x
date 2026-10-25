"""materials-science-ai999x — offline materials science answers with their formula, steps and verification.

    import materials_science_ai999x as m
    m.ask("Theoretical density: structure = FCC, A = 63.55 g/mol, R = 0.128 nm").show()
    m.ask("Fick's second law: C0 = 0.25 wt%, Cs = 1.2 wt%, x = 0.5 mm, D = 1.6e-11 m^2/s, t = 7 h").show()
    m.solve("planar_density", structure="BCC", h=1, k=1, l=0, a="0.2866 nm").show()
    m.capabilities()                   # every certified capability

What it covers (81 certified capabilities, beginner to expert):
- crystal structures and diffraction: lattice parameters, packing factors and coordination, theoretical density,
  unit-cell volumes, interplanar spacings (cubic to triclinic), Bragg's law, reflection angles, structure factors and
  allowed reflections, linear and planar density, Miller and Miller-Bravais indices, angles between directions and
  planes, interstitial sites, radius ratios, ionic-crystal densities, space groups;
- defects and diffusion: vacancy concentrations and formation energies, Burgers vectors, Fick's first and second laws
  (concentration, time and depth), Arrhenius fits of diffusion data;
- mechanical properties: true stress and strain, elastic deformation and elastic constants, Young's modulus and the
  0.2 % offset yield from data, toughness and resilience, strain hardening, Hall-Petch, thermal stress, Schmid's law,
  creep parameters, fracture mechanics (Griffith, K_IC), Weibull statistics;
- phase diagrams and transformations: lever rule, eutectic and iron-carbon microconstituents, Gibbs' phase rule,
  wt% / at% conversion, homogeneous nucleation, Avrami kinetics;
- polymers, ceramics and composites: molecular-weight averages, degree of polymerization (also copolymers),
  crystallinity, rubber elasticity, chain dimensions, viscoelastic models, rule of mixtures, critical fibre length;
- electronic and thermal properties: intrinsic carriers, band gap from conductivity, Fermi energy, Wiedemann-Franz,
  Debye and Einstein heat capacities.

Questions are written as the capability phrase, a colon and `name = value` pairs (see templates()). Built by the
AI999X model machine from the same engine as every domain; a question that is not determined is refused rather than
guessed.

Creator / engineer: Ibnul Jaif Farabi. Company: AI999X.
"""
from __future__ import annotations

from pathlib import Path

try:                                       # the installed package carries its own copy of the engine
    from ._engine import Answer
    from ._engine.product import Product
except ImportError:                        # the factory's source tree, during development
    from ai999x_factory.runtime import Answer
    from ai999x_factory.runtime.product import Product

__version__ = "1.0.0"
_P = Product("materials_science", [Path(__file__).resolve().parent / "pack", Path(__file__).resolve().parents[3]])
pack, ask, solve, compile_problem = _P.pack, _P.ask, _P.solve, _P.compile_problem
formula, formulas, quantities, conditions, capabilities = _P.formula, _P.formulas, _P.quantities, _P.conditions, _P.capabilities
examples, family_examples, structured_example, to_si = _P.examples, _P.family_examples, _P.structured_example, _P.to_si
templates, procedure_examples = _P.templates, _P.procedure_examples
prove, derivations, about = _P.prove, _P.derivations, (lambda: {**_P.about(), "version": __version__})
knowledge, lookup, related = _P.knowledge, _P.lookup, _P.related
plot, plot_relation, search = _P.plot, _P.plot_relation, _P.search   # figures (SVG) and capability search ([search])
__all__ = ["ask", "solve", "prove", "derivations", "about", "formula", "formulas", "quantities", "conditions", "capabilities",
           "compile_problem", "pack", "Answer", "plot", "plot_relation", "search", "__version__"]
