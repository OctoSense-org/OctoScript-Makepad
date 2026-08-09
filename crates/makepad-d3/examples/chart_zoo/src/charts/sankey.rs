//! Sankey Diagram Widget
//!
//! GPU-accelerated flow diagram with smooth bezier curves and animated transitions.
//! Features curved flow links, gradient colors, and staggered reveal animations.

use makepad_widgets::*;
use super::draw_primitives::{DrawChartLine, DrawTriangle, DrawPoint};
use super::axis_renderer::DrawAxisText;
use super::animation::{ChartAnimator, EasingType};

live_design! {
    link widgets;
    use link::shaders::*;
    use super::draw_primitives::DrawChartLine;
    use super::draw_primitives::DrawTriangle;
    use super::draw_primitives::DrawPoint;
    use super::axis_renderer::DrawAxisText;

    pub SankeyWidget = {{SankeyWidget}} {
        width: Fill,
        height: Fill,
        draw_label: {
            color: #333333,
            text_style: {
                font_size: 10.0,
            }
        }
    }
}

#[derive(Clone)]
struct SankeyNode {
    name: String,
    layer: usize,
    value: f64,
    y: f64,
    height: f64,
    color: Vec4,
}

#[derive(Clone)]
struct SankeyLink {
    source: usize,
    target: usize,
    value: f64,
    source_y: f64,
    target_y: f64,
}

#[derive(Live, LiveHook, Widget)]
pub struct SankeyWidget {
    #[redraw]
    #[live]
    draw_flow: DrawTriangle,

    #[redraw]
    #[live]
    draw_line: DrawChartLine,

    #[redraw]
    #[live]
    draw_point: DrawPoint,

    #[redraw]
    #[live]
    draw_label: DrawAxisText,

    #[walk]
    walk: Walk,

    #[rust]
    nodes: Vec<SankeyNode>,

    #[rust]
    links: Vec<SankeyLink>,

    #[rust]
    animator: ChartAnimator,

    #[rust]
    initialized: bool,

    #[rust]
    area: Area,

    #[rust]
    chart_rect: Rect,

    #[rust]
    hovered_node: Option<usize>,

    #[rust]
    hovered_link: Option<usize>,
}

impl Widget for SankeyWidget {
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
                self.initialize_data();
                self.start_animation(cx);
                self.initialized = true;
            }

            self.draw_sankey(cx);
        }

        DrawStep::done()
    }
}

impl SankeyWidget {
    fn initialize_data(&mut self) {
        // Energy flow diagram example
        self.nodes = vec![
            // Layer 0 - Sources
            SankeyNode { name: "Coal".into(), layer: 0, value: 100.0, y: 0.0, height: 0.0,
                        color: vec4(0.35, 0.35, 0.40, 1.0) },
            SankeyNode { name: "Natural Gas".into(), layer: 0, value: 80.0, y: 0.0, height: 0.0,
                        color: vec4(0.30, 0.65, 0.85, 1.0) },
            SankeyNode { name: "Nuclear".into(), layer: 0, value: 60.0, y: 0.0, height: 0.0,
                        color: vec4(0.75, 0.40, 0.80, 1.0) },
            SankeyNode { name: "Renewables".into(), layer: 0, value: 50.0, y: 0.0, height: 0.0,
                        color: vec4(0.25, 0.72, 0.38, 1.0) },
            // Layer 1 - Processing
            SankeyNode { name: "Electricity".into(), layer: 1, value: 0.0, y: 0.0, height: 0.0,
                        color: vec4(1.0, 0.82, 0.25, 1.0) },
            SankeyNode { name: "Heat".into(), layer: 1, value: 0.0, y: 0.0, height: 0.0,
                        color: vec4(0.92, 0.38, 0.28, 1.0) },
            // Layer 2 - End use
            SankeyNode { name: "Residential".into(), layer: 2, value: 0.0, y: 0.0, height: 0.0,
                        color: vec4(0.30, 0.55, 0.95, 1.0) },
            SankeyNode { name: "Commercial".into(), layer: 2, value: 0.0, y: 0.0, height: 0.0,
                        color: vec4(0.25, 0.70, 0.38, 1.0) },
            SankeyNode { name: "Industrial".into(), layer: 2, value: 0.0, y: 0.0, height: 0.0,
                        color: vec4(0.90, 0.30, 0.25, 1.0) },
        ];

        // Links: source -> target with value
        self.links = vec![
            // Sources to Processing
            SankeyLink { source: 0, target: 4, value: 80.0, source_y: 0.0, target_y: 0.0 },
            SankeyLink { source: 0, target: 5, value: 20.0, source_y: 0.0, target_y: 0.0 },
            SankeyLink { source: 1, target: 4, value: 50.0, source_y: 0.0, target_y: 0.0 },
            SankeyLink { source: 1, target: 5, value: 30.0, source_y: 0.0, target_y: 0.0 },
            SankeyLink { source: 2, target: 4, value: 60.0, source_y: 0.0, target_y: 0.0 },
            SankeyLink { source: 3, target: 4, value: 50.0, source_y: 0.0, target_y: 0.0 },
            // Processing to End use
            SankeyLink { source: 4, target: 6, value: 80.0, source_y: 0.0, target_y: 0.0 },
            SankeyLink { source: 4, target: 7, value: 70.0, source_y: 0.0, target_y: 0.0 },
            SankeyLink { source: 4, target: 8, value: 90.0, source_y: 0.0, target_y: 0.0 },
            SankeyLink { source: 5, target: 6, value: 20.0, source_y: 0.0, target_y: 0.0 },
            SankeyLink { source: 5, target: 7, value: 15.0, source_y: 0.0, target_y: 0.0 },
            SankeyLink { source: 5, target: 8, value: 15.0, source_y: 0.0, target_y: 0.0 },
        ];

        self.compute_layout();
    }

