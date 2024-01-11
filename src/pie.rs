// primitive draw operations

use crate::{
    common::{Chart, Data},
    xml::*,
};

use std::fmt::Display;

type Val<T1, T2> = Vec<(f32, T1, T2)>;
pub struct Pie<T1, T2>
where
    T1: Display,
    T2: Display,
{
    x: u32,
    y: u32,
    radius: u32,
    data: Val<T1, T2>,
}

impl<T1, T2> Pie<T1, T2>
where
    T1: Display,
    T2: Display,
{
    pub fn new(x: u32, y: u32, radius: u32, data: Val<T1, T2>) -> Self {
        Pie { x, y, radius, data }
    }

    pub fn build(&self) -> Tag {
        let mut tag = Tag::new("g");
        let mut angle: f32 = 0.0;
        // pies
        for (ratio, c, t) in self.data.iter() {
            tag.inner_ref(
                Tag::pie(
                    self.x + self.radius,
                    self.y + self.radius,
                    self.radius,
                    angle,
                    angle + *ratio,
                )
                .attr("fill", c)
                .attr("stroke", "white")
                .inner(Tag::hover(&t.to_string())),
            );
            angle += ratio;
        }

        tag
    }
}

impl<T1, T2> Chart for Pie<T1, T2>
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
        (self.radius * 2, self.radius * 2)
    }

    fn set_size(&mut self, width: u32, _height: u32) {
        assert!(width == _height, "setting radius requires width == height");
        self.radius = width;
    }

    fn build_trait(self) -> Tag {
        self.build()
    }
}

impl<T1, T2> Data for Pie<T1, T2>
where
    T1: Display,
    T2: Display,
{
    ///
    fn get_labels(&self) -> Vec<impl Display> {
        self.data.iter().map(|(_v, _c, t)| t).collect()
    }
    ///
    fn get_colors(&self) -> Vec<impl Display> {
        self.data.iter().map(|(_v, c, _t)| c).collect()
    }
}

#[cfg(test)]
pub mod test {
    use super::*;
    use crate::draw::test::test;

    #[test]
    fn pie() {
        test(
            vec![Pie::new(
                100,
                100,
                100,
                vec![
                    (0.1, "green", "Task1"),
                    (0.15, "blue", "Task2"),
                    (0.55, "yellow", "Task3"),
                    (0.05, "red", "Task4"),
                ],
            )
            .build()],
            "xml/pie_chart.svg",
        )
    }
}
