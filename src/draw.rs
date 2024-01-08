// primitive draw operations

use crate::xml::*;
use std::f32::consts::PI;
use std::fmt::Display;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;

// mod mono {
//     use std::sync::atomic::{AtomicU32, Ordering};
//     // start from 2, ids 0 and 1 reserved
//     static mut MONO: AtomicU32 = AtomicU32::new(2);

//     pub(crate) fn get_new() -> u32 {
//         let mono = unsafe { MONO.load(Ordering::SeqCst) };
//         unsafe { MONO.store(mono + 1, Ordering::SeqCst) }
//         mono
//     }
// }

impl Tag {
    pub fn arc(x: u32, y: u32, radius: u32, start_angle: f32, end_angle: f32) -> Self {
        let x = x as f32;
        let y = y as f32;
        let radius = radius as f32;
        let start_sin = (start_angle * 2.0 * PI).sin();
        let start_cos = (start_angle * 2.0 * PI).cos();

        let end_sin = (end_angle * 2.0 * PI).sin();
        let end_cos = (end_angle * 2.0 * PI).cos();

        println!("start sin {}", start_sin);
        println!("start cos {}", start_cos);

        println!("end sin {}", end_sin);
        println!("end cos {}", end_cos);

        let start_x = x + (start_cos * radius);
        let start_y = y - (start_sin * radius);

        let end_x = x + (end_cos * radius);
        let end_y = y - (end_sin * radius);

        println!("start_x {}", start_x);
        println!("start_y {}", start_y);

        println!("end_x {}", end_x);
        println!("end_y {}", end_y);

        Tag::new("path").attr(
            "d",
            format!(
                "M {} {} A {} {}, {}, {}, {}, {} {} L {} {} Z",
                start_x,
                start_y,
                radius,
                radius,
                0,
                if end_angle - start_angle > 0.5 { 1 } else { 0 },
                0,
                end_x,
                end_y,
                x,
                y
            ),
        )
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
        // <text x="20" y="35" class="small">My</text>
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

    // #[test]
    // fn test_mono() {
    //     let m1 = mono::get_new();
    //     let m2 = mono::get_new();
    //     assert!(m2 > m1);
    // }

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
            vec![Tag::arc(100, 100, 100, 0.25, 0.950)
                .attr("stroke", "white")
                .attr("fill", "green")],
            "xml/my_arc.svg",
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
    // fn test_pie() {
    //     let x = 100;
    //     let y = 100;
    //     let radius = 100;
    //     let io = Tag::draw(vec![
    //         Tag::pie(x, y, radius, radius, 0.0, 0.25),
    //         Tag::pie(x, y, radius, radius, 0.5, 0.75),
    //     ]);
    //     println!("{}", io);
    //     io.save(&PathBuf::from_str("xml/pie_chart.drawio").unwrap())
    //         .unwrap();
    // }

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

    // #[test]
    // fn test_srp() {
    //     use srp::common::*;
    //     let tasks = srp::task_sets::task_set1();
    //     tasks.store(&PathBuf::from("task_sets/task_set1.json")).ok();
    //     let tasks_loaded = Tasks::load(&PathBuf::from("task_sets/task_set1.json")).unwrap();
    //     assert_eq!(tasks, tasks_loaded);
    // }
}
