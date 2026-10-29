# S-Wind Runtime Logic (as implemented)

> Code-traced reference, 2026-10-30. Describes what the code actually does,
> in execution order. Claims in `pitch/SWIND_PITCH.md` refer to the
> **verified path** (§2) unless stated otherwise.

## 0. Topology — one product, one entry point

S-Wind is a standalone product (this repo). It is **not** part of the
Valhalla workspace (separated 2026-10-30); the only shared component,
Voyager, is vendored under `vendor/voyager`.

**The single supported user entry point is the `swind` CLI:**

```bash
swind explore "10,17,24"                       # continued exploration (default)
swind explore "12,21,34,51,72,97,126,159" --mode verified --save-pending
swind explore "1/2,1/3,1/4" --mode full
```

`explore` accepts integers or `a/b` fractions, runs the Fate engine
(§1–§3, selected by `--mode`), prints the graded assessment, and with
`--save-pending` writes surviving candidates into the KnowledgeBase
pending-review queue (§5) — the propose → review loop through one command.
Other CLI verbs (`list`, `optimize`, `paper`, `batch`) are project-catalog
utilities. The 87 binaries under `src/bin/` are archived research scripts,
not entry points (a usage census found only 12 of them call the engine at
all). The library API (`Fate`, `KnowledgeBase`, `ExperimentRunner`) remains
available for embedding, and the Voyager report runtime
(`interface::ExperimentRunner`) still powers derivation-trace documents,
but neither is a separate product entry.

