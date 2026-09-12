//! Generic data/binding adapter and inspection evidence for the beauty host.
//! No screen IDs or app-specific layout branches belong here.
use makepad_widgets::*;
use makepad_plot::{LinePlot,ScatterPlot,DonutArcStyle,DonutChart,DonutSlice,BarPlot,BarInterval,RadarChart,RadarSeries,ChartPaint,Series,LineStyle,MarkerStyle,LegendPosition};
use serde_json::{Value,json};
use sha2::{Digest,Sha256};
use std::{collections::BTreeMap,path::{Path,PathBuf}};
use splash_widgets::progress::DesignProgressBar;

#[derive(Default)]
pub struct Session {
    entries: Vec<Value>,
    ids: BTreeMap<String,String>,
    state: Value,
    output: PathBuf,
    probe: PathBuf,
    last_probe: String,
}

fn color(v:&Value)->Vec4 {
    let s=v.as_str().unwrap_or("#000000").trim_start_matches('#');
    let n=u32::from_str_radix(s,16).unwrap_or(0);
    if s.len()==8 {vec4(((n>>24)&255) as f32/255.,((n>>16)&255) as f32/255.,((n>>8)&255) as f32/255.,(n&255) as f32/255.)}
    else {vec4(((n>>16)&255) as f32/255.,((n>>8)&255) as f32/255.,(n&255) as f32/255.,1.)}
}

fn paint(v:&Value)->Option<ChartPaint> {
    let mut stops=v["stops"].as_array()?.iter().map(|s|Some((s[0].as_f64()? as f32,color(&s[1])))).collect::<Option<Vec<_>>>()?;
    stops.sort_by(|a,b|a.0.total_cmp(&b.0));
    Some(ChartPaint {stops,angular_center:(v["angular_center"][0].as_f64().unwrap_or(0.5) as f32,v["angular_center"][1].as_f64().unwrap_or(0.5) as f32),angular:v["angular"].as_bool().unwrap_or(false),radial:v["radial"].as_bool().unwrap_or(false),
        from:(v["from"][0].as_f64()? as f32,v["from"][1].as_f64()? as f32),
        to:(v["to"][0].as_f64()? as f32,v["to"][1].as_f64()? as f32)})
}

