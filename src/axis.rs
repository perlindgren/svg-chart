// legend
use crate::{
    common::{Chart, Data, Values},
    xml::*,
};

pub struct Axis<C>
where
    C: Chart + Data + Values,
{
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    x_margin: u32,
    y_margin: u32,
    chart: C,
}

impl<C> Axis<C>
where
    C: Chart + Data + Values,
{
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        x: u32,
        y: u32,
        width: u32,
        height: u32,
        x_margin: u32,
        y_margin: u32,
        chart: C,
    ) -> Self {
        Axis {
            x,
            y,
            width,
            height,
            x_margin,
            y_margin,
            chart,
        }
    }

    pub fn build(mut self) -> Tag {
        let x = self.x;
        let y = self.y;
        let height = self.height;
        let width = self.width;
        let x_margin = self.x_margin;
        let y_margin = self.y_margin;

        let labels = self.chart.get_labels();
        let values = self.chart.get_values();

        let y_max = values.iter().max().unwrap();

        let x_scale = (width - x_margin) as f32 / (labels.len() as f32);

        let mut tag = Tag::new("g"); // a group

        // for debugging
        tag.inner_ref(
            Tag::rect(x, y, width, height)
                .attr("fill", "transparent")
                .attr("stroke", "white"),
        );

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
        drop(labels); // <- seems like a BUG

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

        // setup inner
        self.chart.set_pos(x + x_margin, y);
        self.chart.set_size(width - x_margin, height - y_margin);
        tag.inner(self.chart.build_trait())
    }
}

impl<C> Chart for Axis<C>
where
    C: Chart + Data + Values,
{
    fn get_pos(&self) -> (u32, u32) {
        (self.x, self.y)
    }

    fn set_pos(&mut self, x: u32, y: u32) {
        self.x = x;
        self.y = y;
    }

    fn get_size(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    fn set_size(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
    }

    fn build_trait(self) -> Tag {
        self.build()
    }
}
#[cfg(test)]
mod test {
    use super::*;
    use crate::{bar::Bar, draw::test::test};

    #[test]
    fn axis() {
        let axis = Axis::new(
            100,
            50,
            400,
            300,
            50,
            20,
            Bar::new(
                0,
                0,
                0,
                0,
                vec![(10, "yellow", "T1"), (20, "green", "T2"), (30, "red", "T3")],
            ),
        );
        let svg = axis.build();
        test(vec![svg], "xml/axis.svg")
    }
}
