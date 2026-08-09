//! Random Tree Chart Widget
//!
//! D3-style random tree that grows by periodically adding nodes to random parents.
//! Features smooth animated transitions as the tree layout updates.

use makepad_widgets::*;
use super::draw_primitives::{DrawChartLine, DrawPoint};

live_design! {
    link widgets;
    use link::shaders::*;
    use super::draw_primitives::DrawChartLine;
    use super::draw_primitives::DrawPoint;

    pub TreeChartWidget = {{TreeChartWidget}} {
        width: Fill,
        height: Fill,
    }
}

#[derive(Clone)]
struct TreeNode {
    parent: Option<usize>,
    children: Vec<usize>,
    depth: usize,
    // Current computed position (normalized 0-1)
    x: f64,
    y: f64,
    // Previous position for animation
    px: f64,
    py: f64,
}

impl TreeNode {
    fn new(parent: Option<usize>, depth: usize) -> Self {
        Self {
            parent,
            children: Vec::new(),
            depth,
            x: 0.0,
            y: 0.0,
            px: 0.0,
            py: 0.0,
        }
    }
}

#[derive(Clone)]
struct Link {
    source: usize,
    target: usize,
}

#[derive(Live, LiveHook, Widget)]
pub struct TreeChartWidget {
    #[redraw]
    #[live]
    draw_line: DrawChartLine,

    #[redraw]
    #[live]
    draw_point: DrawPoint,

    #[walk]
    walk: Walk,

    #[rust]
    nodes: Vec<TreeNode>,

    #[rust]
    links: Vec<Link>,

    #[rust]
    initialized: bool,

    #[rust]
    area: Area,

    #[rust]
    chart_rect: Rect,

    /// Is the tree actively growing?
    #[rust(true)]
    growing: bool,

    /// Maximum number of nodes before stopping growth
    #[rust]
    max_nodes: usize,

    /// Duration of each animation cycle in milliseconds
    #[rust(750.0)]
    duration_ms: f64,

    /// Time when last node was added
    #[rust(0.0)]
    last_add_time: f64,

    /// Current animation progress (0.0 to 1.0)
    #[rust(1.0)]
    anim_progress: f64,

    /// Animation start time
    #[rust(0.0)]
    anim_start_time: f64,
}

impl Widget for TreeChartWidget {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, _scope: &mut Scope) {
        match event {
            Event::NextFrame(nf) => {
                let current_time = nf.time * 1000.0; // Convert to ms

                // Update animation progress
                if self.anim_progress < 1.0 {
                    let elapsed = current_time - self.anim_start_time;
                    self.anim_progress = (elapsed / self.duration_ms).min(1.0);
                    self.redraw(cx);
                }

                // Check if it's time to add a new node
                if self.growing && self.nodes.len() < self.max_nodes {
                    let time_since_add = current_time - self.last_add_time;
                    if time_since_add >= self.duration_ms {
                        self.add_random_child();
                        self.compute_layout();
                        self.anim_start_time = current_time;
                        self.anim_progress = 0.0;
                        self.last_add_time = current_time;
                        self.redraw(cx);
                    }
                }

                // Keep requesting frames while growing or animating
                if self.growing || self.anim_progress < 1.0 {
                    cx.new_next_frame();
                }
            }
            Event::WindowGeomChange(_) => {
                self.redraw(cx);
            }
            _ => {}
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        let rect = cx.walk_turtle_with_area(&mut self.area, walk);

        if rect.size.x > 0.0 && rect.size.y > 0.0 {
            self.chart_rect = rect;

            if !self.initialized {
                self.initialize_random_tree();
                self.initialized = true;
                cx.new_next_frame();
            }

            self.draw_tree(cx);
        }

        DrawStep::done()
    }
}

impl TreeChartWidget {
    fn initialize_random_tree(&mut self) {
        // Start with a single root node
        self.nodes.clear();
        self.links.clear();

        let root = TreeNode::new(None, 0);
        self.nodes.push(root);

        self.compute_layout();

        // Set initial previous positions to current positions
        for node in &mut self.nodes {
            node.px = node.x;
            node.py = node.y;
        }

        self.max_nodes = 500;
        self.growing = true;
        self.anim_progress = 1.0;
        self.last_add_time = 0.0;
    }

    fn add_random_child(&mut self) {
        if self.nodes.is_empty() {
            return;
        }

        // Save current positions as previous positions
        for node in &mut self.nodes {
            node.px = node.x;
            node.py = node.y;
        }

        // Pick a random parent
        let parent_idx = (rand_u64() as usize) % self.nodes.len();
        let parent_depth = self.nodes[parent_idx].depth;

        // Create new child node
        let child_idx = self.nodes.len();
        let mut child = TreeNode::new(Some(parent_idx), parent_depth + 1);

        // New node starts at parent's previous position
        child.px = self.nodes[parent_idx].px;
        child.py = self.nodes[parent_idx].py;

        self.nodes.push(child);
        self.nodes[parent_idx].children.push(child_idx);

        // Add link
        self.links.push(Link {
            source: parent_idx,
            target: child_idx,
        });
    }

