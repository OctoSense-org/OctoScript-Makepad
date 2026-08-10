//! Chord Diagram Widget
//!
//! Shows relationships between groups using arcs and connecting ribbons.
//! A classic D3 visualization for flow/relationship data.
//! GPU-accelerated with proper animation and hover support.

use makepad_widgets::*;
use std::f64::consts::PI;
use super::draw_primitives::{DrawArc, DrawTriangle};
use super::axis_renderer::DrawAxisText;
use super::animation::{ChartAnimator, EasingType, get_color, lighten};

live_design! {
    link widgets;
    use link::shaders::*;
    use super::draw_primitives::DrawArc;
    use super::draw_primitives::DrawTriangle;
    use super::axis_renderer::DrawAxisText;

    pub ChordDiagramWidget = {{ChordDiagramWidget}} {
        width: Fill,
        height: Fill,
        draw_label: {
            color: #333333,
            text_style: {
                font_size: 9.0,
            }
        }
    }
}

/// Data structure for chord diagrams
#[derive(Clone, Debug, Default)]
pub struct ChordData {
    pub labels: Vec<String>,
    pub matrix: Vec<Vec<f64>>,
}

impl ChordData {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_labels<S: Into<String>>(mut self, labels: Vec<S>) -> Self {
        self.labels = labels.into_iter().map(|s| s.into()).collect();
        self
    }

    pub fn with_matrix(mut self, matrix: Vec<Vec<f64>>) -> Self {
        self.matrix = matrix;
        self
    }
}

/// Computed group arc for layout
#[derive(Clone, Debug)]
struct GroupArc {
    index: usize,
    start_angle: f64,
    end_angle: f64,
    value: f64,
    color: Vec4,
    label: String,
    /// Track how much of this group's angle has been consumed by chords (single offset for both in/out)
    offset: f64,
}

/// Computed chord for layout
#[derive(Clone, Debug)]
struct Chord {
    source_index: usize,
    #[allow(dead_code)]
    target_index: usize,
    source_start: f64,
    source_end: f64,
    target_start: f64,
    target_end: f64,
    #[allow(dead_code)]
    value: f64,
}

#[derive(Live, LiveHook, Widget)]
pub struct ChordDiagramWidget {
    #[live]
    #[deref]
    view: View,

    #[redraw]
    #[live]
    draw_arc: DrawArc,

    #[redraw]
    #[live]
    draw_ribbon: DrawTriangle,

    #[redraw]
    #[live]
    draw_label: DrawAxisText,

    #[walk]
    walk: Walk,

    #[rust]
    chord_data: ChordData,

    #[rust]
    animator: ChartAnimator,

    #[rust]
    initialized: bool,

    #[rust]
    center: DVec2,

    #[rust]
    radius: f64,

    #[rust(0.0)]
    padding: f64,

    /// Gap between groups in radians
    #[rust(0.04)]
    gap_angle: f64,

    /// Outer arc thickness ratio (0.0-1.0)
    #[rust(0.08)]
    arc_thickness: f64,

    #[rust(-1)]
    hovered_group: i32,

    #[rust(-1)]
    hovered_chord: i32,

    /// Enable gradient on ribbons (radial from center)
    #[rust(true)]
    gradient_enabled: bool,

    /// Enable directed mode (asymmetric ribbons)
    #[rust(false)]
    directed_mode: bool,

    /// Enable arc gradient
    #[rust(false)]
    arc_gradient_enabled: bool,

    /// Show tick labels around perimeter (phone brand style)
    #[live(false)]
    show_tick_labels: bool,

    /// Use absolute values for tick labels instead of percentages
    #[rust(false)]
    use_absolute_labels: bool,

    /// Custom tick step (0.0 means auto-calculate)
    #[rust(0.0)]
    tick_step: f64,

    /// Computed layout
    #[rust]
    groups: Vec<GroupArc>,

    #[rust]
    chords: Vec<Chord>,

    #[rust]
    area: Area,

    /// Custom colors for groups (optional, overrides default palette)
    #[rust]
    custom_colors: Option<Vec<Vec4>>,
}

impl Widget for ChordDiagramWidget {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);

        match event {
            Event::MouseMove(e) => {
                self.handle_mouse_move(cx, e.abs);
            }
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
            self.update_layout(rect);

            if !self.initialized {
                self.initialize_data();
                self.compute_chord_layout();
                self.start_animation(cx);
                self.initialized = true;
            }

            self.draw_chords(cx);
            self.draw_group_arcs(cx);
            self.draw_labels(cx);
        }

        DrawStep::done()
    }
}

impl ChordDiagramWidget {
    fn initialize_data(&mut self) {
        // Default to phone brand data (matches first chart in detail page)
        if self.chord_data.matrix.is_empty() {
            self.initialize_phone_data();
        }
    }

    pub fn set_data(&mut self, data: ChordData) {
        self.chord_data = data;
        self.initialized = false;
    }

    pub fn set_gap_angle(&mut self, gap: f64) {
        self.gap_angle = gap;
        self.initialized = false;
    }