impl Session {
    fn widget(&self,cx:&mut Cx,ui:&WidgetRef,id:&str)->WidgetRef {
        self.ids.get(id).map(|id|ui.widget(cx,&[LiveId::from_str(id)])).unwrap_or_default()
    }
    pub fn mount(&mut self,cx:&mut Cx,ui:&WidgetRef,request:&Value,elements:&[Value])->Result<(),String> {
        *self=Self::default();
        let Some(path)=request["semantic"].as_str() else {return Ok(());};
        let root=Path::new(path).parent().ok_or("semantic path needs a parent")?;
        let data_path=request["data"].as_str().ok_or("missing runtime kit data")?;
        let data:Value=serde_json::from_slice(&std::fs::read(data_path).map_err(|e|e.to_string())?).map_err(|e|e.to_string())?;
        self.entries=data["$kit"]["bindings"].as_array().ok_or("missing runtime kit bindings")?.clone();
        self.ids=elements.iter().filter_map(|e|Some((e["original_id"].as_str()?.to_owned(),e["id"].as_str()?.to_owned()))).collect();
        self.output=PathBuf::from(request["semantic_result"].as_str().ok_or("missing semantic_result")?);
        self.probe=PathBuf::from(request["semantic_probe"].as_str().ok_or("missing semantic_probe")?);
        self.state=json!({"build_id":request["build_id"],"nonce":request["nonce"],"elements":{}});
        for entry in self.entries.clone() {
            let id=entry["id"].as_str().ok_or("semantic element needs an ID")?;
            let widget=self.widget(cx,ui,id);
            let mut result=json!({"native_id":self.ids.get(id)});
            if let Some(relative)=entry["data"]["path"].as_str() {
                let bytes=std::fs::read(root.join(relative)).map_err(|e|e.to_string())?;
                let checksum=format!("{:x}",Sha256::digest(&bytes));
                if entry["data"]["sha256"].as_str()!=Some(checksum.as_str()) {return Err("stale native chart data".into());}
                let rows:Vec<Value>=serde_json::from_slice(&bytes).map_err(|e|e.to_string())?;
                let xkey=entry["data"]["x_key"].as_str().ok_or("missing numeric x key")?;
                let ykeys=entry["data"]["y_keys"].as_array().ok_or("missing series keys")?;
                let xs:Vec<f64>=rows.iter().map(|r|r[xkey].as_f64().unwrap_or(f64::NAN)).collect();
                if xs.iter().any(|v|!v.is_finite()) {return Err("invalid x samples".into());}
                if let Some(mut plot)=widget.borrow_mut::<LinePlot>() {
                    plot.clear();plot.demo_data=false;
                    plot.plot_view.legend=LegendPosition::None;
                    plot.plot_view.data_padding=0.;
                    plot.gradient_fill=entry["plot"]["gradient_fill"].as_bool().unwrap_or(false);
                    for (i,key) in ykeys.iter().enumerate() {
                        let key=key.as_str().ok_or("invalid series key")?;
                        let ys:Vec<f64>=rows.iter().map(|r|r[key].as_f64().unwrap_or(f64::NAN)).collect();
                        if ys.iter().any(|v|!v.is_finite()) {return Err("invalid y samples".into());}
                        let mut series=Series::new("").with_data(xs.clone(),ys.clone()).with_color(color(&entry["plot"]["colors"][i]));
                        series.line_width=Some(entry["plot"]["line_widths"][i].as_f64().or(entry["plot"]["line_width"].as_f64()).unwrap_or(1.2));
                        if entry["plot"]["mode"]=="waveform" {
                            for (&x,&amplitude) in xs.iter().zip(&ys) {
                                let mut bar=Series::new("").with_data(vec![x,x],vec![-amplitude,amplitude]).with_color(color(&entry["plot"]["colors"][i]));
                                bar.line_width=series.line_width;plot.add_series(bar);
                            }
                        } else {plot.add_series(series);plot.series_paints.push(paint(&entry["plot"]["line_paints"][i]));}
                        if let Some(fill)=entry["plot"]["fills"].get(i).filter(|v|!v.is_null()) {
                            plot.fill_between_baseline(xs.clone(),ys,entry["data"]["domain"]["y"][0].as_f64().unwrap_or(0.),color(fill));
                            plot.fill_paints.push(paint(&entry["plot"]["fill_paints"][i]));
                        }
                    }
                    let domain=&entry["data"]["domain"];
                    plot.set_xlim(domain["x"][0].as_f64().unwrap_or(0.),domain["x"][1].as_f64().unwrap_or(1.));
                    plot.set_ylim(domain["y"][0].as_f64().unwrap_or(0.),domain["y"][1].as_f64().unwrap_or(1.));
                    for axis in ["horizontal","vertical"] {
                        if let Some(lines)=entry["plot"][axis].as_array() {
                            for line in lines {
                                let v=line["value"].as_f64().unwrap_or(0.);
                                let ink=color(&line["color"]);
                                let width=line["width"].as_f64().unwrap_or(0.5);
                                let style=if line["dotted"].as_bool()==Some(true) {LineStyle::Dotted}else{LineStyle::Solid};
                                if axis=="horizontal" {plot.axhline(v,ink,width,style)} else {plot.axvline(v,ink,width,style)}
                            }
                        }
                    }
                    if let Some(guides)=entry["plot"]["guides"].as_array() {
                        for guide in guides {
                            let x=guide["x"].as_f64().unwrap_or(0.);
                            let mut series=Series::new("").with_data(vec![x,x],vec![guide["y0"].as_f64().unwrap_or(0.),guide["y1"].as_f64().unwrap_or(1.)]).with_color(color(&guide["color"]));
                            series.line_style=LineStyle::Dotted;series.line_width=Some(guide["width"].as_f64().unwrap_or(0.5));plot.add_series(series);
                        }
                    }
                    if let Some(markers)=entry["plot"]["markers"].as_array() {
                        for marker in markers {
                            let mut series=Series::new("").with_data(vec![marker["x"].as_f64().unwrap_or(0.)],vec![marker["y"].as_f64().unwrap_or(0.)]).with_color(color(&marker["color"]));
                            series.marker_style=MarkerStyle::Circle;
                            series.marker_size=Some(marker["radius"].as_f64().unwrap_or(3.));
                            plot.add_series(series);
                        }
                    }
                    result["series"]=json!(plot.series.iter().map(|s|json!({"x":s.x,"y":s.y})).collect::<Vec<_>>());
                    result["domain"]=domain.clone();
                    plot.redraw(cx);
                } else if let Some(mut plot)=widget.borrow_mut::<DonutChart>() {
                    let key=ykeys[0].as_str().ok_or("missing donut value key")?;
                    plot.demo_data=false;plot.show_center_label=entry["plot"]["show_center_label"].as_bool().unwrap_or(false);
                    plot.plot_view.legend=LegendPosition::None;
                    plot.slice_gap=entry["plot"]["slice_gap"].as_f64().unwrap_or(0.);
                    plot.styled_arcs=entry["plot"]["styled_arcs"].as_bool().unwrap_or(false);
                    plot.rounded_caps=entry["plot"]["rounded_caps"].as_bool().unwrap_or(false);
                    plot.start_angle=entry["plot"]["start_angle"].as_f64().unwrap_or(-std::f64::consts::FRAC_PI_2);
                    plot.direction=entry["plot"]["direction"].as_f64().unwrap_or(1.);
                    plot.radius_scale=entry["plot"]["radius_scale"].as_f64().unwrap_or(0.95);
                    plot.track_color=entry["plot"]["track_color"].as_str().map(|_|color(&entry["plot"]["track_color"])).unwrap_or(vec4(0.,0.,0.,0.));
                    plot.independent_arcs=entry["plot"]["arcs"].as_array().map(|a|a.iter().map(|v|DonutArcStyle {
                        center:(v["center"][0].as_f64().unwrap_or(0.5),v["center"][1].as_f64().unwrap_or(0.5)),
                        radius_scale:v["radius_scale"].as_f64().unwrap_or(1.),inner:v["inner_radius"].as_f64().unwrap_or(0.7),
                        start:v["start_angle"].as_f64().unwrap_or(0.),direction:v["direction"].as_f64().unwrap_or(1.),
                        rounded:v["rounded_caps"].as_bool().unwrap_or(false),
                        dash_length:v["dash_length"].as_f64().unwrap_or(0.) as f32,dash_gap:v["dash_gap"].as_f64().unwrap_or(0.) as f32,
                    }).collect()).unwrap_or_default();
                    plot.slice_paints=rows.iter().enumerate().map(|(i,_)|paint(&entry["plot"]["paints"][i])).collect();
                    plot.set_slices(rows.iter().enumerate().map(|(i,r)|DonutSlice::new(r["label"].as_str().unwrap_or(""),r[key].as_f64().unwrap_or(0.)).with_color(color(&entry["plot"]["colors"][i]))).collect());
                    plot.set_inner_radius_ratio(entry["plot"]["inner_radius"].as_f64().unwrap_or(0.7));
                    result["values"]=json!(plot.slices.iter().map(|s|s.value).collect::<Vec<_>>());
                    plot.redraw(cx);
                } else if let Some(mut plot)=widget.borrow_mut::<BarPlot>() {
                    plot.demo_data=false;plot.plot_view.legend=LegendPosition::None;
                    let intervals=rows.iter().enumerate().map(|(i,r)| {
                        let style=&entry["plot"]["bars"][i];
                        Ok(BarInterval{x:r[xkey].as_f64().ok_or("invalid bar category")?,
                            low:r["low"].as_f64().ok_or("invalid bar baseline")?,
                            high:r["high"].as_f64().ok_or("invalid bar value")?,
                            width:r["width"].as_f64().ok_or("invalid bar width")?,
                            color:color(&style["color"]),paint:paint(&style["paint"]),
                            radius:style["radius"].as_f64().unwrap_or(0.) as f32,
                            depth:style["depth"].as_f64().unwrap_or(0.) as f32,
                            side_colors:style["side_colors"].as_array().filter(|a|a.len()==2).map(|a|(color(&a[0]),color(&a[1])))})
                    }).collect::<Result<Vec<_>,String>>()?;
                    let d=&entry["data"]["domain"];
                    plot.set_intervals(intervals,[d["x"][0].as_f64().unwrap_or(0.),d["x"][1].as_f64().unwrap_or(1.),d["y"][0].as_f64().unwrap_or(0.),d["y"][1].as_f64().unwrap_or(1.)]);
                    result["intervals"]=json!(plot.intervals.iter().map(|b|json!({"x":b.x,"low":b.low,"high":b.high,"width":b.width})).collect::<Vec<_>>());
                    plot.redraw(cx);
                } else if let Some(mut plot)=widget.borrow_mut::<RadarChart>() {
                    plot.clear();plot.demo_data=false;plot.plot_view.legend=LegendPosition::None;
                    plot.show_spokes=entry["plot"]["show_spokes"].as_bool().unwrap_or(false);
                    plot.polygon_width=entry["plot"]["line_width"].as_f64().unwrap_or(0.);
                    plot.vertex_radius=entry["plot"]["vertex_radius"].as_f64().unwrap_or(0.);
                    plot.axis_center=(entry["plot"]["center"][0].as_f64().unwrap_or(0.5),entry["plot"]["center"][1].as_f64().unwrap_or(0.5));
                    plot.axis_vectors=entry["plot"]["axis_vectors"].as_array().map(|v|v.iter().map(|a|(a[0].as_f64().unwrap_or(0.),a[1].as_f64().unwrap_or(0.))).collect()).unwrap_or_default();
                    plot.set_axes(rows.iter().map(|r|r["label"].as_str().unwrap_or("").to_string()).collect());
                    plot.set_max_value(entry["data"]["domain"]["y"][1].as_f64().unwrap_or(1.));
                    for (i,key) in ykeys.iter().enumerate() {
                        let key=key.as_str().ok_or("invalid radar key")?;
                        let values=rows.iter().map(|r|r[key].as_f64().ok_or("invalid radar value")).collect::<Result<Vec<_>,_>>()?;
                        plot.fill_paints.push(paint(&entry["plot"]["fill_paints"][i]));
                        plot.add_series(RadarSeries::new("",values).with_color(color(&entry["plot"]["colors"][i]))
                            .with_fill_alpha(entry["plot"]["fill_alphas"][i].as_f64().unwrap_or(0.2)));
                    }
                    result["series"]=json!(plot.series.iter().map(|s|json!({"x":xs,"y":s.values})).collect::<Vec<_>>());
                    plot.redraw(cx);
                } else if let Some(mut plot)=widget.borrow_mut::<ScatterPlot>() {
                    plot.clear();plot.demo_data=false;plot.plot_view.legend=LegendPosition::None;plot.plot_view.data_padding=0.;
                    for (i,r) in rows.iter().enumerate() {
                        let mut series=Series::new("").with_data(vec![r[xkey].as_f64().ok_or("invalid bubble x")?],vec![r["y"].as_f64().ok_or("invalid bubble y")?]).with_color(color(&entry["plot"]["colors"][i])).with_marker(MarkerStyle::Circle);
                        series.marker_size=Some(r["radius"].as_f64().ok_or("invalid bubble radius")?);
                        plot.add_series(series);
                    }
                    plot.set_xlim(0.,1.);plot.set_ylim(0.,1.);
                    result["points"]=json!(plot.series.iter().map(|s|json!({"x":s.x[0],"y":s.y[0],"radius":s.marker_size})).collect::<Vec<_>>());
                    plot.redraw(cx);
                } else {return Err(format!("{id}: expected a native data plot"));}
                result["data_sha256"]=checksum.into();
            }
            if let Some(mut progress)=widget.borrow_mut::<DesignProgressBar>() {
                let colors=entry["progress"]["gradient"].as_array().filter(|a|a.len()==2).map(|a|(color(&a[0]),color(&a[1])));
                progress.set_style(cx,entry["progress"]["radius"].as_f64().unwrap_or(-1.) as f32,colors);
                result["value"]=progress.value.into();
            }
            if let Some(indicator)=entry["behavior"]["indicator"].as_str() {
                if let Some(view)=self.widget(cx,ui,indicator).borrow::<View>() {
                    if let Some(pos)=view.walk.abs_pos {
                        result["selected"]=json!((pos.x-entry["behavior"]["indicator_bounds"][0].as_f64().unwrap_or(0.)).abs()<0.1);
                    }
                }
            }
            self.state["elements"][id]=result;
        }
        self.flush();
        Ok(())
    }
    fn change(&mut self,cx:&mut Cx,ui:&WidgetRef,entry:&Value,value:&Value) {
        let Some(id)=entry["id"].as_str() else{return;};
        let binding=&entry["behavior"];
        let Some(target)=binding["target"].as_str() else{return;};
        let widget=self.widget(cx,ui,target);
        let mut before=Value::Null;let mut after=Value::Null;
        if binding["property"]=="viewport" {
            if let Some(mut plot)=widget.borrow_mut::<LinePlot>() {
                before=json!([plot.plot_view.viewport.x_min,plot.plot_view.viewport.x_max]);
                let lo=value[0].as_f64().unwrap_or(0.);let hi=value[1].as_f64().unwrap_or(1.);
                plot.set_xlim(lo,hi);plot.redraw(cx);after=json!([lo,hi]);
            }
        } else if binding["property"]=="value" {
            if let Some(mut progress)=widget.borrow_mut::<DesignProgressBar>() {
                before=progress.value.into();progress.set_value(cx,value.as_f64().unwrap_or(0.));after=progress.value.into();
                self.state["elements"][target]["value"]=after.clone();
            }
        }
        if before!=after && !before.is_null() {
            let state=&mut self.state["elements"][id];
            state["event"]=binding["event"].clone();state["target"]=binding["target"].clone();state["property"]=binding["property"].clone();
            state["before"]=before;state["after"]=after;
        }
        if let Some(indicator)=binding["indicator"].as_str() {
            if let Some(mut view)=self.widget(cx,ui,indicator).borrow_mut::<View>() {
                let b=&binding["indicator_bounds"];
                view.walk.abs_pos=Some(dvec2(b[0].as_f64().unwrap_or(0.),b[1].as_f64().unwrap_or(0.)));
                view.walk.width=Size::Fixed(b[2].as_f64().unwrap_or(0.));
                view.walk.height=Size::Fixed(b[3].as_f64().unwrap_or(0.));
                self.state["elements"][id]["indicator_bounds"]=b.clone();
            }
            for e in &self.entries {
                if e["behavior"]["indicator"]==binding["indicator"] {
                    if let Some(other)=e["id"].as_str(){
                        let selected=other==id;
                        self.state["elements"][other]["selected"]=json!(selected);
                        if let Some(label_id)=e["behavior"]["label"].as_str() {
                            let key=if selected {"active_color"} else {"inactive_color"};
                            let label_widget=self.widget(cx,ui,label_id);
                            if let Some(mut label)=label_widget.borrow_mut::<Label>() {
                                label.draw_text.color=color(&e["behavior"][key]);
                                let ink=label.draw_text.color;
                                self.state["elements"][other]["label_color"]=json!([ink.x,ink.y,ink.z,ink.w]);
                            }
                            label_widget.redraw(cx);
                        }
                    }
                }
            }
        }
        cx.redraw_all();self.flush();
    }
    pub fn actions(&mut self,cx:&mut Cx,ui:&WidgetRef,actions:&Actions) {
        for entry in self.entries.clone() {
            if entry["behavior"]["event"]=="click" {
                let id=entry["id"].as_str().unwrap_or("");
                if self.widget(cx,ui,id).as_button().clicked(actions) {
                    self.change(cx,ui,&entry,&entry["behavior"]["value"]);
                }
            }
        }
    }
    pub fn poll(&mut self,cx:&mut Cx,ui:&WidgetRef) {
        let mut changed=false;
        for entry in &self.entries {
            let Some(id)=entry["id"].as_str() else {continue;};
            if let Some(progress)=self.widget(cx,ui,id).borrow::<DesignProgressBar>() {
                let painted=json!(progress.painted_value());
                if self.state["elements"][id]["painted_value"]!=painted {
                    self.state["elements"][id]["painted_value"]=painted;changed=true;
                }
            }
        }
        if changed {self.flush();}
        if self.probe.as_os_str().is_empty(){return;}
        let Ok(source)=std::fs::read_to_string(&self.probe) else{return;};
        if source==self.last_probe{return;}
        self.last_probe=source.clone();
        if let Ok(probe)=serde_json::from_str::<Value>(&source) {
            if let Some(entry)=self.entries.iter().find(|e|e["id"]==probe["id"]).cloned() {
                self.change(cx,ui,&entry,&probe["value"]);
            }
        }
    }
    fn flush(&self) {
        if !self.output.as_os_str().is_empty() {
            let temp=self.output.with_extension("new");
            if let Ok(bytes)=serde_json::to_vec_pretty(&self.state) {
                if std::fs::write(&temp,bytes).is_ok(){let _=std::fs::rename(temp,&self.output);}
            }
        }
    }
}
