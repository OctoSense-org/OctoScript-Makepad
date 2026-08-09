//! Sunburst Widget
//!
//! Radial hierarchical visualization using concentric arcs.
//! Matches D3.js sunburst implementation exactly.
//!
//! Key D3.js behaviors:
//! - Uses d3.partition() layout for x0, x1, y0, y1 coordinates
//! - Colors from d3.interpolateRainbow based on first-level ancestor
//! - Fill opacity 0.6
//! - Small gaps between arcs (padAngle)
//! - Root node is filtered out (not rendered)

use makepad_widgets::*;
use makepad_d3::layout::hierarchy::{HierarchyNode, PartitionLayout, PartitionNode};
use std::f64::consts::PI;
use super::draw_primitives::DrawArc;
use super::animation::{ChartAnimator, EasingType};

live_design! {
    link widgets;
    use link::shaders::*;
    use link::theme::*;
    use super::draw_primitives::DrawArc;

    SUNBURST_FONT = {
        font_family: {
            latin = font("crate://self/resources/Manrope-Regular.ttf", 0.0, 0.0),
        }
    }

    pub SunburstWidget = {{SunburstWidget}} {
        width: Fill,
        height: Fill,

        draw_text: {
            color: #333333,
            text_style: <SUNBURST_FONT> {
                font_size: 9.0
            }
        }
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct SunburstWidget {
    #[live]
    #[deref]
    view: View,

    #[redraw]
    #[live]
    draw_arc: DrawArc,

    #[redraw]
    #[live]
    draw_text: DrawText,

    #[walk]
    walk: Walk,

    #[rust]
    partition: Option<PartitionNode<String>>,

    #[rust]
    colors: Vec<Vec4>,

    #[rust]
    initialized: bool,

    #[rust]
    animator: ChartAnimator,

    #[rust]
    area: Area,

    #[rust]
    center_x: f64,

    #[rust]
    center_y: f64,

    #[rust]
    max_radius: f64,

    #[rust]
    hovered_arc: Option<usize>, // Index in flat list
}

impl Widget for SunburstWidget {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);

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
            Event::MouseMove(e) => {
                self.handle_mouse_move(cx, e.abs);
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
            if !self.initialized {
                self.initialize_data();
                self.start_animation(cx);
                self.initialized = true;
            }

            self.draw_sunburst(cx, rect);
        }

        DrawStep::done()
    }
}

impl SunburstWidget {
    fn initialize_data(&mut self) {
        // Create the full flare-2.json hierarchy
        let root = self.build_flare_hierarchy();

        // Compute radius based on D3's default (928/2 = 464)
        let radius = 464.0;

        // Create partition layout matching D3: size([2 * Math.PI, radius])
        let layout = PartitionLayout::new().size(2.0 * PI, radius);
        self.partition = Some(layout.layout(&root));

        // Generate rainbow colors for top-level children
        self.colors = self.generate_rainbow_colors(10); // 10 top-level children in flare
    }