    pub fn set_arc_thickness(&mut self, thickness: f64) {
        self.arc_thickness = thickness.clamp(0.01, 0.5);
        self.initialized = false;
    }

    pub fn set_gradient(&mut self, enabled: bool) {
        self.gradient_enabled = enabled;
    }

    pub fn set_directed(&mut self, enabled: bool) {
        self.directed_mode = enabled;
    }

    pub fn set_arc_gradient(&mut self, enabled: bool) {
        self.arc_gradient_enabled = enabled;
    }

    pub fn set_show_tick_labels(&mut self, enabled: bool) {
        self.show_tick_labels = enabled;
    }

    fn update_layout(&mut self, rect: Rect) {
        let size = rect.size.x.min(rect.size.y) - self.padding * 2.0;
        self.radius = size / 2.0;
        self.center = dvec2(
            rect.pos.x + rect.size.x / 2.0,
            rect.pos.y + rect.size.y / 2.0,
        );
    }

    fn start_animation(&mut self, cx: &mut Cx) {
        let time = cx.seconds_since_app_start();
        self.animator = ChartAnimator::new(800.0) // 800ms animation
            .with_easing(EasingType::EaseOutCubic);
        self.animator.start(time);
        cx.new_next_frame();
    }

    pub fn replay_animation(&mut self, cx: &mut Cx) {
        self.initialized = false;
        self.animator.reset();
        self.redraw(cx);
    }

    pub fn is_animating(&self) -> bool {
        self.animator.is_running()
    }

    fn compute_chord_layout(&mut self) {
        self.groups.clear();
        self.chords.clear();

        let n = self.chord_data.matrix.len();
        if n == 0 {
            return;
        }

        // Group totals depend on mode:
        // - Directed: row_sum + column_sum (outgoing + incoming ribbons)
        // - Undirected: row_sum only (ribbons are merged)
        let mut group_totals: Vec<f64> = vec![0.0; n];
        for i in 0..n {
            for j in 0..n {
                if i < self.chord_data.matrix.len() && j < self.chord_data.matrix[i].len() {
                    // Outgoing from i
                    group_totals[i] += self.chord_data.matrix[i][j];
                }
                if self.directed_mode {
                    // In directed mode, also add incoming to i (column sum)
                    if j < self.chord_data.matrix.len() && i < self.chord_data.matrix[j].len() {
                        group_totals[i] += self.chord_data.matrix[j][i];
                    }
                }
            }
        }

        let total_value: f64 = group_totals.iter().sum();
        if total_value == 0.0 {
            return;
        }

        // Calculate available angle (after gaps)
        let total_gap = self.gap_angle * n as f64;
        let available_angle = 2.0 * PI - total_gap;

        // Scaling factor: maps flow values to angles
        let k = available_angle / total_value;

        // Assign angles to groups
        let mut current_angle = -PI / 2.0;
        for i in 0..n {
            let group_angle = group_totals[i] * k;

            let label = if i < self.chord_data.labels.len() {
                self.chord_data.labels[i].clone()
            } else {
                format!("Group {}", i + 1)
            };

            let color = if let Some(ref colors) = self.custom_colors {
                if i < colors.len() {
                    colors[i]
                } else {
                    get_color(i)
                }
            } else {
                get_color(i)
            };

            self.groups.push(GroupArc {
                index: i,
                start_angle: current_angle,
                end_angle: current_angle + group_angle,
                value: group_totals[i],
                color,
                label,
                offset: 0.0,
            });

            current_angle += group_angle + self.gap_angle;
        }

        if self.directed_mode {
            // DIRECTED MODE (d3.chordDirected):
            // For each group, process INCOMING flows first, then OUTGOING flows
            // Both consume angle within that group's arc
            // This matches D3's approach where subgroupIndex includes both [-n..0) and [0..n)

            // We need to track: for each (i,j) pair, where does source start and target start?
            // Source angle is assigned when processing outgoing from i
            // Target angle is assigned when processing incoming to j

            // First pass: assign target angles (incoming flows to each group)
            // For group j, incoming flows are matrix[i][j] for all i
            let mut target_angles: Vec<Vec<(f64, f64)>> = vec![vec![(0.0, 0.0); n]; n];

            for j in 0..n {
                for i in 0..n {
                    if i >= self.chord_data.matrix.len() || j >= self.chord_data.matrix[i].len() {
                        continue;
                    }
                    let value = self.chord_data.matrix[i][j];
                    if value <= 0.0 {
                        continue;
                    }

                    let chord_angle = value * k;
                    let target_start = self.groups[j].start_angle + self.groups[j].offset;
                    let target_end = target_start + chord_angle;
                    self.groups[j].offset += chord_angle;

                    target_angles[i][j] = (target_start, target_end);
                }
            }

            // Second pass: assign source angles (outgoing flows from each group) and create chords
            for i in 0..n {
                for j in 0..n {
                    if i >= self.chord_data.matrix.len() || j >= self.chord_data.matrix[i].len() {
                        continue;
                    }
                    let value = self.chord_data.matrix[i][j];
                    if value <= 0.0 {
                        continue;
                    }

                    let chord_angle = value * k;
                    let source_start = self.groups[i].start_angle + self.groups[i].offset;
                    let source_end = source_start + chord_angle;
                    self.groups[i].offset += chord_angle;

                    let (target_start, target_end) = target_angles[i][j];

                    self.chords.push(Chord {
                        source_index: i,
                        target_index: j,
                        source_start,
                        source_end,
                        target_start,
                        target_end,
                        value,
                    });
                }
            }
        } else {
            // UNDIRECTED MODE (d3.chord): Merge [i][j] and [j][i] into single ribbon
            use std::collections::HashMap;
            let mut chord_map: HashMap<(usize, usize), Chord> = HashMap::new();

            for i in 0..n {
                for j in 0..n {
                    if i >= self.chord_data.matrix.len() || j >= self.chord_data.matrix[i].len() {
                        continue;
                    }

                    let value = self.chord_data.matrix[i][j];
                    if value <= 0.0 {
                        continue;
                    }

                    // Consume angle at group i
                    let chord_angle = value * k;
                    let angle_start = self.groups[i].start_angle + self.groups[i].offset;
                    let angle_end = angle_start + chord_angle;
                    self.groups[i].offset += chord_angle;

                    // Key for merging: (smaller_index, larger_index)
                    let key = if i <= j { (i, j) } else { (j, i) };

                    let chord = chord_map.entry(key).or_insert(Chord {
                        source_index: key.0,
                        target_index: key.1,
                        source_start: 0.0,
                        source_end: 0.0,
                        target_start: 0.0,
                        target_end: 0.0,
                        value: 0.0,
                    });

                    if i <= j {
                        chord.source_start = angle_start;
                        chord.source_end = angle_end;
                        chord.value += value;
                        if i == j {
                            chord.target_start = angle_start;
                            chord.target_end = angle_end;
                        }
                    } else {
                        chord.target_start = angle_start;
                        chord.target_end = angle_end;
                    }
                }
            }

            self.chords = chord_map.into_values()
                .filter(|c| c.source_end > c.source_start || c.target_end > c.target_start)
                .collect();
        }
    }

