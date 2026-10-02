# Working in Octoscript-Makepad

Read [README.md](README.md) and the
[code walkthrough](docs/architecture-walkthrough.md) before changing the runtime.

- This repository owns portable UI nodes, checked DSL evaluation, Makepad
  translation, native themed widgets and preview/catalog hosts. App installation,
  persistent app stores and system/app agents belong to consuming hosts.
- `runtime.json` and root `Cargo.toml` select the source set. Keep Makepad and
  OctoScript pins consistent, preserve one Makepad instance per resolved graph,
  and do not move a consumer's dependency pin as an incidental fix.
- `octoscript-node` deliberately has no dependencies. Keep its portable boundary
  free of VM, platform and agent dependencies.
- Trace L0 realization, native kit lowering and measured-design translation
  separately. Malformed or budget-exhausted input must fail as a whole.
- The current `kit-host` and `beauty-host` mount a `View` evaluated on the main
  VM into `Splash.view`. Do not repeat historical claims that they must wait
  for an `isolate: false` field or currently use `Splash::set_text`.
- Keep UI state, app persistence and agent memory distinct in code and docs.
  Capabilities exist only when a host supplies them. A preview is not a
  production app service or an agent permission grant.
- For render changes run appropriate portable tests, normally
  `cargo test --release -p octoscript-node -p octoscript-render -p octoscript-makepad`.
  Widget/layout changes also need native acceptance with `beauty-host --remote`;
  use actual measurements and screenshots. Record unavailable GPU/device checks
  honestly. Do not label structural tests as visual acceptance.
- Keep English and Chinese README architecture/build claims consistent. Use
  symbol/file links and label historical observations and unrun recipes.
