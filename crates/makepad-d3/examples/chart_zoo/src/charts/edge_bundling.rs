//! Bilevel Edge Bundling Widget
//!
//! Circular layout with bundled edges showing relationships between nodes.
//! Implements D3's hierarchical edge bundling visualization.

use makepad_widgets::*;
use std::f64::consts::PI;
use std::collections::HashMap;
use super::draw_primitives::DrawChartLine;

live_design! {
    link widgets;
    use link::shaders::*;
    use link::theme::*;

    EDGE_FONT = {
        font_family: {
            latin = font("crate://self/resources/Manrope-Regular.ttf", 0.0, 0.0),
        }
    }

    pub EdgeBundlingWidget = {{EdgeBundlingWidget}} {
        width: Fill,
        height: Fill,

        draw_text: {
            color: #333333,
            text_style: <EDGE_FONT> {
                font_size: 10.0
            }
        }

        draw_line: {
            color: #cccccc
        }
    }
}

/// Node in the graph
#[derive(Clone, Debug)]
pub struct GraphNode {
    pub id: String,
    pub group: usize,
    pub x: f64,  // Angle in radians
    pub y: f64,  // Radius
    pub targets: Vec<String>,
    pub incoming: Vec<usize>,  // Indices of incoming edges
    pub outgoing: Vec<usize>,  // Indices of outgoing edges
}

/// Edge in the graph
#[derive(Clone, Debug)]
pub struct GraphEdge {
    pub source_idx: usize,
    pub target_idx: usize,
}

/// Positioned node after cluster layout
#[derive(Clone, Debug)]
pub struct LayoutNode {
    pub id: String,
    pub group: usize,
    pub angle: f64,   // Position angle (0 to 2*PI)
    pub radius: f64,  // Distance from center
    pub incoming: Vec<usize>,
    pub outgoing: Vec<usize>,
}

#[derive(Live, LiveHook, Widget)]
pub struct EdgeBundlingWidget {
    #[live]
    #[deref]
    view: View,

    #[redraw]
    #[live]
    draw_text: DrawText,

    #[walk]
    walk: Walk,

    #[rust]
    nodes: Vec<LayoutNode>,

    #[rust]
    edges: Vec<GraphEdge>,

    #[rust]
    initialized: bool,

    #[rust]
    area: Area,

    #[rust]
    center_x: f64,

    #[rust]
    center_y: f64,

    #[rust]
    radius: f64,

    #[rust]
    hovered_node: Option<usize>,

    #[live]
    draw_line: DrawChartLine,
}

impl Widget for EdgeBundlingWidget {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);

        match event {
            Event::MouseMove(e) => {
                self.handle_mouse_move(cx, e.abs);
            }
            _ => {}
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        let rect = cx.walk_turtle_with_area(&mut self.area, walk);

        if rect.size.x > 0.0 && rect.size.y > 0.0 {
            if !self.initialized {
                self.initialize_data();
                self.initialized = true;
            }

            self.draw_chart(cx, rect);
        }

        DrawStep::done()
    }
}

