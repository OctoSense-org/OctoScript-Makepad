//! Word cloud visualization (d3-cloud inspired)
//!
//! Implements a Wordle-style layout algorithm that places words using
//! spiral positioning. Words are sorted by weight and placed starting
//! from the center, moving outward along an Archimedean spiral.

use makepad_widgets::*;
use super::draw_primitives::DrawBar;
use super::animation::ChartAnimator;

live_design! {
    link widgets;
    use link::shaders::*;
    use super::draw_primitives::DrawBar;

    pub WordCloud = {{WordCloud}} {
        width: Fill,
        height: Fill,
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum SpiralType {
    #[default]
    Archimedean,
    Rectangular,
}

#[derive(Clone, Debug)]
pub struct WordEntry {
    pub text: String,
    pub weight: f64,
    pub color: Option<Vec4>,
    // Computed layout
    x: f64,
    y: f64,
    width: f64,
    height: f64,
    font_size: f64,
    rotation: f64,  // Rotation angle in radians
    placed: bool,
}

impl WordEntry {
    pub fn new(text: impl Into<String>, weight: f64) -> Self {
        Self {
            text: text.into(),
            weight,
            color: None,
            x: 0.0,
            y: 0.0,
            width: 0.0,
            height: 0.0,
            font_size: 0.0,
            rotation: 0.0,
            placed: false,
        }
    }

    pub fn with_color(mut self, color: Vec4) -> Self {
        self.color = Some(color);
        self
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct WordCloud {
    #[redraw]
    #[live]
    draw_bar: DrawBar,

    #[walk]
    walk: Walk,

    #[rust]
    words: Vec<WordEntry>,

    #[rust]
    min_font_size: f64,

    #[rust]
    max_font_size: f64,

    #[rust]
    padding: f64,

    #[rust]
    spiral_type: SpiralType,

    #[rust]
    rotation_angles: Vec<f64>,  // Possible rotation angles (d3-cloud defaults: -90, 0, 90)

    #[rust]
    random_seed: u64,

    #[rust]
    animator: ChartAnimator,

    #[rust]
    initialized: bool,

    #[rust]
    layout_done: bool,

    #[rust]
    area: Area,
}

impl WordCloud {
    pub fn set_data(&mut self, words: Vec<WordEntry>) {
        self.words = words;
        self.initialized = false;
        self.layout_done = false;
    }

    pub fn set_font_size_range(&mut self, min: f64, max: f64) {
        self.min_font_size = min;
        self.max_font_size = max;
    }

    pub fn set_padding(&mut self, padding: f64) {
        self.padding = padding;
    }

    pub fn set_spiral_type(&mut self, spiral_type: SpiralType) {
        self.spiral_type = spiral_type;
    }

    pub fn set_rotation_angles(&mut self, angles: Vec<f64>) {
        self.rotation_angles = angles;
    }

    // Simple pseudo-random number generator (LCG)
    fn next_random(&mut self) -> f64 {
        self.random_seed = self.random_seed.wrapping_mul(1103515245).wrapping_add(12345);
        ((self.random_seed >> 16) & 0x7FFF) as f64 / 32768.0
    }

    fn layout_words(&mut self, width: f64, height: f64) {
        // Only layout once to prevent flickering
        if self.layout_done || self.words.is_empty() {
            return;
        }

        // Sort by weight descending (largest words first, like d3-cloud)
        self.words.sort_by(|a, b| b.weight.partial_cmp(&a.weight).unwrap_or(std::cmp::Ordering::Equal));

        // Find weight range for font scaling (d3-cloud uses sqrt by default)
        let max_weight = self.words.iter().map(|w| w.weight).fold(0.0_f64, f64::max);
        let min_weight = self.words.iter().map(|w| w.weight).fold(f64::INFINITY, f64::min);
        let weight_range = (max_weight - min_weight).max(1.0);

        let center_x = width / 2.0;
        let center_y = height / 2.0;

        // Calculate font sizes and assign rotations
        let rotation_angles = self.rotation_angles.clone();
        let n_angles = rotation_angles.len();
        let min_font = self.min_font_size;
        let max_font = self.max_font_size;

        // Pre-compute random values to avoid borrow issues
        let n_words = self.words.len();
        let mut random_values: Vec<f64> = Vec::with_capacity(n_words);
        for _ in 0..n_words {
            random_values.push(self.next_random());
        }

        for (i, word) in self.words.iter_mut().enumerate() {
            // D3-cloud uses sqrt scaling by default
            let t = ((word.weight - min_weight) / weight_range).sqrt();
            word.font_size = min_font + t * (max_font - min_font);

            // Assign rotation (d3-cloud picks from available angles)
            if n_angles > 0 {
                let angle_idx = (random_values[i] * n_angles as f64) as usize % n_angles;
                word.rotation = rotation_angles[angle_idx];
            } else {
                word.rotation = 0.0;
            }

            // Calculate bounding box considering rotation
            let base_width = word.text.len() as f64 * word.font_size * 0.55;
            let base_height = word.font_size * 1.1;

            if word.rotation.abs() > 0.01 {
                // For rotated text, swap width and height
                word.width = base_height;
                word.height = base_width;
            } else {
                word.width = base_width;
                word.height = base_height;
            }

            word.placed = false;
        }

        // Spiral placement (d3-cloud style)
        let max_iterations = 5000;
        let padding = self.padding;
        let n_words = self.words.len();
        let spiral_type = self.spiral_type;

        for word_idx in 0..n_words {
            let word_width = self.words[word_idx].width;
            let word_height = self.words[word_idx].height;

            // Start from center and spiral outward
            let mut t = 0.0_f64;
            let dt = 0.5;  // Smaller steps for better packing

            for _ in 0..max_iterations {
                let (dx, dy) = Self::spiral_position(spiral_type, t, width, height);
                let x = center_x + dx - word_width / 2.0;
                let y = center_y + dy - word_height / 2.0;

                // Check bounds
                if x < padding || x + word_width > width - padding ||
                   y < padding || y + word_height > height - padding {
                    t += dt;
                    continue;
                }

                // Check collision with placed words
                let mut collision = false;
                for other_idx in 0..word_idx {
                    let other = &self.words[other_idx];
                    if other.placed && Self::rects_overlap(
                        x, y, word_width, word_height,
                        other.x, other.y, other.width, other.height,
                        padding,
                    ) {
                        collision = true;
                        break;
                    }
                }

                if !collision {
                    self.words[word_idx].x = x;
                    self.words[word_idx].y = y;
                    self.words[word_idx].placed = true;
                    break;
                }

                t += dt;
            }
        }

        // Mark layout as done to prevent recalculation
        self.layout_done = true;
    }

    // Archimedean or rectangular spiral (d3-cloud style)
    fn spiral_position(spiral_type: SpiralType, t: f64, width: f64, height: f64) -> (f64, f64) {
        match spiral_type {
            SpiralType::Archimedean => {
                // Archimedean spiral: r = a + b*θ
                let a = 0.5;
                let b = 0.5;
                let angle = t * 0.1;
                let r = a + b * angle;
                (r * angle.cos(), r * angle.sin())
            }
            SpiralType::Rectangular => {
                // Rectangular spiral for filling rectangular areas
                let aspect = width / height;
                let step = 4.0;
                let dx = (t * step) % (width * 0.8) - width * 0.4;
                let dy = ((t * step / width).floor() * step) % (height * 0.8) - height * 0.4;
                (dx * aspect.sqrt(), dy / aspect.sqrt())
            }
        }
    }

    fn rects_overlap(x1: f64, y1: f64, w1: f64, h1: f64,
                     x2: f64, y2: f64, w2: f64, h2: f64, pad: f64) -> bool {
        !(x1 + w1 + pad < x2 || x2 + w2 + pad < x1 ||
          y1 + h1 + pad < y2 || y2 + h2 + pad < y1)
    }

    fn draw_chart(&mut self, cx: &mut Cx2d, rect: Rect) {
        // Layout words if needed
        self.layout_words(rect.size.x, rect.size.y);

        // Color palette (d3-cloud style - more muted, professional colors)
        let colors = [
            vec4(0.18, 0.55, 0.75, 1.0),  // Steel blue
            vec4(0.85, 0.40, 0.30, 1.0),  // Terracotta
            vec4(0.30, 0.65, 0.45, 1.0),  // Sea green
            vec4(0.70, 0.50, 0.75, 1.0),  // Lavender
            vec4(0.90, 0.65, 0.30, 1.0),  // Orange
            vec4(0.45, 0.70, 0.80, 1.0),  // Sky blue
            vec4(0.75, 0.35, 0.55, 1.0),  // Rose
            vec4(0.50, 0.75, 0.35, 1.0),  // Lime
            vec4(0.60, 0.45, 0.65, 1.0),  // Purple
            vec4(0.35, 0.60, 0.65, 1.0),  // Teal
        ];

        // Get animation progress
        let progress = self.animator.get_progress();

        // Clone word data to avoid borrow conflicts
        let word_data: Vec<_> = self.words.iter().enumerate().map(|(i, word)| {
            (i, word.placed, word.x, word.y, word.width, word.height,
             word.font_size, word.rotation, word.text.clone(), word.color)
        }).collect();

        // Draw words
        for (i, placed, word_x, word_y, word_width, word_height, font_size, rotation, text, custom_color) in word_data {
            if !placed {
                continue;
            }

            let color = custom_color.unwrap_or(colors[i % colors.len()]);

            // Animate word appearance (scale up from center)
            let word_progress = ((progress as f32 * 2.5) - (i as f32 * 0.1)).clamp(0.0, 1.0);

            if word_progress < 0.01 {
                continue;
            }

            let x = rect.pos.x + word_x;
            let y = rect.pos.y + word_y;

            // Draw word - handle both horizontal and rotated text
            let is_rotated = rotation.abs() > 0.01;

            if is_rotated {
                // Vertical text (rotated 90 degrees)
                let char_height = font_size * 0.5 * word_progress as f64;
                let char_width = font_size * 0.8 * word_progress as f64;

                for (j, _ch) in text.chars().enumerate() {
                    let cx_pos = x + (word_width - char_width) / 2.0;
                    let cy_pos = y + j as f64 * char_height * 1.1;

                    // Vary the rectangle slightly for visual interest
                    let w_var = if j % 3 == 0 { char_width * 0.85 } else { char_width };
                    let x_offset = if j % 3 == 0 { char_width * 0.075 } else { 0.0 };

                    self.draw_bar.color = color;
                    self.draw_bar.draw_bar(cx, Rect {
                        pos: dvec2(cx_pos + x_offset, cy_pos),
                        size: dvec2(w_var, char_height * 0.9),
                    });
                }
            } else {
                // Horizontal text
                let char_width = font_size * 0.5 * word_progress as f64;
                let char_height = font_size * 0.8 * word_progress as f64;

                for (j, _ch) in text.chars().enumerate() {
                    let cx_pos = x + j as f64 * char_width * 1.1;
                    let cy_pos = y + (word_height - char_height) / 2.0;

                    // Vary the rectangle slightly for visual interest
                    let h_var = if j % 3 == 0 { char_height * 0.85 } else { char_height };
                    let y_offset = if j % 3 == 0 { char_height * 0.075 } else { 0.0 };

                    self.draw_bar.color = color;
                    self.draw_bar.draw_bar(cx, Rect {
                        pos: dvec2(cx_pos, cy_pos + y_offset),
                        size: dvec2(char_width * 0.9, h_var),
                    });
                }
            }
        }
    }
}

impl WordCloud {
    fn initialize_demo_data(&mut self) {
        // More words for a denser cloud (d3-cloud style)
        self.words = vec![
            WordEntry::new("Makepad", 100.0),
            WordEntry::new("Rust", 90.0),
            WordEntry::new("GPU", 75.0),
            WordEntry::new("Charts", 70.0),
            WordEntry::new("Data", 65.0),
            WordEntry::new("Visualization", 60.0),
            WordEntry::new("Graphics", 55.0),
            WordEntry::new("WebAssembly", 50.0),
            WordEntry::new("Performance", 48.0),
            WordEntry::new("Interactive", 45.0),
            WordEntry::new("Real-time", 42.0),
            WordEntry::new("Animation", 40.0),
            WordEntry::new("Shader", 38.0),
            WordEntry::new("Widget", 36.0),
            WordEntry::new("Layout", 34.0),
            WordEntry::new("Canvas", 32.0),
            WordEntry::new("Render", 30.0),
            WordEntry::new("UI", 28.0),
            WordEntry::new("Design", 26.0),
            WordEntry::new("Code", 24.0),
            WordEntry::new("Fast", 22.0),
            WordEntry::new("Native", 20.0),
            WordEntry::new("Cross", 18.0),
            WordEntry::new("Platform", 16.0),
            WordEntry::new("Live", 15.0),
            WordEntry::new("DSL", 14.0),
            WordEntry::new("Draw", 13.0),
            WordEntry::new("Style", 12.0),
        ];

        self.min_font_size = 14.0;
        self.max_font_size = 56.0;
        self.padding = 3.0;

        // D3-cloud style settings
        self.spiral_type = SpiralType::Archimedean;
        // D3-cloud default rotations: 0° and 90° (horizontal and vertical)
        self.rotation_angles = vec![0.0, std::f64::consts::FRAC_PI_2];
        self.random_seed = 42;  // Consistent layout
    }
}

impl Widget for WordCloud {
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