Supporting modules: `quad` (four Rational physics signal generators used by
experiment binaries as signal sources/demos — not inside Fate's loop),
`blondle` (static STEM knowledge packs loaded into the KnowledgeBase),
`search`/`conjecture*` (constant-search and conjecture-verification tooling
used by research binaries).

## 1. Fate fast path — `explore()` / `explore_with_domain()`

Per call (`state.tick += 1`):

1. **Structure extraction** (`extract_structure`): difference tables, ratios,
   period, structure type, consistency score. The structure is stored for
   later cross-domain comparison.
2. **Known-sequence check** (`check_known_sequences`): exact prefix match,
   then subsequence-offset match, then closed-form match against
   `kb.known_sequences` (130 built-ins; OEIS b-files can be merged via
   `OeisLoader`). Hit ⇒ `KnownSequence`, **`is_discovery` is always false**,
   early return. (Known issue: no minimum-length guard — very short signals
   can match vacuously.)
3. **Algebraic exploration** (`try_algebraic_exploration`), fixed order:
   ① recurrence relations → ② closed forms → ③ algebraic identities →
   ④ equation solving (arithmetic / geometric / quadratic / cubic-Δ³
   detection) → ⑤ rational functions → ⑥ multiplicative structure.
   First hit wins. On this fast path a hit is returned immediately as
   `EmergentLaw` + `is_discovery = true` **without gating** — callers who
   need rigor must use §2.
4. **Pattern-class loop**: KnowledgeBase pattern classes in strategy-priority
   order (constant-difference/Newton, ratio, product, periodic, factorial,
   order-2 recurrence). Each fit is checked for non-triviality
   (**DoF = n − (degree+1) ≥ 2**); trivial fits are excluded from discovery
   statistics. Non-trivial fits are compared against the signal pool for
   cross-domain isomorphism / generalization, and may create a `NewRule`
   candidate (repeat sightings merge and raise `support_count`).
5. Otherwise: `Unknown`.

Persistent per-`Fate` state across calls: exploration history, discoveries,
**signal pool** (all processed signals + structures), **candidate pool**,
and **strategy statistics** (per-pattern success rates that re-order step 4 —
the engine's only learning loop). `enable_meta` exists in config but is
not implemented (`false`).

## 2. Verified path — `explore_with_verification()` (the gated path)

Wraps §1. Only when a pattern was detected:

1. **Fit analysis**: parameter count estimated from the formula, degree,
   DoF; classifies `InterpolantFit` (DoF ≤ 2) vs `DescriptiveModel`.
2. **Cross-domain verification** (`verify_cross_domain`): predicts further
   points from the fitted formula and checks them for self-consistency
   (this is internal-consistency extrapolation, not external holdout data).
3. **Upgrade rules**: DoF > 0 + extrapolation pass ⇒ `EmergentLaw`
   (discovery candidate); DoF = 0 + pass ⇒ `CandidateLaw`; fail ⇒ stays
   descriptive. Novelty is checked against the KnowledgeBase.
4. **Auto-archive**: discoveries enter Fate's candidate pool with a
   DoF-based confidence (0.7 / 0.8 / 0.9). A Space container record is
   created per discovery (metadata only; not yet persistent).
5. **Discovery assessment**: six-way status
   (Known / TrivialInterpolation / DescriptiveOnly / CandidateLaw /
   StrongCandidate / GenuineDiscovery) with negative evidence
   (OEIS miss count — computed live — DoF, extrapolation result).

`promote_candidate_to_knowledge(id, kb)` moves a candidate into the KB as a
new pattern class (gate: confidence ≥ 0.8 and support ≥ 3). The KB
`PendingReview` queue (§4) is a separate track, fed by explicit
`add_discovery` calls; wiring Fate's pool into it is roadmap M1.

## 3. Self-loop — `explore_with_self_loop()` (default max 1,000 rounds)

Wraps §2 and keeps going when the answer is not settled:

- `KnownSequence` or non-trivial pattern ⇒ stop immediately.
- `InterpolantFit` ⇒ Space-guided dynamic transforms (signal analysis →
  prioritized transform list) and retry.
- `Unknown` ⇒ fixed-order transform chain: **Δ, Δ², Σ, |·|, n·f(n), ratio,
  f², sgn, MA(3), Δ³, pairwise product, mod 10**. Each produced signal is
  signature-checked against all previously seen signals (closure = no new
  signal can be produced).
- All transforms exhausted ⇒ **MYSTERY** (`truly_exhausted`), with the
  tested hypothesis space H and transform set T self-reported in
  `SelfLoopStats`.

Returns the **last** round's result plus full history. Note: formula-based
signal *extension* is not wired in (the helper exists, uncalled); the loop
extends coverage by transformation, not by generating new points.

## 4. Integrated path — `full_exploration()`

Runs §2, Voyager detection, Voyager dimension management, and Space
fractional-dimension estimation over the same signal, then an OEIS
**post**-check (labels only, never blocks), then merges everything into a
`FinalStatus`. Inputs are converted EPR → i128 rationals for Voyager/Space;
values beyond i128 currently degrade silently (known limitation A5 in the
pitch appendix).

**Voyager** (`PreciseSignalDetector`): checks the signal against its
equation archive (starts empty; only equations it creates itself), creates
a new equation with a full derivation trace, and archives it. Used by the
interface runtime to emit answer / reverse-verification / derivation files.

## 5. KnowledgeBase runtime

`initialize_fundamentals()` loads: 20 physical constants (CODATA 2022,
stored as reference strings), 15 math constants, 81 theorems, 60 physical
laws, pattern classes, and 130 known sequences. `data/oeis/` (183 b-files)
merges via `OeisLoader` on demand. Blondle packs add 588 entries across
14 domains. Review track: `add_discovery` → `PendingReview` →
`verify_discovery` → JSON persistence / Tex export for human review.

## 6. Arithmetic substrate

- `EPR` (ExtendedInt numerator/denominator): exact rational arithmetic;
  fuzz-verified against Python `Fraction` (2,675 cases). Known bug:
  `divmod` shortcut corrupts values in [2¹²⁷, 2¹²⁸) (pitch appendix A1).
- `Rational` (i128): used by Space/Quad; `VoyagerRational` (i128) by Voyager.
