# The semantic widget vocabulary — what a generator may write

The contract for authoring a catalog screen. Derived from what the renderers
actually read (`splash-makepad/src/material.rs`, Splash-Android `Builder.java`),
not from what they could. **Anything not listed here is silently dropped — an
unknown `t:` produces no node, an unknown attribute is ignored, an unknown
variant falls back to a default. Nothing warns. Stay inside this page.**

## The model

A screen is one Splash-DSL expression evaluating to a tree of plain objects:

    {t:"<kind>", <attrs>, c:[ <children> ]}

You name **what a thing is and what it means** — kind, variant, content, state.
You never name **how it looks**: no colours, no fonts, no dp sizes, no padding.
The renderer owns Material 3 styling per backend. A screen that writes `bg:` or
`size:` is wrong even if it renders.

## State — the whole interaction model

There is one global store of number slots, read by name, written by taps.

- **read**: `N("slot", default)` → number. `S("slot")` → string ("" unset).
- **write**: interactive widgets take `key:"slot"` and write it themselves
  (chip/checkbox/toggle flip 0↔1, segmented writes the index, slider the value).
- After every write the whole screen re-evaluates, so any expression built from
  `N(...)` — a label, a price, an `if` — updates by itself. Compute freely in
  `fn`s; the DSL has `if`/`for`/`let`, string `+`, arithmetic.
- Concatenation must START with a string: `"x: " + N(...)` is text,
  `N(...) + " x"` is arithmetic against a word.
- A slot name is global to the app: prefix yours (`brew_size`, not `size`).

## Layout containers

| t | attrs | notes |
|---|---|---|
| `scroll` | — | page root, exactly one, direct child of nothing |
| `col` / `row` | `spacing`, `pad` | pad only on the page-level col |
| `flow` | `spacing` | a row that wraps |
| `stack` | — | children overlay |
| `spacer` | `h` or `w` | fixed empty space |

Helpers already in scope (from the kit): `section(title)`, `caption(text)`,
`group(kids)` (a wrapping row).

**The screen's final expression must be a LITERAL object, not a function call.**
A call as the module result evaluates to nothing and the screen renders blank —
no error anywhere. End every screen with the literal root, calls inside it:

    {t:"scroll", c:[ {t:"col", pad: 16, spacing: 16, c: my_body()} ]}

## Text

`{t:"text", variant:"<role>", text:"..."}` — variant is an M3 type role:
`displayLarge|displayMedium|displaySmall|headlineLarge|headlineMedium|headlineSmall|titleLarge|titleMedium|titleSmall|bodyLarge|bodyMedium|bodySmall|labelLarge|labelMedium|labelSmall`.
No size, no weight, no color.

## Components

| t | required | optional | behaviour |
|---|---|---|---|
| `button` | `label`, `variant:` `elevated\|filled\|tonal\|outlined\|text` | `icon`, `key`, `enabled:0` | tap writes `key` (0↔1) |
| `chip` | `text`, `variant:` `assist\|filter\|input\|suggestion` | `icon`, `key`, `on: N(...)`, `enabled:0` | filter chips show a check when `on`; tap flips `key` |
| `checkbox` | `text`, `key`, `on: N(...)` | `indeterminate:1`, `enabled:0` | tap flips `key` |
| `toggle` | `key`, `on: N(...)` | `enabled:0` | a switch; tap flips `key` |
| `radio` | `text`, `key`, `on:` | `enabled:0` | one `key` per option; you keep them exclusive |
| `segmented` | `items:"A;B;C"`, `key`, `selected: N(...)` | — | joined single-select; tap writes the index |
| `slider` | `key`, `value: N(...)`, `min`, `max` | `step`, `enabled:0` | drag writes `key` |
| `input` | `hint` | `key`, `helper`, `error`, `enabled:0` | text field; typed text lands in `S(key)` |
| `listitem` | `label` | `supporting`, `icon`, `lines:1..3`, `action:"switch"\|"more"`, `on`, `key` | M3 list row; `action:"switch"` puts a switch at the trailing edge |
| `card` | children `c:` | `variant:` `elevated\|filled\|outlined`, `title` | a container with M3 surface treatment |
| `divider` | — | `variant:"inset"` | hairline rule |
| `fab` | `icon` | `variant:` `small\|regular\|large\|extended`, `label` (extended), `key` | floating action button |
| `badgeicon` | `icon` | `count` | icon with a count badge |
| `progress` | — | `value:0..100` (absent = indeterminate) | spinner / bar |
| `tabs` | `items:"A;B"`, `key`, `selected: N(...)` | `variant:"scrollable"` | tab strip |
| `searchbar` | `hint` | `key` | M3 search bar |
| `dropdown` | `items:"A;B"`, `key`, `selected: N(...)` | `label` | exposed dropdown menu |

`icon` takes a **name** — the set used by the screens: `add share bookmark
place call check close star favorite settings search more menu home person
delete edit info warning`.

## Rules that are easy to break

1. One `scroll` at the root, everything inside it. A `fill` inside the host's
   `Fit` mount collapses the screen to nothing.
2. Every interactive widget needs BOTH `key:` (where taps write) and the
   matching read (`on:`/`selected:`/`value:` from `N(...)`) — the widget does
   not remember itself; the store does. A control missing its read renders but
   snaps back on the next tap.
3. Radio exclusivity is yours: on tap a radio writes its own key 0↔1; to make a
   group, give each option its own slot and derive the winner in DSL, or use
   `segmented`, which is exclusive by construction.
4. Text goes through `variant:` roles only. If you reach for `size:` you are
   styling, and the styling is not yours.
5. Everything unknown fails silently. If a widget you want is not in the table,
   compose it from listed parts — do not invent a `t:`.
