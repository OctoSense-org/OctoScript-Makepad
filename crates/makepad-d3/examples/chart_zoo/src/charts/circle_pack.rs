//! Circle Packing Widget
//!
//! Hierarchical visualization using nested circles.

use makepad_widgets::*;
use makepad_d3::layout::hierarchy::{HierarchyNode, PackLayout};
use super::draw_primitives::DrawPoint;

live_design! {
    link widgets;
    use link::shaders::*;
    use super::draw_primitives::DrawPoint;

    // Circle with border effect
    pub DrawCircleBorder = {{DrawCircleBorder}} {
        fn pixel(self) -> vec4 {
            let uv = self.pos;
            let center = vec2(0.5, 0.5);
            let dist = distance(uv, center) * 2.0;

            if dist > 1.0 {
                return vec4(0.0, 0.0, 0.0, 0.0);
            }

            // Anti-alias
            let aa = 0.03;
            let alpha = 1.0 - smoothstep(1.0 - aa, 1.0, dist);

            // Border vs fill transition
            let border_start = 1.0 - self.border_width * 2.0;
            let in_border = smoothstep(border_start - 0.02, border_start + 0.02, dist);

            // Fill color with slight gradient
            let grad = mix(self.fill_color, self.border_color, dist * 0.3);
            let fill_result = vec4(grad.rgb * self.fill_alpha * alpha, self.fill_alpha * alpha);

            // Border color
            let border_result = vec4(self.border_color.rgb * alpha, alpha);

            // Mix between fill and border
            return mix(fill_result, border_result, in_border);
        }
    }

    pub CirclePackWidget = {{CirclePackWidget}} {
        width: Fill,
        height: Fill,
    }
}

#[derive(Live, LiveHook, LiveRegister)]
#[repr(C)]
pub struct DrawCircleBorder {
    #[deref] pub draw_super: DrawQuad,
    #[live] pub fill_color: Vec4,
    #[live] pub border_color: Vec4,
    #[live(0.05)] pub border_width: f32,
    #[live(0.3)] pub fill_alpha: f32,
}

#[derive(Live, LiveHook, Widget)]
pub struct CirclePackWidget {
    #[redraw]
    #[live]
    draw_circle: DrawCircleBorder,

    #[redraw]
    #[live]
    draw_point: DrawPoint,

    #[walk]
    walk: Walk,

    #[rust]
    tree: Option<HierarchyNode<PackNode>>,

    #[rust]
    positioned_tree: Option<HierarchyNode<PackNode>>,

    #[rust]
    colors: Vec<Vec4>,

    #[rust]
    initialized: bool,

    #[rust]
    area: Area,

    #[rust]
    offset_x: f64,

    #[rust]
    offset_y: f64,

    #[rust]
    hovered_circle: Option<(f64, f64, f64)>, // x, y, radius
}

#[derive(Clone)]
struct PackNode {
    name: String,
    depth: usize,
}

impl Widget for CirclePackWidget {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, _scope: &mut Scope) {
        match event {
            Event::NextFrame(_) => {
                if !self.initialized {
                    self.initialize_data();
                    self.redraw(cx);
                }
            }
            Event::MouseMove(e) => {
                self.handle_mouse_move(cx, e.abs);
            }
            _ => {}
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        if !self.initialized {
            self.initialize_data();
        }

        let rect = cx.walk_turtle_with_area(&mut self.area, walk);

        if rect.size.x > 0.0 && rect.size.y > 0.0 {
            self.draw_pack(cx, rect);
        }

        DrawStep::done()
    }
}

impl CirclePackWidget {
    fn initialize_data(&mut self) {
        if self.initialized {
            return;
        }

        // Create a hierarchical dataset
        let mut root = HierarchyNode::new(
            PackNode { name: "World".to_string(), depth: 0 },
            0.0,
        );

        // Americas
        let mut americas = HierarchyNode::new(
            PackNode { name: "Americas".to_string(), depth: 1 },
            0.0,
        );
        americas.add_child(HierarchyNode::new(
            PackNode { name: "USA".to_string(), depth: 2 },
            330.0,
        ));
        americas.add_child(HierarchyNode::new(
            PackNode { name: "Brazil".to_string(), depth: 2 },
            215.0,
        ));
        americas.add_child(HierarchyNode::new(
            PackNode { name: "Mexico".to_string(), depth: 2 },
            130.0,
        ));
        americas.add_child(HierarchyNode::new(
            PackNode { name: "Canada".to_string(), depth: 2 },
            38.0,
        ));

        // Europe
        let mut europe = HierarchyNode::new(
            PackNode { name: "Europe".to_string(), depth: 1 },
            0.0,
        );
        europe.add_child(HierarchyNode::new(
            PackNode { name: "Germany".to_string(), depth: 2 },
            84.0,
        ));
        europe.add_child(HierarchyNode::new(
            PackNode { name: "UK".to_string(), depth: 2 },
            68.0,
        ));
        europe.add_child(HierarchyNode::new(
            PackNode { name: "France".to_string(), depth: 2 },
            67.0,
        ));
        europe.add_child(HierarchyNode::new(
            PackNode { name: "Italy".to_string(), depth: 2 },
            59.0,
        ));
        europe.add_child(HierarchyNode::new(
            PackNode { name: "Spain".to_string(), depth: 2 },
            47.0,
        ));

        // Asia
        let mut asia = HierarchyNode::new(
            PackNode { name: "Asia".to_string(), depth: 1 },
            0.0,
        );
        asia.add_child(HierarchyNode::new(
            PackNode { name: "China".to_string(), depth: 2 },
            1400.0,
        ));
        asia.add_child(HierarchyNode::new(
            PackNode { name: "India".to_string(), depth: 2 },
            1380.0,
        ));
        asia.add_child(HierarchyNode::new(
            PackNode { name: "Japan".to_string(), depth: 2 },
            125.0,
        ));
        asia.add_child(HierarchyNode::new(
            PackNode { name: "Indonesia".to_string(), depth: 2 },
            275.0,
        ));

        // Africa
        let mut africa = HierarchyNode::new(
            PackNode { name: "Africa".to_string(), depth: 1 },
            0.0,
        );
        africa.add_child(HierarchyNode::new(
            PackNode { name: "Nigeria".to_string(), depth: 2 },
            220.0,
        ));
        africa.add_child(HierarchyNode::new(
            PackNode { name: "Ethiopia".to_string(), depth: 2 },
            120.0,
        ));
        africa.add_child(HierarchyNode::new(
            PackNode { name: "Egypt".to_string(), depth: 2 },
            105.0,
        ));

        root.add_child(americas);
        root.add_child(europe);
        root.add_child(asia);
        root.add_child(africa);

        self.tree = Some(root);

        // Colors by depth level
        self.colors = vec![
            vec4(0.20, 0.25, 0.35, 1.0), // Root - dark
            vec4(0.35, 0.55, 0.80, 1.0), // Continent - blue
            vec4(0.50, 0.75, 0.95, 1.0), // Country - light blue
        ];

        self.initialized = true;
    }

