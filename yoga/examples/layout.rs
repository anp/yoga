#[macro_use]
extern crate log;

extern crate chrono;
extern crate fern;
extern crate ygg;

use ygg::prelude::*;

fn main() {
    // init logging
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(log::LevelFilter::Debug)
        .chain(std::io::stdout())
        .apply()
        .unwrap();

    let mut tree = Ygg::new(
        MeasuredDimensions {
            width: r32(512.0),
            height: r32(512.0),
        },
        Direction::LTR,
    );

    let node = tree.new_node();
    let child = tree.new_node();
    let other_child = tree.new_node();

    tree.push_child(node, child);
    tree.push_child(node, other_child);

    tree.apply_style(
        node,
        Margin {
            all: Some(10.points()),
            ..Default::default()
        },
    );

    tree.apply_style(node, MarginLeft(Auto));
    tree.apply_style(node, PaddingHorizontal(4i32.points()));
    tree.apply_style(node, Left(16.percent()));

    // FIXME(anp): i guess maybe i need to resurrect the undefined arm of value?
    // tree.apply_style(node, Bottom(None));

    // let child_styles = make_styles!(
    // 	Width(32 pt),
    // 	Height(32 pt),
    // 	Margin(Auto),
    // 	FlexGrow(1.0)
    // );

    // child.apply_styles(&child_styles);
    // other_child.apply_styles(&child_styles);

    println!("Layout is {:#?}", tree.get_layout());
}
