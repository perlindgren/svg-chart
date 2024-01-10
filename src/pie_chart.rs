// primitive draw operations

use crate::xml::*;

use std::fmt::Display;

type Val<T1, T2> = Vec<(f32, T1, T2)>;
pub struct PieChart<T1, T2>
where
    T1: Display,
    T2: Display,
{
    x: u32,
    y: u32,
    radius: u32,
    data: Val<T1, T2>,
}

impl<T1, T2> PieChart<T1, T2>
where
    T1: Display,
    T2: Display,
{
    pub fn new(x: u32, y: u32, radius: u32, data: Val<T1, T2>) -> Self {
        PieChart { x, y, radius, data }
    }

    pub fn build(&self) -> Tag {
        let mut tag = Tag::new("g");
        let mut angle: f32 = 0.0;
        // pies
        for (ratio, c, t) in self.data.iter() {
            tag.inner_ref(
                Tag::pie(self.x, self.y, self.radius, angle, angle + *ratio)
                    .attr("fill", c)
                    .attr("stroke", "white")
                    .inner(Tag::hover(&t.to_string())),
            );
            angle += ratio;
        }

        tag
    }
}

#[cfg(test)]
pub mod test {
    use super::*;
    use crate::draw::test::test;

    #[test]
    fn pie() {
        test(
            vec![PieChart::new(
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
