// legend
use crate::xml::*;
use std::fmt::Display;

type Val<T1, T2> = (T1, T2);

#[derive(Default)]
pub struct Legend<T1, T2>
where
    T1: Display,
    T2: Display,
{
    x: u32,
    y: u32,

    legend: Vec<Val<T1, T2>>,
    legend_height: u32,
}

impl<T1, T2> Legend<T1, T2>
where
    T1: Display,
    T2: Display,
{
    pub fn new(x: u32, y: u32, legend: Vec<Val<T1, T2>>, legend_height: u32) -> Self {
        Legend {
            x,
            y,

            legend,
            legend_height,
        }
    }

    pub fn build(&self) -> Tag {
        let mut tag = Tag::new("g"); // a group

        // tag.inner_ref(
        //     Tag::rect(self.x, self.y, self.width, self.height)
        //         .attr("fill", "transparent")
        //         .attr("stroke", "white"),
        // );
        for (i, (c, t)) in self.legend.iter().enumerate() {
            println!("{} {} {}", i, c, t);
            tag.inner_ref(
                Tag::rect(
                    self.x,
                    self.y + self.legend_height * i as u32,
                    self.legend_height,
                    self.legend_height,
                )
                .attr("fill", c),
            );
            tag.inner_ref(
                Tag::text(
                    t,
                    self.x + (self.legend_height as f32 * 1.5) as u32,
                    self.y + (self.legend_height as f32 * (i as f32 + 0.75)) as u32,
                )
                .attr("stroke", "white"),
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
    fn legend() {
        let legend = Legend::new(100, 50, vec![("yellow", "Task1"), ("green", "Task2")], 20);
        let svg = legend.build();
        test(vec![svg], "xml/legend.svg")
    }
}