    fn compute_layout(&mut self) {
        // Sankey layout algorithm with sankeyJustify alignment
        // Key: node heights are proportional to ABSOLUTE flow, not relative to layer
        // This creates gaps for bypass traffic

        let n = self.nodes.len();
        if n == 0 { return; }

        // Build adjacency lists
        let mut source_links: Vec<Vec<usize>> = vec![Vec::new(); n]; // outgoing
        let mut target_links: Vec<Vec<usize>> = vec![Vec::new(); n]; // incoming

        for (link_idx, link) in self.links.iter().enumerate() {
            source_links[link.source].push(link_idx);
            target_links[link.target].push(link_idx);
        }

        // Calculate incoming and outgoing totals
        let mut incoming: Vec<f64> = vec![0.0; n];
        let mut outgoing: Vec<f64> = vec![0.0; n];

        for link in &self.links {
            outgoing[link.source] += link.value;
            incoming[link.target] += link.value;
        }

        // Compute depths using two passes:
        // Pass 1: BFS from true sources (nodes with no incoming)
        // Pass 2: Adjust source nodes to align with their targets
        let mut depths: Vec<i32> = vec![-1; n];

        // Find true sources (no incoming links) - temporarily set to 0
        for i in 0..n {
            if target_links[i].is_empty() {
                depths[i] = 0;
            }
        }

        // BFS to compute depths for non-source nodes
        let mut changed = true;
        let mut iterations = 0;
        while changed && iterations < n * 2 {
            changed = false;
            iterations += 1;
            for i in 0..n {
                if !target_links[i].is_empty() && depths[i] < 0 {
                    // Find max depth of sources
                    let mut max_source_depth: i32 = -1;
                    let mut all_sources_computed = true;

                    for &link_idx in &target_links[i] {
                        let source = self.links[link_idx].source;
                        if depths[source] >= 0 {
                            max_source_depth = max_source_depth.max(depths[source]);
                        } else {
                            all_sources_computed = false;
                        }
                    }

                    if all_sources_computed && max_source_depth >= 0 {
                        depths[i] = max_source_depth + 1;
                        changed = true;
                    }
                }
            }
        }

        // Handle any remaining uncomputed nodes
        for i in 0..n {
            if depths[i] < 0 {
                depths[i] = 0;
            }
        }

        // Pass 2: Adjust source nodes to be one layer before their minimum target
        // This ensures nodes like Converse align with NIKE Brand (both feed Revenues)
        for i in 0..n {
            if target_links[i].is_empty() && !source_links[i].is_empty() {
                // This is a source node with outgoing links
                // Find the minimum depth of its targets
                let mut min_target_depth = i32::MAX;
                for &link_idx in &source_links[i] {
                    let target = self.links[link_idx].target;
                    if depths[target] < min_target_depth {
                        min_target_depth = depths[target];
                    }
                }
                if min_target_depth > 0 && min_target_depth < i32::MAX {
                    // Place this source one layer before its earliest target
                    depths[i] = min_target_depth - 1;
                }
            }
        }

        // Normalize depths so minimum is 0
        let min_depth = depths.iter().copied().min().unwrap_or(0);
        if min_depth < 0 {
            for d in &mut depths {
                *d -= min_depth;
            }
        }

        // Node value calculation:
        // - For sink nodes (no outgoing): use total incoming traffic
        // - For source nodes (no incoming): use total outgoing traffic
        // - For intermediate nodes: use max(incoming, outgoing)
        for i in 0..n {
            let is_sink = source_links[i].is_empty();
            let is_source = target_links[i].is_empty();

            let node_value = if is_sink {
                // Sink nodes: size = total incoming traffic
                incoming[i]
            } else if is_source {
                // Source nodes: size = total outgoing traffic
                outgoing[i]
            } else {
                // Intermediate nodes: size = max flow through
                incoming[i].max(outgoing[i])
            };

            self.nodes[i].value = if node_value > 0.0 { node_value } else { self.nodes[i].value };
        }

        // SANKEY JUSTIFY: Move sinks to rightmost layer, BUT keep sinks that receive
        // from a single source layer at that layer (like Interest expense from Gross profit)
        // Sinks receiving from multiple layers (like Losses) go to rightmost
        let max_depth = depths.iter().copied().max().unwrap_or(0);

        for i in 0..n {
            if source_links[i].is_empty() && self.nodes[i].value > 0.0 {
                // This is a sink - check how many different source layers it has
                let source_layers: std::collections::HashSet<i32> = target_links[i].iter()
                    .map(|&link_idx| depths[self.links[link_idx].source])
                    .collect();

                // If sink receives from multiple layers, move to rightmost (like Losses)
                // If sink receives from single layer, keep it there (like Interest expense)
                if source_layers.len() > 1 {
                    depths[i] = max_depth;
                }
                // Otherwise, sink stays at its natural depth (max source depth + 1)
            }
        }

        // Assign layers to nodes
        for i in 0..n {
            self.nodes[i].layer = depths[i] as usize;
        }

        let max_layer = self.nodes.iter().map(|n| n.layer).max().unwrap_or(0);

        // Node padding as fixed ratio
        let node_padding = 0.012; // Small fixed padding between nodes

        // D3-style: compute global scale factor based on densest layer
        // ky = min over all layers of: (available_height - padding) / layer_total_value
        let mut ky = f64::MAX;
        for layer in 0..=max_layer {
            let layer_nodes: Vec<usize> = self.nodes.iter()
                .enumerate()
                .filter(|(_, nd)| nd.layer == layer && nd.value > 0.0 && !nd.name.is_empty())
                .map(|(i, _)| i)
                .collect();

            if layer_nodes.is_empty() { continue; }

            let layer_total: f64 = layer_nodes.iter().map(|&i| self.nodes[i].value).sum();
            let num_gaps = if layer_nodes.len() > 1 { layer_nodes.len() - 1 } else { 0 };
            let available = 1.0 - (node_padding * num_gaps as f64);

            if layer_total > 0.0 {
                let layer_ky = available / layer_total;
                if layer_ky < ky {
                    ky = layer_ky;
                }
            }
        }

        if ky == f64::MAX || ky <= 0.0 {
            ky = 0.001; // Fallback
        }

        // Apply global scale to all node heights
        for i in 0..n {
            if self.nodes[i].value > 0.0 && !self.nodes[i].name.is_empty() {
                self.nodes[i].height = self.nodes[i].value * ky;
            } else {
                self.nodes[i].height = 0.0;
            }
        }

        // Group nodes by layer
        let mut layers: Vec<Vec<usize>> = vec![Vec::new(); max_layer + 1];
        for i in 0..n {
            if self.nodes[i].value > 0.0 && !self.nodes[i].name.is_empty() {
                layers[self.nodes[i].layer].push(i);
            }
        }

        // Initial vertical positioning - stack nodes in each layer
        for layer in 0..=max_layer {
            self.initialize_layer_positions(&layers[layer], node_padding);
        }

        // D3-style relaxation with alpha decay
        let iterations = 32;
        for iteration in 0..iterations {
            // Alpha decreases from 1.0 to near 0
            let alpha = 1.0 - (iteration as f64 / iterations as f64);

            // Left-to-right: move nodes toward weighted center of sources
            for layer in 1..=max_layer {
                self.relax_left_to_right(&layers[layer], alpha);
                self.resolve_collisions(&mut layers[layer], node_padding);
            }

            // Right-to-left: move nodes toward weighted center of targets
            for layer in (0..max_layer).rev() {
                self.relax_right_to_left(&layers[layer], alpha);
                self.resolve_collisions(&mut layers[layer], node_padding);
            }
        }

        // Compute link positions
        self.compute_link_positions(&incoming, &outgoing);
    }

    /// Initialize layer positions - stack nodes from top
    fn initialize_layer_positions(&mut self, layer_nodes: &[usize], node_padding: f64) {
        if layer_nodes.is_empty() { return; }

        // Calculate total height this layer needs
        let total_height: f64 = layer_nodes.iter().map(|&i| self.nodes[i].height).sum();
        let num_gaps = if layer_nodes.len() > 1 { layer_nodes.len() - 1 } else { 0 };
        let total_with_padding = total_height + node_padding * num_gaps as f64;

        // Center the layer vertically
        let start_y = (1.0 - total_with_padding) / 2.0;
        let start_y = start_y.max(0.0);

        let mut y = start_y;
        for &idx in layer_nodes {
            self.nodes[idx].y = y;
            y += self.nodes[idx].height + node_padding;
        }
    }

    /// D3-style left-to-right relaxation: move nodes toward weighted center of sources
    fn relax_left_to_right(&mut self, layer_nodes: &[usize], alpha: f64) {
        for &idx in layer_nodes {
            // Get weighted center of source nodes (incoming links)
            let mut weighted_sum = 0.0;
            let mut total_weight = 0.0;

            for link in &self.links {
                if link.target == idx {
                    let source = &self.nodes[link.source];
                    // Weighted by link value, position is center of source node
                    weighted_sum += (source.y + source.height / 2.0) * link.value;
                    total_weight += link.value;
                }
            }

            if total_weight > 0.0 {
                let target_center = weighted_sum / total_weight;
                let current_center = self.nodes[idx].y + self.nodes[idx].height / 2.0;
                // Move toward target, scaled by alpha
                let delta = (target_center - current_center) * alpha;
                self.nodes[idx].y += delta;
            }
        }
    }

    /// D3-style right-to-left relaxation: move nodes toward weighted center of targets
    fn relax_right_to_left(&mut self, layer_nodes: &[usize], alpha: f64) {
        for &idx in layer_nodes {
            // Get weighted center of target nodes (outgoing links)
            let mut weighted_sum = 0.0;
            let mut total_weight = 0.0;

            for link in &self.links {
                if link.source == idx {
                    let target = &self.nodes[link.target];
                    // Weighted by link value, position is center of target node
                    weighted_sum += (target.y + target.height / 2.0) * link.value;
                    total_weight += link.value;
                }
            }

            if total_weight > 0.0 {
                let target_center = weighted_sum / total_weight;
                let current_center = self.nodes[idx].y + self.nodes[idx].height / 2.0;
                // Move toward target, scaled by alpha
                let delta = (target_center - current_center) * alpha;
                self.nodes[idx].y += delta;
            }
        }
    }