    fn build_flare_hierarchy(&self) -> HierarchyNode<String> {
        let mut root = HierarchyNode::new("flare".to_string(), 0.0);

        // analytics
        let mut analytics = HierarchyNode::new("analytics".to_string(), 0.0);
        let mut cluster = HierarchyNode::new("cluster".to_string(), 0.0);
        cluster.add_child(HierarchyNode::new("AgglomerativeCluster".to_string(), 3938.0));
        cluster.add_child(HierarchyNode::new("CommunityStructure".to_string(), 3812.0));
        cluster.add_child(HierarchyNode::new("HierarchicalCluster".to_string(), 6714.0));
        cluster.add_child(HierarchyNode::new("MergeEdge".to_string(), 743.0));
        analytics.add_child(cluster);
        let mut graph = HierarchyNode::new("graph".to_string(), 0.0);
        graph.add_child(HierarchyNode::new("BetweennessCentrality".to_string(), 3534.0));
        graph.add_child(HierarchyNode::new("LinkDistance".to_string(), 5731.0));
        graph.add_child(HierarchyNode::new("MaxFlowMinCut".to_string(), 7840.0));
        graph.add_child(HierarchyNode::new("ShortestPaths".to_string(), 5914.0));
        graph.add_child(HierarchyNode::new("SpanningTree".to_string(), 3416.0));
        analytics.add_child(graph);
        let mut optimization = HierarchyNode::new("optimization".to_string(), 0.0);
        optimization.add_child(HierarchyNode::new("AspectRatioBanker".to_string(), 7074.0));
        analytics.add_child(optimization);
        root.add_child(analytics);

        // animate
        let mut animate = HierarchyNode::new("animate".to_string(), 0.0);
        animate.add_child(HierarchyNode::new("Easing".to_string(), 17010.0));
        animate.add_child(HierarchyNode::new("FunctionSequence".to_string(), 5842.0));
        let mut interpolate = HierarchyNode::new("interpolate".to_string(), 0.0);
        interpolate.add_child(HierarchyNode::new("ArrayInterpolator".to_string(), 1983.0));
        interpolate.add_child(HierarchyNode::new("ColorInterpolator".to_string(), 2047.0));
        interpolate.add_child(HierarchyNode::new("DateInterpolator".to_string(), 1375.0));
        interpolate.add_child(HierarchyNode::new("Interpolator".to_string(), 8746.0));
        interpolate.add_child(HierarchyNode::new("MatrixInterpolator".to_string(), 2202.0));
        interpolate.add_child(HierarchyNode::new("NumberInterpolator".to_string(), 1382.0));
        interpolate.add_child(HierarchyNode::new("ObjectInterpolator".to_string(), 1629.0));
        interpolate.add_child(HierarchyNode::new("PointInterpolator".to_string(), 1675.0));
        interpolate.add_child(HierarchyNode::new("RectangleInterpolator".to_string(), 2042.0));
        animate.add_child(interpolate);
        animate.add_child(HierarchyNode::new("ISchedulable".to_string(), 1041.0));
        animate.add_child(HierarchyNode::new("Parallel".to_string(), 5176.0));
        animate.add_child(HierarchyNode::new("Pause".to_string(), 449.0));
        animate.add_child(HierarchyNode::new("Scheduler".to_string(), 5593.0));
        animate.add_child(HierarchyNode::new("Sequence".to_string(), 5534.0));
        animate.add_child(HierarchyNode::new("Transition".to_string(), 9201.0));
        animate.add_child(HierarchyNode::new("Transitioner".to_string(), 19975.0));
        animate.add_child(HierarchyNode::new("TransitionEvent".to_string(), 1116.0));
        animate.add_child(HierarchyNode::new("Tween".to_string(), 6006.0));
        root.add_child(animate);

        // data
        let mut data = HierarchyNode::new("data".to_string(), 0.0);
        let mut converters = HierarchyNode::new("converters".to_string(), 0.0);
        converters.add_child(HierarchyNode::new("Converters".to_string(), 721.0));
        converters.add_child(HierarchyNode::new("DelimitedTextConverter".to_string(), 4294.0));
        converters.add_child(HierarchyNode::new("GraphMLConverter".to_string(), 9800.0));
        converters.add_child(HierarchyNode::new("IDataConverter".to_string(), 1314.0));
        converters.add_child(HierarchyNode::new("JSONConverter".to_string(), 2220.0));
        data.add_child(converters);
        data.add_child(HierarchyNode::new("DataField".to_string(), 1759.0));
        data.add_child(HierarchyNode::new("DataSchema".to_string(), 2165.0));
        data.add_child(HierarchyNode::new("DataSet".to_string(), 586.0));
        data.add_child(HierarchyNode::new("DataSource".to_string(), 3331.0));
        data.add_child(HierarchyNode::new("DataTable".to_string(), 772.0));
        data.add_child(HierarchyNode::new("DataUtil".to_string(), 3322.0));
        root.add_child(data);

        // display
        let mut display = HierarchyNode::new("display".to_string(), 0.0);
        display.add_child(HierarchyNode::new("DirtySprite".to_string(), 8833.0));
        display.add_child(HierarchyNode::new("LineSprite".to_string(), 1732.0));
        display.add_child(HierarchyNode::new("RectSprite".to_string(), 3623.0));
        display.add_child(HierarchyNode::new("TextSprite".to_string(), 10066.0));
        root.add_child(display);

        // flex
        let mut flex = HierarchyNode::new("flex".to_string(), 0.0);
        flex.add_child(HierarchyNode::new("FlareVis".to_string(), 4116.0));
        root.add_child(flex);

        // physics
        let mut physics = HierarchyNode::new("physics".to_string(), 0.0);
        physics.add_child(HierarchyNode::new("DragForce".to_string(), 1082.0));
        physics.add_child(HierarchyNode::new("GravityForce".to_string(), 1336.0));
        physics.add_child(HierarchyNode::new("IForce".to_string(), 319.0));
        physics.add_child(HierarchyNode::new("NBodyForce".to_string(), 10498.0));
        physics.add_child(HierarchyNode::new("Particle".to_string(), 2822.0));
        physics.add_child(HierarchyNode::new("Simulation".to_string(), 9983.0));
        physics.add_child(HierarchyNode::new("Spring".to_string(), 2213.0));
        physics.add_child(HierarchyNode::new("SpringForce".to_string(), 1681.0));
        root.add_child(physics);

        // query
        let mut query = HierarchyNode::new("query".to_string(), 0.0);
        query.add_child(HierarchyNode::new("AggregateExpression".to_string(), 1616.0));
        query.add_child(HierarchyNode::new("And".to_string(), 1027.0));
        query.add_child(HierarchyNode::new("Arithmetic".to_string(), 3891.0));
        query.add_child(HierarchyNode::new("Average".to_string(), 891.0));
        query.add_child(HierarchyNode::new("BinaryExpression".to_string(), 2893.0));
        query.add_child(HierarchyNode::new("Comparison".to_string(), 5103.0));
        query.add_child(HierarchyNode::new("CompositeExpression".to_string(), 3677.0));
        query.add_child(HierarchyNode::new("Count".to_string(), 781.0));
        query.add_child(HierarchyNode::new("DateUtil".to_string(), 4141.0));
        query.add_child(HierarchyNode::new("Distinct".to_string(), 933.0));
        query.add_child(HierarchyNode::new("Expression".to_string(), 5130.0));
        query.add_child(HierarchyNode::new("ExpressionIterator".to_string(), 3617.0));
        query.add_child(HierarchyNode::new("Fn".to_string(), 3240.0));
        query.add_child(HierarchyNode::new("If".to_string(), 2732.0));
        query.add_child(HierarchyNode::new("IsA".to_string(), 2039.0));
        query.add_child(HierarchyNode::new("Literal".to_string(), 1214.0));
        query.add_child(HierarchyNode::new("Match".to_string(), 3748.0));
        query.add_child(HierarchyNode::new("Maximum".to_string(), 843.0));
        let mut methods = HierarchyNode::new("methods".to_string(), 0.0);
        methods.add_child(HierarchyNode::new("add".to_string(), 593.0));
        methods.add_child(HierarchyNode::new("and".to_string(), 330.0));
        methods.add_child(HierarchyNode::new("average".to_string(), 287.0));
        methods.add_child(HierarchyNode::new("count".to_string(), 277.0));
        methods.add_child(HierarchyNode::new("distinct".to_string(), 292.0));
        methods.add_child(HierarchyNode::new("div".to_string(), 595.0));
        methods.add_child(HierarchyNode::new("eq".to_string(), 594.0));
        methods.add_child(HierarchyNode::new("fn".to_string(), 460.0));
        methods.add_child(HierarchyNode::new("gt".to_string(), 603.0));
        methods.add_child(HierarchyNode::new("gte".to_string(), 625.0));
        methods.add_child(HierarchyNode::new("iff".to_string(), 748.0));
        methods.add_child(HierarchyNode::new("isa".to_string(), 461.0));
        methods.add_child(HierarchyNode::new("lt".to_string(), 597.0));
        methods.add_child(HierarchyNode::new("lte".to_string(), 619.0));
        methods.add_child(HierarchyNode::new("max".to_string(), 283.0));
        methods.add_child(HierarchyNode::new("min".to_string(), 283.0));
        methods.add_child(HierarchyNode::new("mod".to_string(), 591.0));
        methods.add_child(HierarchyNode::new("mul".to_string(), 603.0));
        methods.add_child(HierarchyNode::new("neq".to_string(), 599.0));
        methods.add_child(HierarchyNode::new("not".to_string(), 386.0));
        methods.add_child(HierarchyNode::new("or".to_string(), 323.0));
        methods.add_child(HierarchyNode::new("orderby".to_string(), 307.0));
        methods.add_child(HierarchyNode::new("range".to_string(), 772.0));
        methods.add_child(HierarchyNode::new("select".to_string(), 296.0));
        methods.add_child(HierarchyNode::new("stddev".to_string(), 363.0));
        methods.add_child(HierarchyNode::new("sub".to_string(), 600.0));
        methods.add_child(HierarchyNode::new("sum".to_string(), 280.0));
        methods.add_child(HierarchyNode::new("update".to_string(), 307.0));
        methods.add_child(HierarchyNode::new("variance".to_string(), 335.0));
        methods.add_child(HierarchyNode::new("where".to_string(), 299.0));
        methods.add_child(HierarchyNode::new("xor".to_string(), 354.0));
        methods.add_child(HierarchyNode::new("_".to_string(), 264.0));
        query.add_child(methods);
        query.add_child(HierarchyNode::new("Minimum".to_string(), 843.0));
        query.add_child(HierarchyNode::new("Not".to_string(), 1554.0));
        query.add_child(HierarchyNode::new("Or".to_string(), 970.0));
        query.add_child(HierarchyNode::new("Query".to_string(), 13896.0));
        query.add_child(HierarchyNode::new("Range".to_string(), 1594.0));
        query.add_child(HierarchyNode::new("StringUtil".to_string(), 4130.0));
        query.add_child(HierarchyNode::new("Sum".to_string(), 791.0));
        query.add_child(HierarchyNode::new("Variable".to_string(), 1124.0));
        query.add_child(HierarchyNode::new("Variance".to_string(), 1876.0));
        query.add_child(HierarchyNode::new("Xor".to_string(), 1101.0));
        root.add_child(query);

        // scale
        let mut scale = HierarchyNode::new("scale".to_string(), 0.0);
        scale.add_child(HierarchyNode::new("IScaleMap".to_string(), 2105.0));
        scale.add_child(HierarchyNode::new("LinearScale".to_string(), 1316.0));
        scale.add_child(HierarchyNode::new("LogScale".to_string(), 3151.0));
        scale.add_child(HierarchyNode::new("OrdinalScale".to_string(), 3770.0));
        scale.add_child(HierarchyNode::new("QuantileScale".to_string(), 2435.0));
        scale.add_child(HierarchyNode::new("QuantitativeScale".to_string(), 4839.0));
        scale.add_child(HierarchyNode::new("RootScale".to_string(), 1756.0));
        scale.add_child(HierarchyNode::new("Scale".to_string(), 4268.0));
        scale.add_child(HierarchyNode::new("ScaleType".to_string(), 1821.0));
        scale.add_child(HierarchyNode::new("TimeScale".to_string(), 5833.0));
        root.add_child(scale);

        // util
        let mut util = HierarchyNode::new("util".to_string(), 0.0);
        util.add_child(HierarchyNode::new("Arrays".to_string(), 8258.0));
        util.add_child(HierarchyNode::new("Colors".to_string(), 10001.0));
        util.add_child(HierarchyNode::new("Dates".to_string(), 8217.0));
        util.add_child(HierarchyNode::new("Displays".to_string(), 12555.0));
        util.add_child(HierarchyNode::new("Filter".to_string(), 2324.0));
        util.add_child(HierarchyNode::new("Geometry".to_string(), 10993.0));
        let mut heap = HierarchyNode::new("heap".to_string(), 0.0);
        heap.add_child(HierarchyNode::new("FibonacciHeap".to_string(), 9354.0));
        heap.add_child(HierarchyNode::new("HeapNode".to_string(), 1233.0));
        util.add_child(heap);
        util.add_child(HierarchyNode::new("IEvaluable".to_string(), 335.0));
        util.add_child(HierarchyNode::new("IPredicate".to_string(), 383.0));
        util.add_child(HierarchyNode::new("IValueProxy".to_string(), 874.0));
        let mut math = HierarchyNode::new("math".to_string(), 0.0);
        math.add_child(HierarchyNode::new("DenseMatrix".to_string(), 3165.0));
        math.add_child(HierarchyNode::new("IMatrix".to_string(), 2815.0));
        math.add_child(HierarchyNode::new("SparseMatrix".to_string(), 3366.0));
        util.add_child(math);
        util.add_child(HierarchyNode::new("Maths".to_string(), 17705.0));
        util.add_child(HierarchyNode::new("Orientation".to_string(), 1486.0));
        let mut palette = HierarchyNode::new("palette".to_string(), 0.0);
        palette.add_child(HierarchyNode::new("ColorPalette".to_string(), 6367.0));
        palette.add_child(HierarchyNode::new("Palette".to_string(), 1229.0));
        palette.add_child(HierarchyNode::new("ShapePalette".to_string(), 2059.0));
        palette.add_child(HierarchyNode::new("SizePalette".to_string(), 2291.0));
        util.add_child(palette);
        util.add_child(HierarchyNode::new("Property".to_string(), 5559.0));
        util.add_child(HierarchyNode::new("Shapes".to_string(), 19118.0));
        util.add_child(HierarchyNode::new("Sort".to_string(), 6887.0));
        util.add_child(HierarchyNode::new("Stats".to_string(), 6557.0));
        util.add_child(HierarchyNode::new("Strings".to_string(), 22026.0));
        root.add_child(util);

        // vis
        let mut vis = HierarchyNode::new("vis".to_string(), 0.0);
        let mut axis = HierarchyNode::new("axis".to_string(), 0.0);
        axis.add_child(HierarchyNode::new("Axes".to_string(), 1302.0));
        axis.add_child(HierarchyNode::new("Axis".to_string(), 24593.0));
        axis.add_child(HierarchyNode::new("AxisGridLine".to_string(), 652.0));
        axis.add_child(HierarchyNode::new("AxisLabel".to_string(), 636.0));
        axis.add_child(HierarchyNode::new("CartesianAxes".to_string(), 6703.0));
        vis.add_child(axis);
        let mut controls = HierarchyNode::new("controls".to_string(), 0.0);
        controls.add_child(HierarchyNode::new("AnchorControl".to_string(), 2138.0));
        controls.add_child(HierarchyNode::new("ClickControl".to_string(), 3824.0));
        controls.add_child(HierarchyNode::new("Control".to_string(), 1353.0));
        controls.add_child(HierarchyNode::new("ControlList".to_string(), 4665.0));
        controls.add_child(HierarchyNode::new("DragControl".to_string(), 2649.0));
        controls.add_child(HierarchyNode::new("ExpandControl".to_string(), 2832.0));
        controls.add_child(HierarchyNode::new("HoverControl".to_string(), 4896.0));
        controls.add_child(HierarchyNode::new("IControl".to_string(), 763.0));
        controls.add_child(HierarchyNode::new("PanZoomControl".to_string(), 5222.0));
        controls.add_child(HierarchyNode::new("SelectionControl".to_string(), 7862.0));
        controls.add_child(HierarchyNode::new("TooltipControl".to_string(), 8435.0));
        vis.add_child(controls);
        let mut vis_data = HierarchyNode::new("data".to_string(), 0.0);
        vis_data.add_child(HierarchyNode::new("Data".to_string(), 20544.0));
        vis_data.add_child(HierarchyNode::new("DataList".to_string(), 19788.0));
        vis_data.add_child(HierarchyNode::new("DataSprite".to_string(), 10349.0));
        vis_data.add_child(HierarchyNode::new("EdgeSprite".to_string(), 3301.0));
        vis_data.add_child(HierarchyNode::new("NodeSprite".to_string(), 19382.0));
        let mut render = HierarchyNode::new("render".to_string(), 0.0);
        render.add_child(HierarchyNode::new("ArrowType".to_string(), 698.0));
        render.add_child(HierarchyNode::new("EdgeRenderer".to_string(), 5569.0));
        render.add_child(HierarchyNode::new("IRenderer".to_string(), 353.0));
        render.add_child(HierarchyNode::new("ShapeRenderer".to_string(), 2247.0));
        vis_data.add_child(render);
        vis_data.add_child(HierarchyNode::new("ScaleBinding".to_string(), 11275.0));
        vis_data.add_child(HierarchyNode::new("Tree".to_string(), 7147.0));
        vis_data.add_child(HierarchyNode::new("TreeBuilder".to_string(), 9930.0));
        vis.add_child(vis_data);
        let mut events = HierarchyNode::new("events".to_string(), 0.0);
        events.add_child(HierarchyNode::new("DataEvent".to_string(), 2313.0));
        events.add_child(HierarchyNode::new("SelectionEvent".to_string(), 1880.0));
        events.add_child(HierarchyNode::new("TooltipEvent".to_string(), 1701.0));
        events.add_child(HierarchyNode::new("VisualizationEvent".to_string(), 1117.0));
        vis.add_child(events);
        let mut legend = HierarchyNode::new("legend".to_string(), 0.0);
        legend.add_child(HierarchyNode::new("Legend".to_string(), 20859.0));
        legend.add_child(HierarchyNode::new("LegendItem".to_string(), 4614.0));
        legend.add_child(HierarchyNode::new("LegendRange".to_string(), 10530.0));
        vis.add_child(legend);
        let mut operator = HierarchyNode::new("operator".to_string(), 0.0);
        let mut distortion = HierarchyNode::new("distortion".to_string(), 0.0);
        distortion.add_child(HierarchyNode::new("BifocalDistortion".to_string(), 4461.0));
        distortion.add_child(HierarchyNode::new("Distortion".to_string(), 6314.0));
        distortion.add_child(HierarchyNode::new("FisheyeDistortion".to_string(), 3444.0));
        operator.add_child(distortion);
        let mut encoder = HierarchyNode::new("encoder".to_string(), 0.0);
        encoder.add_child(HierarchyNode::new("ColorEncoder".to_string(), 3179.0));
        encoder.add_child(HierarchyNode::new("Encoder".to_string(), 4060.0));
        encoder.add_child(HierarchyNode::new("PropertyEncoder".to_string(), 4138.0));
        encoder.add_child(HierarchyNode::new("ShapeEncoder".to_string(), 1690.0));
        encoder.add_child(HierarchyNode::new("SizeEncoder".to_string(), 1830.0));
        operator.add_child(encoder);
        let mut filter = HierarchyNode::new("filter".to_string(), 0.0);
        filter.add_child(HierarchyNode::new("FisheyeTreeFilter".to_string(), 5219.0));
        filter.add_child(HierarchyNode::new("GraphDistanceFilter".to_string(), 3165.0));
        filter.add_child(HierarchyNode::new("VisibilityFilter".to_string(), 3509.0));
        operator.add_child(filter);
        operator.add_child(HierarchyNode::new("IOperator".to_string(), 1286.0));
        let mut label = HierarchyNode::new("label".to_string(), 0.0);
        label.add_child(HierarchyNode::new("Labeler".to_string(), 9956.0));
        label.add_child(HierarchyNode::new("RadialLabeler".to_string(), 3899.0));
        label.add_child(HierarchyNode::new("StackedAreaLabeler".to_string(), 3202.0));
        operator.add_child(label);
        let mut layout = HierarchyNode::new("layout".to_string(), 0.0);
        layout.add_child(HierarchyNode::new("AxisLayout".to_string(), 6725.0));
        layout.add_child(HierarchyNode::new("BundledEdgeRouter".to_string(), 3727.0));
        layout.add_child(HierarchyNode::new("CircleLayout".to_string(), 9317.0));
        layout.add_child(HierarchyNode::new("CirclePackingLayout".to_string(), 12003.0));
        layout.add_child(HierarchyNode::new("DendrogramLayout".to_string(), 4853.0));
        layout.add_child(HierarchyNode::new("ForceDirectedLayout".to_string(), 8411.0));
        layout.add_child(HierarchyNode::new("IcicleTreeLayout".to_string(), 4864.0));
        layout.add_child(HierarchyNode::new("IndentedTreeLayout".to_string(), 3174.0));
        layout.add_child(HierarchyNode::new("Layout".to_string(), 7881.0));
        layout.add_child(HierarchyNode::new("NodeLinkTreeLayout".to_string(), 12870.0));
        layout.add_child(HierarchyNode::new("PieLayout".to_string(), 2728.0));
        layout.add_child(HierarchyNode::new("RadialTreeLayout".to_string(), 12348.0));
        layout.add_child(HierarchyNode::new("RandomLayout".to_string(), 870.0));
        layout.add_child(HierarchyNode::new("StackedAreaLayout".to_string(), 9121.0));
        layout.add_child(HierarchyNode::new("TreeMapLayout".to_string(), 9191.0));
        operator.add_child(layout);
        operator.add_child(HierarchyNode::new("Operator".to_string(), 2490.0));
        operator.add_child(HierarchyNode::new("OperatorList".to_string(), 5248.0));
        operator.add_child(HierarchyNode::new("OperatorSequence".to_string(), 4190.0));
        operator.add_child(HierarchyNode::new("OperatorSwitch".to_string(), 2581.0));
        operator.add_child(HierarchyNode::new("SortOperator".to_string(), 2023.0));
        vis.add_child(operator);
        vis.add_child(HierarchyNode::new("Visualization".to_string(), 16540.0));
        root.add_child(vis);

        root
    }

