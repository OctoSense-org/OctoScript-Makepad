//! Reusable source-styled native widgets for registered design kits.
use makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    mod.widgets.DesignText = #(DesignText::register_widget(vm))
    mod.prelude.widgets.DesignText = mod.widgets.DesignText
    mod.widgets.DesignRotatedLabel = #(DesignRotatedLabel::register_widget(vm)) {
        draw_text: mod.draw.DrawRotatedText {}
    }
    mod.prelude.widgets.DesignRotatedLabel = mod.widgets.DesignRotatedLabel
    mod.widgets.DesignButton = #(DesignButton::register_widget(vm))
    mod.prelude.widgets.DesignButton = mod.widgets.DesignButton
    mod.widgets.DesignOverlay = #(DesignOverlay::register_widget(vm))
    mod.prelude.widgets.DesignOverlay = mod.widgets.DesignOverlay
    mod.widgets.DesignGlassSvg = #(DesignGlassSvg::register_widget(vm)) {
        draw_svg +: {
            backdrop_texture: texture_2d(float)
            backdrop_high_texture: texture_2d(float)
            backdrop_low_kernel: uniform(0.0)
            backdrop_high_kernel: uniform(0.0)
            backdrop_mix: uniform(0.0)
            source_size: uniform(vec2(1.0,1.0))
            source_y_flip: uniform(0.0)
            backdrop_tint: uniform(#0000)
            overlay_blend: uniform(0.0)
            sample_low: fn(uv: vec2) -> vec3 {
                if self.backdrop_low_kernel<0.5 {return self.backdrop_texture.sample_as_bgra(uv).rgb}
                let texel=vec2(1.0)/max(self.backdrop_texture.size(),vec2(1.0))
                return self.backdrop_texture.sample_as_bgra(uv).rgb*0.20
                    +self.backdrop_texture.sample_as_bgra(clamp(uv+texel*vec2(1.0,0.0),vec2(0.0),vec2(1.0))).rgb*0.12
                    +self.backdrop_texture.sample_as_bgra(clamp(uv+texel*vec2(-1.0,0.0),vec2(0.0),vec2(1.0))).rgb*0.12
                    +self.backdrop_texture.sample_as_bgra(clamp(uv+texel*vec2(0.0,1.0),vec2(0.0),vec2(1.0))).rgb*0.12
                    +self.backdrop_texture.sample_as_bgra(clamp(uv+texel*vec2(0.0,-1.0),vec2(0.0),vec2(1.0))).rgb*0.12
                    +self.backdrop_texture.sample_as_bgra(clamp(uv+texel*vec2(1.0,1.0),vec2(0.0),vec2(1.0))).rgb*0.06
                    +self.backdrop_texture.sample_as_bgra(clamp(uv+texel*vec2(-1.0,1.0),vec2(0.0),vec2(1.0))).rgb*0.06
                    +self.backdrop_texture.sample_as_bgra(clamp(uv+texel*vec2(1.0,-1.0),vec2(0.0),vec2(1.0))).rgb*0.06
                    +self.backdrop_texture.sample_as_bgra(clamp(uv+texel*vec2(-1.0,-1.0),vec2(0.0),vec2(1.0))).rgb*0.06
                    +self.backdrop_texture.sample_as_bgra(clamp(uv+texel*vec2(2.0,0.0),vec2(0.0),vec2(1.0))).rgb*0.02
                    +self.backdrop_texture.sample_as_bgra(clamp(uv+texel*vec2(-2.0,0.0),vec2(0.0),vec2(1.0))).rgb*0.02
                    +self.backdrop_texture.sample_as_bgra(clamp(uv+texel*vec2(0.0,2.0),vec2(0.0),vec2(1.0))).rgb*0.02
                    +self.backdrop_texture.sample_as_bgra(clamp(uv+texel*vec2(0.0,-2.0),vec2(0.0),vec2(1.0))).rgb*0.02
            }
            sample_high: fn(uv: vec2) -> vec3 {
                if self.backdrop_high_kernel<0.5 {return self.backdrop_high_texture.sample_as_bgra(uv).rgb}
                let texel=vec2(1.0)/max(self.backdrop_high_texture.size(),vec2(1.0))
                return self.backdrop_high_texture.sample_as_bgra(uv).rgb*0.20
                    +self.backdrop_high_texture.sample_as_bgra(clamp(uv+texel*vec2(1.0,0.0),vec2(0.0),vec2(1.0))).rgb*0.12
                    +self.backdrop_high_texture.sample_as_bgra(clamp(uv+texel*vec2(-1.0,0.0),vec2(0.0),vec2(1.0))).rgb*0.12
                    +self.backdrop_high_texture.sample_as_bgra(clamp(uv+texel*vec2(0.0,1.0),vec2(0.0),vec2(1.0))).rgb*0.12
                    +self.backdrop_high_texture.sample_as_bgra(clamp(uv+texel*vec2(0.0,-1.0),vec2(0.0),vec2(1.0))).rgb*0.12
                    +self.backdrop_high_texture.sample_as_bgra(clamp(uv+texel*vec2(1.0,1.0),vec2(0.0),vec2(1.0))).rgb*0.06
                    +self.backdrop_high_texture.sample_as_bgra(clamp(uv+texel*vec2(-1.0,1.0),vec2(0.0),vec2(1.0))).rgb*0.06
                    +self.backdrop_high_texture.sample_as_bgra(clamp(uv+texel*vec2(1.0,-1.0),vec2(0.0),vec2(1.0))).rgb*0.06
                    +self.backdrop_high_texture.sample_as_bgra(clamp(uv+texel*vec2(-1.0,-1.0),vec2(0.0),vec2(1.0))).rgb*0.06
                    +self.backdrop_high_texture.sample_as_bgra(clamp(uv+texel*vec2(2.0,0.0),vec2(0.0),vec2(1.0))).rgb*0.02
                    +self.backdrop_high_texture.sample_as_bgra(clamp(uv+texel*vec2(-2.0,0.0),vec2(0.0),vec2(1.0))).rgb*0.02
                    +self.backdrop_high_texture.sample_as_bgra(clamp(uv+texel*vec2(0.0,2.0),vec2(0.0),vec2(1.0))).rgb*0.02
                    +self.backdrop_high_texture.sample_as_bgra(clamp(uv+texel*vec2(0.0,-2.0),vec2(0.0),vec2(1.0))).rgb*0.02
            }
            get_color: fn() {
                let tint=self.eval_gradient()
                let uv=self.v_world/max(self.source_size,vec2(1.0,1.0))
                let sample_uv=clamp(vec2(uv.x,mix(uv.y,1.0-uv.y,self.source_y_flip)),vec2(0.0),vec2(1.0))
                let blurred=mix(self.sample_low(sample_uv),self.sample_high(sample_uv),self.backdrop_mix)
                let base=mix(blurred,self.backdrop_tint.rgb,self.backdrop_tint.a)
                let color=tint.rgb/max(tint.a,0.0001)
                let overlay=mix(2.0*base*color,vec3(1.0)-2.0*(vec3(1.0)-base)*(vec3(1.0)-color),step(vec3(0.5),base))
                return vec4(base*(1.0-tint.a)+mix(tint.rgb,overlay*tint.a,self.overlay_blend),1.0)
            }
        }
    }
    mod.prelude.widgets.DesignGlassSvg = mod.widgets.DesignGlassSvg
    mod.widgets.DesignPill = #(DesignPill::register_widget(vm))
    mod.prelude.widgets.DesignPill = mod.widgets.DesignPill
    mod.widgets.DesignNativeButton = Button {
        text: "" padding: 0 margin: 0
        icon_walk: Walk{width: 0 height: 0}
        label_walk: Walk{width: 0 height: 0}
        draw_bg +: {pixel: fn(){return vec4(0.0)}}
    }
    mod.prelude.widgets.DesignNativeButton = mod.widgets.DesignNativeButton
    mod.widgets.DesignTextShadow = CachedView {
        clip_x: false clip_y: false
        draw_bg +: {
            tint_color: instance(#ff7a2847)
            sigma: instance(8.0)
            pixel: fn() {
                let uv=self.pos*self.scale+self.shift
                let step_size=self.sigma*0.25/max(self.rect_size,vec2(1.0))
                let mut sum=0.0
                let mut weights=0.0
                for iy in 0..25 {
                    for ix in 0..25 {
                        let offset=vec2(float(ix)-12.0,float(iy)-12.0)
                        let weight=exp(-dot(offset,offset)*0.03125)
                        sum+=self.image.sample(clamp(uv+offset*step_size,vec2(0.0),vec2(1.0))).a*weight
                        weights+=weight
                    }
                }
                let alpha=sum/max(weights,0.0001)*self.tint_color.a
                return vec4(self.tint_color.rgb*alpha,alpha)
            }
        }
    }
    mod.prelude.widgets.DesignTextShadow = mod.widgets.DesignTextShadow
    mod.widgets.DesignAtroPill = CheckBox {
        text: "" padding: 0 margin: 0
        icon_walk: Walk{width: 0 height: 0}
        label_walk: Walk{width: 0 height: 0}
        draw_bg +: {
            active_color: instance(#ff3a79)
            inactive_color: instance(#191922)
            mark_color: instance(#fff)
            pixel: fn() {
                let sdf=Sdf2d.viewport(self.pos*self.rect_size)
                sdf.box(0.0,0.0,self.rect_size.x,self.rect_size.y,self.rect_size.y*0.25)
                sdf.fill(mix(self.inactive_color,self.active_color,self.active))
                if self.active>0.5 {
                    let x=self.rect_size.x-22.9
                    let y=self.rect_size.y*0.5
                    sdf.move_to(x,y-0.2)
                    sdf.line_to(x+3.9,y+3.5)
                    sdf.line_to(x+12.5,y-5.0)
                    sdf.stroke(self.mark_color,1.5)
                } else {
                    sdf.circle(self.rect_size.x-16.9,self.rect_size.y*0.5,11.25)
                    sdf.stroke(#ffffff80,1.5)
                }
                return sdf.result
            }
        }
    }
    mod.prelude.widgets.DesignAtroPill = mod.widgets.DesignAtroPill
    mod.widgets.DesignAtroSlider = SliderMinimal {
        text: "" padding: 0 margin: 0 flow: Overlay
        label_walk: Walk{width: 0 height: 0}
        text_input +: {width: 0 height: 0 padding: 0 margin: 0 draw_text.color: #0000 draw_bg +: {pixel: fn(){return vec4(0.0)}}}
        draw_bg +: {
            ink: instance(#x4c5fef)
            handle_border: instance(2.5)
            pixel: fn() {
                let sdf=Sdf2d.viewport(self.pos*self.rect_size)
                let y=self.rect_size.y*0.5
                let lo=self.slide_pos*self.rect_size.x
                let hi=self.slide_pos2*self.rect_size.x
                sdf.box(0.0,y-1.25,self.rect_size.x,2.5,0.625)
                sdf.fill(#f4f6f9)
                let start=mix(0.0,lo,self.range_mode)
                let end=mix(lo,hi,self.range_mode)
                sdf.rect(start,y-1.0,max(end-start,0.0),2.0)
                sdf.fill(self.ink)
                sdf.circle(lo,y,10.0+self.handle_border)
                sdf.fill(#fff)
                sdf.circle(lo,y,10.0)
                sdf.fill(self.ink)
                if self.handle_border>0.0 {
                    sdf.circle(lo,y,8.5)
                    sdf.fill(#fff)
                }
                if self.range_mode>0.5 {
                    sdf.circle(hi,y,10.0+self.handle_border)
                    sdf.fill(#fff)
                    sdf.circle(hi,y,10.0)
                    sdf.fill(self.ink)
                    if self.handle_border>0.0 {
                        sdf.circle(hi,y,8.5)
                        sdf.fill(#fff)
                    }
                }
                return sdf.result
            }
        }
    }
    mod.prelude.widgets.DesignAtroSlider = mod.widgets.DesignAtroSlider
    mod.widgets.DesignGlassSurface = GaussRoundedView {
        draw_bg +: {
            overlay_blend: instance(0.0)
            backdrop_tint: instance(#0000)
            shadow_radius: 0.0
            shadow_offset: vec2(0.0,0.0)
            shadow_color: #0000
            border_width: 0.0
            pixel: fn() {
                let sdf=Sdf2d.viewport(self.pos*self.rect_size3)
                // Sdf2d.box uses twice its radius argument as the corner radius.
                let radius=min(self.corner_radius,min(self.sdf_rect_size.x,self.sdf_rect_size.y)*0.5)
                sdf.box(self.sdf_rect_pos.x,self.sdf_rect_pos.y,self.sdf_rect_size.x,self.sdf_rect_size.y,radius*0.5)
                let screen_pos=self.rect_pos2+self.pos*self.rect_size3
                let uv=screen_pos/max(self.source_size,vec2(1.0,1.0))
                // Design radii are logical points; Gauss mip levels operate
                // on physical texels. Keep blur strength stable across DPI.
                let dpi=max(self.scene_texture.size().x/max(self.source_size.x,1.0),1.0)
                // The Gauss reconstruction kernel widens the nominal mip
                // radius. Calibration against the source Gaussian gives 3/4.
                let blurred=self.sample_blur(self.blur_level+log2(dpi*0.75),uv)
                let base=mix(blurred.rgb,self.backdrop_tint.rgb,self.backdrop_tint.a)
                let tint=self.tint_color.rgb
                let overlay=mix(2.0*base*tint,vec3(1.0)-2.0*(vec3(1.0)-base)*(vec3(1.0)-tint),step(vec3(0.5),base))
                let color=mix(base,mix(tint,overlay,self.overlay_blend),self.tint_color.a)
                sdf.fill(vec4(color,1.0))
                return sdf.result
            }
        }
    }
    mod.prelude.widgets.DesignGlassSurface = mod.widgets.DesignGlassSurface
    mod.widgets.DesignRadio = #(DesignRadio::register_widget(vm))
    mod.prelude.widgets.DesignRadio = mod.widgets.DesignRadio
    mod.widgets.DesignSurface = View {
        show_bg: true
        draw_bg +: {
            color: instance(#0000)
            color2: instance(#0000)
            gradient: instance(0.0)
            radius: instance(0.0)
            ellipse: instance(0.0)
            border_width: instance(0.0)
            border_position: instance(0.0)
            border_color: instance(#0000)
            pixel: fn() {
                let half_size = self.rect_size * 0.5
                let p = self.pos * self.rect_size - half_size
                let radius = min(self.radius, min(half_size.x, half_size.y))
                let q = abs(p) - half_size + vec2(radius, radius)
                let rect_distance = length(max(q, vec2(0.0, 0.0))) + min(max(q.x, q.y), 0.0) - radius
                let ellipse_distance = (length(p / max(half_size, vec2(0.01, 0.01))) - 1.0) * min(half_size.x, half_size.y)
                let distance = mix(rect_distance, ellipse_distance, self.ellipse)
                let aa=max(length(vec2(dFdx(distance),dFdy(distance))),0.001)
                let fill_alpha = clamp(0.5 - distance/aa, 0.0, 1.0)
                let inside_offset = mix(0.0, self.border_width * 0.5, step(0.5, self.border_position))
                let stroke_distance = abs(distance + inside_offset) - self.border_width * 0.5
                let stroke_alpha = clamp(0.5 - stroke_distance/aa, 0.0, 1.0) * step(0.001, self.border_width)
                let fill = Pal.premul(mix(self.color,self.color2,self.pos.y*self.gradient)) * fill_alpha
                let stroke = Pal.premul(self.border_color) * stroke_alpha
                return stroke + fill * (1.0 - stroke.a)
            }
        }
    }
    mod.prelude.widgets.DesignSurface = mod.widgets.DesignSurface
    mod.widgets.DesignInput = TextInput {
        padding: 0 margin: 0
        flow: Right
        blink_speed: 3600.0
        draw_bg +: {pixel: fn() {return vec4(0.0, 0.0, 0.0, 0.0)}}
        draw_text +: {get_color: fn() {return self.color}}
        draw_cursor.color: #111927
    }
    mod.prelude.widgets.DesignInput = mod.widgets.DesignInput
    mod.widgets.DesignImage = Image {
        draw_bg +: {
            pixel: fn() {
                let size = vec2(self.image_dim_w, self.image_dim_h)
                let pixel = self.pos * size - vec2(0.5, 0.5)
                let origin = floor(pixel)
                let fraction = fract(pixel)
                let a = Pal.premul(self.image_texture.sample_as_bgra((origin + vec2(0.5, 0.5)) / size))
                let b = Pal.premul(self.image_texture.sample_as_bgra((origin + vec2(1.5, 0.5)) / size))
                let c = Pal.premul(self.image_texture.sample_as_bgra((origin + vec2(0.5, 1.5)) / size))
                let d = Pal.premul(self.image_texture.sample_as_bgra((origin + vec2(1.5, 1.5)) / size))
                return mix(mix(a, b, fraction.x), mix(c, d, fraction.x), fraction.y) * self.opacity
            }
        }
    }
    mod.prelude.widgets.DesignImage = mod.widgets.DesignImage
    mod.widgets.DesignToggle = Toggle {
        padding: 0
        icon_walk: Walk{width: 0 height: 0}
        label_walk: Walk{width: 0 height: 0 margin: Inset{}}
        text: ""
        draw_bg +: {
            ink: instance(#1e8eba)
            off_color: instance(#9da4ae)
            mark_color: instance(#fff)
            pixel: fn() {
                let sdf = Sdf2d.viewport(self.pos * self.rect_size)
                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, self.rect_size.y * 0.25)
                sdf.fill(mix(self.off_color, self.ink, self.active))
                let radius = self.rect_size.y * 0.5 - 3.
                let center_x = mix(3. + radius, self.rect_size.x - 3. - radius, self.active)
                sdf.circle(center_x, self.rect_size.y * 0.5, radius)
                sdf.fill(self.mark_color)
                return sdf.result
            }
        }
    }
    mod.prelude.widgets.DesignToggle = mod.widgets.DesignToggle
    mod.widgets.DesignAtroToggle = mod.widgets.DesignToggle {
        draw_bg +: {
            pixel: fn() {
                let sdf = Sdf2d.viewport(self.pos * self.rect_size)
                sdf.box(0.0,0.0,self.rect_size.x,self.rect_size.y,self.rect_size.y*0.25)
                sdf.fill(#f4f6f9)
                let radius=self.rect_size.y*0.4
                let x=mix(self.rect_size.y*0.5,self.rect_size.x-self.rect_size.y*0.5,self.active)
                sdf.circle(x,self.rect_size.y*0.5,radius)
                sdf.fill(mix(#dfe5ee,#x4c5fef,self.active))
                return sdf.result
            }
        }
    }
    mod.prelude.widgets.DesignAtroToggle = mod.widgets.DesignAtroToggle
    mod.widgets.DesignAtroCheckbox = CheckBox {
        padding: 0 text: ""
        icon_walk: Walk{width: 0 height: 0}
        label_walk: Walk{width: 0 height: 0 margin: Inset{}}
        draw_bg +: {
            silent: instance(0.0)
            ink: instance(#x4c5fef)
            mark_color: instance(#fff)
            corner_radius: instance(0.0)
            pixel: fn() {
                let sdf=Sdf2d.viewport(self.pos*self.rect_size)
                let size=self.rect_size.x*0.75
                let offset=(self.rect_size-vec2(size,size))*0.5
                let ink=self.ink
                if self.silent<0.5 {
                    if self.active<0.5 {
                        sdf.box(offset.x+0.75,offset.y+0.75,size-1.5,size-1.5,max(0.0,min(self.corner_radius,size*0.5)-0.75)*0.5)
                        sdf.stroke(ink,0.75)
                    } else {
                        sdf.box(offset.x,offset.y,size,size,min(self.corner_radius,size*0.5)*0.5)
                        sdf.fill(ink)
                    }
                }
                if self.active>0.5 {
                    sdf.move_to(offset.x+size*0.25,offset.y+size*0.52)
                    sdf.line_to(offset.x+size*0.43,offset.y+size*0.70)
                    sdf.line_to(offset.x+size*0.76,offset.y+size*0.31)
                    let width=max(1.0,size*0.09)
                    if self.mark_color.a<0.01 && self.silent<0.5 {
                        // Sketch's compound checkbox subtracts its checkmark:
                        // the visible color belongs to the actual backdrop.
                        let coverage=sdf.calc_blur(abs(sdf.shape)-width/sdf.scale_factor)
                        return sdf.result*(1.0-coverage)
                    }
                    sdf.stroke(mix(self.mark_color,#dfe5ee,self.silent),width)
                }
                return sdf.result
            }
        }
    }
    mod.prelude.widgets.DesignAtroCheckbox = mod.widgets.DesignAtroCheckbox
    mod.widgets.DesignCamoCheckbox = CheckBox {
        padding: 0 text: ""
        icon_walk: Walk{width: 0 height: 0}
        label_walk: Walk{width: 0 height: 0 margin: Inset{}}
        draw_bg +: {
            ink: instance(#0077ff)
            off_color: instance(#0000001a)
            mark_color: instance(#fff)
            corner_radius: instance(3.0)
            pixel: fn() {
                let sdf=Sdf2d.viewport(self.pos*self.rect_size)
                let w=self.rect_size.x
                let h=self.rect_size.y
                if self.active<0.5 {
                    sdf.box(0.0,0.0,w,h,self.corner_radius*0.5)
                    sdf.fill(self.off_color)
                } else {
                    sdf.box(0.0,0.0,w,h,self.corner_radius*0.5)
                    sdf.fill(self.ink)
                    sdf.move_to(w*0.28,h*0.5)
                    sdf.line_to(w*0.44,h*0.64)
                    sdf.line_to(w*0.72,h*0.34)
                    sdf.stroke(self.mark_color,w*0.055)
                }
                return sdf.result
            }
        }
    }
    mod.prelude.widgets.DesignCamoCheckbox = mod.widgets.DesignCamoCheckbox
    mod.widgets.DesignCamoRadio = RadioButton {
        padding: 0 text: ""
        icon_walk: Walk{width: 0 height: 0}
        label_walk: Walk{width: 0 height: 0 margin: Inset{}}
        draw_bg +: {
            ink: instance(#0077ff)
            off_color: instance(#0000001a)
            mark_color: instance(#fff)
            corner_radius: instance(9.0)
            pixel: fn() {
                let sdf=Sdf2d.viewport(self.pos*self.rect_size)
                let center=self.rect_size*0.5
                let radius=min(center.x,center.y)
                if self.active<0.5 {
                    sdf.circle(center.x,center.y,radius)
                    sdf.fill(self.off_color)
                } else {
                    sdf.circle(center.x,center.y,radius)
                    sdf.fill(self.ink)
                    sdf.circle(center.x,center.y,radius*4.0/9.0)
                    sdf.fill(self.mark_color)
                }
                return sdf.result
            }
        }
    }
    mod.prelude.widgets.DesignCamoRadio = mod.widgets.DesignCamoRadio
    mod.widgets.DesignCamoToggle = mod.widgets.DesignToggle {
        draw_bg +: {
            ink: instance(#0077ff)
            off_color: instance(#0000001a)
            mark_color: instance(#fff)
            corner_radius: instance(12.0)
            pixel: fn() {
                let sdf=Sdf2d.viewport(self.pos*self.rect_size)
                sdf.box(0.0,0.0,self.rect_size.x,self.rect_size.y,self.corner_radius*0.5)
                sdf.fill(mix(self.off_color,self.ink,self.active))
                let radius=self.rect_size.y*0.5-4.0
                let x=mix(4.0+radius,self.rect_size.x-4.0-radius,self.active)
                sdf.circle(x,self.rect_size.y*0.5,radius)
                sdf.fill(self.mark_color)
                return sdf.result
            }
        }
    }
    mod.prelude.widgets.DesignCamoToggle = mod.widgets.DesignCamoToggle
}

/// Rich design text retains individually inspectable native labels. Its text
/// value comes from those mounted children, so Studio checks rendered content.
#[derive(Script, ScriptHook, Widget)]
pub struct DesignText {
    #[deref]
    view: View,
    #[live]
    foreground_only: bool,
}

/// Makepad Label has no rotation property; use its native rotated text renderer
/// while preserving the source text, frame and Studio identity.
#[derive(Script, ScriptHook, Widget)]
pub struct DesignRotatedLabel {
    #[uid]
    uid: WidgetUid,
    #[source]
    source: ScriptObjectRef,
    #[walk]
    walk: Walk,
    #[live]
    align: Align,
    #[live]
    padding: Inset,
    #[live]
    clip_x: bool,
    #[live]
    clip_y: bool,
    #[live]
    flow: Flow,
    #[live]
    text: String,
    #[redraw]
    #[live]
    pub draw_text: DrawRotatedText,
    #[area]
    #[rust]
    area: Area,
    #[rust]
    pub text_layout_rect: Rect,
}

impl Widget for DesignRotatedLabel {
    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        cx.begin_turtle(
            walk,
            Layout {
                flow: self.flow,
                clip_x: false,
                clip_y: false,
                ..Default::default()
            },
        );
        let frame = cx.turtle().rect();
        self.draw_text.rotation_origin = vec2(
            (frame.pos.x + frame.size.x / 2.) as f32,
            (frame.pos.y + frame.size.y / 2.) as f32,
        );
        self.text_layout_rect = self.draw_text.draw_walk(cx, walk, self.align, &self.text);
        cx.end_turtle_with_area(&mut self.area);
        DrawStep::done()
    }
    fn handle_event(&mut self, _cx: &mut Cx, _event: &Event, _scope: &mut Scope) {}
    fn text(&self) -> String {
        self.text.clone()
    }
}

/// Source-styled buttons retain their native child hierarchy and enforce the
/// enabled state through the widget event contract inspected by Studio.
#[derive(Script, ScriptHook, Widget)]
pub struct DesignButton {
    #[deref]
    view: View,
    #[live(true)]
    enabled: bool,
    #[live]
    glass: bool,
    #[rust]
    pub draw_list: Option<DrawList2d>,
}

#[derive(Script, ScriptHook, Widget)]
pub struct DesignOverlay {
    #[deref]
    view: View,
    #[rust]
    pub draw_list: Option<DrawList2d>,
}

/// SVG supplies the native tooltip silhouette; Gauss supplies its backdrop.
#[derive(Script, ScriptHook, Widget)]
pub struct DesignGlassSvg {
    #[deref]
    pub svg: Svg,
    #[live(1.0)]
    blur: f32,
    #[rust]
    pub draw_list: Option<DrawList2d>,
    #[rust]
    pub snapshot_ready: bool,
}

impl Widget for DesignGlassSvg {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let inline = cx.is_drawing_overlay();
        if !inline {
            self.draw_list
                .get_or_insert_with(|| DrawList2d::new(cx))
                .begin_overlay_reuse(cx);
        }
        let snapshot = makepad_widgets::gauss_view::request_window_gauss(cx);
        self.snapshot_ready = snapshot.is_some();
        if let Some(snapshot) = snapshot {
            // Match GaussRoundedView's reconstructed fractional mip sampling,
            // including the same DPI and kernel calibration used by surfaces.
            let level = (self.blur * snapshot.dpi_factor as f32 * 0.75)
                .max(1.0)
                .log2()
                .clamp(0.0, 6.0);
            let low = level.floor() as usize;
            let high = (low + 1).min(6);
            let texture = |level: usize| {
                if level == 0 {
                    &snapshot.scene_texture
                } else {
                    snapshot
                        .mip_textures
                        .get(level - 1)
                        .unwrap_or(&snapshot.scene_texture)
                }
            };
            let vars = &mut self.svg.draw_svg.draw_vars;
            vars.set_texture(1, texture(low));
            vars.set_texture(2, texture(high));
            let mix = level.fract();
            vars.set_uniform(cx, live_id!(backdrop_mix), &[mix * mix * (3.0 - 2.0 * mix)]);
            vars.set_uniform(
                cx,
                live_id!(backdrop_low_kernel),
                &[if low == 0 { 0.0 } else { 1.0 }],
            );
            vars.set_uniform(
                cx,
                live_id!(backdrop_high_kernel),
                &[if high == 0 { 0.0 } else { 1.0 }],
            );
            vars.set_uniform(
                cx,
                live_id!(source_size),
                &[snapshot.source_size.x as f32, snapshot.source_size.y as f32],
            );
            vars.set_uniform(cx, live_id!(source_y_flip), &[snapshot.source_y_flip]);
        }
        let step = self.svg.draw_walk(cx, scope, walk);
        if !inline {
            self.draw_list.as_mut().unwrap().end(cx);
        }
        step
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.svg.handle_event(cx, event, scope)
    }
}

/// Pill input is a native CheckBox; this container keeps its source Label
/// independently inspectable and follows the checkbox's color state.
#[derive(Script, ScriptHook, Widget)]
pub struct DesignPill {
    #[deref]
    view: View,
    #[rust]
    initial_active: Option<bool>,
    #[rust]
    initial_labels: Vec<(WidgetRef, Vec4)>,
}

impl Widget for DesignPill {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let before = self
            .view
            .children
            .iter()
            .find_map(|(_, w)| w.borrow::<CheckBox>().map(|c| c.active(cx)));
        if self.initial_active.is_none() {
            self.initial_active = before;
            self.initial_labels = self
                .view
                .children
                .iter()
                .filter_map(|(_, w)| w.borrow::<Label>().map(|l| (w.clone(), l.draw_text.color)))
                .collect();
        }
        self.view.handle_event(cx, event, scope);
        let after = self
            .view
            .children
            .iter()
            .find_map(|(_, w)| w.borrow::<CheckBox>().map(|c| c.active(cx)));
        if before != after {
            for (child, color) in &self.initial_labels {
                if let Some(mut label) = child.borrow_mut::<Label>() {
                    label.draw_text.color = if after == self.initial_active {
                        *color
                    } else {
                        vec4(1., 1., 1., 1.)
                    };
                }
            }
            self.view.redraw(cx);
        }
    }
}

impl Widget for DesignOverlay {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        if cx.is_drawing_overlay() {
            return self.view.draw_walk(cx, scope, walk);
        }
        let list = self.draw_list.get_or_insert_with(|| DrawList2d::new(cx));
        list.begin_overlay_reuse(cx);
        let step = self.view.draw_walk(cx, scope, walk);
        list.end(cx);
        step
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope)
    }
}

impl DesignButton {
    pub fn set_selected(&mut self, cx: &mut Cx, selected: bool) {
        self.view.selected=Some(selected);
        self.view.redraw(cx);
    }
}

impl Widget for DesignButton {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        if self.glass && !cx.is_drawing_overlay() {
            let list = self.draw_list.get_or_insert_with(|| DrawList2d::new(cx));
            list.begin_overlay_reuse(cx);
            let step = self.view.draw_walk(cx, scope, walk);
            list.end(cx);
            step
        } else {
            self.view.draw_walk(cx, scope, walk)
        }
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        if self.enabled || !event.requires_visibility() {
            self.view.handle_event(cx, event, scope);
        }
    }
    fn disabled(&self, _cx: &Cx) -> bool {
        !self.enabled
    }
    fn set_disabled(&mut self, cx: &mut Cx, disabled: bool) {
        self.enabled = !disabled;
        self.view.redraw(cx);
    }
    fn is_interactive(&self) -> bool {
        self.enabled
    }
    fn selected_value(&self, _cx: &Cx) -> Option<String> {
        self.view.selected.map(|selected| selected.to_string())
    }
}

/// A source-styled radio retains its label and icon hierarchy. Activating an
/// already selected radio keeps it selected, as with a standard radio button.
#[derive(Script, ScriptHook, Widget)]
pub struct DesignRadio {
    #[deref]
    view: View,
    #[live]
    active: bool,
}

impl Widget for DesignRadio {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
        if let Hit::FingerDown(_) = event.hits(cx, self.view.area()) {
            self.active = true;
            self.view.redraw(cx);
        }
    }
    fn checked(&self, _cx: &Cx) -> Option<bool> {
        Some(self.active)
    }
    fn is_interactive(&self) -> bool {
        true
    }
}

impl Widget for DesignText {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
    }
    fn text(&self) -> String {
        if self.foreground_only {
            return self
                .view
                .children
                .last()
                .map(|(_, child)| child.text())
                .unwrap_or_default();
        }
        fn content(widget: &WidgetRef) -> String {
            let text = widget.text();
            if !text.is_empty() {
                return text;
            }
            let mut result = String::new();
            widget.children(&mut |_, child| result.push_str(&content(&child)));
            result
        }
        self.view
            .children
            .iter()
            .map(|(_, child)| content(child))
            .collect()
    }
}