impl EdgeBundlingWidget {
    fn initialize_data(&mut self) {
        // Les Misérables data - nodes and links
        let raw_nodes = vec![
            ("Myriel", 1), ("Napoleon", 1), ("Mlle.Baptistine", 1), ("Mme.Magloire", 1),
            ("CountessdeLo", 1), ("Geborand", 1), ("Champtercier", 1), ("Cravatte", 1),
            ("Count", 1), ("OldMan", 1), ("Labarre", 2), ("Valjean", 2),
            ("Marguerite", 3), ("Mme.deR", 2), ("Isabeau", 2), ("Gervais", 2),
            ("Tholomyes", 3), ("Listolier", 3), ("Fameuil", 3), ("Blacheville", 3),
            ("Favourite", 3), ("Dahlia", 3), ("Zephine", 3), ("Fantine", 3),
            ("Mme.Thenardier", 4), ("Thenardier", 4), ("Cosette", 5), ("Javert", 4),
            ("Fauchelevent", 0), ("Bamatabois", 2), ("Perpetue", 3), ("Simplice", 2),
            ("Scaufflaire", 2), ("Woman1", 2), ("Judge", 2), ("Champmathieu", 2),
            ("Brevet", 2), ("Chenildieu", 2), ("Cochepaille", 2), ("Pontmercy", 4),
            ("Boulatruelle", 6), ("Eponine", 4), ("Anzelma", 4), ("Woman2", 5),
            ("MotherInnocent", 0), ("Gribier", 0), ("Jondrette", 7), ("Mme.Burgon", 7),
            ("Gavroche", 8), ("Gillenormand", 5), ("Magnon", 5), ("Mlle.Gillenormand", 5),
            ("Mme.Pontmercy", 5), ("Mlle.Vaubois", 5), ("Lt.Gillenormand", 5), ("Marius", 8),
            ("BaronessT", 5), ("Mabeuf", 8), ("Enjolras", 8), ("Combeferre", 8),
            ("Prouvaire", 8), ("Feuilly", 8), ("Courfeyrac", 8), ("Bahorel", 8),
            ("Bossuet", 8), ("Joly", 8), ("Grantaire", 8), ("MotherPlutarch", 9),
            ("Gueulemer", 4), ("Babet", 4), ("Claquesous", 4), ("Montparnasse", 4),
            ("Toussaint", 5), ("Child1", 10), ("Child2", 10), ("Brujon", 4),
            ("Mme.Hucheloup", 8),
        ];

        let raw_links: Vec<(&str, &str)> = vec![
            ("Napoleon", "Myriel"), ("Mlle.Baptistine", "Myriel"), ("Mme.Magloire", "Myriel"),
            ("Mme.Magloire", "Mlle.Baptistine"), ("CountessdeLo", "Myriel"), ("Geborand", "Myriel"),
            ("Champtercier", "Myriel"), ("Cravatte", "Myriel"), ("Count", "Myriel"),
            ("OldMan", "Myriel"), ("Valjean", "Labarre"), ("Valjean", "Mme.Magloire"),
            ("Valjean", "Mlle.Baptistine"), ("Valjean", "Myriel"), ("Marguerite", "Valjean"),
            ("Mme.deR", "Valjean"), ("Isabeau", "Valjean"), ("Gervais", "Valjean"),
            ("Listolier", "Tholomyes"), ("Fameuil", "Tholomyes"), ("Fameuil", "Listolier"),
            ("Blacheville", "Tholomyes"), ("Blacheville", "Listolier"), ("Blacheville", "Fameuil"),
            ("Favourite", "Tholomyes"), ("Favourite", "Listolier"), ("Favourite", "Fameuil"),
            ("Favourite", "Blacheville"), ("Dahlia", "Tholomyes"), ("Dahlia", "Listolier"),
            ("Dahlia", "Fameuil"), ("Dahlia", "Blacheville"), ("Dahlia", "Favourite"),
            ("Zephine", "Tholomyes"), ("Zephine", "Listolier"), ("Zephine", "Fameuil"),
            ("Zephine", "Blacheville"), ("Zephine", "Favourite"), ("Zephine", "Dahlia"),
            ("Fantine", "Tholomyes"), ("Fantine", "Listolier"), ("Fantine", "Fameuil"),
            ("Fantine", "Blacheville"), ("Fantine", "Favourite"), ("Fantine", "Dahlia"),
            ("Fantine", "Zephine"), ("Fantine", "Marguerite"), ("Fantine", "Valjean"),
            ("Mme.Thenardier", "Fantine"), ("Mme.Thenardier", "Valjean"),
            ("Thenardier", "Mme.Thenardier"), ("Thenardier", "Fantine"), ("Thenardier", "Valjean"),
            ("Cosette", "Mme.Thenardier"), ("Cosette", "Valjean"), ("Cosette", "Tholomyes"),
            ("Cosette", "Thenardier"), ("Javert", "Valjean"), ("Javert", "Fantine"),
            ("Javert", "Thenardier"), ("Javert", "Mme.Thenardier"), ("Javert", "Cosette"),
            ("Fauchelevent", "Valjean"), ("Fauchelevent", "Javert"), ("Bamatabois", "Fantine"),
            ("Bamatabois", "Javert"), ("Bamatabois", "Valjean"), ("Perpetue", "Fantine"),
            ("Simplice", "Perpetue"), ("Simplice", "Valjean"), ("Simplice", "Fantine"),
            ("Simplice", "Javert"), ("Scaufflaire", "Valjean"), ("Woman1", "Valjean"),
            ("Woman1", "Javert"), ("Judge", "Valjean"), ("Judge", "Bamatabois"),
            ("Champmathieu", "Valjean"), ("Champmathieu", "Judge"), ("Champmathieu", "Bamatabois"),
            ("Brevet", "Judge"), ("Brevet", "Champmathieu"), ("Brevet", "Valjean"),
            ("Brevet", "Bamatabois"), ("Chenildieu", "Judge"), ("Chenildieu", "Champmathieu"),
            ("Chenildieu", "Brevet"), ("Chenildieu", "Valjean"), ("Chenildieu", "Bamatabois"),
            ("Cochepaille", "Judge"), ("Cochepaille", "Champmathieu"), ("Cochepaille", "Brevet"),
            ("Cochepaille", "Chenildieu"), ("Cochepaille", "Valjean"), ("Cochepaille", "Bamatabois"),
            ("Pontmercy", "Thenardier"), ("Boulatruelle", "Thenardier"),
            ("Eponine", "Mme.Thenardier"), ("Eponine", "Thenardier"),
            ("Anzelma", "Eponine"), ("Anzelma", "Thenardier"), ("Anzelma", "Mme.Thenardier"),
            ("Woman2", "Valjean"), ("Woman2", "Cosette"), ("Woman2", "Javert"),
            ("MotherInnocent", "Fauchelevent"), ("MotherInnocent", "Valjean"),
            ("Gribier", "Fauchelevent"), ("Mme.Burgon", "Jondrette"),
            ("Gavroche", "Mme.Burgon"), ("Gavroche", "Thenardier"), ("Gavroche", "Javert"),
            ("Gavroche", "Valjean"), ("Gillenormand", "Cosette"), ("Gillenormand", "Valjean"),
            ("Magnon", "Gillenormand"), ("Magnon", "Mme.Thenardier"),
            ("Mlle.Gillenormand", "Gillenormand"), ("Mlle.Gillenormand", "Cosette"),
            ("Mlle.Gillenormand", "Valjean"), ("Mme.Pontmercy", "Mlle.Gillenormand"),
            ("Mme.Pontmercy", "Pontmercy"), ("Mlle.Vaubois", "Mlle.Gillenormand"),
            ("Lt.Gillenormand", "Mlle.Gillenormand"), ("Lt.Gillenormand", "Gillenormand"),
            ("Lt.Gillenormand", "Cosette"), ("Marius", "Mlle.Gillenormand"),
            ("Marius", "Gillenormand"), ("Marius", "Pontmercy"), ("Marius", "Lt.Gillenormand"),
            ("Marius", "Cosette"), ("Marius", "Valjean"), ("Marius", "Tholomyes"),
            ("Marius", "Thenardier"), ("Marius", "Eponine"), ("Marius", "Gavroche"),
            ("BaronessT", "Gillenormand"), ("BaronessT", "Marius"),
            ("Mabeuf", "Marius"), ("Mabeuf", "Eponine"), ("Mabeuf", "Gavroche"),
            ("Enjolras", "Marius"), ("Enjolras", "Gavroche"), ("Enjolras", "Javert"),
            ("Enjolras", "Mabeuf"), ("Enjolras", "Valjean"),
            ("Combeferre", "Enjolras"), ("Combeferre", "Marius"), ("Combeferre", "Gavroche"),
            ("Combeferre", "Mabeuf"), ("Prouvaire", "Gavroche"), ("Prouvaire", "Enjolras"),
            ("Prouvaire", "Combeferre"), ("Feuilly", "Gavroche"), ("Feuilly", "Enjolras"),
            ("Feuilly", "Prouvaire"), ("Feuilly", "Combeferre"), ("Feuilly", "Mabeuf"),
            ("Feuilly", "Marius"), ("Courfeyrac", "Marius"), ("Courfeyrac", "Enjolras"),
            ("Courfeyrac", "Combeferre"), ("Courfeyrac", "Gavroche"), ("Courfeyrac", "Mabeuf"),
            ("Courfeyrac", "Eponine"), ("Courfeyrac", "Feuilly"), ("Courfeyrac", "Prouvaire"),
            ("Bahorel", "Combeferre"), ("Bahorel", "Gavroche"), ("Bahorel", "Courfeyrac"),
            ("Bahorel", "Mabeuf"), ("Bahorel", "Enjolras"), ("Bahorel", "Feuilly"),
            ("Bahorel", "Prouvaire"), ("Bahorel", "Marius"),
            ("Bossuet", "Marius"), ("Bossuet", "Courfeyrac"), ("Bossuet", "Gavroche"),
            ("Bossuet", "Bahorel"), ("Bossuet", "Enjolras"), ("Bossuet", "Feuilly"),
            ("Bossuet", "Prouvaire"), ("Bossuet", "Combeferre"), ("Bossuet", "Mabeuf"),
            ("Bossuet", "Valjean"), ("Joly", "Bahorel"), ("Joly", "Bossuet"),
            ("Joly", "Gavroche"), ("Joly", "Courfeyrac"), ("Joly", "Enjolras"),
            ("Joly", "Feuilly"), ("Joly", "Prouvaire"), ("Joly", "Combeferre"),
            ("Joly", "Mabeuf"), ("Joly", "Marius"),
            ("Grantaire", "Bossuet"), ("Grantaire", "Enjolras"), ("Grantaire", "Combeferre"),
            ("Grantaire", "Courfeyrac"), ("Grantaire", "Joly"), ("Grantaire", "Gavroche"),
            ("Grantaire", "Bahorel"), ("Grantaire", "Feuilly"), ("Grantaire", "Prouvaire"),
            ("MotherPlutarch", "Mabeuf"),
            ("Gueulemer", "Thenardier"), ("Gueulemer", "Valjean"), ("Gueulemer", "Mme.Thenardier"),
            ("Gueulemer", "Javert"), ("Gueulemer", "Gavroche"), ("Gueulemer", "Eponine"),
            ("Babet", "Thenardier"), ("Babet", "Gueulemer"), ("Babet", "Valjean"),
            ("Babet", "Mme.Thenardier"), ("Babet", "Javert"), ("Babet", "Gavroche"),
            ("Babet", "Eponine"), ("Claquesous", "Thenardier"), ("Claquesous", "Babet"),
            ("Claquesous", "Gueulemer"), ("Claquesous", "Valjean"), ("Claquesous", "Mme.Thenardier"),
            ("Claquesous", "Javert"), ("Claquesous", "Eponine"), ("Claquesous", "Enjolras"),
            ("Montparnasse", "Javert"), ("Montparnasse", "Babet"), ("Montparnasse", "Gueulemer"),
            ("Montparnasse", "Claquesous"), ("Montparnasse", "Valjean"), ("Montparnasse", "Gavroche"),
            ("Montparnasse", "Eponine"), ("Montparnasse", "Thenardier"),
            ("Toussaint", "Cosette"), ("Toussaint", "Javert"), ("Toussaint", "Valjean"),
            ("Child1", "Gavroche"), ("Child2", "Gavroche"), ("Child2", "Child1"),
            ("Brujon", "Babet"), ("Brujon", "Gueulemer"), ("Brujon", "Thenardier"),
            ("Brujon", "Gavroche"), ("Brujon", "Eponine"), ("Brujon", "Claquesous"),
            ("Brujon", "Montparnasse"),
            ("Mme.Hucheloup", "Bossuet"), ("Mme.Hucheloup", "Joly"), ("Mme.Hucheloup", "Grantaire"),
            ("Mme.Hucheloup", "Bahorel"), ("Mme.Hucheloup", "Courfeyrac"), ("Mme.Hucheloup", "Gavroche"),
            ("Mme.Hucheloup", "Enjolras"),
        ];

        // Build node index map
        let node_map: HashMap<&str, usize> = raw_nodes.iter()
            .enumerate()
            .map(|(i, (id, _))| (*id, i))
            .collect();

        // Create edges
        self.edges = raw_links.iter()
            .filter_map(|(source, target)| {
                let source_idx = node_map.get(source)?;
                let target_idx = node_map.get(target)?;
                Some(GraphEdge {
                    source_idx: *source_idx,
                    target_idx: *target_idx,
                })
            })
            .collect();

        // Group nodes by their group
        let mut groups: HashMap<usize, Vec<(usize, &str)>> = HashMap::new();
        for (idx, (id, group)) in raw_nodes.iter().enumerate() {
            groups.entry(*group).or_insert_with(Vec::new).push((idx, *id));
        }

        // Sort groups by group number
        let mut group_ids: Vec<usize> = groups.keys().cloned().collect();
        group_ids.sort();

        // Layout nodes in a circle, grouped together
        let total_nodes = raw_nodes.len();
        let mut angle_offset = 0.0;
        let angle_per_node = 2.0 * PI / total_nodes as f64;

        self.nodes = vec![LayoutNode {
            id: String::new(),
            group: 0,
            angle: 0.0,
            radius: 0.0,
            incoming: Vec::new(),
            outgoing: Vec::new(),
        }; total_nodes];

        for group_id in group_ids {
            if let Some(group_nodes) = groups.get(&group_id) {
                for (orig_idx, id) in group_nodes {
                    self.nodes[*orig_idx] = LayoutNode {
                        id: id.to_string(),
                        group: group_id,
                        angle: angle_offset,
                        radius: 1.0, // Will be scaled during drawing
                        incoming: Vec::new(),
                        outgoing: Vec::new(),
                    };
                    angle_offset += angle_per_node;
                }
            }
        }

        // Build incoming/outgoing lists
        for (edge_idx, edge) in self.edges.iter().enumerate() {
            self.nodes[edge.source_idx].outgoing.push(edge_idx);
            self.nodes[edge.target_idx].incoming.push(edge_idx);
        }
    }

