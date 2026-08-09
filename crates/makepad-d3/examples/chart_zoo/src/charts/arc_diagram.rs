//! Arc diagram - network visualization with curved arcs
//!
//! Displays nodes on a line with arcs connecting related nodes.
//! Good for showing relationships in sequential or ordered data.

use makepad_widgets::*;
use super::draw_primitives::{DrawPoint, DrawChartLine};
use super::animation::ChartAnimator;

live_design! {
    link widgets;
    use link::shaders::*;
    use super::draw_primitives::DrawPoint;
    use super::draw_primitives::DrawChartLine;

    pub ArcDiagram = {{ArcDiagram}} {
        width: Fill,
        height: Fill,
    }
}

#[derive(Clone, Debug)]
pub struct ArcNode {
    pub id: String,
    pub label: String,
    pub group: Option<String>,
    pub value: f64,
}

impl ArcNode {
    pub fn new(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            group: None,
            value: 1.0,
        }
    }

    pub fn with_group(mut self, group: impl Into<String>) -> Self {
        self.group = Some(group.into());
        self
    }

    pub fn with_value(mut self, value: f64) -> Self {
        self.value = value;
        self
    }
}

#[derive(Clone, Debug)]
pub struct ArcLink {
    pub source: String,
    pub target: String,
    pub weight: f64,
}

impl ArcLink {
    pub fn new(source: impl Into<String>, target: impl Into<String>) -> Self {
        Self {
            source: source.into(),
            target: target.into(),
            weight: 1.0,
        }
    }

