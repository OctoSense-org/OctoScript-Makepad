//! Globe Map Widget
//!
//! Orthographic projection showing Earth as a 3D globe.

use makepad_widgets::*;
use makepad_d3::geo::{OrthographicProjection, Projection, ProjectionBuilder};
use std::f64::consts::PI;
use super::draw_primitives::DrawPoint;

live_design! {
    link widgets;
    use link::shaders::*;
    use super::draw_primitives::DrawPoint;

    // Globe background with gradient
    pub DrawGlobe = {{DrawGlobe}} {
        fn pixel(self) -> vec4 {
            let uv = self.pos;
            let center = vec2(0.5, 0.5);
            let dist = distance(uv, center) * 2.0;

            if dist > 1.0 {
                return vec4(0.0, 0.0, 0.0, 0.0);
            }

            // Anti-alias edge
            let aa = 0.02;
            let alpha = 1.0 - smoothstep(1.0 - aa, 1.0, dist);

            // Gradient from light (top-left) to dark (bottom-right)
            // Simulates 3D sphere lighting
            let light_dir = normalize(vec2(-0.5, -0.5));
            let normal = vec2(uv.x - 0.5, uv.y - 0.5) * 2.0;
            let z = sqrt(max(0.0, 1.0 - normal.x * normal.x - normal.y * normal.y));

            let diffuse = max(0.0, dot(vec3(normal.x, normal.y, z), vec3(light_dir.x, light_dir.y, 0.7)));

            // Ocean blue color
            let ocean_dark = vec3(0.08, 0.18, 0.35);
            let ocean_light = vec3(0.15, 0.35, 0.55);
            let ocean = mix(ocean_dark, ocean_light, diffuse * 0.8 + 0.2);

            return vec4(ocean * alpha, alpha);
        }
    }

    // Graticule line shader
    pub DrawGraticuleLine = {{DrawGraticuleLine}} {
        fn pixel(self) -> vec4 {
            let uv = self.pos;
            let p1 = vec2(self.x1, self.y1);
            let p2 = vec2(self.x2, self.y2);
            let line_vec = p2 - p1;
            let line_len = length(line_vec);

            if line_len < 0.001 {
                return vec4(0.0, 0.0, 0.0, 0.0);
            }

            let t = clamp(dot(uv - p1, line_vec) / (line_len * line_len), 0.0, 1.0);
            let closest = p1 + t * line_vec;
            let dist = length(uv - closest);
            let half_width = self.line_width * 0.5;
            let aa = 0.02;
            let alpha = 1.0 - smoothstep(half_width - aa, half_width + aa, dist);

            if alpha < 0.01 {
                return vec4(0.0, 0.0, 0.0, 0.0);
            }

            return vec4(self.color.rgb * alpha, self.color.a * alpha);
        }
    }

    pub GlobeMapWidget = {{GlobeMapWidget}} {
        width: Fill,
        height: Fill,
    }
}

#[derive(Live, LiveHook, LiveRegister)]
#[repr(C)]
pub struct DrawGlobe {
    #[deref] pub draw_super: DrawQuad,
}

#[derive(Live, LiveHook, LiveRegister)]
#[repr(C)]
pub struct DrawGraticuleLine {
    #[deref] pub draw_super: DrawQuad,
    #[live] pub color: Vec4,
    #[live] pub x1: f32,
    #[live] pub y1: f32,
    #[live] pub x2: f32,
    #[live] pub y2: f32,
    #[live] pub line_width: f32,
}

#[derive(Live, LiveHook, Widget)]
pub struct GlobeMapWidget {
    #[redraw]
    #[live]
    draw_globe: DrawGlobe,

    #[redraw]
    #[live]
    draw_line: DrawGraticuleLine,

    #[redraw]
    #[live]
    draw_point: DrawPoint,

    #[walk]
    walk: Walk,

    #[rust]
    rotation: f64,

    #[rust]
    initialized: bool,

    #[rust]
    area: Area,

    #[rust]
    cities: Vec<(f64, f64, &'static str)>, // lon, lat, name
}

impl Widget for GlobeMapWidget {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, _scope: &mut Scope) {
        match event {
            Event::NextFrame(_) => {
                // Slowly rotate the globe
                self.rotation += 0.3;
                if self.rotation > 360.0 {
                    self.rotation -= 360.0;
                }
                self.redraw(cx);
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
            self.draw_map(cx, rect);
        }

        DrawStep::done()
    }
}

impl GlobeMapWidget {
    fn initialize_data(&mut self) {
        if self.initialized {
            return;
        }

        // Major cities (lon, lat)
        self.cities = vec![
            (-74.0, 40.7, "New York"),
            (-118.2, 34.0, "Los Angeles"),
            (-0.1, 51.5, "London"),
            (2.3, 48.9, "Paris"),
            (139.7, 35.7, "Tokyo"),
            (116.4, 39.9, "Beijing"),
            (77.2, 28.6, "Delhi"),
            (-43.2, -22.9, "Rio"),
            (151.2, -33.9, "Sydney"),
            (37.6, 55.8, "Moscow"),
            (31.2, 30.0, "Cairo"),
            (-99.1, 19.4, "Mexico City"),
        ];

        self.rotation = 0.0;
        self.initialized = true;
    }