    fn draw_group_arcs(&mut self, cx: &mut Cx2d) {
        let progress = self.animator.get_progress();

        let outer_radius = self.radius;
        let inner_radius = self.radius * (1.0 - self.arc_thickness);

        // Collect arc info to avoid borrow issues
        let arc_info: Vec<_> = self.groups.iter().enumerate().map(|(i, group)| {
            let is_hovered = self.hovered_group >= 0 && self.hovered_group as usize == i;
            let sweep = (group.end_angle - group.start_angle) * progress;
            let color = if is_hovered {
                lighten(group.color, 0.2)
            } else {
                group.color
            };
            (group.start_angle, sweep, color)
        }).collect();

        for (start_angle, sweep, color) in arc_info {
            if sweep < 0.001 {
                continue;
            }

            self.draw_arc.color = color;

            if self.arc_gradient_enabled {
                let inner_color = lighten(color, 0.3);
                self.draw_arc.set_radial_gradient(inner_color, color);
            } else {
                self.draw_arc.disable_gradient();
            }

            self.draw_arc.set_arc(start_angle, sweep, inner_radius, outer_radius);
            self.draw_arc.draw_arc(cx, self.center, outer_radius);
        }
    }

    fn draw_labels(&mut self, cx: &mut Cx2d) {
        let progress = self.animator.get_progress();
        if progress < 0.5 {
            return; // Don't show labels until animation is halfway
        }

        let label_alpha = ((progress - 0.5) * 2.0).min(1.0) as f32;
        let outer_radius = self.radius;
        let label_radius = outer_radius + 8.0;

        // Clone groups to avoid borrow issues
        let groups: Vec<_> = self.groups.iter().map(|g| {
            (g.start_angle, g.end_angle, g.label.clone(), g.value)
        }).collect();

        if self.show_tick_labels {
            // Tick marks with labels (percentage or absolute values)
            let tick_outer = outer_radius + 8.0;
            let tick_label_radius = outer_radius + 18.0;

            // Calculate total value for percentage mode
            let total_value: f64 = self.groups.iter().map(|g| g.value).sum();

            // Determine tick step
            let tick_step_value = if self.tick_step > 0.0 {
                self.tick_step
            } else {
                // Default: 1% of total for percentage mode
                total_value * 0.01
            };

            // Major tick interval for labels (show every Nth tick)
            let major_tick_interval = if self.use_absolute_labels {
                tick_step_value  // Show label at every tick for absolute mode
            } else {
                total_value * 0.01  // 1% intervals for percentage mode
            };

            // Draw tick marks and labels for each group
            for (start_angle, end_angle, group_name, value) in &groups {
                if *value <= 0.0 {
                    continue;
                }

                // k converts value to angle within this group's arc
                let k = (end_angle - start_angle) / value;

                // Generate ticks from 0 to group's value
                let mut tick_value = 0.0;
                let mut tick_count = 0;
                let mut is_first_tick = true;
                while tick_value <= *value + 0.0001 {
                    let angle = start_angle + tick_value * k;

                    // Draw tick line
                    let inner_x = self.center.x + outer_radius * angle.cos();
                    let inner_y = self.center.y + outer_radius * angle.sin();
                    let outer_x = self.center.x + tick_outer * angle.cos();
                    let outer_y = self.center.y + tick_outer * angle.sin();

                    let perp_x = -angle.sin() * 0.5;
                    let perp_y = angle.cos() * 0.5;
                    self.draw_ribbon.color = vec4(0.3, 0.3, 0.3, label_alpha);
                    self.draw_ribbon.draw_triangle(
                        cx,
                        dvec2(inner_x - perp_x, inner_y - perp_y),
                        dvec2(inner_x + perp_x, inner_y + perp_y),
                        dvec2(outer_x, outer_y),
                    );

                    // Determine if we should show a label at this tick
                    let show_label = if self.use_absolute_labels {
                        // Show label at major tick intervals
                        is_first_tick || (tick_value % major_tick_interval).abs() < 0.1
                    } else {
                        // Percentage mode: show at 1% intervals
                        let tick_pct = if total_value > 0.0 {
                            (tick_value / total_value) * 100.0
                        } else {
                            0.0
                        };
                        let rounded_pct = (tick_pct * 10.0).round() / 10.0;
                        is_first_tick || (rounded_pct % 1.0).abs() < 0.1
                    };

                    if show_label {
                        let lx = self.center.x + tick_label_radius * angle.cos();
                        let ly = self.center.y + tick_label_radius * angle.sin();

                        let label_text = if is_first_tick {
                            group_name.clone()
                        } else if self.use_absolute_labels {
                            // Format as "5K", "10K", etc.
                            if tick_value >= 1000.0 {
                                format!("{}K", (tick_value / 1000.0) as i64)
                            } else {
                                format!("{}", tick_value as i64)
                            }
                        } else {
                            // Percentage format
                            let tick_pct = (tick_value / total_value) * 100.0;
                            format!("{:.0}%", tick_pct)
                        };
                        let text_width = label_text.len() as f64 * 5.0;

                        let (final_x, final_y) = if angle > -PI / 2.0 && angle < PI / 2.0 {
                            (lx + 3.0, ly - 4.0)
                        } else {
                            (lx - text_width - 3.0, ly - 4.0)
                        };

                        // Dark text for visibility
                        self.draw_label.color = vec4(0.1, 0.1, 0.1, label_alpha);
                        self.draw_label.draw_abs(cx, dvec2(final_x, final_y), &label_text);
                    }

                    is_first_tick = false;
                    tick_value += tick_step_value;
                    tick_count += 1;
                    if tick_count > 1000 {
                        break;
                    }
                }
            }
        } else {
            // Simple style: just group names at center of each arc
            for (start_angle, end_angle, label, _value) in groups {
                let mid_angle = (start_angle + end_angle) / 2.0;

                let x = self.center.x + label_radius * mid_angle.cos();
                let y = self.center.y + label_radius * mid_angle.sin();

                let text_width = label.len() as f64 * 5.5;

                let (label_x, label_y) = if mid_angle > -PI / 2.0 && mid_angle < PI / 2.0 {
                    (x + 3.0, y - 4.0)
                } else {
                    (x - text_width - 3.0, y - 4.0)
                };

                self.draw_label.color = vec4(0.25, 0.25, 0.25, label_alpha);
                self.draw_label.draw_abs(cx, dvec2(label_x, label_y), &label);
            }
        }
    }

