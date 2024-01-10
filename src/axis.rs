// legend
use crate::xml::*;
use std::fmt::Display;

type Val<T> = (u32, T);

#[derive(Default)]
pub struct Axis<T>
where
    T: Display,
{
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    x_margin: u32,
    y_margin: u32,
    labels: Vec<Val<T>>,
}

impl<T> Axis<T>
where
    T: Display + Copy,
{
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        x: u32,
        y: u32,
        width: u32,
        height: u32,
        x_margin: u32,
        y_margin: u32,
        labels: Vec<Val<T>>,
    ) -> Self {
        Axis {
            x,
            y,
            width,
            height,
            x_margin,
            y_margin,
            labels,
        }
    }

    pub fn build(&self) -> Tag {
        let x = self.x;
        let y = self.y;
        let height = self.height;
        let width = self.width;
        let x_margin = self.x_margin;
        let y_margin = self.y_margin;
        let labels = &self.labels;

        let y_max = labels.iter().map(|(v, _)| *v).max().unwrap();

        let x_scale = (width - x_margin) as f32 / (labels.len() as f32);

        let mut tag = Tag::new("g"); // a group

        // for debugging
        // tag.inner_ref(
        //     Tag::rect(x, y, width, height)
        //         .attr("fill", "transparent")
        //         .attr("stroke", "white"),
        // );

        // vertical
        tag.inner_ref(
            Tag::line(x + x_margin, y, x + x_margin, y + height - y_margin).attr("stroke", "white"),
        );
        // horizontal
        tag.inner_ref(
            Tag::line(
                x + x_margin,
                y + height - y_margin,
                x + width,
                y + height - y_margin,
            )
            .attr("stroke", "white"),
        );
        for (i, (_, t)) in labels.iter().enumerate() {
            tag.inner_ref(
                Tag::text(
                    t,
                    x + x_margin + ((i as f32 + 0.5) * x_scale) as u32,
                    y + height,
                )
                .attr("fill", "white")
                .attr("text-anchor", "middle"),
            );
        }

        // scaling
        tag.inner_ref(
            Tag::text(0, x + x_margin - 5, y + height - y_margin)
                .attr("fill", "white")
                .attr("text-anchor", "end"),
        );
        tag.inner_ref(
            Tag::text(y_max, x + x_margin - 5, y)
                .attr("fill", "white")
                .attr("text-anchor", "end"),
        );
        tag
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{bar_chart::*, draw::test::test, legend::*};

    #[test]
    fn axis() {
        let axis = Axis::new(
            100,
            50,
            200,
            100,
            50,
            20,
            vec![(10, "T1"), (20, "Task2"), (30, "Task3")],
        );
        let svg = axis.build();
        test(vec![svg], "xml/axis.svg")
    }

    #[test]
    fn axis_bar_chart_legend() {
        let v = [20, 40, 30, 5];
        let c = ["green", "red", "blue", "yellow"];
        let t = ["Hi", "Mid", "Low", "Idle"];

        let y_max = v.into_iter().max().unwrap();

        let vct: Vec<(u32, _, _)> = v
            .iter()
            .zip(c.iter())
            .zip(t.iter())
            .map(|((v, c), t)| (*v, c, t))
            .collect();
        let ct: Vec<_> = c.iter().zip(t.iter()).map(|(c, t)| (c, t)).collect();
        let vt: Vec<_> = v.iter().zip(t.iter()).map(|(v, t)| (*v, t)).collect();

        test(
            vec![
                BarChart::new(100, 100, 200, 180, vct).build(),
                Axis::new(50, 100, 250, 200, 50, 20, vt).build(),
                Legend::new(350, 100, 100, 180, ct, 30).build(),
            ],
            "xml/bar_chart_axis.svg",
        )
    }
}
