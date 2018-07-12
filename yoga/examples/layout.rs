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
        .level(log::LevelFilter::Trace)
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

    {
        let mut child_styles = |c| {
            tree.apply_style(c, Width(32.points()));
            tree.apply_style(c, Height(32.points()));
            tree.apply_style(
                c,
                Margin {
                    all: Some(Auto),
                    ..Default::default()
                },
            );
            tree.apply_style(c, FlexGrow(r32(1.0)));
        };

        child_styles(child);
        child_styles(other_child);
    }

    println!("Layout is {:#?}", tree.get_layout());
}
