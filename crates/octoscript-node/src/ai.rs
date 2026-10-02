//! The AI-written mark (`ui-profile-l0.md` §4.2).
//!
//! An L0 card may now show words the model wrote — a digest summary, a draft,
//! the agent's side of a chat — and every such text slot is marked: the kit's
//! `l0_ai_text(node)` stamps `ai: 1` on the node. This module is what that
//! stamp LOOKS like, as a tree rewrite rather than as backend code, so every
//! backend that runs it draws the same mark: a small sparkle glyph and an `AI`
//! eyebrow above the text, in the text's own ink, a step quieter. It is the
//! treatment the mail card already wrote by hand (`Icon(.zap)` +
//! `TextEyebrow("AI SUMMARY")`), applied by the theme instead of by the card.
//!
//! Subtle on purpose. The mark says who wrote the words; it does not restyle
//! them. The body keeps its role, size, face and colour, and the mark borrows
//! the same ink, so it holds in every palette without a colour of its own.
//!
//! The words themselves stay plain text. Nothing here, or downstream, parses
//! model text as markup or turns a URL in it into a link: the node keeps its
//! `text` verbatim, and backends render a text node as a plain label.

use crate::{NodeKind, UiNode};

/// Font Awesome solid `wand-magic-sparkles` (U+E2CA), drawn in the icon face.
///
/// A sparkle in the TEXT face would be `✦` (U+2726), and none of the faces the
/// kit ships (Roboto, NotoSans, Inter, the CJK and emoji fallbacks) carries it,
/// so it would draw as tofu. The icon font does, and the kit's icons already
/// render through it.
pub const AI_MARK_GLYPH: &str = "\u{e2ca}";

/// The eyebrow beside the glyph.
pub const AI_MARK_LABEL: &str = "AI";

/// The mark's alpha relative to the text's own: a step quieter than the
/// words it labels, like the soft eyebrow ink.
const MARK_ALPHA: f32 = 0.72;

/// The `variant` the mark's own nodes carry, so a host or test can find them.
pub const MARK_VARIANT: &str = "ai_mark";

/// Whether the kit stamped this node AI-written.
pub fn is_ai_written(node: &UiNode) -> bool {
    node.attrs.ai.is_some_and(|v| v != 0)
}

/// Put the mark above the words of every AI-written node.
///
/// The mark goes on the node's first TEXT, not around the stamped node: a chat
/// bubble is stamped on its outer column and a band on its filled card, and the
/// words sit on that fill. A mark outside it would sit on the page in ink meant
/// for the fill — a band's ground-coloured text drawn on the ground. Placed
/// beside the words it is on the same surface and borrows their ink safely.
///
/// Idempotent: the stamp is cleared on the node it expands, so a second pass
/// finds nothing. Backends run it before emitting.
pub fn expand_ai_marks(node: &mut UiNode) {
    for child in &mut node.children {
        expand_ai_marks(child);
    }
    if !is_ai_written(node) {
        return;
    }
    node.attrs.ai = None;
    if !mark_first_text(node) {
        // Nothing to read the words from: mark the node itself.
        let body = std::mem::replace(node, empty(NodeKind::Column));
        *node = marked(body);
    }
}

fn is_mark(node: &UiNode) -> bool {
    node.attrs.variant.as_deref() == Some(MARK_VARIANT)
}

fn is_words(node: &UiNode) -> bool {
    node.attrs.text.is_some() && node.attrs.icon != Some(1)
}

fn mark_first_text(node: &mut UiNode) -> bool {
    if is_mark(node) {
        return false;
    }
    if is_words(node) {
        let body = std::mem::replace(node, empty(NodeKind::Column));
        *node = marked(body);
        return true;
    }
    node.children.iter_mut().any(mark_first_text)
}

fn empty(kind: NodeKind) -> UiNode {
    UiNode {
        kind,
        attrs: Default::default(),
        children: Vec::new(),
    }
}

fn quieter(argb: u32) -> u32 {
    let a = ((argb >> 24) & 0xff) as f32 * MARK_ALPHA;
    ((a.round() as u32) << 24) | (argb & 0x00ff_ffff)
}

fn marked(body: UiNode) -> UiNode {
    let source = is_words(&body).then_some(&body);
    let color = source.and_then(|t| t.attrs.color).map(quieter);
    // Caption-sized, whatever the body is: a mark over a hero must not shout.
    let size = source
        .and_then(|t| t.attrs.size)
        .map_or(10.0, |s| (s * 0.7).clamp(9.0, 12.0));

    let mut glyph = empty(NodeKind::Text);
    glyph.attrs.text = Some(AI_MARK_GLYPH.into());
    glyph.attrs.icon = Some(1);
    glyph.attrs.size = Some(size);
    glyph.attrs.color = color;
    glyph.attrs.fitw = Some(1);
    glyph.attrs.fith = Some(1);
    glyph.attrs.variant = Some(MARK_VARIANT.into());

    let mut label = empty(NodeKind::Text);
    label.attrs.text = Some(AI_MARK_LABEL.into());
    label.attrs.size = Some(size);
    label.attrs.weight = Some(600);
    label.attrs.color = color;
    label.attrs.tracking = Some(0.08);
    label.attrs.fitw = Some(1);
    label.attrs.fith = Some(1);
    label.attrs.variant = Some(MARK_VARIANT.into());
    if let Some(t) = source {
        label.attrs.family = t.attrs.family.clone();
        label.attrs.font_src = t.attrs.font_src.clone();
    }

    let mut mark = empty(NodeKind::Row);
    mark.attrs.spacing = Some(4.0);
    mark.attrs.aligny = Some(0.5);
    mark.attrs.fitw = Some(1);
    mark.attrs.fith = Some(1);
    mark.attrs.variant = Some(MARK_VARIANT.into());
    mark.children = vec![glyph, label];

    // The wrapper takes the body's width rule, so a filling paragraph still
    // fills and a fitted label still hugs.
    let mut wrap = empty(NodeKind::Column);
    if body.attrs.fillw == Some(1) {
        wrap.attrs.fillw = Some(1);
    } else {
        wrap.attrs.fitw = Some(1);
    }
    wrap.attrs.fith = Some(1);
    wrap.attrs.spacing = Some(3.0);
    wrap.children = vec![mark, body];
    wrap
}