    fn draw_chords(&mut self, cx: &mut Cx2d) {
        let progress = self.animator.get_progress();

        let inner_radius = self.radius * (1.0 - self.arc_thickness);

        // Collect chord drawing info to avoid borrow issues
        let draw_info: Vec<_> = self.chords.iter().enumerate().map(|(chord_idx, chord)| {
            let is_hovered = self.hovered_chord >= 0 && self.hovered_chord as usize == chord_idx;

            // D3 colors ribbons by TARGET index (the recipient of the flow)
            let color_index = if self.directed_mode {
                chord.target_index
            } else {
                chord.source_index
            };
            let base_color = if color_index < self.groups.len() {
                self.groups[color_index].color
            } else {
                get_color(color_index)
            };

            let alpha = if is_hovered { 0.6 } else { 0.3 };  // Semi-transparent ribbons
            let color = vec4(base_color.x, base_color.y, base_color.z, alpha as f32);

            (chord.source_start, chord.source_end, chord.target_start, chord.target_end, color, base_color)
        }).collect();

        // Animate the ribbon by scaling the radius
        let animated_radius = inner_radius * progress;

        if animated_radius < 1.0 {
            return;
        }

        let directed = self.directed_mode;
        let gradient = self.gradient_enabled;

        for (source_start, source_end, target_start, target_end, color, base_color) in draw_info {
            if directed {
                self.draw_directed_ribbon(
                    cx,
                    source_start,
                    source_end,
                    target_start,
                    target_end,
                    animated_radius,
                    color,
                    base_color,
                    gradient,
                );
            } else {
                self.draw_ribbon_shape(
                    cx,
                    source_start,
                    source_end,
                    target_start,
                    target_end,
                    animated_radius,
                    color,
                    base_color,
                    gradient,
                );
            }
        }
    }