    fn draw_chart(&mut self, cx: &mut Cx2d, rect: Rect) {
        self.center_x = (rect.pos.x + rect.size.x / 2.0) as f64;
        self.center_y = (rect.pos.y + rect.size.y / 2.0) as f64;
        self.radius = (rect.size.x.min(rect.size.y) / 2.0 - 80.0) as f64;

        // Draw edges first (behind labels)
        self.draw_edges(cx);

        // Draw node labels
        self.draw_labels(cx);
    }

    fn draw_edges(&mut self, cx: &mut Cx2d) {
        let beta = 0.85; // Bundle tension

        // Collect edge data to avoid borrow conflicts
        let edge_data: Vec<(f64, f64, f64, f64, Vec4)> = self.edges.iter().map(|edge| {
            let source_angle = self.nodes[edge.source_idx].angle;
            let target_angle = self.nodes[edge.target_idx].angle;

            // Determine color based on hover state
            let color = if let Some(hovered) = self.hovered_node {
                if edge.source_idx == hovered {
                    vec4(1.0, 0.0, 0.0, 0.8) // Red for outgoing
                } else if edge.target_idx == hovered {
                    vec4(0.0, 0.0, 1.0, 0.8) // Blue for incoming
                } else {
                    vec4(0.8, 0.8, 0.8, 0.15) // Dimmed
                }
            } else {
                vec4(0.8, 0.8, 0.8, 0.4) // Default gray
            };

            (source_angle, target_angle, self.radius, beta, color)
        }).collect();

        // Draw all edges
        for (source_angle, target_angle, radius, beta, color) in edge_data {
            self.draw_bundled_edge(cx, source_angle, target_angle, radius, beta, color);
        }
    }

