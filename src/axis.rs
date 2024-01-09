// legend
use crate::xml::*;
use std::fmt::Display;

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
    y_min: u32,
    y_max: u32,
    labels: Vec<T>,
}

impl<T> Axis<T>
where
    T: Display + Copy,
{
    pub fn new(
        x: u32,
        y: u32,
        width: u32,
        height: u32,
        x_margin: u32,
        y_margin: u32,
        y_min: u32,
        y_max: u32,
        labels: Vec<T>,
    ) -> Self {
        Axis {
            x,
            y,
            width,
            height,
            x_margin,
            y_margin,
            y_min,
            y_max,
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
        let y_min = self.y_min;
        let y_max: u32 = self.y_max;

        let labels = &self.labels;

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
        for (i, t) in labels.iter().enumerate() {
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
        tag.inner_ref(Tag::text(y_min, x, y + height - y_margin).attr("fill", "white"));
        tag.inner_ref(Tag::text(y_max, x, y).attr("fill", "white"));
        tag
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::bar_chart::*;
    use crate::draw::test::test;

    #[test]
    fn axis() {
        let axis = Axis::new(
            100,
            50,
            200,
            100,
            50,
            20,
            0,
            100,
            vec!["T1", "Task2", "Task3"],
        );
        let svg = axis.build();
        test(vec![svg], "xml/axis.svg")
    }

    #[test]
    fn axis_bar_chart() {
        let v = [20, 40, 30];
        let c = ["green", "red", "blue"];
        let t = ["Hi", "Mid", "Low"];

        let y_max = v.into_iter().max().unwrap();

        let vct: Vec<(u32, _, _)> = v
            .iter()
            .zip(c.iter())
            .zip(t.iter())
            .map(|((v, c), t)| (*v, c, t))
            .collect();

        test(
            vec![
                BarChart::new(100, 100, 200, 80, vct).build(),
                Axis::new(50, 100, 250, 100, 50, 20, 0, y_max, t.into()).build(),
            ],
            "xml/bar_chart_axis.svg",
        )
    }
}