    fn draw_ribbon_shape(
        &mut self,
        cx: &mut Cx2d,
        source_start: f64,
        source_end: f64,
        target_start: f64,
        target_end: f64,
        radius: f64,
        color: Vec4,
        _base_color: Vec4,
        _gradient: bool,
    ) {
        // D3 ribbon "saddle" shape:
        // - Wide at source (source arc on circle)
        // - Narrow in middle (beziers curve toward center)
        // - Wide at target (target arc on circle)
        //
        // The ribbon has two "edges" (bezier curves) and two "caps" (arcs):
        // - Edge 1: bezier from source_end to target_start
        // - Edge 2: bezier from source_start to target_end
        // - Source cap: arc from source_start to source_end
        // - Target cap: arc from target_start to target_end

        let segments = 20;

        // Set up color
        self.draw_ribbon.color = color;
        self.draw_ribbon.disable_gradient();

        // Corner points
        let p_source_start = dvec2(
            self.center.x + radius * source_start.cos(),
            self.center.y + radius * source_start.sin(),
        );
        let p_source_end = dvec2(
            self.center.x + radius * source_end.cos(),
            self.center.y + radius * source_end.sin(),
        );
        let p_target_start = dvec2(
            self.center.x + radius * target_start.cos(),
            self.center.y + radius * target_start.sin(),
        );
        let p_target_end = dvec2(
            self.center.x + radius * target_end.cos(),
            self.center.y + radius * target_end.sin(),
        );

        // Edge 1: bezier from source_end to target_start (one side of ribbon)
        let mut edge1: Vec<DVec2> = Vec::with_capacity(segments + 1);
        for i in 0..=segments {
            let t = i as f64 / segments as f64;
            edge1.push(self.quadratic_bezier(p_source_end, self.center, p_target_start, t));
        }

        // Edge 2: bezier from source_start to target_end (other side of ribbon)
        let mut edge2: Vec<DVec2> = Vec::with_capacity(segments + 1);
        for i in 0..=segments {
            let t = i as f64 / segments as f64;
            edge2.push(self.quadratic_bezier(p_source_start, self.center, p_target_end, t));
        }

        // Triangle strip between edge1 and edge2 (main ribbon body)
        // At t=0: edge1[0]=source_end, edge2[0]=source_start (source side, wide)
        // At t=0.5: both edges near center (narrow waist)
        // At t=1: edge1[n]=target_start, edge2[n]=target_end (target side, wide)
        for i in 0..segments {
            let a = edge1[i];
            let b = edge1[i + 1];
            let c = edge2[i];
            let d = edge2[i + 1];
            self.draw_ribbon.draw_triangle(cx, a, b, c);
            self.draw_ribbon.draw_triangle(cx, b, d, c);
        }

        // Source cap: fill arc from source_start to source_end
        // This is a circular segment - fan triangulate from source_end
        for i in 1..=segments {
            let t_prev = (i - 1) as f64 / segments as f64;
            let t_curr = i as f64 / segments as f64;
            let angle_prev = source_start + (source_end - source_start) * t_prev;
            let angle_curr = source_start + (source_end - source_start) * t_curr;
            let p_prev = dvec2(
                self.center.x + radius * angle_prev.cos(),
                self.center.y + radius * angle_prev.sin(),
            );
            let p_curr = dvec2(
                self.center.x + radius * angle_curr.cos(),
                self.center.y + radius * angle_curr.sin(),
            );
            self.draw_ribbon.draw_triangle(cx, p_source_end, p_prev, p_curr);
        }

        // Target cap: fill arc from target_start to target_end
        // Fan triangulate from target_end
        for i in 1..=segments {
            let t_prev = (i - 1) as f64 / segments as f64;
            let t_curr = i as f64 / segments as f64;
            let angle_prev = target_start + (target_end - target_start) * t_prev;
            let angle_curr = target_start + (target_end - target_start) * t_curr;
            let p_prev = dvec2(
                self.center.x + radius * angle_prev.cos(),
                self.center.y + radius * angle_prev.sin(),
            );
            let p_curr = dvec2(
                self.center.x + radius * angle_curr.cos(),
                self.center.y + radius * angle_curr.sin(),
            );
            self.draw_ribbon.draw_triangle(cx, p_target_end, p_prev, p_curr);
        }
    }

