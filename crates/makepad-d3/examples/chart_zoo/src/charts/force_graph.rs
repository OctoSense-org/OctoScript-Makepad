//! Force-Directed Graph Widget
//!
//! Interactive network visualization using makepad-d3 force simulation.

use makepad_widgets::*;
use makepad_d3::prelude::*;
use makepad_d3::layout::force::{
    ForceSimulation, SimulationNode, SimulationLink,
    ManyBodyForce, LinkForce, CenterForce, CollideForce,
};
use super::draw_primitives::{DrawPoint, DrawChartLine};

live_design! {
    link widgets;
    use link::shaders::*;
    use super::draw_primitives::DrawPoint;
    use super::draw_primitives::DrawChartLine;

    pub ForceGraphWidget = {{ForceGraphWidget}} {
        width: Fill,
        height: Fill,
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct ForceGraphWidget {
    #[redraw]
    #[live]
    draw_node: DrawPoint,

    #[redraw]
    #[live]
    draw_link: DrawChartLine,

    #[walk]
    walk: Walk,

    #[rust]
    simulation: Option<ForceSimulation>,

    #[rust]
    links: Vec<(usize, usize)>,

    #[rust]
    node_colors: Vec<Vec4>,

    #[rust]
    initialized: bool,

    #[rust]
    area: Area,

    #[rust]
    center_x: f64,

    #[rust]
    center_y: f64,

    #[rust]
    scale: f64,

    #[rust]
    hovered_node: Option<usize>,

    #[rust]
    chart_rect: Rect,
}

impl Widget for ForceGraphWidget {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, _scope: &mut Scope) {
        match event {
            Event::NextFrame(_) => {
                if let Some(ref mut sim) = self.simulation {
                    if !sim.is_stable() {
                        sim.tick();
                        self.redraw(cx);
                        cx.new_next_frame();
                    }
                }
            }
            Event::MouseMove(e) => {
                self.handle_mouse_move(cx, e.abs);
            }
            _ => {}
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        let rect = cx.walk_turtle_with_area(&mut self.area, walk);

        if rect.size.x > 0.0 && rect.size.y > 0.0 {
            self.chart_rect = rect;
            self.center_x = rect.pos.x as f64 + rect.size.x as f64 / 2.0;
            self.center_y = rect.pos.y as f64 + rect.size.y as f64 / 2.0;
            self.scale = 1.5;

            if !self.initialized {
                self.initialize_graph();
                cx.new_next_frame();
            }

            self.draw_graph(cx);
        }

        DrawStep::done()
    }
}

impl ForceGraphWidget {
    fn initialize_graph(&mut self) {
        if self.initialized {
            return;
        }

        // Create a social network-like graph
        let node_count = 30;
        let nodes: Vec<SimulationNode> = (0..node_count)
            .map(|i| {
                SimulationNode::new(i)
                    .with_radius(if i < 5 { 12.0 } else { 6.0 + (i % 4) as f64 * 2.0 })
            })
            .collect();

        // Create links forming clusters
        let mut links = Vec::new();

        // Central hub connections
        for i in 1..6 {
            links.push((0, i));
        }

        // Cluster 1
        for i in 6..12 {
            links.push((1, i));
            if i > 6 {
                links.push((i - 1, i));
            }
        }

        // Cluster 2
        for i in 12..18 {
            links.push((2, i));
            if i > 12 {
                links.push((i - 1, i));
            }
        }

        // Cluster 3
        for i in 18..24 {
            links.push((3, i));
            if i > 18 {
                links.push((i - 1, i));
            }
        }

        // Cluster 4
        for i in 24..30 {
            links.push((4, i));
            if i > 24 {
                links.push((i - 1, i));
            }
        }

        // Inter-cluster connections
        links.push((6, 12));
        links.push((12, 18));
        links.push((18, 24));
        links.push((24, 6));
        links.push((9, 15));
        links.push((15, 21));
        links.push((21, 27));

        self.links = links.clone();

        // Create simulation with forces
        let simulation = ForceSimulation::new(nodes)
            .add_force("charge", ManyBodyForce::new().strength(-150.0))
            .add_force("link", LinkForce::new(links).distance(40.0).strength(0.8))
            .add_force("center", CenterForce::new().center(0.0, 0.0))
            .add_force("collide", CollideForce::new().radius(15.0).strength(0.8));

        self.simulation = Some(simulation);

        // Node colors by cluster
        self.node_colors = vec![
            vec4(0.95, 0.45, 0.45, 1.0), // Red - central
            vec4(0.45, 0.65, 0.95, 1.0), // Blue - cluster 1
            vec4(0.45, 0.85, 0.55, 1.0), // Green - cluster 2
            vec4(0.95, 0.75, 0.35, 1.0), // Orange - cluster 3
            vec4(0.75, 0.55, 0.95, 1.0), // Purple - cluster 4
        ];

        self.initialized = true;
    }

    fn get_node_color(&self, index: usize) -> Vec4 {
        let cluster = if index == 0 {
            0
        } else if index < 6 {
            0
        } else if index < 12 {
            1
        } else if index < 18 {
            2
        } else if index < 24 {
            3
        } else {
            4
        };
        self.node_colors[cluster]
    }

    fn draw_graph(&mut self, cx: &mut Cx2d) {
        let sim = match &self.simulation {
            Some(s) => s,
            None => return,
        };

        let nodes = sim.nodes();
        let scale = self.scale;

        // Build set of connected nodes for hover highlighting
        let connected_nodes: std::collections::HashSet<usize> = if let Some(hovered) = self.hovered_node {
            self.links.iter()
                .filter(|(s, t)| *s == hovered || *t == hovered)
                .flat_map(|(s, t)| vec![*s, *t])
                .collect()
        } else {
            std::collections::HashSet::new()
        };

        // Draw links first (behind nodes)
        for &(source, target) in &self.links {
            if source < nodes.len() && target < nodes.len() {
                let p1 = dvec2(
                    self.center_x + nodes[source].x * scale,
                    self.center_y + nodes[source].y * scale,
                );
                let p2 = dvec2(
                    self.center_x + nodes[target].x * scale,
                    self.center_y + nodes[target].y * scale,
                );

                // Highlight links connected to hovered node
                let is_connected = if let Some(hovered) = self.hovered_node {
                    source == hovered || target == hovered
                } else {
                    false
                };

                if is_connected {
                    self.draw_link.color = vec4(0.3, 0.6, 0.9, 0.9);
                    self.draw_link.draw_line(cx, p1, p2, 3.0);
                } else if self.hovered_node.is_some() {
                    // Dim non-connected links when hovering
                    self.draw_link.color = vec4(0.4, 0.4, 0.5, 0.15);
                    self.draw_link.draw_line(cx, p1, p2, 1.0);
                } else {
                    self.draw_link.color = vec4(0.4, 0.4, 0.5, 0.4);
                    self.draw_link.draw_line(cx, p1, p2, 1.5);
                }
            }
        }

        // Draw nodes
        for node in nodes {
            let x = self.center_x + node.x * scale;
            let y = self.center_y + node.y * scale;
            let mut color = self.get_node_color(node.id);

            let is_hovered = self.hovered_node == Some(node.id);
            let is_connected = connected_nodes.contains(&node.id);
            let has_hover = self.hovered_node.is_some();

            // Adjust color based on hover state
            if is_hovered {
                // Brighten hovered node
                color = vec4(
                    (color.x + 0.2).min(1.0),
                    (color.y + 0.2).min(1.0),
                    (color.z + 0.2).min(1.0),
                    color.w,
                );
            } else if has_hover && !is_connected {
                // Dim non-connected nodes
                color = vec4(color.x * 0.5, color.y * 0.5, color.z * 0.5, 0.4);
            }

            // Gradient effect
            let center_color = vec4(
                (color.x + 0.3).min(1.0),
                (color.y + 0.3).min(1.0),
                (color.z + 0.3).min(1.0),
                color.w,
            );

            self.draw_node.set_radial_gradient(center_color, color);

            // Make hovered node larger
            let size_mult = if is_hovered { 1.3 } else { 1.0 };
            self.draw_node.draw_point(cx, dvec2(x, y), node.radius * 2.0 * scale * size_mult);
        }
    }

    fn handle_mouse_move(&mut self, cx: &mut Cx, pos: DVec2) {
        let old_hovered = self.hovered_node;
        self.hovered_node = self.find_node_at(pos);

        if old_hovered != self.hovered_node {
            self.redraw(cx);
        }
    }

    fn find_node_at(&self, pos: DVec2) -> Option<usize> {
        let sim = self.simulation.as_ref()?;
        let nodes = sim.nodes();
        let scale = self.scale;

        for node in nodes {
            let nx = self.center_x + node.x * scale;
            let ny = self.center_y + node.y * scale;
            let radius = node.radius * scale * 1.5; // Slightly larger hit area

            let dx = pos.x - nx;
            let dy = pos.y - ny;
            let dist_sq = dx * dx + dy * dy;

            if dist_sq <= radius * radius {
                return Some(node.id);
            }
        }
        None
    }
}