    /// D3-style collision resolution: sort by Y, push apart, constrain to bounds
    fn resolve_collisions(&mut self, layer_nodes: &mut Vec<usize>, node_padding: f64) {
        if layer_nodes.is_empty() { return; }

        // Sort by current Y position
        layer_nodes.sort_by(|&a, &b| {
            self.nodes[a].y.partial_cmp(&self.nodes[b].y)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        // Push overlapping nodes downward
        let mut y0 = 0.0;
        for &idx in layer_nodes.iter() {
            let dy = y0 - self.nodes[idx].y;
            if dy > 0.0 {
                self.nodes[idx].y += dy;
            }
            y0 = self.nodes[idx].y + self.nodes[idx].height + node_padding;
        }

        // If we exceeded the bottom bound, push back up
        if let Some(&last_idx) = layer_nodes.last() {
            let overflow = y0 - node_padding - 1.0;
            if overflow > 0.0 {
                // Push last node up
                self.nodes[last_idx].y -= overflow;
                y0 = self.nodes[last_idx].y;

                // Cascade upward: push overlapping nodes up
                for i in (0..layer_nodes.len() - 1).rev() {
                    let idx = layer_nodes[i];
                    let next_idx = layer_nodes[i + 1];
                    let dy = self.nodes[idx].y + self.nodes[idx].height + node_padding - self.nodes[next_idx].y;
                    if dy > 0.0 {
                        self.nodes[idx].y -= dy;
                    }
                }
            }
        }

        // Ensure no node goes above y=0
        let mut y0 = 0.0;
        for &idx in layer_nodes.iter() {
            if self.nodes[idx].y < y0 {
                self.nodes[idx].y = y0;
            }
            y0 = self.nodes[idx].y + self.nodes[idx].height + node_padding;
        }
    }

    fn compute_link_positions(&mut self, _incoming: &[f64], _outgoing: &[f64]) {
        let n = self.nodes.len();

        // Sort links by source position then target position
        let mut link_indices: Vec<usize> = (0..self.links.len()).collect();
        link_indices.sort_by(|&a, &b| {
            let link_a = &self.links[a];
            let link_b = &self.links[b];
            let a_src = &self.nodes[link_a.source];
            let b_src = &self.nodes[link_b.source];
            let a_tgt = &self.nodes[link_a.target];
            let b_tgt = &self.nodes[link_b.target];

            a_src.layer.cmp(&b_src.layer)
                .then(a_src.y.partial_cmp(&b_src.y).unwrap_or(std::cmp::Ordering::Equal))
                .then(a_tgt.y.partial_cmp(&b_tgt.y).unwrap_or(std::cmp::Ordering::Equal))
        });

        let mut source_offsets: Vec<f64> = vec![0.0; n];
        let mut target_offsets: Vec<f64> = vec![0.0; n];

        for &i in &link_indices {
            let source_idx = self.links[i].source;
            let target_idx = self.links[i].target;
            let link_value = self.links[i].value;

            // Use node.value for consistent scaling (same value used for height computation)
            let source_value = self.nodes[source_idx].value;
            let target_value = self.nodes[target_idx].value;

            self.links[i].source_y = self.nodes[source_idx].y + source_offsets[source_idx];
            self.links[i].target_y = self.nodes[target_idx].y + target_offsets[target_idx];

            if source_value > 0.0 {
                source_offsets[source_idx] += link_value / source_value * self.nodes[source_idx].height;
            }
            if target_value > 0.0 {
                target_offsets[target_idx] += link_value / target_value * self.nodes[target_idx].height;
            }
        }
    }

    /// Calculate weighted position of a node based on its source connections
    fn get_weighted_source_position(&self, node_idx: usize) -> f64 {
        let mut total_weight = 0.0;
        let mut weighted_pos = 0.0;

        for link in &self.links {
            if link.target == node_idx {
                let source = &self.nodes[link.source];
                weighted_pos += (source.y + source.height / 2.0) * link.value;
                total_weight += link.value;
            }
        }

        if total_weight > 0.0 {
            weighted_pos / total_weight
        } else {
            0.5
        }
    }

    /// Calculate weighted position of a node based on its target connections
    fn get_weighted_target_position(&self, node_idx: usize) -> f64 {
        let mut total_weight = 0.0;
        let mut weighted_pos = 0.0;

        for link in &self.links {
            if link.source == node_idx {
                let target = &self.nodes[link.target];
                weighted_pos += (target.y + target.height / 2.0) * link.value;
                total_weight += link.value;
            }
        }

        if total_weight > 0.0 {
            weighted_pos / total_weight
        } else {
            0.5
        }
    }

    fn start_animation(&mut self, cx: &mut Cx) {
        let time = cx.seconds_since_app_start();
        self.animator = ChartAnimator::new(1800.0)
            .with_easing(EasingType::EaseOutCubic);
        self.animator.start(time);
        cx.new_next_frame();
    }

    pub fn replay_animation(&mut self, cx: &mut Cx) {
        self.animator.reset();
        self.start_animation(cx);
        self.redraw(cx);
    }

    fn draw_sankey(&mut self, cx: &mut Cx2d) {
        let progress = self.animator.get_progress();
        let rect = self.chart_rect;

        let padding = 50.0;
        let chart_x = rect.pos.x + padding;
        let chart_y = rect.pos.y + 40.0;
        let chart_width = rect.size.x - padding * 2.0;
        let chart_height = rect.size.y - padding - 50.0;

        if chart_width <= 0.0 || chart_height <= 0.0 {
            return;
        }

        let max_layer = self.nodes.iter().map(|n| n.layer).max().unwrap_or(0);
        let node_width = 24.0;
        let layer_spacing = if max_layer > 0 {
            (chart_width - node_width) / max_layer as f64
        } else {
            chart_width
        };

        // Precompute totals (same as in compute_layout)
        let n = self.nodes.len();
        let mut incoming: Vec<f64> = vec![0.0; n];
        let mut outgoing: Vec<f64> = vec![0.0; n];

        for link in &self.links {
            outgoing[link.source] += link.value;
            incoming[link.target] += link.value;
        }

        // Clone data for iteration
        let nodes: Vec<_> = self.nodes.clone();
        let links: Vec<_> = self.links.clone();

        // Draw links first (behind nodes)
        for (link_idx, link) in links.iter().enumerate() {
            // Stagger animation by link index, but cap it to prevent too much delay
            let stagger = (link_idx as f64 * 0.01).min(0.3);
            let link_progress = ((progress - stagger) / 0.5).clamp(0.0, 1.0);
            if link_progress <= 0.0 {
                continue;
            }

            let source = &nodes[link.source];
            let target = &nodes[link.target];

            let sx = chart_x + source.layer as f64 * layer_spacing + node_width;
            let sy = chart_y + link.source_y * chart_height;
            let tx = chart_x + target.layer as f64 * layer_spacing;
            let ty = chart_y + link.target_y * chart_height;

            // Use node.value (the actual value used for height computation) for consistent scaling
            // node.value = incoming for sinks, outgoing for sources, max(in,out) for intermediate
            let source_value = source.value;
            let target_value = target.value;

            let link_height_source = if source_value > 0.0 {
                (link.value / source_value) * source.height * chart_height
            } else { 0.0 };
            let link_height_target = if target_value > 0.0 {
                (link.value / target_value) * target.height * chart_height
            } else { 0.0 };

            // Draw curved flow with gradient
            self.draw_flow_curve(
                cx,
                sx, sy, tx, ty,
                link_height_source, link_height_target,
                source.color, target.color,
                link_progress,
            );
        }

        // Calculate layer offset for staggered animation - ensure all layers complete by progress=1.0
        let layer_offset = if max_layer > 0 { 0.3 / max_layer as f64 } else { 0.0 };

        // Draw nodes (skip zero-value or empty-name nodes)
        for (node_idx, node) in nodes.iter().enumerate() {
            // Skip nodes with no value (no connections) or empty names
            if node.value <= 0.0 || node.name.is_empty() {
                continue;
            }

            // Stagger animation by layer, but ensure all reach 1.0 when progress=1.0
            let node_progress = ((progress - 0.2 - node.layer as f64 * layer_offset) / 0.5).clamp(0.0, 1.0);
            if node_progress <= 0.0 {
                continue;
            }

            let x = chart_x + node.layer as f64 * layer_spacing;
            let y = chart_y + node.y * chart_height;

            // Node height is already correctly computed in compute_layout with global ky scale
            // Sink nodes have height = incoming * ky, which matches link widths
            let height = node.height * chart_height;

            let is_hovered = self.hovered_node == Some(node_idx);
            let scale = if is_hovered { 1.05 } else { 1.0 };

            // Draw node rectangle with gradient
            self.draw_node_rect(cx, x, y, node_width * scale, height, node.color, node_progress);
        }

        // Draw labels after nodes (on top, skip zero-value or empty-name nodes)
        for node in nodes.iter() {
            // Skip nodes with no value or empty names
            if node.value <= 0.0 || node.name.is_empty() {
                continue;
            }

            let node_progress = ((progress - 0.2 - node.layer as f64 * 0.1) / 0.4).clamp(0.0, 1.0);
            if node_progress <= 0.0 {
                continue;
            }

            let x = chart_x + node.layer as f64 * layer_spacing;
            let y = chart_y + node.y * chart_height;
            let height = node.height * chart_height;
            let label_y = y + height / 2.0 - 5.0;

            self.draw_label.color = vec4(0.25, 0.25, 0.25, node_progress as f32);

            if node.layer == 0 {
                // First layer: labels on left, right-aligned
                let text_width = node.name.len() as f64 * 6.0;
                self.draw_label.draw_abs(cx, dvec2(x - 5.0 - text_width, label_y), &node.name);
            } else if node.layer == max_layer {
                // Last layer: labels on right
                self.draw_label.draw_abs(cx, dvec2(x + node_width + 5.0, label_y), &node.name);
            } else {
                // Middle layers: labels on right of node
                self.draw_label.draw_abs(cx, dvec2(x + node_width + 3.0, label_y), &node.name);
            }
        }
    }

    fn draw_flow_curve(
        &mut self,
        cx: &mut Cx2d,
        sx: f64, sy: f64,
        tx: f64, ty: f64,
        height_source: f64, height_target: f64,
        color_source: Vec4, color_target: Vec4,
        progress: f64,
    ) {
        let segments = 32;
        let draw_segments = ((segments as f64 * progress) as usize).max(1);

        for i in 0..draw_segments {
            let t1 = i as f64 / segments as f64;
            let t2 = (i + 1) as f64 / segments as f64;

            // Smooth S-curve interpolation
            let ease1 = t1 * t1 * (3.0 - 2.0 * t1);
            let ease2 = t2 * t2 * (3.0 - 2.0 * t2);

            // Calculate positions
            let x1 = sx + (tx - sx) * t1;
            let x2 = sx + (tx - sx) * t2;
            let y1_top = sy + (ty - sy) * ease1;
            let y2_top = sy + (ty - sy) * ease2;

            // Interpolate height
            let h1 = height_source + (height_target - height_source) * ease1;
            let h2 = height_source + (height_target - height_source) * ease2;

            // Interpolate color
            let t_color = t1 as f32;
            let color = vec4(
                color_source.x + (color_target.x - color_source.x) * t_color,
                color_source.y + (color_target.y - color_source.y) * t_color,
                color_source.z + (color_target.z - color_source.z) * t_color,
                0.55 * progress as f32,
            );

            // Draw quad as two triangles
            let p1 = dvec2(x1, y1_top);
            let p2 = dvec2(x2, y2_top);
            let p3 = dvec2(x2, y2_top + h2);
            let p4 = dvec2(x1, y1_top + h1);

            self.draw_flow.color = color;
            self.draw_flow.disable_gradient();
            self.draw_flow.draw_triangle(cx, p1, p2, p3);
            self.draw_flow.draw_triangle(cx, p1, p3, p4);
        }

        // Draw edge highlights
        if progress > 0.5 {
            let edge_alpha = ((progress - 0.5) * 2.0).min(1.0) as f32;
            self.draw_line.color = vec4(1.0, 1.0, 1.0, 0.25 * edge_alpha);

            // Top edge
            for i in 0..draw_segments {
                let t1 = i as f64 / segments as f64;
                let t2 = (i + 1) as f64 / segments as f64;
                let ease1 = t1 * t1 * (3.0 - 2.0 * t1);
                let ease2 = t2 * t2 * (3.0 - 2.0 * t2);

                let x1 = sx + (tx - sx) * t1;
                let x2 = sx + (tx - sx) * t2;
                let y1 = sy + (ty - sy) * ease1;
                let y2 = sy + (ty - sy) * ease2;

                self.draw_line.draw_line(cx, dvec2(x1, y1), dvec2(x2, y2), 1.0);
            }
        }
    }

    fn draw_node_rect(
        &mut self,
        cx: &mut Cx2d,
        x: f64, y: f64,
        width: f64, height: f64,
        color: Vec4,
        _progress: f64,
    ) {
        // Flat solid color
        self.draw_flow.color = color;
        self.draw_flow.disable_gradient();

        let p1 = dvec2(x, y);
        let p2 = dvec2(x + width, y);
        let p3 = dvec2(x + width, y + height);
        let p4 = dvec2(x, y + height);

        self.draw_flow.draw_triangle(cx, p1, p2, p3);
        self.draw_flow.draw_triangle(cx, p1, p3, p4);

        // Draw border
        self.draw_line.color = vec4(0.15, 0.15, 0.18, 0.8);
        self.draw_line.draw_line(cx, p1, p2, 1.0);
        self.draw_line.draw_line(cx, p2, p3, 1.0);
        self.draw_line.draw_line(cx, p3, p4, 1.0);
        self.draw_line.draw_line(cx, p4, p1, 1.0);
    }

    /// Initialize with Titanic survival data (D3 parallel sets style)
    /// Data from D3 brexit-voting example's Titanic dataset
    pub fn initialize_titanic_data(&mut self) {
        // Titanic data: Class -> Sex -> Age -> Survived
        // From D3 CSV: Survived,Sex,Age,Class,value
        let color_first = vec4(0.12, 0.47, 0.71, 1.0);   // Blue
        let color_second = vec4(0.17, 0.63, 0.17, 1.0);  // Green
        let color_third = vec4(1.00, 0.50, 0.05, 1.0);   // Orange
        let color_crew = vec4(0.58, 0.40, 0.74, 1.0);    // Purple
        let color_male = vec4(0.30, 0.55, 0.85, 1.0);    // Light blue
        let color_female = vec4(0.85, 0.45, 0.65, 1.0);  // Pink
        let color_adult = vec4(0.55, 0.34, 0.29, 1.0);   // Brown
        let color_child = vec4(0.89, 0.47, 0.76, 1.0);   // Light pink
        let color_survived = vec4(0.25, 0.72, 0.38, 1.0); // Green
        let color_perished = vec4(0.85, 0.31, 0.31, 1.0); // Red

        self.nodes = vec![
            // 0: First Class
            SankeyNode { name: "First Class".into(), layer: 0, value: 0.0, y: 0.0, height: 0.0, color: color_first },
            // 1: Second Class
            SankeyNode { name: "Second Class".into(), layer: 0, value: 0.0, y: 0.0, height: 0.0, color: color_second },
            // 2: Third Class
            SankeyNode { name: "Third Class".into(), layer: 0, value: 0.0, y: 0.0, height: 0.0, color: color_third },
            // 3: Crew
            SankeyNode { name: "Crew".into(), layer: 0, value: 0.0, y: 0.0, height: 0.0, color: color_crew },
            // 4: Male
            SankeyNode { name: "Male".into(), layer: 0, value: 0.0, y: 0.0, height: 0.0, color: color_male },
            // 5: Female
            SankeyNode { name: "Female".into(), layer: 0, value: 0.0, y: 0.0, height: 0.0, color: color_female },
            // 6: Adult
            SankeyNode { name: "Adult".into(), layer: 0, value: 0.0, y: 0.0, height: 0.0, color: color_adult },
            // 7: Child
            SankeyNode { name: "Child".into(), layer: 0, value: 0.0, y: 0.0, height: 0.0, color: color_child },
            // 8: Survived
            SankeyNode { name: "Survived".into(), layer: 0, value: 0.0, y: 0.0, height: 0.0, color: color_survived },
            // 9: Perished
            SankeyNode { name: "Perished".into(), layer: 0, value: 0.0, y: 0.0, height: 0.0, color: color_perished },
        ];

        // Links aggregated from Titanic CSV data
        // Class -> Sex
        self.links = vec![
            // First Class to Sex (total 325: Male 180, Female 145)
            SankeyLink { source: 0, target: 4, value: 180.0, source_y: 0.0, target_y: 0.0 },
            SankeyLink { source: 0, target: 5, value: 145.0, source_y: 0.0, target_y: 0.0 },
            // Second Class to Sex (total 285: Male 179, Female 106)
            SankeyLink { source: 1, target: 4, value: 179.0, source_y: 0.0, target_y: 0.0 },
            SankeyLink { source: 1, target: 5, value: 106.0, source_y: 0.0, target_y: 0.0 },
            // Third Class to Sex (total 706: Male 510, Female 196)
            SankeyLink { source: 2, target: 4, value: 510.0, source_y: 0.0, target_y: 0.0 },
            SankeyLink { source: 2, target: 5, value: 196.0, source_y: 0.0, target_y: 0.0 },
            // Crew to Sex (total 885: Male 862, Female 23)
            SankeyLink { source: 3, target: 4, value: 862.0, source_y: 0.0, target_y: 0.0 },
            SankeyLink { source: 3, target: 5, value: 23.0, source_y: 0.0, target_y: 0.0 },
            // Sex to Age
            // Male (1731 total): Adult 1667, Child 64
            SankeyLink { source: 4, target: 6, value: 1667.0, source_y: 0.0, target_y: 0.0 },
            SankeyLink { source: 4, target: 7, value: 64.0, source_y: 0.0, target_y: 0.0 },
            // Female (470 total): Adult 425, Child 45
            SankeyLink { source: 5, target: 6, value: 425.0, source_y: 0.0, target_y: 0.0 },
            SankeyLink { source: 5, target: 7, value: 45.0, source_y: 0.0, target_y: 0.0 },
            // Age to Outcome
            // Adult (2092): Survived 654, Perished 1438
            SankeyLink { source: 6, target: 8, value: 654.0, source_y: 0.0, target_y: 0.0 },
            SankeyLink { source: 6, target: 9, value: 1438.0, source_y: 0.0, target_y: 0.0 },
            // Child (109): Survived 57, Perished 52
            SankeyLink { source: 7, target: 8, value: 57.0, source_y: 0.0, target_y: 0.0 },
            SankeyLink { source: 7, target: 9, value: 52.0, source_y: 0.0, target_y: 0.0 },
        ];

        self.compute_layout();
        self.initialized = true;
    }

    /// Initialize with Nike quarterly financial data (D3 nike-quarterly-statement style)
    pub fn initialize_nike_data(&mut self) {
        // Nike Q3 FY2019 quarterly financial flow from D3 example
        // Complete flow: Products -> Regions -> Brands -> Revenues -> Costs/Profit
        let color_blue = vec4(0.12, 0.47, 0.71, 1.0);
        let color_orange = vec4(1.00, 0.50, 0.05, 1.0);
        let color_green = vec4(0.17, 0.63, 0.17, 1.0);
        let color_red = vec4(0.84, 0.15, 0.16, 1.0);
        let color_purple = vec4(0.58, 0.40, 0.74, 1.0);
        let color_brown = vec4(0.55, 0.34, 0.29, 1.0);
        let color_pink = vec4(0.89, 0.47, 0.76, 1.0);
        let color_gray = vec4(0.50, 0.50, 0.50, 1.0);
        let color_teal = vec4(0.09, 0.75, 0.81, 1.0);

        self.nodes = vec![
            // 0: Footwear
            SankeyNode { name: "Footwear".into(), layer: 0, value: 0.0, y: 0.0, height: 0.0, color: color_blue },
            // 1: Apparel
            SankeyNode { name: "Apparel".into(), layer: 0, value: 0.0, y: 0.0, height: 0.0, color: color_orange },
            // 2: Equipment
            SankeyNode { name: "Equipment".into(), layer: 0, value: 0.0, y: 0.0, height: 0.0, color: color_green },
            // 3: North America
            SankeyNode { name: "North America".into(), layer: 0, value: 0.0, y: 0.0, height: 0.0, color: color_red },
            // 4: EMEA
            SankeyNode { name: "EMEA".into(), layer: 0, value: 0.0, y: 0.0, height: 0.0, color: color_purple },
            // 5: Greater China
            SankeyNode { name: "Greater China".into(), layer: 0, value: 0.0, y: 0.0, height: 0.0, color: color_brown },
            // 6: Asia Pacific & Latin America
            SankeyNode { name: "APLA".into(), layer: 0, value: 0.0, y: 0.0, height: 0.0, color: color_pink },
            // 7: Global Brand Divisions
            SankeyNode { name: "Global Brand".into(), layer: 0, value: 0.0, y: 0.0, height: 0.0, color: color_gray },
            // 8: NIKE Brand
            SankeyNode { name: "NIKE Brand".into(), layer: 0, value: 0.0, y: 0.0, height: 0.0, color: color_teal },
            // 9: Converse
            SankeyNode { name: "Converse".into(), layer: 0, value: 0.0, y: 0.0, height: 0.0, color: color_red },
            // 10: Corporate
            SankeyNode { name: "Corporate".into(), layer: 0, value: 0.0, y: 0.0, height: 0.0, color: color_gray },
            // 11: Revenues
            SankeyNode { name: "Revenues".into(), layer: 0, value: 0.0, y: 0.0, height: 0.0, color: color_green },
            // 12: Cost of sales
            SankeyNode { name: "Cost of sales".into(), layer: 0, value: 0.0, y: 0.0, height: 0.0, color: color_red },
            // 13: Gross profit
            SankeyNode { name: "Gross profit".into(), layer: 0, value: 0.0, y: 0.0, height: 0.0, color: color_green },
            // 14: Selling & admin expense
            SankeyNode { name: "Selling & admin".into(), layer: 0, value: 0.0, y: 0.0, height: 0.0, color: color_orange },
            // 15: Interest expense
            SankeyNode { name: "Interest expense".into(), layer: 0, value: 0.0, y: 0.0, height: 0.0, color: color_red },
            // 16: Income before taxes
            SankeyNode { name: "Income before tax".into(), layer: 0, value: 0.0, y: 0.0, height: 0.0, color: color_blue },
            // 17: Other income
            SankeyNode { name: "Other income".into(), layer: 0, value: 0.0, y: 0.0, height: 0.0, color: color_teal },
            // 18: Demand creation expense
            SankeyNode { name: "Demand creation".into(), layer: 0, value: 0.0, y: 0.0, height: 0.0, color: color_purple },
            // 19: Operating overhead expense
            SankeyNode { name: "Operating overhead".into(), layer: 0, value: 0.0, y: 0.0, height: 0.0, color: color_brown },
            // 20: Tax expense
            SankeyNode { name: "Tax expense".into(), layer: 0, value: 0.0, y: 0.0, height: 0.0, color: color_red },
            // 21: Net income
            SankeyNode { name: "Net income".into(), layer: 0, value: 0.0, y: 0.0, height: 0.0, color: color_green },
        ];

        // Links from D3 nike-quarterly-statement data
        self.links = vec![
            // Products to Regions
            SankeyLink { source: 0, target: 3, value: 2245.0, source_y: 0.0, target_y: 0.0 }, // Footwear -> North America
            SankeyLink { source: 0, target: 4, value: 1419.0, source_y: 0.0, target_y: 0.0 }, // Footwear -> EMEA
            SankeyLink { source: 0, target: 5, value: 1022.0, source_y: 0.0, target_y: 0.0 }, // Footwear -> Greater China
            SankeyLink { source: 0, target: 6, value: 879.0, source_y: 0.0, target_y: 0.0 },  // Footwear -> APLA
            SankeyLink { source: 1, target: 3, value: 1405.0, source_y: 0.0, target_y: 0.0 }, // Apparel -> North America
            SankeyLink { source: 1, target: 4, value: 794.0, source_y: 0.0, target_y: 0.0 },  // Apparel -> EMEA
            SankeyLink { source: 1, target: 6, value: 360.0, source_y: 0.0, target_y: 0.0 },  // Apparel -> APLA
            SankeyLink { source: 1, target: 5, value: 490.0, source_y: 0.0, target_y: 0.0 },  // Apparel -> Greater China
            SankeyLink { source: 2, target: 3, value: 132.0, source_y: 0.0, target_y: 0.0 },  // Equipment -> North America
            SankeyLink { source: 2, target: 4, value: 100.0, source_y: 0.0, target_y: 0.0 },  // Equipment -> EMEA
            SankeyLink { source: 2, target: 5, value: 32.0, source_y: 0.0, target_y: 0.0 },   // Equipment -> Greater China
            SankeyLink { source: 2, target: 6, value: 59.0, source_y: 0.0, target_y: 0.0 },   // Equipment -> APLA
            // Regions to NIKE Brand
            SankeyLink { source: 3, target: 8, value: 3782.0, source_y: 0.0, target_y: 0.0 }, // North America -> NIKE Brand
            SankeyLink { source: 4, target: 8, value: 2313.0, source_y: 0.0, target_y: 0.0 }, // EMEA -> NIKE Brand
            SankeyLink { source: 5, target: 8, value: 1544.0, source_y: 0.0, target_y: 0.0 }, // Greater China -> NIKE Brand
            SankeyLink { source: 6, target: 8, value: 1298.0, source_y: 0.0, target_y: 0.0 }, // APLA -> NIKE Brand
            SankeyLink { source: 7, target: 8, value: 9.0, source_y: 0.0, target_y: 0.0 },    // Global Brand -> NIKE Brand
            // Brands to Revenues
            SankeyLink { source: 8, target: 11, value: 8946.0, source_y: 0.0, target_y: 0.0 }, // NIKE Brand -> Revenues
            SankeyLink { source: 9, target: 11, value: 425.0, source_y: 0.0, target_y: 0.0 },  // Converse -> Revenues
            SankeyLink { source: 10, target: 11, value: 3.0, source_y: 0.0, target_y: 0.0 },   // Corporate -> Revenues
            // Revenues split
            SankeyLink { source: 11, target: 12, value: 5269.0, source_y: 0.0, target_y: 0.0 }, // Revenues -> Cost of sales
            SankeyLink { source: 11, target: 13, value: 4105.0, source_y: 0.0, target_y: 0.0 }, // Revenues -> Gross profit
            // Gross profit split
            SankeyLink { source: 13, target: 14, value: 3142.0, source_y: 0.0, target_y: 0.0 }, // Gross profit -> Selling & admin
            SankeyLink { source: 13, target: 15, value: 14.0, source_y: 0.0, target_y: 0.0 },   // Gross profit -> Interest expense
            SankeyLink { source: 13, target: 16, value: 949.0, source_y: 0.0, target_y: 0.0 },  // Gross profit -> Income before taxes
            // Other income
            SankeyLink { source: 17, target: 16, value: 48.0, source_y: 0.0, target_y: 0.0 },   // Other income -> Income before taxes
            // Selling & admin split
            SankeyLink { source: 14, target: 18, value: 910.0, source_y: 0.0, target_y: 0.0 },  // Selling & admin -> Demand creation
            SankeyLink { source: 14, target: 19, value: 2232.0, source_y: 0.0, target_y: 0.0 }, // Selling & admin -> Operating overhead
            // Income before taxes split
            SankeyLink { source: 16, target: 20, value: 150.0, source_y: 0.0, target_y: 0.0 },  // Income before taxes -> Tax expense
            SankeyLink { source: 16, target: 21, value: 847.0, source_y: 0.0, target_y: 0.0 },  // Income before taxes -> Net income
        ];

        self.compute_layout();
        self.initialized = true;
    }

    /// Initialize with UK energy flow data (D3 basic sankey style)
    /// Complete data from the actual D3 energy-sankey example (48 nodes, 68 links)
    pub fn initialize_energy_uk_data(&mut self) {
        // Complete UK energy flow diagram from D3 example
        // Node layers will be auto-computed by BFS algorithm (like D3)
        // All nodes start with layer 0, compute_layout will assign proper depths

        // Use D3's schemeCategory10 colors based on category prefix
        let color_agricultural = vec4(0.45, 0.72, 0.32, 1.0);  // Green for bio
        let color_coal = vec4(0.35, 0.35, 0.40, 1.0);          // Gray
        let color_oil = vec4(0.55, 0.27, 0.07, 1.0);           // Brown
        let color_gas = vec4(0.30, 0.65, 0.85, 1.0);           // Blue
        let color_nuclear = vec4(0.75, 0.40, 0.80, 1.0);       // Purple
        let color_solar = vec4(1.0, 0.82, 0.25, 1.0);          // Yellow
        let color_renewable = vec4(0.55, 0.85, 0.95, 1.0);     // Light blue
        let color_thermal = vec4(0.92, 0.38, 0.28, 1.0);       // Red-orange
        let color_electricity = vec4(0.90, 0.55, 0.20, 1.0);   // Orange
        let color_heating = vec4(0.85, 0.35, 0.35, 1.0);       // Red
        let color_industry = vec4(0.84, 0.15, 0.16, 1.0);      // Dark red
        let color_transport = vec4(0.50, 0.50, 0.70, 1.0);     // Blue-gray
        let color_losses = vec4(0.65, 0.65, 0.65, 1.0);        // Gray
        let color_other = vec4(0.60, 0.40, 0.60, 1.0);         // Purple

        self.nodes = vec![
            // Index 0: Agricultural 'waste'
            SankeyNode { name: "Agricultural 'waste'".into(), layer: 0, value: 0.0, y: 0.0, height: 0.0, color: color_agricultural },
            // Index 1: Bio-conversion
            SankeyNode { name: "Bio-conversion".into(), layer: 0, value: 0.0, y: 0.0, height: 0.0, color: color_agricultural },
            // Index 2: Liquid
            SankeyNode { name: "Liquid".into(), layer: 0, value: 0.0, y: 0.0, height: 0.0, color: color_oil },
            // Index 3: Losses
            SankeyNode { name: "Losses".into(), layer: 0, value: 0.0, y: 0.0, height: 0.0, color: color_losses },
            // Index 4: Solid
            SankeyNode { name: "Solid".into(), layer: 0, value: 0.0, y: 0.0, height: 0.0, color: color_coal },
            // Index 5: Gas
            SankeyNode { name: "Gas".into(), layer: 0, value: 0.0, y: 0.0, height: 0.0, color: color_gas },
            // Index 6: Biofuel imports
            SankeyNode { name: "Biofuel imports".into(), layer: 0, value: 0.0, y: 0.0, height: 0.0, color: color_agricultural },
            // Index 7: Biomass imports
            SankeyNode { name: "Biomass imports".into(), layer: 0, value: 0.0, y: 0.0, height: 0.0, color: color_agricultural },
            // Index 8: Coal imports
            SankeyNode { name: "Coal imports".into(), layer: 0, value: 0.0, y: 0.0, height: 0.0, color: color_coal },
            // Index 9: Coal reserves
            SankeyNode { name: "Coal reserves".into(), layer: 0, value: 0.0, y: 0.0, height: 0.0, color: color_coal },
            // Index 10: Coal
            SankeyNode { name: "Coal".into(), layer: 0, value: 0.0, y: 0.0, height: 0.0, color: color_coal },
            // Index 11: District heating
            SankeyNode { name: "District heating".into(), layer: 0, value: 0.0, y: 0.0, height: 0.0, color: color_heating },
            // Index 12: Industry
            SankeyNode { name: "Industry".into(), layer: 0, value: 0.0, y: 0.0, height: 0.0, color: color_industry },
            // Index 13: Heating and cooling - commercial
            SankeyNode { name: "Heating - Loss".into(), layer: 0, value: 0.0, y: 0.0, height: 0.0, color: color_heating },
            // Index 14: Heating and cooling - homes
            SankeyNode { name: "Heating - homes".into(), layer: 0, value: 0.0, y: 0.0, height: 0.0, color: color_heating },
            // Index 15: Electricity grid
            SankeyNode { name: "Electricity grid".into(), layer: 0, value: 0.0, y: 0.0, height: 0.0, color: color_electricity },
            // Index 16: Over generation / exports
            SankeyNode { name: "Over generation".into(), layer: 0, value: 0.0, y: 0.0, height: 0.0, color: color_other },
            // Index 17: H2 conversion
            SankeyNode { name: "H2 conversion".into(), layer: 0, value: 0.0, y: 0.0, height: 0.0, color: color_gas },
            // Index 18: Road transport
            SankeyNode { name: "Road transport".into(), layer: 0, value: 0.0, y: 0.0, height: 0.0, color: color_transport },
            // Index 19: Agriculture
            SankeyNode { name: "Agriculture".into(), layer: 0, value: 0.0, y: 0.0, height: 0.0, color: color_agricultural },
            // Index 20: Rail transport
            SankeyNode { name: "Rail transport".into(), layer: 0, value: 0.0, y: 0.0, height: 0.0, color: color_transport },
            // Index 21: Lighting & appliances - commercial
            SankeyNode { name: "Lighting - comm".into(), layer: 0, value: 0.0, y: 0.0, height: 0.0, color: color_solar },
            // Index 22: Lighting & appliances - homes
            SankeyNode { name: "Lighting - homes".into(), layer: 0, value: 0.0, y: 0.0, height: 0.0, color: color_solar },
            // Index 23: Gas imports
            SankeyNode { name: "Gas imports".into(), layer: 0, value: 0.0, y: 0.0, height: 0.0, color: color_gas },
            // Index 24: Gas reserves
            SankeyNode { name: "Gas reserves".into(), layer: 0, value: 0.0, y: 0.0, height: 0.0, color: color_gas },
            // Index 25: Ngas
            SankeyNode { name: "Ngas".into(), layer: 0, value: 0.0, y: 0.0, height: 0.0, color: color_gas },
            // Index 26: Thermal generation
            SankeyNode { name: "Thermal generation".into(), layer: 0, value: 0.0, y: 0.0, height: 0.0, color: color_thermal },
            // Index 27: Geothermal
            SankeyNode { name: "Geothermal".into(), layer: 0, value: 0.0, y: 0.0, height: 0.0, color: color_thermal },
            // Index 28: H2
            SankeyNode { name: "H2".into(), layer: 0, value: 0.0, y: 0.0, height: 0.0, color: color_gas },
            // Index 29: Hydro
            SankeyNode { name: "Hydro".into(), layer: 0, value: 0.0, y: 0.0, height: 0.0, color: color_renewable },
            // Index 30: International shipping
            SankeyNode { name: "Int'l shipping".into(), layer: 0, value: 0.0, y: 0.0, height: 0.0, color: color_transport },
            // Index 31: Domestic aviation
            SankeyNode { name: "Domestic aviation".into(), layer: 0, value: 0.0, y: 0.0, height: 0.0, color: color_transport },
            // Index 32: International aviation
            SankeyNode { name: "Int'l aviation".into(), layer: 0, value: 0.0, y: 0.0, height: 0.0, color: color_transport },
            // Index 33: National navigation
            SankeyNode { name: "National navigation".into(), layer: 0, value: 0.0, y: 0.0, height: 0.0, color: color_transport },
            // Index 34: Marine algae
            SankeyNode { name: "Marine algae".into(), layer: 0, value: 0.0, y: 0.0, height: 0.0, color: color_agricultural },
            // Index 35: Nuclear
            SankeyNode { name: "Nuclear".into(), layer: 0, value: 0.0, y: 0.0, height: 0.0, color: color_nuclear },
            // Index 36: Oil imports
            SankeyNode { name: "Oil imports".into(), layer: 0, value: 0.0, y: 0.0, height: 0.0, color: color_oil },
            // Index 37: Oil reserves
            SankeyNode { name: "Oil reserves".into(), layer: 0, value: 0.0, y: 0.0, height: 0.0, color: color_oil },
            // Index 38: Oil
            SankeyNode { name: "Oil".into(), layer: 0, value: 0.0, y: 0.0, height: 0.0, color: color_oil },
            // Index 39: Other waste
            SankeyNode { name: "Other waste".into(), layer: 0, value: 0.0, y: 0.0, height: 0.0, color: color_other },
            // Index 40: Pumped heat
            SankeyNode { name: "Pumped heat".into(), layer: 0, value: 0.0, y: 0.0, height: 0.0, color: color_heating },
            // Index 41: Solar PV
            SankeyNode { name: "Solar PV".into(), layer: 0, value: 0.0, y: 0.0, height: 0.0, color: color_solar },
            // Index 42: Solar Thermal
            SankeyNode { name: "Solar Thermal".into(), layer: 0, value: 0.0, y: 0.0, height: 0.0, color: color_solar },
            // Index 43: Solar
            SankeyNode { name: "Solar".into(), layer: 0, value: 0.0, y: 0.0, height: 0.0, color: color_solar },
            // Index 44: Tidal
            SankeyNode { name: "Tidal".into(), layer: 0, value: 0.0, y: 0.0, height: 0.0, color: color_renewable },
            // Index 45: UK land based bioenergy
            SankeyNode { name: "UK land bioenergy".into(), layer: 0, value: 0.0, y: 0.0, height: 0.0, color: color_agricultural },
            // Index 46: Wave
            SankeyNode { name: "Wave".into(), layer: 0, value: 0.0, y: 0.0, height: 0.0, color: color_renewable },
            // Index 47: Wind
            SankeyNode { name: "Wind".into(), layer: 0, value: 0.0, y: 0.0, height: 0.0, color: color_renewable },
        ];

        // All 68 links from D3 energy.csv
        self.links = vec![
            // Agricultural 'waste' -> Bio-conversion: 124.729
            SankeyLink { source: 0, target: 1, value: 124.729, source_y: 0.0, target_y: 0.0 },
            // Bio-conversion -> Liquid: 0.597
            SankeyLink { source: 1, target: 2, value: 0.597, source_y: 0.0, target_y: 0.0 },
            // Bio-conversion -> Losses: 26.862
            SankeyLink { source: 1, target: 3, value: 26.862, source_y: 0.0, target_y: 0.0 },
            // Bio-conversion -> Solid: 280.322
            SankeyLink { source: 1, target: 4, value: 280.322, source_y: 0.0, target_y: 0.0 },
            // Bio-conversion -> Gas: 81.144
            SankeyLink { source: 1, target: 5, value: 81.144, source_y: 0.0, target_y: 0.0 },
            // Biofuel imports -> Liquid: 35
            SankeyLink { source: 6, target: 2, value: 35.0, source_y: 0.0, target_y: 0.0 },
            // Biomass imports -> Solid: 35
            SankeyLink { source: 7, target: 4, value: 35.0, source_y: 0.0, target_y: 0.0 },
            // Coal imports -> Coal: 11.606
            SankeyLink { source: 8, target: 10, value: 11.606, source_y: 0.0, target_y: 0.0 },
            // Coal reserves -> Coal: 63.965
            SankeyLink { source: 9, target: 10, value: 63.965, source_y: 0.0, target_y: 0.0 },
            // Coal -> Solid: 75.571
            SankeyLink { source: 10, target: 4, value: 75.571, source_y: 0.0, target_y: 0.0 },
            // District heating -> Industry: 10.639
            SankeyLink { source: 11, target: 12, value: 10.639, source_y: 0.0, target_y: 0.0 },
            // District heating -> Heating and cooling - commercial: 22.505
            SankeyLink { source: 11, target: 13, value: 22.505, source_y: 0.0, target_y: 0.0 },
            // District heating -> Heating and cooling - homes: 46.184
            SankeyLink { source: 11, target: 14, value: 46.184, source_y: 0.0, target_y: 0.0 },
            // Electricity grid -> Over generation / exports: 104.453
            SankeyLink { source: 15, target: 16, value: 104.453, source_y: 0.0, target_y: 0.0 },
            // Electricity grid -> Heating and cooling - homes: 113.726
            SankeyLink { source: 15, target: 14, value: 113.726, source_y: 0.0, target_y: 0.0 },
            // Electricity grid -> H2 conversion: 27.14
            SankeyLink { source: 15, target: 17, value: 27.14, source_y: 0.0, target_y: 0.0 },
            // Electricity grid -> Industry: 342.165
            SankeyLink { source: 15, target: 12, value: 342.165, source_y: 0.0, target_y: 0.0 },
            // Electricity grid -> Road transport: 37.797
            SankeyLink { source: 15, target: 18, value: 37.797, source_y: 0.0, target_y: 0.0 },
            // Electricity grid -> Agriculture: 4.412
            SankeyLink { source: 15, target: 19, value: 4.412, source_y: 0.0, target_y: 0.0 },
            // Electricity grid -> Heating and cooling - commercial: 40.858
            SankeyLink { source: 15, target: 13, value: 40.858, source_y: 0.0, target_y: 0.0 },
            // Electricity grid -> Losses: 56.691
            SankeyLink { source: 15, target: 3, value: 56.691, source_y: 0.0, target_y: 0.0 },
            // Electricity grid -> Rail transport: 7.863
            SankeyLink { source: 15, target: 20, value: 7.863, source_y: 0.0, target_y: 0.0 },
            // Electricity grid -> Lighting & appliances - commercial: 90.008
            SankeyLink { source: 15, target: 21, value: 90.008, source_y: 0.0, target_y: 0.0 },
            // Electricity grid -> Lighting & appliances - homes: 93.494
            SankeyLink { source: 15, target: 22, value: 93.494, source_y: 0.0, target_y: 0.0 },
            // Gas imports -> Ngas: 40.719
            SankeyLink { source: 23, target: 25, value: 40.719, source_y: 0.0, target_y: 0.0 },
            // Gas reserves -> Ngas: 82.233
            SankeyLink { source: 24, target: 25, value: 82.233, source_y: 0.0, target_y: 0.0 },
            // Gas -> Heating and cooling - commercial: 0.129
            SankeyLink { source: 5, target: 13, value: 0.129, source_y: 0.0, target_y: 0.0 },
            // Gas -> Losses: 1.401
            SankeyLink { source: 5, target: 3, value: 1.401, source_y: 0.0, target_y: 0.0 },
            // Gas -> Thermal generation: 151.891
            SankeyLink { source: 5, target: 26, value: 151.891, source_y: 0.0, target_y: 0.0 },
            // Gas -> Agriculture: 2.096
            SankeyLink { source: 5, target: 19, value: 2.096, source_y: 0.0, target_y: 0.0 },
            // Gas -> Industry: 48.58
            SankeyLink { source: 5, target: 12, value: 48.58, source_y: 0.0, target_y: 0.0 },
            // Geothermal -> Electricity grid: 7.013
            SankeyLink { source: 27, target: 15, value: 7.013, source_y: 0.0, target_y: 0.0 },
            // H2 conversion -> H2: 20.897
            SankeyLink { source: 17, target: 28, value: 20.897, source_y: 0.0, target_y: 0.0 },
            // H2 conversion -> Losses: 6.242
            SankeyLink { source: 17, target: 3, value: 6.242, source_y: 0.0, target_y: 0.0 },
            // H2 -> Road transport: 20.897
            SankeyLink { source: 28, target: 18, value: 20.897, source_y: 0.0, target_y: 0.0 },
            // Hydro -> Electricity grid: 6.995
            SankeyLink { source: 29, target: 15, value: 6.995, source_y: 0.0, target_y: 0.0 },
            // Liquid -> Industry: 121.066
            SankeyLink { source: 2, target: 12, value: 121.066, source_y: 0.0, target_y: 0.0 },
            // Liquid -> International shipping: 128.69
            SankeyLink { source: 2, target: 30, value: 128.69, source_y: 0.0, target_y: 0.0 },
            // Liquid -> Road transport: 135.835
            SankeyLink { source: 2, target: 18, value: 135.835, source_y: 0.0, target_y: 0.0 },
            // Liquid -> Domestic aviation: 14.458
            SankeyLink { source: 2, target: 31, value: 14.458, source_y: 0.0, target_y: 0.0 },
            // Liquid -> International aviation: 206.267
            SankeyLink { source: 2, target: 32, value: 206.267, source_y: 0.0, target_y: 0.0 },
            // Liquid -> Agriculture: 3.64
            SankeyLink { source: 2, target: 19, value: 3.64, source_y: 0.0, target_y: 0.0 },
            // Liquid -> National navigation: 33.218
            SankeyLink { source: 2, target: 33, value: 33.218, source_y: 0.0, target_y: 0.0 },
            // Liquid -> Rail transport: 4.413
            SankeyLink { source: 2, target: 20, value: 4.413, source_y: 0.0, target_y: 0.0 },
            // Marine algae -> Bio-conversion: 4.375
            SankeyLink { source: 34, target: 1, value: 4.375, source_y: 0.0, target_y: 0.0 },
            // Ngas -> Gas: 122.952
            SankeyLink { source: 25, target: 5, value: 122.952, source_y: 0.0, target_y: 0.0 },
            // Nuclear -> Thermal generation: 839.978
            SankeyLink { source: 35, target: 26, value: 839.978, source_y: 0.0, target_y: 0.0 },
            // Oil imports -> Oil: 504.287
            SankeyLink { source: 36, target: 38, value: 504.287, source_y: 0.0, target_y: 0.0 },
            // Oil reserves -> Oil: 107.703
            SankeyLink { source: 37, target: 38, value: 107.703, source_y: 0.0, target_y: 0.0 },
            // Oil -> Liquid: 611.99
            SankeyLink { source: 38, target: 2, value: 611.99, source_y: 0.0, target_y: 0.0 },
            // Other waste -> Solid: 56.587
            SankeyLink { source: 39, target: 4, value: 56.587, source_y: 0.0, target_y: 0.0 },
            // Other waste -> Bio-conversion: 77.81
            SankeyLink { source: 39, target: 1, value: 77.81, source_y: 0.0, target_y: 0.0 },
            // Pumped heat -> Heating and cooling - homes: 193.026
            SankeyLink { source: 40, target: 14, value: 193.026, source_y: 0.0, target_y: 0.0 },
            // Pumped heat -> Heating and cooling - commercial: 70.672
            SankeyLink { source: 40, target: 13, value: 70.672, source_y: 0.0, target_y: 0.0 },
            // Solar PV -> Electricity grid: 59.901
            SankeyLink { source: 41, target: 15, value: 59.901, source_y: 0.0, target_y: 0.0 },
            // Solar Thermal -> Heating and cooling - homes: 19.263
            SankeyLink { source: 42, target: 14, value: 19.263, source_y: 0.0, target_y: 0.0 },
            // Solar -> Solar Thermal: 19.263
            SankeyLink { source: 43, target: 42, value: 19.263, source_y: 0.0, target_y: 0.0 },
            // Solar -> Solar PV: 59.901
            SankeyLink { source: 43, target: 41, value: 59.901, source_y: 0.0, target_y: 0.0 },
            // Solid -> Agriculture: 0.882
            SankeyLink { source: 4, target: 19, value: 0.882, source_y: 0.0, target_y: 0.0 },
            // Solid -> Thermal generation: 400.12
            SankeyLink { source: 4, target: 26, value: 400.12, source_y: 0.0, target_y: 0.0 },
            // Solid -> Industry: 46.477
            SankeyLink { source: 4, target: 12, value: 46.477, source_y: 0.0, target_y: 0.0 },
            // Thermal generation -> Electricity grid: 525.531
            SankeyLink { source: 26, target: 15, value: 525.531, source_y: 0.0, target_y: 0.0 },
            // Thermal generation -> Losses: 787.129
            SankeyLink { source: 26, target: 3, value: 787.129, source_y: 0.0, target_y: 0.0 },
            // Thermal generation -> District heating: 79.329
            SankeyLink { source: 26, target: 11, value: 79.329, source_y: 0.0, target_y: 0.0 },
            // Tidal -> Electricity grid: 9.452
            SankeyLink { source: 44, target: 15, value: 9.452, source_y: 0.0, target_y: 0.0 },
            // UK land based bioenergy -> Bio-conversion: 182.01
            SankeyLink { source: 45, target: 1, value: 182.01, source_y: 0.0, target_y: 0.0 },
            // Wave -> Electricity grid: 19.013
            SankeyLink { source: 46, target: 15, value: 19.013, source_y: 0.0, target_y: 0.0 },
            // Wind -> Electricity grid: 289.366
            SankeyLink { source: 47, target: 15, value: 289.366, source_y: 0.0, target_y: 0.0 },
        ];

        self.compute_layout();
        self.initialized = true;
    }
}

/// Widget reference implementation for external initialization
impl SankeyWidgetRef {
    pub fn initialize_titanic_data(&self, cx: &mut Cx) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.initialize_titanic_data();
            inner.start_animation(cx);
        }
    }

    pub fn initialize_nike_data(&self, cx: &mut Cx) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.initialize_nike_data();
            inner.start_animation(cx);
        }
    }

    pub fn initialize_energy_uk_data(&self, cx: &mut Cx) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.initialize_energy_uk_data();
            inner.start_animation(cx);
        }
    }
}