    /// Quadratic bezier point at parameter t
    fn quadratic_bezier(&self, p0: DVec2, p1: DVec2, p2: DVec2, t: f64) -> DVec2 {
        let mt = 1.0 - t;
        dvec2(
            mt * mt * p0.x + 2.0 * mt * t * p1.x + t * t * p2.x,
            mt * mt * p0.y + 2.0 * mt * t * p1.y + t * t * p2.y,
        )
    }

    fn draw_directed_ribbon(
        &mut self,
        cx: &mut Cx2d,
        source_start: f64,
        source_end: f64,
        target_start: f64,
        target_end: f64,
        radius: f64,
        color: Vec4,
        base_color: Vec4,
        gradient: bool,
    ) {
        // Directed ribbon uses the same saddle shape as undirected
        self.draw_ribbon_shape(
            cx,
            source_start,
            source_end,
            target_start,
            target_end,
            radius,
            color,
            base_color,
            gradient,
        );
    }

    fn handle_mouse_move(&mut self, cx: &mut Cx, pos: DVec2) {
        let old_group = self.hovered_group;
        let old_chord = self.hovered_chord;
        self.hovered_group = -1;
        self.hovered_chord = -1;

        let dx = pos.x - self.center.x;
        let dy = pos.y - self.center.y;
        let dist = (dx * dx + dy * dy).sqrt();

        if dist > self.radius {
            if old_group != self.hovered_group || old_chord != self.hovered_chord {
                self.redraw(cx);
            }
            return;
        }

        let angle = dy.atan2(dx);
        let outer_radius = self.radius;
        let inner_radius = self.radius * (1.0 - self.arc_thickness);

        // Check if hovering over group arcs
        if dist >= inner_radius && dist <= outer_radius {
            for (i, group) in self.groups.iter().enumerate() {
                if self.is_angle_in_range(angle, group.start_angle, group.end_angle) {
                    self.hovered_group = i as i32;
                    break;
                }
            }
        }

        if old_group != self.hovered_group || old_chord != self.hovered_chord {
            self.redraw(cx);
        }
    }

    fn is_angle_in_range(&self, angle: f64, start: f64, end: f64) -> bool {
        let mut check_angle = angle;
        let mut range_start = start;
        let mut range_end = end;

        while check_angle < 0.0 {
            check_angle += 2.0 * PI;
        }
        while range_start < 0.0 {
            range_start += 2.0 * PI;
        }
        while range_end < 0.0 {
            range_end += 2.0 * PI;
        }

        check_angle >= range_start && check_angle < range_end
    }

    /// Initialize with phone brand switching data (D3 basic chord example)
    pub fn initialize_phone_data(&mut self) {
        // Phone brand switching data from D3 chord diagram example
        // Shows consumer shifts between phone brands
        self.chord_data = ChordData::new()
            .with_labels(vec!["Apple", "HTC", "Huawei", "LG", "Nokia", "Samsung", "Sony", "Other"])
            .with_matrix(vec![
                vec![0.096899, 0.008859, 0.000554, 0.004430, 0.025471, 0.024363, 0.005537, 0.025471],
                vec![0.001107, 0.018272, 0.000000, 0.004983, 0.011074, 0.010520, 0.002215, 0.004983],
                vec![0.000554, 0.002769, 0.002215, 0.002215, 0.003876, 0.008306, 0.000554, 0.003322],
                vec![0.000554, 0.001107, 0.000554, 0.012182, 0.011628, 0.006645, 0.004983, 0.010520],
                vec![0.002215, 0.004430, 0.000000, 0.002769, 0.104097, 0.012182, 0.004983, 0.028239],
                vec![0.011628, 0.026024, 0.000000, 0.013843, 0.087486, 0.168328, 0.017165, 0.055925],
                vec![0.000554, 0.004983, 0.000000, 0.003322, 0.004430, 0.008859, 0.017719, 0.004430],
                vec![0.002215, 0.007198, 0.000000, 0.003322, 0.016611, 0.014950, 0.001107, 0.054264],
            ]);

        // Use phone brand colors
        self.custom_colors = Some(vec![
            vec4(0.77, 0.77, 0.77, 1.0),  // Apple - gray
            vec4(0.41, 0.70, 0.06, 1.0),  // HTC - green
            vec4(0.93, 0.11, 0.15, 1.0),  // Huawei - red
            vec4(0.78, 0.07, 0.36, 1.0),  // LG - magenta
            vec4(0.00, 0.56, 0.78, 1.0),  // Nokia - blue
            vec4(0.06, 0.13, 0.55, 1.0),  // Samsung - dark blue
            vec4(0.08, 0.29, 0.14, 1.0),  // Sony - dark green
            vec4(0.45, 0.45, 0.45, 1.0),  // Other - dark gray
        ]);

        self.directed_mode = false;
        self.gradient_enabled = true;
        // Note: show_tick_labels is controlled by live property, not set here
        self.initialized = true;
    }