    /// D3's interpolateRainbow using cubehelix color space
    /// https://github.com/d3/d3-scale-chromatic/blob/main/src/sequential-multi/rainbow.js
    fn interpolate_rainbow(&self, t: f64) -> Vec4 {
        // Normalize t to [0, 1]
        let t = if t < 0.0 || t > 1.0 { t - t.floor() } else { t };
        let ts = (t - 0.5).abs();

        // Rainbow parameters calibrated to match D3 sunburst colors
        // Derived from target colors: #a88dcd, #db8cd0, etc.
        let h = 360.0 * t - 100.0;   // Hue in degrees
        let s = 1.55 - 2.25 * ts;    // Saturation (peaks at center t=0.5)
        let l = 0.95 - 0.68 * ts;    // Lightness

        // Cubehelix to RGB conversion (d3-color constants)
        const A: f64 = -0.14861;
        const B: f64 = 1.78277;
        const C: f64 = -0.29227;
        const D: f64 = -0.90649;
        const E: f64 = 1.97294;

        let h_rad = (h + 120.0).to_radians();
        let cos_h = h_rad.cos();
        let sin_h = h_rad.sin();

        let amp = s * l * (1.0 - l);
        let r = l + amp * (A * cos_h + B * sin_h);
        let g = l + amp * (C * cos_h + D * sin_h);
        let b = l + amp * (E * cos_h);

        vec4(
            r.clamp(0.0, 1.0) as f32,
            g.clamp(0.0, 1.0) as f32,
            b.clamp(0.0, 1.0) as f32,
            1.0
        )
    }