    fn compute_layout(&mut self) {
        if self.nodes.is_empty() {
            return;
        }

        // D3 tree layout algorithm (tidy tree)
        // 1. Compute leaf positions
        // 2. Position internal nodes at center of children

        let max_depth = self.nodes.iter().map(|n| n.depth).max().unwrap_or(0);

        // First pass: count leaves and assign x positions
        let mut leaf_count = 0usize;
        self.assign_leaf_positions(0, &mut leaf_count);

        // Second pass: position internal nodes at center of children (bottom-up)
        for depth in (0..=max_depth).rev() {
            for i in 0..self.nodes.len() {
                if self.nodes[i].depth == depth && !self.nodes[i].children.is_empty() {
                    let children = self.nodes[i].children.clone();
                    let sum_x: f64 = children.iter().map(|&c| self.nodes[c].x).sum();
                    self.nodes[i].x = sum_x / children.len() as f64;
                }
            }
        }

        // Normalize x positions to 0-1 range
        let max_x = self.nodes.iter().map(|n| n.x).fold(0.0f64, |a, b| a.max(b));
        let min_x = self.nodes.iter().map(|n| n.x).fold(f64::MAX, |a, b| a.min(b));
        let x_range = max_x - min_x;

        if x_range > 0.0 {
            for node in &mut self.nodes {
                node.x = (node.x - min_x) / x_range;
            }
        } else {
            // Single column - center it
            for node in &mut self.nodes {
                node.x = 0.5;
            }
        }

        // Set y positions based on depth (normalized 0-1, root at top)
        if max_depth > 0 {
            for node in &mut self.nodes {
                node.y = node.depth as f64 / max_depth as f64;
            }
        } else {
            for node in &mut self.nodes {
                node.y = 0.0;
            }
        }
    }

    fn assign_leaf_positions(&mut self, node_idx: usize, leaf_count: &mut usize) {
        let children = self.nodes[node_idx].children.clone();

        if children.is_empty() {
            // Leaf node
            self.nodes[node_idx].x = *leaf_count as f64;
            *leaf_count += 1;
        } else {
            // Internal node - recurse to children first
            for child_idx in children {
                self.assign_leaf_positions(child_idx, leaf_count);
            }
        }
    }

    fn draw_tree(&mut self, cx: &mut Cx2d) {
        let rect = self.chart_rect;
        let padding = 20.0;

        let chart_x = rect.pos.x + padding;
        let chart_y = rect.pos.y + padding;
        let chart_w = rect.size.x - padding * 2.0;
        let chart_h = rect.size.y - padding * 2.0;

        if chart_w <= 0.0 || chart_h <= 0.0 || self.nodes.is_empty() {
            return;
        }

        // Ease function (ease-out cubic)
        let t = self.anim_progress;
        let eased_t = 1.0 - (1.0 - t).powi(3);

        // Collect link coordinates to avoid borrow issues
        let link_coords: Vec<_> = self.links.iter().map(|link| {
            let source = &self.nodes[link.source];
            let target = &self.nodes[link.target];

            // Interpolate positions
            let sx = source.px + (source.x - source.px) * eased_t;
            let sy = source.py + (source.y - source.py) * eased_t;
            let tx = target.px + (target.x - target.px) * eased_t;
            let ty = target.py + (target.y - target.py) * eased_t;

            // Convert to screen coordinates
            let x1 = chart_x + sx * chart_w;
            let y1 = chart_y + sy * chart_h;
            let x2 = chart_x + tx * chart_w;
            let y2 = chart_y + ty * chart_h;

            (x1, y1, x2, y2)
        }).collect();

        // Draw links first (behind nodes)
        self.draw_line.color = vec4(0.0, 0.0, 0.0, 1.0);

        for (x1, y1, x2, y2) in link_coords {
            // Draw vertical bezier link (d3.linkVertical)
            self.draw_vertical_link(cx, x1, y1, x2, y2);
        }

        // Draw nodes
        for node in &self.nodes {
            // Interpolate position
            let nx = node.px + (node.x - node.px) * eased_t;
            let ny = node.py + (node.y - node.py) * eased_t;

            // Convert to screen coordinates
            let x = chart_x + nx * chart_w;
            let y = chart_y + ny * chart_h;

            // Draw node circle with white stroke
            self.draw_point.color = vec4(0.0, 0.0, 0.0, 1.0);
            self.draw_point.draw_point(cx, dvec2(x, y), 4.0);
        }
    }

