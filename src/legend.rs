// legend
use crate::{
    common::{Chart, Data},
    xml::*,
};
use std::fmt::Display;

#[derive(Default)]
pub struct Legend<C>
where
    C: Chart + Data,
{
    chart: C,
    legend_width: u32,
    legend_box_height: u32,
}

impl<C> Legend<C>
where
    C: Chart + Data,
{
    pub fn new(chart: C, width: u32, legend_height: u32) -> Self {
        Legend {
            chart,
            legend_width: width,
            legend_box_height: legend_height,
        }
    }

    pub fn build(self) -> Tag {
        let mut tag = Tag::new("g"); // a group
        let (x, y) = self.chart.get_pos();
        let (width, _height) = self.chart.get_size();

        tag.inner_ref(
            Tag::rect(x, y, self.legend_width + width, _height)
                .attr("fill", "transparent")
                .attr("stroke", "white"),
        );

        let labels = self.chart.get_labels();
        let colors = self.chart.get_colors();

        for (i, (t, c)) in labels.iter().zip(colors.iter()).enumerate() {
            println!("{} {} {}", i, c, t);
            tag.inner_ref(
                Tag::rect(
                    x + width + self.legend_box_height,
                    y + self.legend_box_height * i as u32,
                    self.legend_box_height,
                    self.legend_box_height,
                )
                .attr("fill", c),
            );
            tag.inner_ref(
                Tag::text(
                    t,
                    x + width + (self.legend_box_height as f32 * 2.5) as u32,
                    y + (self.legend_box_height as f32 * (i as f32 + 0.75)) as u32,
                )
                .attr("stroke", "white"),
            );
        }

        drop(labels); // <-- BUG?
        drop(colors);

        tag.inner(self.chart.build_trait())
    }
}

impl<C> Chart for Legend<C>
where
    C: Chart + Data,
{
    fn get_pos(&self) -> (u32, u32) {
        self.chart.get_pos()
    }

    fn set_pos(&mut self, x: u32, y: u32) {
        self.chart.set_pos(x, y);
    }

    fn get_size(&self) -> (u32, u32) {
        let (chart_width, chart_heigh) = self.chart.get_size();
        (chart_width + self.legend_width, chart_heigh)
    }

    fn set_size(&mut self, width: u32, height: u32) {
        self.chart.set_size(width - self.legend_width, height);
    }

    fn build_trait(self) -> Tag {
        self.build()
    }
}

impl<C> Data for Legend<C>
where
    C: Chart + Data,
{
    fn get_labels(&self) -> Vec<impl Display> {
        self.chart.get_labels()
    }

    fn get_colors(&self) -> Vec<impl Display> {
        self.chart.get_labels()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{bar::Bar, draw::test::test, pie::Pie};

    #[test]
    fn legend_pie() {
        let legend = Legend::new(
            Pie::new(
                100,
                100,
                100,
                vec![
                    (0.1, "green", "Task1"),
                    (0.15, "blue", "Task2"),
                    (0.55, "yellow", "Task3"),
                    (0.05, "red", "Task4"),
                ],
            ),
            150,
            20,
        );
        let svg = legend.build();
        test(vec![svg], "xml/legend_pie.svg")
    }

    #[test]
    fn legend_bar() {
        let legend = Legend::new(
            Bar::new(
                200,
                100,
                200,
                200,
                vec![(10, "yellow", "T1"), (20, "green", "T2"), (30, "red", "T3")],
            ),
            150,
            20,
        );
        let svg = legend.build();
        test(vec![svg], "xml/legend_bar.svg")
    }
}
