# INTENT - signal-standard

`signal-standard` is the second non-component shared `signal-` library,
alongside `signal-frame`. Per Spirit `eeeo` (the new-shared-schema decision)
and Spirit `t312` (the signal-namespace partition).

Where `signal-frame` owns domain-free **wire mechanics** — short headers,
exchange and stream identifiers, request/reply/stream envelopes, length-prefixed
rkyv archive helpers — `signal-standard` owns domain-free **cross-component
standards**: the vocabulary every component conforms to. Its first and central
content is the cross-component classification.

It owns:

- **`ComponentKind`** — the reconciled cross-component census. One closed,
  partitioned roster of 14 variants, born from collapsing the two prior local
  rosters (`signal-persona`'s 9 and `signal-criome`'s 7, overlapping on
  `Spirit` and `Router`) and cross-checking `active-repositories.md`. The enum
  is **closed-but-partitioned** into five documented zones — Core, Messaging,
  Interaction, Platform, Aggregate — each with reserved room to grow, modeled
  on the `t312` namespace partition. Adding a component is a local insert inside
  its zone, not a workspace-wide rebuild; the closed enum still type-checks
  every consumer.
- **`Differentiator`** — a component distinguished cross-system by which
  `ComponentKind` it is and which `AuthorizedObjectKind` it acts over.
- **`AuthorizedObjectKind`** and the **`AuthorizedObjectInterest`** lattice —
  the four-rung interest narrowing (any / by-component / by-object-kind /
  by-component-object) every subscriber filters against, lifted out of
  `signal-criome` where it was born local.
- **`ComponentClassification`** — the small embeddable nameplate a daemon
  stamps onto its frames so peers classify it without a lookup: the
  differentiator plus the interest it advertises.
- **`StandardSocket`** — the shared typed connection point for local Unix
  sockets and off-host network endpoints. Components use it when they need to
  name ordinary daemon reachability without hard-coding path strings or port
  fields in each contract. The first motivating consumer is Mentci connecting
  to the local criome socket.

Scope discipline (`eeeo`): only **genuine cross-component standards** live here,
not a grab-bag of conveniences. It is not `signal-system` (one component's own
contract) and not `signal-frame` (domain-free frame mechanics, not domain
vocabulary).

This crate is a **pure vocabulary library**: no operation roots, no daemon, no
storage, no wire codec. It lowers through the schema declaration-module emission
target (not the wire-contract target), so the generated module is types and
their inherent/`From` support only — no `Input`/`Output` roots, no frame codec.
Component contracts import these types and reference them inside their own roots,
each retiring its local copy:

```text
{ ComponentKind signal-standard:lib:ComponentKind }
```

The default crate is binary rkyv only. NOTA text projection is the explicit
`nota-text` feature for thin CLIs and human/agent edges; daemons depend on this
crate without compiling a NOTA parser.

The consumer migration — `signal-criome` and `signal-persona` deleting their
local `ComponentKind` / interest-lattice declarations and importing from here —
is a separate, coordinated breaking change (per the no-backward-compat
override: both consumers rebuild at once against the reconciled 14-variant
roster). It is not yet performed.
