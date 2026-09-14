//! A reusable value-bound progress indicator for runtime kit composition.
use makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    set_type_default() do #(DrawDesignProgress::script_shader(vm)) {
        ..mod.draw.DrawQuad
        value: 0.0
        track_color: #dde8e0
        fill_color: #22785d
        fill_color2: #22785d
        corner_radius: -1.0
        gradient: 0.0
        pixel: fn() {
            let sdf=Sdf2d.viewport(self.pos*self.rect_size)
            var radius=self.corner_radius
            if radius<0.0 {radius=self.rect_size.y*0.5}
            radius=min(radius,min(self.rect_size.x,self.rect_size.y)*0.5)
            sdf.box(0.0,0.0,self.rect_size.x,self.rect_size.y,radius)
            sdf.fill(self.track_color)
            let width=self.rect_size.x*clamp(self.value,0.0,1.0)
            if width>0.0 {
                sdf.box(0.0,0.0,width,self.rect_size.y,min(width*0.5,radius))
                let t=clamp(self.pos.x/max(self.value,0.000001),0.0,1.0)*self.gradient
                sdf.fill(mix(self.fill_color,self.fill_color2,t))
            }
            return sdf.result
        }
    }
    mod.widgets.DesignProgressBar = #(DesignProgressBar::register_widget(vm)) {}
    mod.prelude.widgets.DesignProgressBar = mod.widgets.DesignProgressBar
}

#[derive(Script, ScriptHook)]
#[repr(C)]
struct DrawDesignProgress {
    #[deref] draw_super: DrawQuad,
    #[live] value: f32,
    #[live] track_color: Vec4,
    #[live] fill_color: Vec4,
    #[live] fill_color2: Vec4,
    #[live] gradient: f32,
    #[live] corner_radius: f32,
}

#[derive(Script, ScriptHook, Widget)]
pub struct DesignProgressBar {
    #[uid] uid: WidgetUid,
    #[source] source: ScriptObjectRef,
    #[walk] walk: Walk,
    #[layout] layout: Layout,
    #[live] pub value: f64,
    #[redraw] #[live] draw_bg: DrawDesignProgress,
}

impl DesignProgressBar {
    pub fn set_style(&mut self,cx:&mut Cx,radius:f32,colors:Option<(Vec4,Vec4)>) {
        self.draw_bg.corner_radius=radius;
        if let Some((a,b))=colors {self.draw_bg.fill_color=a;self.draw_bg.fill_color2=b;self.draw_bg.gradient=1.;}
        self.redraw(cx);
    }
    pub fn painted_value(&self)->f64 {self.draw_bg.value as f64}
    pub fn set_value(&mut self,cx:&mut Cx,value:f64) {
        self.value=value.clamp(0.,1.);
        self.redraw(cx);
    }
}
impl Widget for DesignProgressBar {
    fn draw_walk(&mut self,cx:&mut Cx2d,_scope:&mut Scope,walk:Walk)->DrawStep {
        self.draw_bg.value=self.value as f32;
        self.draw_bg.draw_walk(cx,walk);
        DrawStep::done()
    }
}
