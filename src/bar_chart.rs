// // bar chart
// use crate::common::*;
// use svg::{
//     node::element::{path::Data, Path},
//     Document,
// };

// #[derive(Default)]
// pub struct BarChart {
//     bar_values: Vec<f32>,
// }

// impl BarChart {
//     pub fn new() -> BarChart {
//         BarChart::default()
//     }

//     pub fn bar_values(mut self, bar_values: Vec<f32>) -> BarChart {
//         self.bar_values = bar_values;
//         self
//     }
// }

// impl From<BarChart> for Document {
//     fn from(bar_chart: BarChart) -> Self {
//         let data1 = rect(30, 10, 10, 30);
//         let data2 = rect(10, 10, 10, 30);

//         let path1 = Path::new()
//             .set("fill", "none")
//             .set("stroke", "black")
//             .set("stroke-width", 3)
//             .set("d", data1);

//         let path2 = Path::new()
//             .set("fill", "#800000")
//             .set("stroke", "black")
//             .set("stroke-width", 1.5)
//             .set("d", data2);

//         Document::new()
//             .set("viewBox", (0, 0, 70, 70))
//             .add(path1)
//             .add(path2)
//     }
// }

// #[cfg(test)]
// mod test {
//     use super::*;
//     #[test]
//     fn bar_chart() {
//         let document: Document = BarChart::new().into();
//         svg::save("image.svg", &document).unwrap();
//     }
// }