    fn draw_bundled_edge(&mut self, cx: &mut Cx2d, source_angle: f64, target_angle: f64, radius: f64, beta: f64, color: Vec4) {
        // Convert polar to cartesian
        let src_x = self.center_x + radius * source_angle.cos();
        let src_y = self.center_y + radius * source_angle.sin();
        let tgt_x = self.center_x + radius * target_angle.cos();
        let tgt_y = self.center_y + radius * target_angle.sin();

        // Control point - D3's curveBundle.beta interpretation:
        // beta = 0.0 means straight line (control at midpoint)
        // beta = 1.0 means maximum bundling (control at center)
        let mid_x = (src_x + tgt_x) / 2.0;
        let mid_y = (src_y + tgt_y) / 2.0;

        // Blend: (1-beta)*midpoint + beta*center
        let ctrl_x = mid_x * (1.0 - beta) + self.center_x * beta;
        let ctrl_y = mid_y * (1.0 - beta) + self.center_y * beta;

        // Draw curve as line segments
        let segments = 20;
        for i in 0..segments {
            let t0 = i as f64 / segments as f64;
            let t1 = (i + 1) as f64 / segments as f64;

            // Quadratic bezier
            let x0 = (1.0 - t0).powi(2) * src_x + 2.0 * (1.0 - t0) * t0 * ctrl_x + t0.powi(2) * tgt_x;
            let y0 = (1.0 - t0).powi(2) * src_y + 2.0 * (1.0 - t0) * t0 * ctrl_y + t0.powi(2) * tgt_y;
            let x1 = (1.0 - t1).powi(2) * src_x + 2.0 * (1.0 - t1) * t1 * ctrl_x + t1.powi(2) * tgt_x;
            let y1 = (1.0 - t1).powi(2) * src_y + 2.0 * (1.0 - t1) * t1 * ctrl_y + t1.powi(2) * tgt_y;

            // Draw line segment
            self.draw_line_segment(cx, x0, y0, x1, y1, color);
        }
    }

