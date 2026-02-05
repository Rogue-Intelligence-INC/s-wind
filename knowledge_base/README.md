# S-wind External Knowledge Base

This directory contains structured JSON files with formulas, constants, and references used by the S-wind paper generation system.

## Structure

```
knowledge_base/
├── physics/
│   ├── thermodynamics.json    # Heat transfer, Carnot, entropy
│   └── fluid_dynamics.json    # Navier-Stokes, Reynolds, Bernoulli
├── engineering/
│   ├── structural.json        # Euler buckling, beam theory, vibration
│   └── electrical.json        # Ohm's law, power, transformers
├── materials/
│   └── mechanics.json         # Hall-Petch, fracture, creep
├── computing/
│   └── complexity.json        # Amdahl, roofline, Shannon
├── biology/
│   └── biochemistry.json      # Michaelis-Menten, Nernst, Gibbs
└── economics/
    └── optimization.json      # EOQ, NPV, LCOE, learning curves
```

## JSON Schema

Each file follows this structure:

```json
{
  "domain": "Domain Name",
  "formulas": [
    {
      "id": "FORMULA_ID",
      "name": "Formula Name",
      "latex": "\\LaTeX formula",
      "variables": {
        "symbol": "Description (units)"
      },
      "source": "Author, Year"
    }
  ]
}
```

## Usage in Batch Processing

The knowledge base is referenced by batch processing binaries:

```rust
// In batch_XX_*.rs
knowledge: vec![
    ("FORMULA_ID", "Name", "latex", "Source"),
]
```

## Adding New Formulas

1. Identify the appropriate domain file
2. Add a new entry to the `formulas` array
3. Include proper LaTeX escaping (double backslashes)
4. Cite original source

## Domain Coverage

| Domain | Files | Formulas |
|--------|-------|----------|
| Physics | 2 | 12 |
| Engineering | 2 | 12 |
| Materials | 1 | 6 |
| Computing | 1 | 6 |
| Biology | 1 | 6 |
| Economics | 1 | 6 |
| **Total** | **8** | **48** |

---

*S-wind Automated Research System - Valhalla Framework*