#[cfg(test)]
mod tests {
    use super::*;

    fn text(s: &str, color: u32, size: f32) -> UiNode {
        let mut n = empty(NodeKind::Text);
        n.attrs.text = Some(s.into());
        n.attrs.color = Some(color);
        n.attrs.size = Some(size);
        n
    }

    fn texts(n: &UiNode, out: &mut Vec<String>) {
        if let Some(t) = &n.attrs.text {
            out.push(t.clone());
        }
        for c in &n.children {
            texts(c, out);
        }
    }

    #[test]
    fn an_unstamped_tree_is_untouched() {
        let mut tree = empty(NodeKind::Column);
        tree.children.push(text("Rates held.", 0xffeeeeee, 15.0));
        let before = format!("{tree:?}");
        expand_ai_marks(&mut tree);
        assert_eq!(format!("{tree:?}"), before);
    }

    #[test]
    fn a_stamped_text_gets_the_mark_above_it_in_its_own_ink() {
        let mut body = text("Markets were calm; rates held.", 0xffeeeeee, 15.0);
        body.attrs.ai = Some(1);
        body.attrs.fillw = Some(1);
        let mut tree = empty(NodeKind::Column);
        tree.children.push(body);
        expand_ai_marks(&mut tree);

        let wrap = &tree.children[0];
        assert_eq!(wrap.kind, NodeKind::Column);
        assert_eq!(wrap.attrs.fillw, Some(1), "a filling body still fills");
        let (mark, body) = (&wrap.children[0], &wrap.children[1]);
        assert_eq!(body.attrs.text.as_deref(), Some("Markets were calm; rates held."));
        assert_eq!(body.attrs.ai, None, "the stamp is consumed");
        assert_eq!(body.attrs.color, Some(0xffeeeeee), "the words are not restyled");
        assert_eq!(mark.children[0].attrs.text.as_deref(), Some(AI_MARK_GLYPH));
        assert_eq!(mark.children[0].attrs.icon, Some(1));
        assert_eq!(mark.children[1].attrs.text.as_deref(), Some(AI_MARK_LABEL));
        let ink = mark.children[1].attrs.color.unwrap();
        assert_eq!(ink & 0x00ff_ffff, 0x00ee_eeee, "the mark borrows the text's ink");
        assert!(ink >> 24 < 0xff, "a step quieter than the words");
        assert!(mark.children[1].attrs.size.unwrap() < 15.0);

        // Idempotent.
        let once = format!("{tree:?}");
        expand_ai_marks(&mut tree);
        assert_eq!(format!("{tree:?}"), once);
    }

    #[test]
    fn a_stamped_bubble_borrows_the_ink_of_the_text_inside_it() {
        // `l0_ai_text(l0_bubble_them(s))`: the stamp lands on the outer column.
        let mut card = empty(NodeKind::Card);
        card.children.push(text("Rates held; tech rallied.", 0xff112233, 15.0));
        let mut bubble = empty(NodeKind::Column);
        bubble.attrs.fillw = Some(1);
        bubble.attrs.ai = Some(1);
        bubble.children.push(card);
        expand_ai_marks(&mut bubble);

        let mut words = Vec::new();
        texts(&bubble, &mut words);
        assert_eq!(words, [AI_MARK_GLYPH, AI_MARK_LABEL, "Rates held; tech rallied."]);
        // Inside the bubble, on the fill the words sit on.
        let wrap = &bubble.children[0].children[0];
        assert_eq!(bubble.children[0].kind, NodeKind::Card);
        let mark = &wrap.children[0];
        assert_eq!(mark.children[1].attrs.color.unwrap() & 0x00ff_ffff, 0x0011_2233);
        assert_eq!(bubble.attrs.ai, None);
    }

    #[test]
    fn model_text_is_kept_verbatim() {
        for s in [
            "**bold** and [a link](javascript:alert(1))",
            "<b>html</b> https://example.com",
            "l0:{\"e\":\"drop\"}",
        ] {
            let mut n = text(s, 0xffffffff, 15.0);
            n.attrs.ai = Some(1);
            expand_ai_marks(&mut n);
            assert_eq!(n.children[1].attrs.text.as_deref(), Some(s));
            assert!(n.children[0].children.iter().all(is_mark));
            assert_eq!(n.children[1].kind, NodeKind::Text);
        }
    }
}