    fn draw_vertical_link(&mut self, cx: &mut Cx2d, x1: f64, y1: f64, x2: f64, y2: f64) {
        // d3.linkVertical - cubic bezier with vertical control points
        // Control points are at (x1, mid_y) and (x2, mid_y)
        let mid_y = (y1 + y2) / 2.0;

        // Use many segments for very smooth curves
        let segments = 48;
        let mut prev_point = cubic_bezier(x1, y1, x1, mid_y, x2, mid_y, x2, y2, 0.0);

        for i in 1..=segments {
            let t = i as f64 / segments as f64;
            let curr_point = cubic_bezier(x1, y1, x1, mid_y, x2, mid_y, x2, y2, t);

            self.draw_line.draw_line(cx, prev_point, curr_point, 1.0);
            prev_point = curr_point;
        }
    }

    /// Initialize with D3 flare-style software package hierarchy (tidy tree layout)
    pub fn initialize_flare_data(&mut self) {
        self.nodes.clear();
        self.links.clear();

        // Build flare hierarchy
        let node_data = vec![
            (None, "flare", vec![1, 2, 3, 4, 5]),
            (Some(0), "analytics", vec![6, 7, 8, 9]),
            (Some(0), "animate", vec![10, 11, 12, 13]),
            (Some(0), "data", vec![14, 15, 16, 17]),
            (Some(0), "display", vec![18, 19, 20, 21]),
            (Some(0), "vis", vec![22, 23, 24]),
            (Some(1), "AgglomerativeCluster", vec![]),
            (Some(1), "CommunityStructure", vec![]),
            (Some(1), "HierarchicalCluster", vec![]),
            (Some(1), "MergeEdge", vec![]),
            (Some(2), "Easing", vec![]),
            (Some(2), "FunctionSequence", vec![]),
            (Some(2), "Tween", vec![]),
            (Some(2), "Transitioner", vec![]),
            (Some(3), "DataField", vec![]),
            (Some(3), "DataSchema", vec![]),
            (Some(3), "DataSource", vec![]),
            (Some(3), "DataUtil", vec![]),
            (Some(4), "DirtySprite", vec![]),
            (Some(4), "LineSprite", vec![]),
            (Some(4), "RectSprite", vec![]),
            (Some(4), "TextSprite", vec![]),
            (Some(5), "ScaleBinding", vec![]),
            (Some(5), "Tree", vec![]),
            (Some(5), "TreeBuilder", vec![]),
        ];

        for (parent, _name, children) in &node_data {
            let depth = if let Some(p) = parent { self.nodes[*p].depth + 1 } else { 0 };
            let mut node = TreeNode::new(*parent, depth);
            node.children = children.clone();
            self.nodes.push(node);
        }

        // Build links
        for (i, node) in self.nodes.iter().enumerate() {
            if let Some(parent_idx) = node.parent {
                self.links.push(Link { source: parent_idx, target: i });
            }
        }

        self.compute_layout();

        // Set px/py to current positions (no animation for initial display)
        for node in &mut self.nodes {
            node.px = node.x;
            node.py = node.y;
        }

        self.growing = false;
        self.anim_progress = 1.0;
        self.initialized = true;
    }

    /// Initialize as random growing tree
    pub fn initialize_random_data(&mut self) {
        self.initialize_random_tree();
        self.initialized = true;
    }
}

/// Simple cubic bezier interpolation
fn cubic_bezier(x0: f64, y0: f64, x1: f64, y1: f64, x2: f64, y2: f64, x3: f64, y3: f64, t: f64) -> DVec2 {
    let mt = 1.0 - t;
    let mt2 = mt * mt;
    let mt3 = mt2 * mt;
    let t2 = t * t;
    let t3 = t2 * t;

    let x = mt3 * x0 + 3.0 * mt2 * t * x1 + 3.0 * mt * t2 * x2 + t3 * x3;
    let y = mt3 * y0 + 3.0 * mt2 * t * y1 + 3.0 * mt * t2 * y2 + t3 * y3;

    dvec2(x, y)
}

/// Simple random number generator using system time
fn rand_u64() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    static mut SEED: u64 = 0;
    unsafe {
        if SEED == 0 {
            SEED = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_nanos() as u64;
        }
        // LCG parameters
        SEED = SEED.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        SEED
    }
}

/// Widget reference implementation for external initialization
impl TreeChartWidgetRef {
    pub fn initialize_flare_data(&self, cx: &mut Cx) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.initialize_flare_data();
            inner.redraw(cx);
        }
    }

    pub fn initialize_random_data(&self, cx: &mut Cx) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.initialize_random_data();
            cx.new_next_frame();
            inner.redraw(cx);
        }
    }
}
