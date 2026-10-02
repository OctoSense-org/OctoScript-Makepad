# Working in OctoScript-Makepad

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
  separately. Preserve whole-tree failure for malformed or budget-exhausted
  input in `octoscript_render::build`. The final widget-source evaluation uses
  `eval_checked` in beauty-host, but `eval_with_append_source` in kit-host;
  keep their budget descriptions distinct.
- The current `kit-host` and `beauty-host` mount a `View` evaluated on the main
  VM into `Splash.view`.
- Keep UI state, app persistence and agent memory distinct in code and docs.
  Capabilities exist only when a host supplies them. A preview is not a
  production app service or an agent permission grant.
- For render changes run appropriate portable tests, normally
  `cargo test --release -p octoscript-node -p octoscript-render -p octoscript-makepad`.
  Widget/layout changes also need native acceptance with `beauty-host --remote`;
  use actual measurements and screenshots. If you could not run the native
  check, say so in the PR.
- Keep English and Chinese README architecture/build claims consistent. Use
  source links and keep operational instructions separate from PR validation notes.
