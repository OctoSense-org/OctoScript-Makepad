//! Source-independent chart styling. Geometry remains owned by numerical widgets.
use makepad_widgets::*;
use makepad_widgets::makepad_draw::vector::{GradientStop, VectorPaint};

#[derive(Clone, Debug)]
pub struct ChartPaint {
    pub stops: Vec<(f32, Vec4)>,
    pub from: (f32, f32),
    pub to: (f32, f32),
    pub angular: bool,
    pub angular_center: (f32, f32),
    pub radial: bool,
}

impl ChartPaint {
    pub fn sample(&self, t: f32) -> Vec4 {
        let Some(&(first, ink)) = self.stops.first() else { return vec4(0.,0.,0.,0.); };
        if t <= first { return ink; }
        for pair in self.stops.windows(2) {
            if t <= pair[1].0 {
                let u = ((t-pair[0].0)/(pair[1].0-pair[0].0).max(1e-6)).clamp(0.,1.);
                return pair[0].1*(1.-u)+pair[1].1*u;
            }
        }
        self.stops.last().unwrap().1
    }

    pub fn apply(&self, draw: &mut DrawVector, x: f32, y: f32, w: f32, h: f32) {
        let stops: Vec<_> = self.stops.iter().map(|&(offset,c)| GradientStop {
            offset, color:[c.x*c.w,c.y*c.w,c.z*c.w,c.w], straight_rgb:Some([c.x,c.y,c.z]),
        }).collect();
        draw.cur_gradient_row_v=draw.add_gradient_row(&stops);
        if self.radial {
            let radius=((w*(self.to.0-self.from.0)).powi(2)+(h*(self.to.1-self.from.1)).powi(2)).sqrt();
            draw.set_paint(VectorPaint::radial_gradient(x+w*self.from.0,y+h*self.from.1,radius,radius,stops));
        } else {draw.set_paint(VectorPaint::linear_gradient(x+w*self.from.0,y+h*self.from.1,
            x+w*self.to.0,y+h*self.to.1,stops));}
    }
}
