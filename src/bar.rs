// bar chart

use crate::{
    common::{Chart, Data, Values},
    xml::*,
};
use std::fmt::Display;

type Val<T1, T2> = (u32, T1, T2);
#[derive(Default)]
pub struct Bar<T1, T2>
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

impl<T1, T2> Bar<T1, T2>
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
    ) -> Bar<T1, T2> {
        Bar {
            x,
            y,
            width,
            height,
            bar_values,
        }
    }

    pub fn build(self) -> Tag {
        let x = self.x;
        let y = self.y;
        let width = self.width;
        let height = self.height;

        let scale_x_bar = width as f32 / (self.bar_values.len()) as f32;
        let max_value = self.bar_values.iter().map(|(v, _, _)| *v).max().unwrap();
        let scale_y_bar = height as f32 / max_value as f32;

        println!("scale_x_bar {}", scale_x_bar);
        println!("scale_y_bar {}", scale_y_bar);

        let mut tag = Tag::new("g"); // a group

        // tag.inner_ref(
        //     Tag::rect(self.x, self.y, self.width, self.height)
        //         .attr("fill", "transparent")
        //         .attr("stroke", "white")
        //         .attr("stroke-dasharray", 4),
        // );
        for (i, (v, c, t)) in self.bar_values.iter().enumerate() {
            println!("{} {} {} {}", i, v, c, t);
            let v_scale = (*v as f32 * scale_y_bar) as u32;
            tag.inner_ref(
                Tag::rect(
                    x + ((i as f32) * scale_x_bar) as u32,
                    y + self.height - v_scale,
                    scale_x_bar as u32,
                    v_scale,
                )
                .attr("fill", c)
                .inner(Tag::hover(&format!("{}: {}", t, v))),
            );
        }
        tag
    }
}

impl<T1, T2> Chart for Bar<T1, T2>
where
    T1: Display,
    T2: Display + Clone,
{
    fn get_pos(&self) -> (u32, u32) {
        (self.x, self.y)
    }

    fn set_pos(&mut self, x: u32, y: u32) {
        self.x = x;
        self.y = y;
    }

    fn get_size(&self) -> (u32, u32) {
        (self.height, self.width)
    }

    fn set_size(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
    }

    fn build_trait(self) -> Tag {
        self.build()
    }
}

impl<T1, T2> Data for Bar<T1, T2>
where
    T1: Display,
    T2: Display,
{
    ///
    fn get_labels(&self) -> Vec<impl Display> {
        self.bar_values.iter().map(|(_v, _c, t)| t).collect()
    }
    ///
    fn get_colors(&self) -> Vec<impl Display> {
        self.bar_values.iter().map(|(_v, c, _t)| c).collect()
    }
}

impl<T1, T2> Values for Bar<T1, T2>
where
    T1: Display,
    T2: Display,
{
    fn get_values(&self) -> Vec<u32> {
        self.bar_values.iter().map(|(v, _c, _t)| *v).collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::draw::test::test;

    #[test]
    fn bar_chart() {
        let bar_chart = Bar::new(
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
