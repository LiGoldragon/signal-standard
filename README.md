# signal-standard

Shared cross-component standards for the primary workspace: the vocabulary
every component conforms to.

`signal-standard` is the second shared `signal-` library, alongside
`signal-frame`. Where `signal-frame` owns domain-free wire mechanics (headers,
envelopes, stream tokens), `signal-standard` owns the domain-free
cross-component classification:

- `ComponentKind` — the reconciled, closed-but-partitioned roster of every
  component (14 variants across five documented zones, each with reserved room
  to grow).
- `Differentiator` — which `ComponentKind` a component is and which
  `AuthorizedObjectKind` it acts over.
- `AuthorizedObjectKind` and the `AuthorizedObjectInterest` lattice — the
  four-rung interest narrowing subscribers filter against.
- `ComponentClassification` — the small embeddable nameplate a daemon stamps
  onto its frames so peers classify it without a lookup.

It is a pure vocabulary library: no operation roots, no daemon, no storage, no
wire codec. The types are emitted from `schema/lib.schema` through the schema
declaration-module target. Component contracts import these types and reference
them inside their own roots:

```text
{ ComponentKind signal-standard:lib:ComponentKind }
```

The default crate is binary rkyv only. The `nota-text` feature adds NOTA text
projection for thin CLIs and human/agent edges.
