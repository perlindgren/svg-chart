// bar chart
use crate::xml::*;
use std::fmt::Display;

type Val<T1, T2> = (u32, T1, T2);
#[derive(Default)]
pub struct BarChart<T1, T2>
where
    T1: Display,
    T2: Display,
{
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    bar_values: Vec<Val<T1, T2>>,
}

impl<T1, T2> BarChart<T1, T2>
where
    T1: Display,
    T2: Display,
{
    pub fn new(
        x: u32,
        y: u32,
        width: u32,
        height: u32,
        bar_values: Vec<Val<T1, T2>>,
    ) -> BarChart<T1, T2> {
        BarChart {
            x,
            y,
            width,
            height,
            bar_values,
        }
    }

    pub fn build(&self) -> Tag {
        let scale_x_bar = self.width as f32 / (self.bar_values.len()) as f32;
        let max_value = self.bar_values.iter().map(|(v, _, _)| *v).max().unwrap();
        let scale_y_bar = self.height as f32 / max_value as f32;

        println!("scale_x_bar {}", scale_x_bar);
        println!("scale_y_bar {}", scale_y_bar);

        let mut tag = Tag::new("g"); // a group
        tag.inner_ref(
            Tag::rect(self.x, self.y, self.width, self.height)
                .attr("fill", "transparent")
                .attr("stroke", "white"),
        );
        for (i, (v, c, t)) in self.bar_values.iter().enumerate() {
            println!("{} {} {} {}", i, v, c, t);
            tag.inner_ref(
                Tag::rect(
                    self.x + ((i as f32) * scale_x_bar) as u32,
                    self.y + self.height - *v,
                    scale_x_bar as u32,
                    *v,
                )
                .attr("fill", c)
                .inner(Tag::hover(&t.to_string())),
            );
        }
        tag
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::draw::test::test;

    #[test]
    fn bar() {
        test(
            vec![Tag::rect(20, 20, 50, 40).attr("fill", "green")],
            "xml/bar.svg",
        );
    }

    #[test]
    fn bar_chart() {
        let bar_chart = BarChart::new(
            100,
            50,
            200,
            100,
            vec![(20, "yellow", "Task1"), (40, "green", "Task2")],
        );
        let group = bar_chart.build();
        test(vec![group], "xml/bars.svg")
    }
}
