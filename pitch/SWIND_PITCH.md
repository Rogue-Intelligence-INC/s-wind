# S-Wind

**An exact-arithmetic engine that proposes, tests, and falsifies mathematical conjectures — with a human in the loop.**

Rogue Intelligence LNC — Proprietary & Confidential

---

## One-Liner

S-wind takes a sequence of numbers and answers three questions, exactly:

1. **Is it already known?** — matched against 130 built-in OEIS sequences (183 OEIS b-files loadable)
2. **If not, what closed form fits it?** — algebraic fitting over ℚ; no floating point in the core engine
3. **Does the candidate survive testing?** — extrapolation gates, complexity gating, self-falsification

Surviving candidates enter a **pending-review queue**. S-wind proposes; humans dispose.

---

## The Problem

Automated science has a credibility gap:

- Pattern miners produce correlations that collapse out-of-sample.
- Generative AI produces plausible-but-unverifiable claims at scale.
- Almost no tooling is built to **kill its own hypotheses cheaply** and keep an audit trail.

The bottleneck in conjecture-driven research is not generating ideas — it is **triaging** them: exact checking, known-result matching, and falsification, before a human spends an hour.

---

## What S-Wind Does (verified 2026-10-30)

| Capability | Status |
|---|---|
| Exact rational arithmetic (EPR) | Fuzz-tested against a Python big-integer oracle: 2,675 cases, zero failures (one boundary bug at 2¹²⁷ disclosed in appendix; fix designed) |
| Algebraic pattern fitting | Arithmetic / geometric / recurrence / polynomial / reciprocal / periodic families fit correctly on probe suite |
| Known-sequence recognition | 130 built-in sequences recognized instantly and **never counted as discoveries** |
| Continued exploration | Unknown signals are transformed (12 transforms) and retried — 16 iterations on prime gaps before declaring exhaustion ("MYSTERY") |
| Candidate review workflow | Candidates land as `PendingReview`; human verifies; JSON persistence round-trips losslessly |
| Deterministic & self-contained | 56/56 unit tests pass; repository builds standalone (`cargo test`); fixed-seed evaluation is bit-reproducible |

---

## The Falsification Engine

We pointed S-wind at our own favourite conjecture — "a unique quartic polynomial explains α⁻¹ ≈ 137":

- **146,410** candidate polynomials tested
- **146,317 falsified** by the system's own exhaustive search
- The tool's own printed conclusion: *uniqueness fails; the pattern is not special*

Blind tests and negative controls (Möbius, random sequences) ship in the repository and **reject** over-claims.

**We sell the referee, not the oracle.** A conjecture engine whose first job is to disprove is the one researchers can trust to pre-screen at scale.

---

## How It Works

```
signal ──► known-sequence match (OEIS) ──► algebraic fitting (ℚ, exact)
              │ known                          │ candidate formula
              ▼                                ▼
         labelled,                       verification gates:
         not a discovery                 extrapolation · DoF · novelty
                                               │
                              ┌────────────────┴────────────────┐
                              ▼ fail                            ▼ pass
                        transformed & retried            PENDING REVIEW
                        (to exhaustion →                 (human verifies
                         "MYSTERY", logged)               or rejects)
```

Every step logs its derivation. No silent answers.

---

## What S-Wind Is Not (disclosed up front)

- It has **not** discovered a validated physical law. Earlier "discovery" claims did not survive our own falsification runs and are withdrawn.
- Verification gates have known bugs (disclosed in appendix) — candidates are therefore labelled candidates, never conclusions.
- Earlier generated paper catalogues contained simulated optima; they are **not** research output and are excluded from this deck.

We would rather show you the list of what it killed than a list of what it "found".

---

## Market

**Beachhead: conjecture triage for research & engineering teams**

- Mathematics / physics groups: sequence mining, identity hunting, pre-screening before human proof effort
- Engineering R&D: exact parameter-space screening with reproducible, auditable runs
- Verification-adjacent tooling: deterministic symbolic checking where f64 pipelines are unacceptable

Expansion: symbolic regression as a service; curated candidate feeds with human-verified labels (data asset).

---

## Business Model (proprietary licensing)

S-wind is closed-source, licensed commercially:

1. **Research licenses** — academic groups (evaluation terms in `LICENSE.md`)
2. **Enterprise licenses & API** — industrial R&D screening pipelines
3. **Custom exploration engagements** — bespoke conjecture-mining runs with audit trails
4. **Verified-candidate data products** — human-reviewed candidate feeds (post-M1)

---

## Traction (engineering, honestly stated)

- 64k-line Rust codebase; 87 runnable binaries; development continuous since 2026-01
- Core engine test-verified (56/56); review workflow demonstrated end-to-end
- Self-falsification suite ships in-repo (`swind_unified_proof`)
- Hardening completed 2026-10-30: zero machine-specific paths, all reported counts computed at runtime, deterministic CLI evaluation, self-contained build

---

## Roadmap (what funding buys)

| Milestone | Deliverable | Why it matters |
|---|---|---|
| M1 | Connect Fate candidate pool → pending-review queue (persistence + dedup) | Closes the propose→review loop end-to-end |
| M2 | Rewrite verification gates on structured formulas (Expr), hold-out validation | Candidates become trustworthy enough to price |
| M3 | Fix disclosed arithmetic boundary bug; property-based fuzz in CI | "Exact" becomes an audited claim |
| M4 | Independent replication package (pinned commit, one-command build, public benchmark) | Third parties can check every number in this deck |

---

## Team

**Rogue Intelligence LNC**

- Research lab behind the Valhalla programme (S-wind, Wave, Valhalla Unified Core)
- Working style: negative controls first, claims second — this deck is an example

---

## The Ask

**Seed: $2M, milestone-tranched against M1–M4.**

- 40% Engineering (verification rewrite, pipeline closure)
- 30% Independent validation partnerships
- 20% Infrastructure (benchmark & replication hosting)
- 10% Operations

**Contact**: licensing@rogue-intelligence.com

---

## Appendix — Known Limitations (full disclosure)

1. `ExtendedInt` division/display bug for values in [2¹²⁷, 2¹²⁸) (affects EPR factorials ≥ 34!); root-caused, fix designed (M3).
2. Cross-domain verification currently branches on formula strings; structured rewrite is M2.
3. Known-sequence matching lacks a minimum-length guard (empty signal vacuously "matches"); fix queued with M2.
4. Self-loop signal *extension* described in a 2026-01 internal experiment is not wired in current code; that experiment's results are not reproducible on HEAD and are not claimed here.
5. Space "containers" are currently per-discovery metadata records, not persistent exploration contexts (persistence layer is M1).

*Reproducibility: clone this repository, run `cargo test --release` (56/56). Every number in this deck traces to a test, a probe, or a shipped binary.*

---

**S-wind: the conjecture refinery.**
© 2026 Rogue Intelligence LNC. All Rights Reserved. Proprietary and confidential — do not redistribute.
