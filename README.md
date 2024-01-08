# svg-chart

Crate for drawing graphics in `svd` format. It is intended to be simple to use, maintain and extent.

Main focus is on to provide a convenient way to draw common diagrams, like bar charts, stacked bar charts and pie charts.

There are plenty of alternatives for generating diagrams, to name a few with Rust support.

- [plotters](https://crates.io/crates/plotters) Native plotting library, with a lot of functionality.
- [plotlib](https://crates.io/crates/plotlib) Native plotting library.
- [gnuplot](https://crates.io/crates/gnuplot) Non-native depends on the `gnuplot` tool. Renders `gnuplot` data files, from which you can generate diagrams, and manipulate with scripts.
- [plotly](https://crates.io/crates/plotly) Non-native, depends on the `plotly` python ecosystem.

Other alternatives include [Graphviz](https://graphviz.org/), which focus on highly scalable automatic graph layout (there exists various Rust based tools for interacting with `graphviz`, e.g., [graphviz-rust](https://crates.io/crates/graphviz-rust), and even [graphviz-sys](https://crates.io/crates/graphviz-sys) library bindings).

For Markdown integration Mermaid provides a set of simple to use diagrams. Good support for native rendering in Markdown, see e.g., [mermaid aquamarine](https://crates.io/crates/aquamarine), however Mermaid diagrams are rather limited in flexibility.

---

## Use

- `.svg` files are stored in the `xml` folder. It can be viewed directly in firefox or converted to other formats (e.g. `pdf`).
  
---

## Design

The `svg` format captures models in a fairly simple `xml` format. While there exists numerous `xml` support crates for handling `dom` structures we provide a light weight `xlm.rs` library with a builder pattern, for creating and manipulating tags, attributes and styles.

Drawing primitives are found in the `draw.rs` file, which adds `svg-chart` specific functionality.

Based on that, `bar_chart.rs`, `stacked_bar_chart.rs`, and `pie_chart.rs` can be used to generate diagrams accordingly (unit tests covers examples of use).

The `pie` primitive uses clockwise direction starting from top. A mathematical `arc` is also available using polar coordinates, and counter clockwise draw direction in `svg` terminology.
