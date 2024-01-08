// primitive draw operations

use crate::xml::*;
use std::f32::consts::PI;
use std::fmt::Display;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;

impl Tag {
    /// Mathematical arc, with polar coordinates
    /// Angles between 0.0 .. 1.0
    /// SVG uses 0,0 as top left, thus y axis is inverted respective angle
    /// angle 0.0  -> x + radius, y
    /// angle 0.25 -> x, y - radius
    /// angle 0.5  -> x - radius, y
    /// angle 0.75 -> x, y + radius
    ///
    /// Should be called with swipe_dir false (counter clockwise)
    pub fn arc(
        x: u32,
        y: u32,
        radius: u32,
        start_angle: f32,
        end_angle: f32,
        swipe_dir: bool,
    ) -> Self {
        let x = x as f32;
        let y = y as f32;
        let radius = radius as f32;
        let start_sin = (start_angle * 2.0 * PI).sin();
        let start_cos = (start_angle * 2.0 * PI).cos();

        let end_sin = (end_angle * 2.0 * PI).sin();
        let end_cos = (end_angle * 2.0 * PI).cos();

        let start_x = x + (start_cos * radius);
        let start_y = y - (start_sin * radius);

        let end_x = x + (end_cos * radius);
        let end_y = y - (end_sin * radius);

        Tag::new("path").attr(
            "d",
            format!(
                "M {} {} A {} {}, {}, {}, {}, {} {} L {} {} Z",
                start_x,
                start_y,
                radius,
                radius,
                0,
                if (end_angle - start_angle).abs() > 0.5 {
                    1
                } else {
                    0
                },
                swipe_dir as u32,
                end_x,
                end_y,
                x,
                y
            ),
        )
    }

    /// Pie chart, clockwise angles
    /// Angles between 0.0 .. 1.0
    /// SVG uses 0,0 as top left, thus y axis is inverted respective angle
    /// angle 0.0  -> x, y - radius
    /// angle 0.25 -> x + radius, y
    /// angle 0.5  -> x, y + radius
    /// angle 0.75 -> x - radius, y
    pub fn pie(x: u32, y: u32, radius: u32, start_angle: f32, end_angle: f32) -> Self {
        Self::arc(x, y, radius, 0.25 - start_angle, 0.25 - end_angle, true)
    }

    pub fn rect(x: u32, y: u32, width: u32, height: u32) -> Self {
        Tag::new("rect")
            .attr("x", x)
            .attr("y", y)
            .attr("width", width)
            .attr("height", height)
    }

    pub fn text<T>(text: T, x: u32, y: u32) -> Self
    where
        T: Display,
    {
        Tag::new("text")
            .attr("x", x)
            .attr("y", y)
            .inner(Tag::raw(&format!("{}", text)))
    }

    pub fn line(x1: u32, y1: u32, x2: u32, y2: u32) -> Self {
        Tag::new("line")
            .attr("x1", x1)
            .attr("y1", y1)
            .attr("x2", x2)
            .attr("y2", y2)
    }

    pub fn document(inner: Vec<Tag>, width: u32, height: u32) -> Self {
        let mut root = Tag::new("svg")
            .attr("viewBox ", format!("0 0 {} {}", width, height))
            .attr("xmlns", "http://www.w3.org/2000/svg");

        for e in inner {
            root.inner_ref(e);
        }

        root
    }

    pub fn save<T>(&self, path: &T) -> io::Result<()>
    where
        T: AsRef<Path>,
    {
        let mut file = File::create(path)?;
        let io_str = format!("{}", self);
        file.write_all(io_str.as_bytes())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn test<T>(inner: Vec<Tag>, path: T)
    where
        T: AsRef<Path>,
    {
        let svg = Tag::document(inner, 320, 200);
        println!("{}", svg);
        svg.save(&path).unwrap();
    }

    #[test]
    fn test_rect() {
        test(vec![Tag::rect(20, 20, 50, 40)], "xml/rectangle.svg")
    }

    #[test]
    fn test_text() {
        test(
            vec![Tag::text("hello world", 20, 20).attr("fill", "white")],
            "xml/text.svg",
        );
    }

    #[test]
    fn test_line() {
        test(
            vec![Tag::line(20, 20, 40, 20).attr("stroke", "white")],
            "xml/line.svg",
        )
    }

    #[test]
    fn test_arc() {
        test(
            vec![Tag::arc(100, 100, 100, 0.25, 0.950, false)
                .attr("stroke", "white")
                .attr("fill", "green")],
            "xml/arc.svg",
        )
    }

    #[test]
    fn test_pie() {
        test(
            vec![Tag::pie(100, 100, 100, 0.20, 0.90)
                .attr("stroke", "white")
                .attr("fill", "green")],
            "xml/pie.svg",
        )
    }

    #[test]
    fn test_rectangles() {
        test(
            vec![
                Tag::rect(20, 20, 40, 40).attr("fill", "white"),
                Tag::rect(100, 20, 40, 40).attr("fill", "yellow"),
            ],
            "xml/rectangles.svg",
        )
    }

    #[test]
    fn test_bar_chart() {
        test(
            [(100, "green"), (200, "blue"), (50, "yellow"), (150, "red")]
                .iter()
                .enumerate()
                .map(|(x, (y, fill))| {
                    Tag::rect((x * 50) as u32, 200 - y, 40, *y).attr("fill", fill)
                })
                .collect(),
            "xml/bar_chart.svg",
        )
    }

    // #[test]
    // fn test_pie_color() {
    //     let x = 100;
    //     let y = 100;
    //     let radius = 100;
    //     let io = Tag::draw(vec![
    //         Tag::pie(x, y, radius, radius, 0.0, 0.25).style("fillColor", "#800000"),
    //         Tag::pie(x, y, radius, radius, 0.25, 0.45).style("fillColor", "#000080"),
    //         Tag::pie(x, y, radius, radius, 0.45, 0.87).style("fillColor", "#008000"),
    //     ]);
    //     println!("{}", io);
    //     io.save(&PathBuf::from_str("xml/pie_chart_color.drawio").unwrap())
    //         .unwrap();
    // }
}