    /// Initialize with debt data (D3 directed chord example)
    pub fn initialize_debt_data(&mut self) {
        // Country debt data from D3 directed chord diagram example
        // Shows debts between countries (in billions)
        // Source: https://observablehq.com/@d3/directed-chord-diagram

        // Order countries by total traffic (creditors first) for visual balance
        // This places high-traffic nodes (US, Japan, France) apart from each other
        let countries = vec![
            "France", "Germany", "Britain", "Japan", "United States",
            "Italy", "Spain", "Ireland", "Portugal", "Greece"
        ];

        // Build matrix from debt relationships
        let n = countries.len();
        let mut matrix = vec![vec![0.0; n]; n];

        // Complete debt data from the D3 example CSV (source owes target)
        let debts = vec![
            // Debts TO Britain
            ("France", "Britain", 22.4),
            ("Greece", "Britain", 0.55),
            ("Italy", "Britain", 26.0),
            ("Portugal", "Britain", 19.4),
            ("United States", "Britain", 345.0),
            // Debts TO France
            ("Germany", "France", 53.8),
            ("Greece", "France", 53.9),
            ("Ireland", "France", 17.3),
            ("Italy", "France", 366.0),
            ("Japan", "France", 7.73),
            ("Portugal", "France", 18.3),
            ("Spain", "France", 118.0),
            ("United States", "France", 322.0),
            // Debts TO Germany
            ("Britain", "Germany", 321.0),
            ("Greece", "Germany", 19.3),
            ("Ireland", "Germany", 48.9),
            ("Portugal", "Germany", 32.5),
            ("Spain", "Germany", 57.6),
            ("United States", "Germany", 324.0),
            // Debts TO Ireland
            ("Britain", "Ireland", 12.0),
            ("Greece", "Ireland", 0.34),
            ("Spain", "Ireland", 6.38),
            // Debts TO Italy
            ("Germany", "Italy", 111.0),
            ("Greece", "Italy", 3.22),
            ("Ireland", "Italy", 2.83),
            ("Portugal", "Italy", 0.87),
            // Debts TO Japan
            ("Britain", "Japan", 28.2),
            ("Germany", "Japan", 88.5),
            ("Greece", "Japan", 1.37),
            ("Ireland", "Japan", 18.9),
            ("Italy", "Japan", 38.8),
            ("Portugal", "Japan", 2.18),
            ("Spain", "Japan", 25.9),
            ("United States", "Japan", 796.0),  // LARGEST flow!
            // Debts TO Portugal
            ("Greece", "Portugal", 10.1),
            ("Ireland", "Portugal", 3.77),
            ("United States", "Portugal", 0.52),
            // Debts TO Spain
            ("Britain", "Spain", 326.0),
            ("Greece", "Spain", 0.78),
            ("Italy", "Spain", 9.79),
            ("Portugal", "Spain", 62.0),
            ("United States", "Spain", 163.0),
            // Debts TO United States
            ("Greece", "United States", 3.1),
            ("Ireland", "United States", 11.1),
            ("Italy", "United States", 3.16),
        ];

        // Create index map
        let index: std::collections::HashMap<&str, usize> = countries.iter()
            .enumerate()
            .map(|(i, &c)| (c, i))
            .collect();

        // Fill matrix
        for (source, target, value) in debts {
            if let (Some(&src_idx), Some(&tgt_idx)) = (index.get(source), index.get(target)) {
                matrix[src_idx][tgt_idx] = value;
            }
        }

        self.chord_data = ChordData::new()
            .with_labels(countries.iter().map(|s| s.to_string()).collect())
            .with_matrix(matrix);

        // Use D3 schemeCategory10 colors (matching new country order)
        self.custom_colors = Some(vec![
            vec4(0.12, 0.47, 0.71, 1.0),  // France - blue
            vec4(1.00, 0.50, 0.05, 1.0),  // Germany - orange
            vec4(0.17, 0.63, 0.17, 1.0),  // Britain - green
            vec4(0.84, 0.15, 0.16, 1.0),  // Japan - red
            vec4(0.58, 0.40, 0.74, 1.0),  // United States - purple
            vec4(0.55, 0.34, 0.29, 1.0),  // Italy - brown
            vec4(0.89, 0.47, 0.76, 1.0),  // Spain - pink
            vec4(0.50, 0.50, 0.50, 1.0),  // Ireland - gray
            vec4(0.74, 0.74, 0.13, 1.0),  // Portugal - olive
            vec4(0.09, 0.75, 0.81, 1.0),  // Greece - cyan
        ]);

        self.directed_mode = true;
        self.gradient_enabled = true;
        self.initialized = true;
    }