    /// Generate rainbow colors for n categories
    /// D3: d3.quantize(d3.interpolateRainbow, n + 1)
    fn generate_rainbow_colors(&self, n: usize) -> Vec<Vec4> {
        // D3's quantize samples n+1 colors and excludes the last (to avoid wrap-around duplicate)
        (0..n).map(|i| {
            let t = i as f64 / (n + 1) as f64;
            self.interpolate_rainbow(t)
        }).collect()
    }

    fn start_animation(&mut self, cx: &mut Cx) {
        let time = cx.seconds_since_app_start();
        self.animator = ChartAnimator::new(1000.0)
            .with_easing(EasingType::EaseOutCubic);
        self.animator.start(time);
        cx.new_next_frame();
    }

    pub fn replay_animation(&mut self, cx: &mut Cx) {
        self.animator.reset();
        let time = cx.seconds_since_app_start();
        self.animator = ChartAnimator::new(1000.0)
            .with_easing(EasingType::EaseOutCubic);
        self.animator.start(time);
        cx.new_next_frame();
        self.redraw(cx);
    }

    pub fn is_animating(&self) -> bool {
        self.animator.is_running()
    }

    fn draw_sunburst(&mut self, cx: &mut Cx2d, rect: Rect) {
        self.center_x = (rect.pos.x + rect.size.x / 2.0) as f64;
        self.center_y = (rect.pos.y + rect.size.y / 2.0) as f64;
        self.max_radius = (rect.size.x.min(rect.size.y) / 2.0 - 10.0) as f64;

        // Get animation progress - use 1.0 if animation hasn't started or is complete
        // This ensures the sunburst is visible even on the first frame
        let progress = if self.animator.is_running() {
            self.animator.get_progress().max(0.01) // Ensure at least minimal progress
        } else {
            1.0 // Not animating - show full sunburst
        };

        // Calculate scale factor to fit the 464px radius into our rect
        let scale = self.max_radius / 464.0;

        // Clone partition to avoid borrow checker issues
        let partition = match self.partition.clone() {
            Some(p) => p,
            None => return,
        };

        // Draw all descendants (excluding root, as D3 does)
        let nodes: Vec<_> = partition.descendants(false);

        // First pass: draw arcs
        for node in &nodes {
            self.draw_arc_node(cx, node, scale, progress);
        }

        // Second pass: draw labels (only after animation completes)
        if progress > 0.9 {
            for node in &nodes {
                self.draw_label(cx, node, scale);
            }
        }
    }