    fn draw_pack(&mut self, cx: &mut Cx2d, rect: Rect) {
        let tree = match &self.tree {
            Some(t) => t,
            None => return,
        };

        let padding = 10.0;
        let size = rect.size.x.min(rect.size.y) - padding * 2.0;

        let layout = PackLayout::new()
            .size(size as f64, size as f64)
            .padding(4.0);

        let positioned = layout.layout(tree);
        self.positioned_tree = Some(positioned.clone());

        // Calculate offset to center
        self.offset_x = (rect.pos.x + (rect.size.x - size) / 2.0) as f64;
        self.offset_y = (rect.pos.y + (rect.size.y - size) / 2.0) as f64;

        // Draw circles from root to leaves (larger to smaller)
        self.draw_node(cx, &positioned, self.offset_x, self.offset_y);
    }

    fn draw_node(&mut self, cx: &mut Cx2d, node: &HierarchyNode<PackNode>, offset_x: f64, offset_y: f64) {
        let depth = node.data.depth;
        let color_idx = depth.min(self.colors.len() - 1);
        let mut color = self.colors[color_idx];

        let x = offset_x + node.x;
        let y = offset_y + node.y;
        let r = node.radius;

        // Check if this circle is hovered
        let is_hovered = self.hovered_circle.map_or(false, |(hx, hy, hr)| {
            (hx - x).abs() < 0.1 && (hy - y).abs() < 0.1 && (hr - r).abs() < 0.1
        });

        if is_hovered {
            color = vec4(
                (color.x + 0.25).min(1.0),
                (color.y + 0.25).min(1.0),
                (color.z + 0.25).min(1.0),
                color.w,
            );
        }

        if r > 1.0 {
            // Draw circle with border
            self.draw_circle.fill_color = color;
            self.draw_circle.border_color = vec4(
                (color.x + 0.2).min(1.0),
                (color.y + 0.2).min(1.0),
                (color.z + 0.2).min(1.0),
                1.0,
            );

            // Adjust fill alpha based on depth and hover
            let base_alpha = if depth == 0 { 0.1 } else if depth == 1 { 0.25 } else { 0.5 };
            self.draw_circle.fill_alpha = if is_hovered { (base_alpha + 0.3).min(1.0) } else { base_alpha };
            self.draw_circle.border_width = if is_hovered { 0.06 } else if depth < 2 { 0.02 } else { 0.04 };

            self.draw_circle.draw_abs(
                cx,
                Rect {
                    pos: dvec2(x - r, y - r),
                    size: dvec2(r * 2.0, r * 2.0),
                },
            );
        }

        // Draw children
        for child in &node.children {
            self.draw_node(cx, child, offset_x, offset_y);
        }
    }

    fn handle_mouse_move(&mut self, cx: &mut Cx, pos: DVec2) {
        let old_hovered = self.hovered_circle;
        self.hovered_circle = self.find_circle_at(pos);

        if old_hovered != self.hovered_circle {
            self.redraw(cx);
        }
    }

    fn find_circle_at(&self, pos: DVec2) -> Option<(f64, f64, f64)> {
        let tree = self.positioned_tree.as_ref()?;
        // Find the smallest circle containing the point (deepest in hierarchy)
        self.find_deepest_circle(tree, pos, self.offset_x, self.offset_y)
    }

    fn find_deepest_circle(&self, node: &HierarchyNode<PackNode>, pos: DVec2, offset_x: f64, offset_y: f64) -> Option<(f64, f64, f64)> {
        let x = offset_x + node.x;
        let y = offset_y + node.y;
        let r = node.radius;

        // Check if point is inside this circle
        let dx = pos.x - x;
        let dy = pos.y - y;
        let dist_sq = dx * dx + dy * dy;

        if dist_sq > r * r {
            return None;
        }

        // Check children first (to find deepest)
        for child in &node.children {
            if let Some(found) = self.find_deepest_circle(child, pos, offset_x, offset_y) {
                return Some(found);
            }
        }

        // If no child contains it, return this node (if it's a leaf or has visible radius)
        if r > 1.0 {
            Some((x, y, r))
        } else {
            None
        }
    }
}