    fn draw_line_segment(&mut self, cx: &mut Cx2d, x0: f64, y0: f64, x1: f64, y1: f64, color: Vec4) {
        let dx = x1 - x0;
        let dy = y1 - y0;
        let len = (dx * dx + dy * dy).sqrt();
        if len < 0.5 {
            return;
        }

        self.draw_line.color = color;
        self.draw_line.draw_line(cx, dvec2(x0, y0), dvec2(x1, y1), 1.0);
    }

    fn draw_labels(&mut self, cx: &mut Cx2d) {
        for (idx, node) in self.nodes.iter().enumerate() {
            let angle = node.angle;
            let label_radius = self.radius + 6.0;

            let x = self.center_x + label_radius * angle.cos();
            let y = self.center_y + label_radius * angle.sin();

            // Determine text anchor and rotation based on position
            let is_right_side = angle.cos() > 0.0;

            // Highlight if hovered or connected to hovered
            let is_highlighted = if let Some(hovered) = self.hovered_node {
                idx == hovered ||
                self.nodes[hovered].outgoing.iter().any(|&e| self.edges[e].target_idx == idx) ||
                self.nodes[hovered].incoming.iter().any(|&e| self.edges[e].source_idx == idx)
            } else {
                false
            };

            if is_highlighted {
                // Bold effect - draw twice with slight offset
                self.draw_text.color = vec4(0.0, 0.0, 0.0, 1.0);
            } else {
                self.draw_text.color = vec4(0.2, 0.2, 0.2, 1.0);
            }

            // Position text
            let text_width = node.id.len() as f64 * 5.5;
            let pos = if is_right_side {
                dvec2(x, y - 5.0)
            } else {
                dvec2(x - text_width, y - 5.0)
            };

            self.draw_text.draw_abs(cx, pos, &node.id);
        }
    }

    fn handle_mouse_move(&mut self, cx: &mut Cx, pos: DVec2) {
        let old_hovered = self.hovered_node;

        // Check if mouse is near any node label
        self.hovered_node = None;
        for (idx, node) in self.nodes.iter().enumerate() {
            let label_radius = self.radius + 20.0;
            let node_x = self.center_x + label_radius * node.angle.cos();
            let node_y = self.center_y + label_radius * node.angle.sin();

            let dx = pos.x - node_x;
            let dy = pos.y - node_y;
            let dist = (dx * dx + dy * dy).sqrt();

            if dist < 30.0 {
                self.hovered_node = Some(idx);
                break;
            }
        }

        if old_hovered != self.hovered_node {
            self.redraw(cx);
        }
    }
}