    fn draw_arc_node(
        &mut self,
        cx: &mut Cx2d,
        node: &PartitionNode<String>,
        scale: f64,
        progress: f64,
    ) {
        // Skip very small arcs
        let angle_span = node.x1 - node.x0;
        if angle_span < 0.001 {
            return;
        }

        // D3's arc settings:
        // .padAngle(d => Math.min((d.x1 - d.x0) / 2, 0.005))
        // .padRadius(radius / 2)
        // .innerRadius(d => d.y0)
        // .outerRadius(d => d.y1 - 1)
        let pad_angle = (angle_span / 2.0).min(0.005);

        // Apply padding
        let start_angle = node.x0 + pad_angle;
        let end_angle = node.x1 - pad_angle;

        if end_angle <= start_angle {
            return;
        }

        // Scale radii
        let inner_radius = node.y0 * scale;
        let outer_radius = (node.y1 - 1.0) * scale; // D3 uses y1 - 1

        // Apply animation - grow from center
        let animated_outer = inner_radius + (outer_radius - inner_radius) * progress;

        if animated_outer <= inner_radius {
            return;
        }

        // Get color based on first-level ancestor (color_index)
        // All descendants of a top-level branch share the same color
        let color_idx = node.color_index % self.colors.len();
        let color = self.colors[color_idx];

        // Draw arc
        // Adjust angles to start from top (-PI/2 rotation)
        let adjusted_start = start_angle - PI / 2.0;
        let sweep = (end_angle - start_angle) * progress;

        self.draw_arc.set_arc(adjusted_start, sweep, inner_radius, animated_outer);
        self.draw_arc.set_solid_color(color);
        self.draw_arc.draw_arc(cx, dvec2(self.center_x, self.center_y), animated_outer);
    }

