// // charts
// use crate::{axis::*, bar::*, legend::*, pie_chart::*, xml::*};
// use std::fmt::Display;

// type Val<T1, T2> = (u32, T1, T2);

// #[derive(Debug, Clone)]
// pub struct Bars<T1, T2>
// where
//     T1: Display + Clone,
//     T2: Display + Clone,
// {
//     x: u32,
//     y: u32,
//     width_bars: u32,

//     height_bars: u32,
//     bars: Vec<Val<T1, T2>>,
// }

// impl<T1, T2> Bars<T1, T2>
// where
//     T1: Display + Clone,
//     T2: Display + Clone,
// {
//     pub fn new(bars: Vec<Val<T1, T2>>) -> Self {
//         Bars {
//             x: 0,
//             y: 0,
//             width_bars: 300,
//             height_bars: 200,

//             bars,
//         }
//     }

//     pub fn build(&self) -> Tag {
//         let x = self.x;
//         let y = self.y;
//         let width_bars = self.width_bars;
//         let height_bars = self.height_bars;

//         let label_margin = 20;
//         let bars = self.bars.clone();

//         let axis_margin = 50;

//         let vt: Vec<(u32, _)> = bars.iter().map(|(v, _c, t)| (*v, t)).collect();
//         let ct: Vec<_> = bars.iter().map(|(_v, c, t)| (c, t)).collect();

//         let mut tag = Tag::new("g");
//         // bar_chart
//         tag.inner_ref(
//             Bar::new(
//                 x + axis_margin,
//                 y + label_margin,
//                 width_bars,
//                 height_bars,
//                 bars.clone(),
//             )
//             .build(),
//         );
//         // axis
//         tag.inner_ref(
//             Axis::new(
//                 x,
//                 y + label_margin,
//                 axis_margin + width_bars,
//                 height_bars + label_margin,
//                 axis_margin,
//                 20,
//                 vt,
//             )
//             .build(),
//         );
//         // legend
//         tag.inner_ref(
//             Legend::new(
//                 x + axis_margin + width_bars + label_margin,
//                 y + label_margin,
//                 ct,
//                 20,
//             )
//             .build(),
//         );

//         tag
//     }
// }

// // struct PieLegend(PieChart);
// #[cfg(test)]
// mod test {
//     use super::*;
//     use crate::draw::test::test;

//     #[test]
//     fn bars() {
//         let bars = Bars::new(vec![
//             (10, "yellow", "Task1"),
//             (20, "green", "Task2"),
//             (30, "red", "Task3"),
//         ]);
//         let svg = bars.build();
//         test(vec![svg], "xml/bars.svg")
//     }

//     fn pies() {
//         test(
//             vec![PieChart::new(
//                 100,
//                 100,
//                 100,
//                 vec![
//                     (0.1, "green", "Task1"),
//                     (0.15, "blue", "Task2"),
//                     (0.55, "yellow", "Task3"),
//                     (0.05, "red", "Task4"),
//                 ],
//             )
//             .build()],
//             "xml/pie_chart.svg",
//         )
//     }
// }