    pub fn with_weight(mut self, weight: f64) -> Self {
        self.weight = weight;
        self
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct ArcDiagram {
    #[redraw]
    #[live]
    draw_point: DrawPoint,

    #[redraw]
    #[live]
    draw_line: DrawChartLine,

    #[walk]
    walk: Walk,

    #[rust]
    nodes: Vec<ArcNode>,

    #[rust]
    links: Vec<ArcLink>,

    #[rust]
    node_radius: f64,

    #[rust]
    arc_opacity: f64,

    #[rust]
    vertical: bool,

    #[rust]
    groups: Vec<String>,

    #[rust]
    animator: ChartAnimator,

    #[rust]
    initialized: bool,

    #[rust]
    area: Area,
}

impl ArcDiagram {
    pub fn set_data(&mut self, nodes: Vec<ArcNode>, links: Vec<ArcLink>) {
        // Extract unique groups
        let mut groups: Vec<String> = nodes
            .iter()
            .filter_map(|n| n.group.clone())
            .collect();
        groups.sort();
        groups.dedup();
        self.groups = groups;

        self.nodes = nodes;
        self.links = links;
        self.initialized = false;
    }

    pub fn set_node_radius(&mut self, radius: f64) {
        self.node_radius = radius;
    }

    pub fn set_arc_opacity(&mut self, opacity: f64) {
        self.arc_opacity = opacity.clamp(0.0, 1.0);
    }

    pub fn set_vertical(&mut self, vertical: bool) {
        self.vertical = vertical;
    }

    fn draw_chart(&mut self, cx: &mut Cx2d, rect: Rect) {
        if self.nodes.is_empty() {
            return;
        }

        let padding = 60.0;
        let chart_x = rect.pos.x + padding;
        let chart_y = rect.pos.y + padding;
        let chart_w = rect.size.x - padding * 2.0;
        let chart_h = rect.size.y - padding * 2.0;

        let n = self.nodes.len();

        // Calculate node positions
        let mut node_positions: Vec<DVec2> = Vec::new();
        for (i, _node) in self.nodes.iter().enumerate() {
            let t = i as f64 / (n - 1).max(1) as f64;
            let pos = if self.vertical {
                DVec2 {
                    x: chart_x + chart_w / 2.0,
                    y: chart_y + t * chart_h,
                }
            } else {
                DVec2 {
                    x: chart_x + t * chart_w,
                    y: chart_y + chart_h / 2.0,
                }
            };
            node_positions.push(pos);
        }

        // Color palette for groups
        let colors = [
            vec4(0.4, 0.76, 0.65, 1.0),
            vec4(0.99, 0.55, 0.38, 1.0),
            vec4(0.55, 0.63, 0.80, 1.0),
            vec4(0.91, 0.84, 0.42, 1.0),
            vec4(0.65, 0.85, 0.33, 1.0),
            vec4(0.90, 0.45, 0.77, 1.0),
        ];

        // Find max weight for scaling
        let max_weight = self.links.iter().map(|l| l.weight).fold(0.0_f64, f64::max).max(1.0);

        // Get animation progress
        let progress = self.animator.get_progress();

        // Draw axis line
        let axis_color = vec4(0.3, 0.3, 0.3, 1.0);
        self.draw_line.color = axis_color;
        if self.vertical {
            self.draw_line.draw_line(
                cx,
                dvec2(chart_x + chart_w / 2.0, chart_y),
                dvec2(chart_x + chart_w / 2.0, chart_y + chart_h),
                1.0,
            );
        } else {
            self.draw_line.draw_line(
                cx,
                dvec2(chart_x, chart_y + chart_h / 2.0),
                dvec2(chart_x + chart_w, chart_y + chart_h / 2.0),
                1.0,
            );
        }

        // Collect arc data to avoid borrow conflict
        let arc_opacity = self.arc_opacity;
        let vertical = self.vertical;
        let arc_data: Vec<_> = self.links.iter().filter_map(|link| {
            let source_idx = self.nodes.iter().position(|n| n.id == link.source);
            let target_idx = self.nodes.iter().position(|n| n.id == link.target);

            if let (Some(si), Some(ti)) = (source_idx, target_idx) {
                let p1 = node_positions[si];
                let p2 = node_positions[ti];

                // Determine arc color based on source node group
                let source_group = self.nodes[si].group.as_ref();
                let color_idx = source_group
                    .and_then(|g| self.groups.iter().position(|gr| gr == g))
                    .unwrap_or(0);
                let base_color = colors[color_idx % colors.len()];
                let arc_color = vec4(base_color.x, base_color.y, base_color.z, (arc_opacity * progress as f64) as f32);

                let width = 1.0 + (link.weight / max_weight) * 3.0;
                Some((p1, p2, arc_color, width))
            } else {
                None
            }
        }).collect();

        // Draw arcs first (behind nodes)
        for (p1, p2, arc_color, width) in arc_data {
            self.draw_arc(cx, p1, p2, arc_color, width, vertical, progress as f64);
        }

        // Draw nodes
        for (i, node) in self.nodes.iter().enumerate() {
            let pos = node_positions[i];

            let color_idx = node.group.as_ref()
                .and_then(|g| self.groups.iter().position(|gr| gr == g))
                .unwrap_or(0);
            let color = colors[color_idx % colors.len()];

            // Animate node appearance
            let node_progress = (progress as f32 * (self.nodes.len() as f32 + 5.0) - i as f32).clamp(0.0, 1.0);
            let animated_radius = self.node_radius * node_progress as f64;

            self.draw_point.color = color;
            self.draw_point.draw_point(cx, pos, animated_radius * 2.0);
        }
    }

    fn draw_arc(&mut self, cx: &mut Cx2d, p1: DVec2, p2: DVec2, color: Vec4, width: f64, vertical: bool, progress: f64) {
        // Calculate arc as half-circle between points
        let mid = DVec2 {
            x: (p1.x + p2.x) / 2.0,
            y: (p1.y + p2.y) / 2.0,
        };

        let dist = if vertical {
            (p2.y - p1.y).abs()
        } else {
            (p2.x - p1.x).abs()
        };

        let radius = dist / 2.0;
        let segments = 32;
        let segments_to_draw = ((segments as f64) * progress) as usize;

        self.draw_line.color = color;

        let mut prev = p1;
        for i in 1..=segments_to_draw {
            let t = i as f64 / segments as f64;
            let angle = std::f64::consts::PI * t;

            let pt = if vertical {
                DVec2 {
                    x: mid.x - radius * angle.cos(),
                    y: p1.y + dist * t,
                }
            } else {
                DVec2 {
                    x: p1.x + dist * t,
                    y: mid.y - radius * angle.sin(),
                }
            };

            self.draw_line.draw_line(cx, prev, pt, width);
            prev = pt;
        }
    }
}

impl ArcDiagram {
    fn initialize_demo_data(&mut self) {
        self.nodes = vec![
            ArcNode::new("A", "Alice").with_group("Team 1"),
            ArcNode::new("B", "Bob").with_group("Team 1"),
            ArcNode::new("C", "Carol").with_group("Team 2"),
            ArcNode::new("D", "Dave").with_group("Team 2"),
            ArcNode::new("E", "Eve").with_group("Team 3"),
            ArcNode::new("F", "Frank").with_group("Team 3"),
            ArcNode::new("G", "Grace").with_group("Team 1"),
        ];
        self.links = vec![
            ArcLink::new("A", "B").with_weight(3.0),
            ArcLink::new("A", "C").with_weight(2.0),
            ArcLink::new("B", "D").with_weight(2.5),
            ArcLink::new("C", "E").with_weight(1.5),
            ArcLink::new("D", "F").with_weight(2.0),
            ArcLink::new("E", "G").with_weight(1.0),
            ArcLink::new("F", "A").with_weight(1.5),
            ArcLink::new("G", "C").with_weight(2.0),
        ];
        self.node_radius = 8.0;
        self.arc_opacity = 0.6;
        self.vertical = false;

        // Update groups list
        let mut groups: Vec<String> = self.nodes.iter()
            .filter_map(|n| n.group.clone())
            .collect();
        groups.sort();
        groups.dedup();
        self.groups = groups;
    }
}

impl Widget for ArcDiagram {
    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        let rect = cx.walk_turtle_with_area(&mut self.area, walk);

        if !self.initialized {
            self.initialize_demo_data();
            self.animator = ChartAnimator::new(1.2 * 1000.0);
            self.animator.start(cx.cx.seconds_since_app_start());
            self.initialized = true;
        }

        self.draw_chart(cx, rect);
        DrawStep::done()
    }

    fn handle_event(&mut self, cx: &mut Cx, event: &Event, _scope: &mut Scope) {
        match event {
            Event::NextFrame(_) => {
                if self.animator.is_running() {
                    let time = cx.seconds_since_app_start();
                    if self.animator.update(time) {
                        self.redraw(cx);
                    }
                    cx.new_next_frame();
                }
            }
            _ => {}
        }
    }
}