    /// Initialize with hair color data (D3 Chord Diagram II example)
    /// Shows relationships between hair colors from Circos
    pub fn initialize_hair_color_data(&mut self) {
        // Hair color data from D3 Chord Diagram II
        // https://observablehq.com/@d3/chord-diagram/2
        self.chord_data = ChordData::new()
            .with_labels(vec!["black", "blond", "brown", "red"])
            .with_matrix(vec![
                vec![11975.0,  5871.0, 8916.0, 2868.0],
                vec![ 1951.0, 10048.0, 2060.0, 6171.0],
                vec![ 8010.0, 16145.0, 8090.0, 8045.0],
                vec![ 1013.0,   990.0,  940.0, 6907.0],
            ]);

        // Use the exact colors from the D3 example
        self.custom_colors = Some(vec![
            vec4(0.0, 0.0, 0.0, 1.0),        // black - #000000
            vec4(1.0, 0.867, 0.537, 1.0),    // blond - #ffdd89
            vec4(0.584, 0.447, 0.267, 1.0),  // brown - #957244
            vec4(0.949, 0.384, 0.137, 1.0),  // red - #f26223
        ]);

        self.directed_mode = false;
        self.gradient_enabled = false;
        // Note: show_tick_labels is controlled by live property/WidgetRef
        self.use_absolute_labels = true;  // Use absolute values like "5K", "10K"
        self.tick_step = 5000.0;          // 5K intervals
        self.initialized = true;
    }

    /// Initialize with dependency data (D3 chord dependency example)
    pub fn initialize_dependency_data(&mut self) {
        // Software package dependency data (simplified from D3 example)
        let packages = vec![
            "analytics", "animate", "data", "display",
            "flex", "physics", "query", "scale",
            "util", "vis"
        ];

        // Dependency matrix (imports between packages)
        // Each row represents imports FROM that package TO others
        let matrix = vec![
            vec![15.0, 8.0, 12.0, 3.0, 2.0, 5.0, 4.0, 6.0, 10.0, 20.0],   // analytics
            vec![5.0, 30.0, 8.0, 15.0, 10.0, 3.0, 2.0, 4.0, 12.0, 18.0],  // animate
            vec![10.0, 5.0, 25.0, 8.0, 6.0, 2.0, 15.0, 3.0, 20.0, 12.0],  // data
            vec![8.0, 20.0, 6.0, 18.0, 12.0, 4.0, 3.0, 2.0, 8.0, 25.0],   // display
            vec![3.0, 8.0, 4.0, 15.0, 10.0, 2.0, 1.0, 1.0, 5.0, 6.0],     // flex
            vec![6.0, 4.0, 3.0, 5.0, 2.0, 12.0, 1.0, 8.0, 10.0, 4.0],     // physics
            vec![4.0, 2.0, 18.0, 3.0, 1.0, 1.0, 8.0, 2.0, 6.0, 5.0],      // query
            vec![8.0, 5.0, 4.0, 2.0, 1.0, 6.0, 2.0, 15.0, 12.0, 8.0],     // scale
            vec![12.0, 10.0, 15.0, 6.0, 4.0, 8.0, 5.0, 10.0, 35.0, 10.0], // util
            vec![25.0, 22.0, 15.0, 30.0, 8.0, 5.0, 6.0, 10.0, 18.0, 40.0],// vis
        ];

        self.chord_data = ChordData::new()
            .with_labels(packages.iter().map(|s| s.to_string()).collect())
            .with_matrix(matrix);

        // Use rainbow colors (d3.quantize(d3.interpolateRainbow, n))
        let n = packages.len();
        let rainbow_colors: Vec<Vec4> = (0..n)
            .map(|i| {
                let t = i as f32 / n as f32;
                // Rainbow color interpolation
                let r = (0.5 + 0.5 * (2.0 * PI as f32 * (t + 0.0 / 3.0)).cos()).clamp(0.0, 1.0);
                let g = (0.5 + 0.5 * (2.0 * PI as f32 * (t + 1.0 / 3.0)).cos()).clamp(0.0, 1.0);
                let b = (0.5 + 0.5 * (2.0 * PI as f32 * (t + 2.0 / 3.0)).cos()).clamp(0.0, 1.0);
                vec4(r, g, b, 1.0)
            })
            .collect();

        self.custom_colors = Some(rainbow_colors);

        self.directed_mode = true;
        self.gradient_enabled = true;
        self.initialized = true;
    }
}

/// Widget reference implementation for external initialization
impl ChordDiagramWidgetRef {
    pub fn initialize_phone_data(&self, cx: &mut Cx) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.initialize_phone_data();
            inner.show_tick_labels = true;  // Enable labels for detail page
            inner.compute_chord_layout();
            inner.start_animation(cx);
        }
    }

    pub fn initialize_debt_data(&self, cx: &mut Cx) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.initialize_debt_data();
            inner.show_tick_labels = true;  // Enable labels for detail page
            inner.compute_chord_layout();
            inner.start_animation(cx);
        }
    }

    pub fn initialize_dependency_data(&self, cx: &mut Cx) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.initialize_dependency_data();
            inner.show_tick_labels = true;  // Enable labels for detail page
            inner.compute_chord_layout();
            inner.start_animation(cx);
        }
    }

    pub fn initialize_hair_color_data(&self, cx: &mut Cx) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.initialize_hair_color_data();
            inner.show_tick_labels = true;  // Enable labels for detail page
            inner.compute_chord_layout();
            inner.start_animation(cx);
        }
    }
}