    fn draw_map(&mut self, cx: &mut Cx2d, rect: Rect) {
        let size = rect.size.x.min(rect.size.y);
        let center_x = rect.pos.x + rect.size.x / 2.0;
        let center_y = rect.pos.y + rect.size.y / 2.0;
        let radius = (size / 2.0 - 10.0) as f64;

        // Draw globe background
        self.draw_globe.draw_abs(
            cx,
            Rect {
                pos: dvec2(center_x as f64 - radius, center_y as f64 - radius),
                size: dvec2(radius * 2.0, radius * 2.0),
            },
        );

        // Create projection
        let projection = OrthographicProjection::new()
            .scale(radius)
            .translate(center_x as f64, center_y as f64)
            .rotate(-self.rotation, -20.0, 0.0);

        // Draw graticule (latitude/longitude lines)
        self.draw_graticule(cx, &projection, center_x as f64, center_y as f64, radius);

        // Draw cities
        self.draw_cities(cx, &projection);
    }

    fn draw_graticule(&mut self, cx: &mut Cx2d, projection: &OrthographicProjection, cx_: f64, cy: f64, radius: f64) {
        self.draw_line.color = vec4(0.3, 0.5, 0.7, 0.4);
        self.draw_line.line_width = 0.015;

        // Draw latitude lines (parallels)
        for lat in (-80..=80).step_by(20) {
            let lat = lat as f64;
            let mut prev_point: Option<(f64, f64)> = None;
            let mut prev_visible = false;

            for lon in (-180..=180).step_by(5) {
                let lon = lon as f64;
                let visible = projection.is_visible(lon, lat);

                if visible {
                    let (x, y) = projection.project(lon, lat);

                    if let Some((px, py)) = prev_point {
                        if prev_visible {
                            self.draw_graticule_segment(cx, px, py, x, y, cx_, cy, radius);
                        }
                    }
                    prev_point = Some((x, y));
                }
                prev_visible = visible;
            }
        }

        // Draw longitude lines (meridians)
        for lon in (-180..180).step_by(30) {
            let lon = lon as f64;
            let mut prev_point: Option<(f64, f64)> = None;
            let mut prev_visible = false;

            for lat in (-90..=90).step_by(5) {
                let lat = lat as f64;
                let visible = projection.is_visible(lon, lat);

                if visible {
                    let (x, y) = projection.project(lon, lat);

                    if let Some((px, py)) = prev_point {
                        if prev_visible {
                            self.draw_graticule_segment(cx, px, py, x, y, cx_, cy, radius);
                        }
                    }
                    prev_point = Some((x, y));
                }
                prev_visible = visible;
            }
        }
    }

    fn draw_graticule_segment(&mut self, cx: &mut Cx2d, x1: f64, y1: f64, x2: f64, y2: f64, cx_: f64, cy: f64, radius: f64) {
        // Calculate bounding box
        let padding = 5.0;
        let min_x = x1.min(x2) - padding;
        let max_x = x1.max(x2) + padding;
        let min_y = y1.min(y2) - padding;
        let max_y = y1.max(y2) + padding;

        let rect_w = max_x - min_x;
        let rect_h = max_y - min_y;

        if rect_w < 1.0 || rect_h < 1.0 {
            return;
        }

        // Normalize coordinates
        self.draw_line.x1 = ((x1 - min_x) / rect_w) as f32;
        self.draw_line.y1 = ((y1 - min_y) / rect_h) as f32;
        self.draw_line.x2 = ((x2 - min_x) / rect_w) as f32;
        self.draw_line.y2 = ((y2 - min_y) / rect_h) as f32;

        self.draw_line.draw_abs(
            cx,
            Rect {
                pos: dvec2(min_x, min_y),
                size: dvec2(rect_w, rect_h),
            },
        );
    }

    fn draw_cities(&mut self, cx: &mut Cx2d, projection: &OrthographicProjection) {
        for &(lon, lat, _name) in &self.cities {
            if projection.is_visible(lon, lat) {
                let (x, y) = projection.project(lon, lat);

                // City marker with gradient
                self.draw_point.set_radial_gradient(
                    vec4(1.0, 0.9, 0.5, 1.0),
                    vec4(0.95, 0.5, 0.3, 1.0),
                );
                self.draw_point.draw_point(cx, dvec2(x, y), 8.0);
            }
        }
    }
}
