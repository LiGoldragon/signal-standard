# signal-standard — architecture

## Overview

`signal-standard` is the second non-component shared `signal-` library, alongside
`signal-frame`. Where `signal-frame` owns domain-free **wire mechanics** — short
headers, exchange and stream identifiers, request/reply/stream envelopes, and
length-prefixed rkyv archive helpers — `signal-standard` owns domain-free
**cross-component standards**: the vocabulary every component conforms to. The
direction traces to Spirit `eeeo` (the new-shared-schema decision) and Spirit
`t312` (the signal-namespace partition).

It is a pure vocabulary library. Its first and central content is the
cross-component classification.

## Owned vocabulary

- **`ComponentKind`** — the reconciled cross-component census: one closed,
  partitioned roster of 14 variants, born from collapsing the two prior local
  rosters (`signal-persona`'s 9 and `signal-criome`'s 7, overlapping on `Spirit`
  and `Router`) and cross-checking `active-repositories.md`. The enum is
  **closed-but-partitioned** into five documented zones — Core, Messaging,
  Interaction, Platform, Aggregate — each with reserved room to grow, modeled on
  the `t312` namespace partition.
- **`Differentiator`** — a component distinguished cross-system by which
  `ComponentKind` it is and which `AuthorizedObjectKind` it acts over.
- **`AuthorizedObjectKind`** and the **`AuthorizedObjectInterest`** lattice — the
  four-rung interest narrowing (any / by-component / by-object-kind /
  by-component-object) every subscriber filters against, lifted out of
  `signal-criome` where it was born local.
- **`ComponentClassification`** — the small embeddable nameplate a daemon stamps
  onto its frames so peers classify it without a lookup: the differentiator plus
  the interest it advertises.
- **`StandardSocket`** — the shared typed connection point for local Unix sockets
  and off-host network endpoints. Components use it when they need to name
  ordinary daemon reachability without hard-coding path strings or port fields in
  each contract. The first motivating consumer is Mentci connecting to the local
  criome socket.

## Scope discipline

Per Spirit `eeeo`, only **genuine cross-component standards** live here, not a
grab-bag of conveniences. This crate is not `signal-system` (one component's own
contract) and not `signal-frame` (domain-free frame mechanics, not domain
vocabulary).

## Emission and build shape

This crate is a **pure vocabulary library**: no operation roots, no daemon, no
storage, no wire codec. Its TrueSchema source lowers through the
**declaration-module** emission target (not the wire-contract target), so the
generated module is types and their inherent/`From` support only — no
`Input`/`Output` roots and no frame codec.

- `schema/lib.schema` is the TrueSchema source.
- `src/schema/lib.rs` is the checked-in generated declaration module; regenerate
  it with `SIGNAL_STANDARD_UPDATE_SCHEMA_ARTIFACTS=1 cargo build` (driven by
  `build.rs`).
- The default crate is binary rkyv only. NOTA text projection is the explicit
  `nota-text` feature for thin CLIs and human/agent edges; daemons depend on this
  crate without compiling a NOTA parser.

## Consumption

Component contracts import these types and reference them inside their own roots,
each retiring its local copy:

```text
{ ComponentKind signal-standard:lib:ComponentKind }
```

## Constraints

- `ComponentKind` is closed-but-partitioned: when a real new component appears,
  insert it into the right zone's reserved room. Do not append blindly and do not
  repartition the zones — a repartition is a major-version event (`t312`).
- Keep the scope to genuine cross-component standards (`eeeo`); do not accumulate
  conveniences.
- Add no operation roots, daemon actors, sockets, redb tables, daemon clients,
  runtime policy, or wire codec. The TrueSchema source lowers through the
  declaration-module target, never the wire-contract target.

## Migration status

The consumer migration — `signal-criome` and `signal-persona` deleting their
local `ComponentKind` / interest-lattice declarations and importing from here — is
a separate, coordinated breaking change. Per the no-backward-compat override,
both consumers rebuild at once against the reconciled 14-variant roster. It is not
yet performed.

## Code map

- `schema/lib.schema` — TrueSchema source of the vocabulary.
- `src/schema/lib.rs`, `src/schema/mod.rs` — checked-in generated declaration
  module.
- `src/lib.rs` — crate root re-exporting the vocabulary and its inherent/`From`
  support.
- `build.rs` — declaration-module emission driver.
- `tests/round_trip.rs` — round-trip test, gated on the `nota-text` feature.
