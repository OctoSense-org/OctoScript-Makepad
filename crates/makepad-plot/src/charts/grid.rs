// SubplotGrid / SubplotRow — containers for laying out multiple plots.
//
// Pure-DSL prototypes over the stock View widget. SubplotGrid stacks rows
// vertically; put plots inside SubplotRow children:
//   SubplotGrid{
//       SubplotRow{ LinePlot{} BarPlot{} }
//       SubplotRow{ PieChart{} HeatmapChart{} }
//   }
//
// NOTE: a wrapping flow (Flow::Right{wrap:true}) is deliberately NOT used here:
// makepad's deferred row-wrap alignment shifts instance rects but not DrawVector
// vertex geometry, so the first plot of every wrapped row would lose its vector
// layer. Explicit rows avoid that engine limitation.

use makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*

    mod.plot.SubplotGrid = View{
        width: Fill
        height: Fill
        flow: Down
        spacing: 10
    }

    mod.plot.SubplotRow = View{
        width: Fill
        height: Fill
        flow: Right
        spacing: 10
    }
}
