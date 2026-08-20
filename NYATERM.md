# NyaTerm fork notes

This branch carries [NyaTerm](https://github.com/nyakang/nyaterm)'s local
change to `zmodem2` on top of an unmodified upstream base.

- Fork: <https://github.com/nyakang/zmodem2>
- Upstream: <https://codeberg.org/jarkko/zmodem2>
- Base revision: `fc6b0fd9bbf5348b2fac00ecb22bc9e40f4251f5` (`0.7.2`)
- Branch: `nyaterm`

No library source is modified: the only patch is to `build.rs`.

## Patches

1. `build: keep the lrzsz probe silent when lrzsz is absent` — `has_lrzsz` gates
   this crate's own integration tests only, so upstream's
   `cargo:warning=lrzsz not found` prints on every build of every dependent for
   no reachable reason, and lrzsz is not normally installed on Windows or macOS.
   Probing, the `has_lrzsz` cfg and the `ZMODEM_RZ_BIN` / `ZMODEM_SZ_BIN`
   emissions are unchanged, so the integration tests still gate correctly where
   lrzsz is installed.

## Not carried here

- `feat(sender): add a ZFILE management-option setter` and its revert. The pair
  had a net effect of zero — the value was stored but never transmitted, and
  NyaTerm never called the setter — so the rebase off `0.5.0` dropped both rather
  than replaying a feature and its undo.
- `build: use if-let for the lrzsz probe result` is folded into patch 1: it only
  existed because emptying the absent-lrzsz arm made clippy's `single_match`
  fire, which is the same concern.
- NyaTerm adds no vendor-layout changes. The old `vendor/zmodem2` snapshot in the
  NyaTerm tree additionally dropped upstream's `.woodpecker` and `release.sh` and
  added crates.io packaging artifacts (`.cargo-ok`, `.cargo_vcs_info.json`,
  normalized `Cargo.toml`, `Cargo.toml.orig`); those are vendoring artifacts, not
  patches, and are deliberately absent from this branch.

## Note for consumers moving from 0.5.0

`0.7` replaced the 0.5 drain/advance/poll stream API and the 0.6 step/effect API
with a single caller-driven `poll` / `submit_*` model (upstream `df41f47`). One
ordering detail is not obvious from the API docs and is easy to get wrong:
`poll` reports pending *events* before pending wire bytes, and both
`Receiver::handle_header` and the sender queue their closing ZFIN *before*
pushing `SessionCompleted`. A consumer that tears the session down as soon as it
sees `SessionCompleted` therefore never writes that ZFIN, and the peer waits for
a frame that never arrives. Drain the remaining `Action::WriteWire` after a
terminal event.

## Validation

On Windows 11:

```sh
cargo fmt --all -- --check   # clean
cargo check                  # clean
cargo test                   # 35 passed (the lrzsz integration tests are
                             # has_lrzsz-gated and skipped here)
```

The `has_lrzsz` integration tests against real `rz`/`sz` run in
`.github/workflows/nyaterm.yml`, which installs lrzsz first.
