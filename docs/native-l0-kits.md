# Source-derived L0 theme and component kits

Taskplan, Atro and Camo have registered token catalogs, typed L0 component
libraries, and source-linked screen instances. The native Makepad implementation
is shared in `octoscript-widgets::design` and `octoscript-widgets::kit`; register both
`script_mod` functions after the base widgets when hosting these packs.

Each theme directory contains:

- `kit.json`: the runtime registry, semantic roles, typed properties, child-slot
  contracts, reusable compound definitions, and source/reuse receipts.
- `tokens.json`: exact measured colors, font resources, type styles, radii,
  effects and common spacing values. Semantic aliases retain source receipts.
- `components.l0`: primitive variants and repeated compound compositions.
- `roles.l0`: shorter public component names for the most-used role variants.

Recognized complete compositions now mount dedicated native widget types:

| Native widget | Public L0 roles | Owned behavior |
| --- | --- | --- |
| `KitButton` | `TaskplanButton`, `AtroButton`, `CamoButton` | Native Button activation, enabled state, complete label/icon/paint composition |
| `KitFormField` | `<Family>FormField` | Native TextInput value, content updates and change actions |
| `KitTabBar` | `<Family>TabBar` | Selected index, exclusive native radio selection, selection actions and source text/background/underline colors |
| `KitBottomNavigation` | `CamoBottomNavigation` | Active item and navigation actions from native child controls |
| `TaskplanProjectCard` | `TaskplanProjectCard` | Title/date/status content, native card activation |
| `CamoTrackRow` | `CamoTrackRow` | Title/artist content, native row activation and named icon actions |

The `semantic_roles` registry selects complete shared definitions. Public Button
roles use those complete definitions; the separate primitive hit target is
available as `<Family>ButtonControl`. Source-specific variants remain in
`components.l0`. A clipped symbol with only a background remaining is not
promoted to a semantic card.

Each semantic call takes one `instance` plus named content/state parameters.
For example, `TaskplanFormField` exposes `label`, `value`, `placeholder`,
`focused`, `password` and `enabled`; navigation exposes `selected_index`.
The shared definition declares its complete hierarchy with local `part` names.
Host data binds `$kit.instances[instance].parts` to the original Sketch widget
IDs, and `$kit.placements` supplies each part's checked component and frame.
Unknown parts, mismatched root identities and duplicate realized IDs fail.
The ledger never needs to pass a separate identity for every inner layer.

Native `*Ref` APIs expose `set_content(role, text)`, `activated(actions)`,
`changed(actions)`, `select(index)` and `selected(actions)` as applicable.
Track rows also expose `action(actions)` for source icon names such as `more`
and `cloud_download`. `Widget::text`/`set_text` address the primary title or
editable value. Native child actions remain available to application consumers.
These actions are component events; the application decides which route,
download or playback operation they trigger.

Available themes are `taskplan_light`, `atro`, `atro_light`, `camo`, and
`camo_light`. Atro and Camo's two modes answer the same variant identifiers.
Taskplan's source has one light theme; no invented dark reference is claimed.
Source-specific variant IDs preserve their measured styles. Use the selected
mode's `roles.l0` when choosing its default variants; changing only a card's
theme does not recolor a variant that explicitly references source-color tokens.

The generated screen cards are ordinary L0 ledgers with `theme`, `copy`, typed
state, component declarations and a root view. Each card includes the exact
subset of shared component declarations it uses, so the L0 checker can validate
it without an implicit import. Geometry and source assets are supplied in its
`.data.json` under `$kit.placements`. Every generated widget is linked to its
Sketch owner in the companion `.l0map.json`.

For new cards, include the needed definitions from `roles.l0` or
`components.l0`, instantiate them with content/state props and child slots, and
provide source-linked placements through host data. There is no `kit camo`
declaration and no arbitrary Makepad widget or shader source inside L0.

The runtime path is:

```text
L0 ledger + host data
  → checked realization of component props, state and slots
  → theme registry + token resolution + checked placement bindings
  → portable native widget tree
  → octoscript-makepad design translator
  → octoscript-widgets native controls, views, labels, SVG and images
```

Unknown components/tokens/props, mismatched placements, duplicate instance IDs,
and unsupported theme-axis overrides fail the entire card. Standard L0 roles
such as `TextBody` also receive the imported theme's source font and measured
type scale through the file-backed L0 loader.

Use `octoscript_makepad::l0::prepare(card, data, kit_dir)` and select the native design
translator when `PreparedCard.native_components` is true. The beauty host and
the `beauty_check` example exercise this path. The legacy direct L0-to-widget
dialect has no pack registry and reports that a registered kit host is required.

The supplied screen layouts preserve the source artboard geometry. Component
reuse, token resolution and native controls do not by themselves establish
responsive page layout or complete application workflows.

Some source controls combine a native hit/state widget with separate source
labels and paint inside the dedicated semantic widget. Taskplan tabs and
previously passive Atro tabs use native RadioButton selection because Makepad's
Tab is not a standalone Widget. Their source paint remains in the compound. Click
validation uses the measured visible portion of a clipped control; fully clipped
controls receive state inspection without a claimed click test.

Regenerate with `lab/sketch/promote_l0.py --kit <family>-native-all`. Validate
the resulting `<family>-l0-all` configuration with native Studio capture,
structural inspection, `gate_kit.py`, native composition, and screenshot review.
See the lab's beauty loop documentation for the complete repair cycle.

The semantic gate requires the actual native widget type for every promoted
boundary. `.semantic-interactions.json` records Studio selection/action probes,
field value ownership, card/row content setters and restoration. The offline
tree audit preserves every original source property and child; its only allowed
additions are explicitly mapped, transparent native action/selection controls.
Progress artwork and member images still follow the supplied source assets;
changing `progress_text` alone does not recompute a chart. Responsive layout,
dynamic collection layout and full application workflows need separate work.
Content setter probes verify that values reach the owned native control or
label. They do not establish layout for arbitrary replacement copy: a longer
track title can exceed the source text allocation and overlap an adjacent icon.
New content lengths require suitable placement/overflow rules and layout tests.
