# Agent Instructions

Read `/home/li/primary/AGENTS.md` first, then this repo's `INTENT.md`.

This repository is a Signal standards library — the second shared `signal-`
crate alongside `signal-frame`. Before editing, read:

- `/home/li/primary/skills/contract-repo.md`
- `/home/li/primary/skills/rust-discipline.md`
- `/home/li/primary/skills/nix-discipline.md`
- `/home/li/primary/skills/naming.md`

This crate owns only **genuine cross-component standards**: the reconciled
`ComponentKind` roster, the `Differentiator`, the `AuthorizedObjectInterest`
lattice, and the `ComponentClassification` nameplate. Per Spirit `eeeo`, keep
the scope narrow — it is not a grab-bag of conveniences, not a component's own
contract, and not frame mechanics.

It is a pure vocabulary library. Do not add operation roots, daemon actors,
sockets, redb tables, daemon clients, runtime policy, or a wire codec. The
schema lowers through the **declaration-module** emission target (`build.rs`),
not the wire-contract target. Regenerate the checked-in `src/schema/lib.rs`
with `SIGNAL_STANDARD_UPDATE_SCHEMA_ARTIFACTS=1 cargo build`.

`ComponentKind` is closed-but-partitioned: when a real new component appears,
insert it into the right zone's reserved room — do not append blindly or
repartition the zones (a repartition is a major-version event, per `t312`).