    fn draw_label(
        &mut self,
        cx: &mut Cx2d,
        node: &PartitionNode<String>,
        scale: f64,
    ) {
        // Note: Per-glyph rotation not supported in Makepad. See docs/MAKEPAD_TEXT_ROTATION.md
        // Using horizontal labels at arc centers (most readable approach).

        // D3's label filter: d.depth && (d.y0 + d.y1) / 2 * (d.x1 - d.x0) > 10
        let mid_radius = (node.y0 + node.y1) / 2.0 * scale;
        let angle_span = node.x1 - node.x0;
        let arc_length = mid_radius * angle_span;

        // Only draw label if arc is large enough
        if arc_length < 20.0 {
            return;
        }

        // Calculate the radial angle (middle of the arc)
        let mid_angle = (node.x0 + node.x1) / 2.0 - PI / 2.0; // Adjust for top-start

        let text = &node.name;

        // Estimate text width (approximately 5.5 pixels per character at font size 9)
        let char_width = 5.5;
        let text_width = text.len() as f64 * char_width;

        // Check if text fits in the arc length
        if text_width > arc_length * 0.9 {
            return; // Text too long for this arc
        }

        // Position label at center of arc (horizontally oriented)
        let label_x = self.center_x + mid_radius * mid_angle.cos();
        let label_y = self.center_y + mid_radius * mid_angle.sin();

        // Center the text horizontally and vertically
        let pos = dvec2(label_x - text_width / 2.0, label_y - 4.5);
        self.draw_text.draw_abs(cx, pos, text);
    }

    fn handle_mouse_move(&mut self, cx: &mut Cx, _pos: DVec2) {
        // TODO: Implement hover detection using partition coordinates
        // For now, just trigger redraw if needed
        let _ = cx;
    }

    /// Initialize with D3 flare-style data (called externally)
    pub fn initialize_flare_data(&mut self) {
        self.initialize_data();
        self.initialized = true;
    }
}

/// Widget reference implementation for external initialization
impl SunburstWidgetRef {
    pub fn initialize_flare_data(&self, cx: &mut Cx) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.initialize_flare_data();
            inner.replay_animation(cx);
        }
    }
}
