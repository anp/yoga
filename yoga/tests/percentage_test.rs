extern crate yoga;

use yoga::{Align, Direction, Edge, FlexDirection, Justify, Node, PositionType, Undefined, Value};

#[test]
fn test_percentage_width_height() {
    let mut root = Node::new();
    root.set_flex_direction(FlexDirection::Row);
    root.set_width(Value::Point(200.0.into()));
    root.set_height(Value::Point(200.0.into()));

    let mut root_child0 = Node::new();
    root_child0.set_width(Value::Percent(30.0.into()));
    root_child0.set_height(Value::Percent(30.0.into()));
    root.insert_child(&mut root_child0, 0);
    root.calculate_layout(Undefined, Undefined, Direction::LTR);

    assert_eq!(0, root.get_layout_left() as i32);
    assert_eq!(0, root.get_layout_top() as i32);
    assert_eq!(200, root.get_layout_width() as i32);
    assert_eq!(200, root.get_layout_height() as i32);

    assert_eq!(0, root_child0.get_layout_left() as i32);
    assert_eq!(0, root_child0.get_layout_top() as i32);
    assert_eq!(60, root_child0.get_layout_width() as i32);
    assert_eq!(60, root_child0.get_layout_height() as i32);

    root.calculate_layout(Undefined, Undefined, Direction::RTL);

    assert_eq!(0, root.get_layout_left() as i32);
    assert_eq!(0, root.get_layout_top() as i32);
    assert_eq!(200, root.get_layout_width() as i32);
    assert_eq!(200, root.get_layout_height() as i32);

    assert_eq!(140, root_child0.get_layout_left() as i32);
    assert_eq!(0, root_child0.get_layout_top() as i32);
    assert_eq!(60, root_child0.get_layout_width() as i32);
    assert_eq!(60, root_child0.get_layout_height() as i32);
}

#[test]
fn test_percentage_position_left_top() {
    let mut root = Node::new();
    root.set_flex_direction(FlexDirection::Row);
    root.set_width(Value::Point(400.0.into()));
    root.set_height(Value::Point(400.0.into()));

    let mut root_child0 = Node::new();
    root_child0.set_position(Edge::Left, Value::Percent(10.0.into()));
    root_child0.set_position(Edge::Top, Value::Percent(20.0.into()));
    root_child0.set_width(Value::Percent(45.0.into()));
    root_child0.set_height(Value::Percent(55.0.into()));
    root.insert_child(&mut root_child0, 0);
    root.calculate_layout(Undefined, Undefined, Direction::LTR);

    assert_eq!(0, root.get_layout_left() as i32);
    assert_eq!(0, root.get_layout_top() as i32);
    assert_eq!(400, root.get_layout_width() as i32);
    assert_eq!(400, root.get_layout_height() as i32);

    assert_eq!(40, root_child0.get_layout_left() as i32);
    assert_eq!(80, root_child0.get_layout_top() as i32);
    assert_eq!(180, root_child0.get_layout_width() as i32);
    assert_eq!(220, root_child0.get_layout_height() as i32);

    root.calculate_layout(Undefined, Undefined, Direction::RTL);

    assert_eq!(0, root.get_layout_left() as i32);
    assert_eq!(0, root.get_layout_top() as i32);
    assert_eq!(400, root.get_layout_width() as i32);
    assert_eq!(400, root.get_layout_height() as i32);

    assert_eq!(260, root_child0.get_layout_left() as i32);
    assert_eq!(80, root_child0.get_layout_top() as i32);
    assert_eq!(180, root_child0.get_layout_width() as i32);
    assert_eq!(220, root_child0.get_layout_height() as i32);
}

#[test]
fn test_percentage_position_bottom_right() {
    let mut root = Node::new();
    root.set_flex_direction(FlexDirection::Row);
    root.set_width(Value::Point(500.0.into()));
    root.set_height(Value::Point(500.0.into()));

    let mut root_child0 = Node::new();
    root_child0.set_position(Edge::Right, Value::Percent(20.0.into()));
    root_child0.set_position(Edge::Bottom, Value::Percent(10.0.into()));
    root_child0.set_width(Value::Percent(55.0.into()));
    root_child0.set_height(Value::Percent(15.0.into()));
    root.insert_child(&mut root_child0, 0);
    root.calculate_layout(Undefined, Undefined, Direction::LTR);

    assert_eq!(0, root.get_layout_left() as i32);
    assert_eq!(0, root.get_layout_top() as i32);
    assert_eq!(500, root.get_layout_width() as i32);
    assert_eq!(500, root.get_layout_height() as i32);

    assert_eq!(-100, root_child0.get_layout_left() as i32);
    assert_eq!(-50, root_child0.get_layout_top() as i32);
    assert_eq!(275, root_child0.get_layout_width() as i32);
    assert_eq!(75, root_child0.get_layout_height() as i32);

    root.calculate_layout(Undefined, Undefined, Direction::RTL);

    assert_eq!(0, root.get_layout_left() as i32);
    assert_eq!(0, root.get_layout_top() as i32);
    assert_eq!(500, root.get_layout_width() as i32);
    assert_eq!(500, root.get_layout_height() as i32);

    assert_eq!(125, root_child0.get_layout_left() as i32);
    assert_eq!(-50, root_child0.get_layout_top() as i32);
    assert_eq!(275, root_child0.get_layout_width() as i32);
    assert_eq!(75, root_child0.get_layout_height() as i32);
}

#[test]
fn test_percentage_flex_basis() {
    let mut root = Node::new();
    root.set_flex_direction(FlexDirection::Row);
    root.set_width(Value::Point(200.0.into()));
    root.set_height(Value::Point(200.0.into()));

    let mut root_child0 = Node::new();
    root_child0.set_flex_grow(1.0);
    root_child0.set_flex_basis(Value::Percent(50.0.into()));
    root.insert_child(&mut root_child0, 0);

    let mut root_child1 = Node::new();
    root_child1.set_flex_grow(1.0);
    root_child1.set_flex_basis(Value::Percent(25.0.into()));
    root.insert_child(&mut root_child1, 1);
    root.calculate_layout(Undefined, Undefined, Direction::LTR);

    assert_eq!(0, root.get_layout_left() as i32);
    assert_eq!(0, root.get_layout_top() as i32);
    assert_eq!(200, root.get_layout_width() as i32);
    assert_eq!(200, root.get_layout_height() as i32);

    assert_eq!(0, root_child0.get_layout_left() as i32);
    assert_eq!(0, root_child0.get_layout_top() as i32);
    assert_eq!(125, root_child0.get_layout_width() as i32);
    assert_eq!(200, root_child0.get_layout_height() as i32);

    assert_eq!(125, root_child1.get_layout_left() as i32);
    assert_eq!(0, root_child1.get_layout_top() as i32);
    assert_eq!(75, root_child1.get_layout_width() as i32);
    assert_eq!(200, root_child1.get_layout_height() as i32);

    root.calculate_layout(Undefined, Undefined, Direction::RTL);

    assert_eq!(0, root.get_layout_left() as i32);
    assert_eq!(0, root.get_layout_top() as i32);
    assert_eq!(200, root.get_layout_width() as i32);
    assert_eq!(200, root.get_layout_height() as i32);

    assert_eq!(75, root_child0.get_layout_left() as i32);
    assert_eq!(0, root_child0.get_layout_top() as i32);
    assert_eq!(125, root_child0.get_layout_width() as i32);
    assert_eq!(200, root_child0.get_layout_height() as i32);

    assert_eq!(0, root_child1.get_layout_left() as i32);
    assert_eq!(0, root_child1.get_layout_top() as i32);
    assert_eq!(75, root_child1.get_layout_width() as i32);
    assert_eq!(200, root_child1.get_layout_height() as i32);
}

#[test]
fn test_percentage_flex_basis_cross() {
    let mut root = Node::new();
    root.set_width(Value::Point(200.0.into()));
    root.set_height(Value::Point(200.0.into()));

    let mut root_child0 = Node::new();
    root_child0.set_flex_grow(1.0);
    root_child0.set_flex_basis(Value::Percent(50.0.into()));
    root.insert_child(&mut root_child0, 0);

    let mut root_child1 = Node::new();
    root_child1.set_flex_grow(1.0);
    root_child1.set_flex_basis(Value::Percent(25.0.into()));
    root.insert_child(&mut root_child1, 1);
    root.calculate_layout(Undefined, Undefined, Direction::LTR);

    assert_eq!(0, root.get_layout_left() as i32);
    assert_eq!(0, root.get_layout_top() as i32);
    assert_eq!(200, root.get_layout_width() as i32);
    assert_eq!(200, root.get_layout_height() as i32);

    assert_eq!(0, root_child0.get_layout_left() as i32);
    assert_eq!(0, root_child0.get_layout_top() as i32);
    assert_eq!(200, root_child0.get_layout_width() as i32);
    assert_eq!(125, root_child0.get_layout_height() as i32);

    assert_eq!(0, root_child1.get_layout_left() as i32);
    assert_eq!(125, root_child1.get_layout_top() as i32);
    assert_eq!(200, root_child1.get_layout_width() as i32);
    assert_eq!(75, root_child1.get_layout_height() as i32);

    root.calculate_layout(Undefined, Undefined, Direction::RTL);

    assert_eq!(0, root.get_layout_left() as i32);
    assert_eq!(0, root.get_layout_top() as i32);
    assert_eq!(200, root.get_layout_width() as i32);
    assert_eq!(200, root.get_layout_height() as i32);

    assert_eq!(0, root_child0.get_layout_left() as i32);
    assert_eq!(0, root_child0.get_layout_top() as i32);
    assert_eq!(200, root_child0.get_layout_width() as i32);
    assert_eq!(125, root_child0.get_layout_height() as i32);

    assert_eq!(0, root_child1.get_layout_left() as i32);
    assert_eq!(125, root_child1.get_layout_top() as i32);
    assert_eq!(200, root_child1.get_layout_width() as i32);
    assert_eq!(75, root_child1.get_layout_height() as i32);
}

#[test]
fn test_percentage_flex_basis_cross_min_height() {
    let mut root = Node::new();
    root.set_width(Value::Point(200.0.into()));
    root.set_height(Value::Point(200.0.into()));

    let mut root_child0 = Node::new();
    root_child0.set_flex_grow(1.0);
    root_child0.set_min_height(Value::Percent(60.0.into()));
    root.insert_child(&mut root_child0, 0);

    let mut root_child1 = Node::new();
    root_child1.set_flex_grow(2.0);
    root_child1.set_min_height(Value::Percent(10.0.into()));
    root.insert_child(&mut root_child1, 1);
    root.calculate_layout(Undefined, Undefined, Direction::LTR);

    assert_eq!(0, root.get_layout_left() as i32);
    assert_eq!(0, root.get_layout_top() as i32);
    assert_eq!(200, root.get_layout_width() as i32);
    assert_eq!(200, root.get_layout_height() as i32);

    assert_eq!(0, root_child0.get_layout_left() as i32);
    assert_eq!(0, root_child0.get_layout_top() as i32);
    assert_eq!(200, root_child0.get_layout_width() as i32);
    assert_eq!(140, root_child0.get_layout_height() as i32);

    assert_eq!(0, root_child1.get_layout_left() as i32);
    assert_eq!(140, root_child1.get_layout_top() as i32);
    assert_eq!(200, root_child1.get_layout_width() as i32);
    assert_eq!(60, root_child1.get_layout_height() as i32);

    root.calculate_layout(Undefined, Undefined, Direction::RTL);

    assert_eq!(0, root.get_layout_left() as i32);
    assert_eq!(0, root.get_layout_top() as i32);
    assert_eq!(200, root.get_layout_width() as i32);
    assert_eq!(200, root.get_layout_height() as i32);

    assert_eq!(0, root_child0.get_layout_left() as i32);
    assert_eq!(0, root_child0.get_layout_top() as i32);
    assert_eq!(200, root_child0.get_layout_width() as i32);
    assert_eq!(140, root_child0.get_layout_height() as i32);

    assert_eq!(0, root_child1.get_layout_left() as i32);
    assert_eq!(140, root_child1.get_layout_top() as i32);
    assert_eq!(200, root_child1.get_layout_width() as i32);
    assert_eq!(60, root_child1.get_layout_height() as i32);
}

#[test]
fn test_percentage_flex_basis_main_max_height() {
    let mut root = Node::new();
    root.set_flex_direction(FlexDirection::Row);
    root.set_width(Value::Point(200.0.into()));
    root.set_height(Value::Point(200.0.into()));

    let mut root_child0 = Node::new();
    root_child0.set_flex_grow(1.0);
    root_child0.set_flex_basis(Value::Percent(10.0.into()));
    root_child0.set_max_height(Value::Percent(60.0.into()));
    root.insert_child(&mut root_child0, 0);

    let mut root_child1 = Node::new();
    root_child1.set_flex_grow(4.0);
    root_child1.set_flex_basis(Value::Percent(10.0.into()));
    root_child1.set_max_height(Value::Percent(20.0.into()));
    root.insert_child(&mut root_child1, 1);
    root.calculate_layout(Undefined, Undefined, Direction::LTR);

    assert_eq!(0, root.get_layout_left() as i32);
    assert_eq!(0, root.get_layout_top() as i32);
    assert_eq!(200, root.get_layout_width() as i32);
    assert_eq!(200, root.get_layout_height() as i32);

    assert_eq!(0, root_child0.get_layout_left() as i32);
    assert_eq!(0, root_child0.get_layout_top() as i32);
    assert_eq!(52, root_child0.get_layout_width() as i32);
    assert_eq!(120, root_child0.get_layout_height() as i32);

    assert_eq!(52, root_child1.get_layout_left() as i32);
    assert_eq!(0, root_child1.get_layout_top() as i32);
    assert_eq!(148, root_child1.get_layout_width() as i32);
    assert_eq!(40, root_child1.get_layout_height() as i32);

    root.calculate_layout(Undefined, Undefined, Direction::RTL);

    assert_eq!(0, root.get_layout_left() as i32);
    assert_eq!(0, root.get_layout_top() as i32);
    assert_eq!(200, root.get_layout_width() as i32);
    assert_eq!(200, root.get_layout_height() as i32);

    assert_eq!(148, root_child0.get_layout_left() as i32);
    assert_eq!(0, root_child0.get_layout_top() as i32);
    assert_eq!(52, root_child0.get_layout_width() as i32);
    assert_eq!(120, root_child0.get_layout_height() as i32);

    assert_eq!(0, root_child1.get_layout_left() as i32);
    assert_eq!(0, root_child1.get_layout_top() as i32);
    assert_eq!(148, root_child1.get_layout_width() as i32);
    assert_eq!(40, root_child1.get_layout_height() as i32);
}

#[test]
fn test_percentage_flex_basis_cross_max_height() {
    let mut root = Node::new();
    root.set_width(Value::Point(200.0.into()));
    root.set_height(Value::Point(200.0.into()));

    let mut root_child0 = Node::new();
    root_child0.set_flex_grow(1.0);
    root_child0.set_flex_basis(Value::Percent(10.0.into()));
    root_child0.set_max_height(Value::Percent(60.0.into()));
    root.insert_child(&mut root_child0, 0);

    let mut root_child1 = Node::new();
    root_child1.set_flex_grow(4.0);
    root_child1.set_flex_basis(Value::Percent(10.0.into()));
    root_child1.set_max_height(Value::Percent(20.0.into()));
    root.insert_child(&mut root_child1, 1);
    root.calculate_layout(Undefined, Undefined, Direction::LTR);

    assert_eq!(0, root.get_layout_left() as i32);
    assert_eq!(0, root.get_layout_top() as i32);
    assert_eq!(200, root.get_layout_width() as i32);
    assert_eq!(200, root.get_layout_height() as i32);

    assert_eq!(0, root_child0.get_layout_left() as i32);
    assert_eq!(0, root_child0.get_layout_top() as i32);
    assert_eq!(200, root_child0.get_layout_width() as i32);
    assert_eq!(120, root_child0.get_layout_height() as i32);

    assert_eq!(0, root_child1.get_layout_left() as i32);
    assert_eq!(120, root_child1.get_layout_top() as i32);
    assert_eq!(200, root_child1.get_layout_width() as i32);
    assert_eq!(40, root_child1.get_layout_height() as i32);

    root.calculate_layout(Undefined, Undefined, Direction::RTL);

    assert_eq!(0, root.get_layout_left() as i32);
    assert_eq!(0, root.get_layout_top() as i32);
    assert_eq!(200, root.get_layout_width() as i32);
    assert_eq!(200, root.get_layout_height() as i32);

    assert_eq!(0, root_child0.get_layout_left() as i32);
    assert_eq!(0, root_child0.get_layout_top() as i32);
    assert_eq!(200, root_child0.get_layout_width() as i32);
    assert_eq!(120, root_child0.get_layout_height() as i32);

    assert_eq!(0, root_child1.get_layout_left() as i32);
    assert_eq!(120, root_child1.get_layout_top() as i32);
    assert_eq!(200, root_child1.get_layout_width() as i32);
    assert_eq!(40, root_child1.get_layout_height() as i32);
}

#[test]
fn test_percentage_flex_basis_main_max_width() {
    let mut root = Node::new();
    root.set_flex_direction(FlexDirection::Row);
    root.set_width(Value::Point(200.0.into()));
    root.set_height(Value::Point(200.0.into()));

    let mut root_child0 = Node::new();
    root_child0.set_flex_grow(1.0);
    root_child0.set_flex_basis(Value::Percent(15.0.into()));
    root_child0.set_max_width(Value::Percent(60.0.into()));
    root.insert_child(&mut root_child0, 0);

    let mut root_child1 = Node::new();
    root_child1.set_flex_grow(4.0);
    root_child1.set_flex_basis(Value::Percent(10.0.into()));
    root_child1.set_max_width(Value::Percent(20.0.into()));
    root.insert_child(&mut root_child1, 1);
    root.calculate_layout(Undefined, Undefined, Direction::LTR);

    assert_eq!(0, root.get_layout_left() as i32);
    assert_eq!(0, root.get_layout_top() as i32);
    assert_eq!(200, root.get_layout_width() as i32);
    assert_eq!(200, root.get_layout_height() as i32);

    assert_eq!(0, root_child0.get_layout_left() as i32);
    assert_eq!(0, root_child0.get_layout_top() as i32);
    assert_eq!(120, root_child0.get_layout_width() as i32);
    assert_eq!(200, root_child0.get_layout_height() as i32);

    assert_eq!(120, root_child1.get_layout_left() as i32);
    assert_eq!(0, root_child1.get_layout_top() as i32);
    assert_eq!(40, root_child1.get_layout_width() as i32);
    assert_eq!(200, root_child1.get_layout_height() as i32);

    root.calculate_layout(Undefined, Undefined, Direction::RTL);

    assert_eq!(0, root.get_layout_left() as i32);
    assert_eq!(0, root.get_layout_top() as i32);
    assert_eq!(200, root.get_layout_width() as i32);
    assert_eq!(200, root.get_layout_height() as i32);

    assert_eq!(80, root_child0.get_layout_left() as i32);
    assert_eq!(0, root_child0.get_layout_top() as i32);
    assert_eq!(120, root_child0.get_layout_width() as i32);
    assert_eq!(200, root_child0.get_layout_height() as i32);

    assert_eq!(40, root_child1.get_layout_left() as i32);
    assert_eq!(0, root_child1.get_layout_top() as i32);
    assert_eq!(40, root_child1.get_layout_width() as i32);
    assert_eq!(200, root_child1.get_layout_height() as i32);
}

#[test]
fn test_percentage_flex_basis_cross_max_width() {
    let mut root = Node::new();
    root.set_width(Value::Point(200.0.into()));
    root.set_height(Value::Point(200.0.into()));

    let mut root_child0 = Node::new();
    root_child0.set_flex_grow(1.0);
    root_child0.set_flex_basis(Value::Percent(10.0.into()));
    root_child0.set_max_width(Value::Percent(60.0.into()));
    root.insert_child(&mut root_child0, 0);

    let mut root_child1 = Node::new();
    root_child1.set_flex_grow(4.0);
    root_child1.set_flex_basis(Value::Percent(15.0.into()));
    root_child1.set_max_width(Value::Percent(20.0.into()));
    root.insert_child(&mut root_child1, 1);
    root.calculate_layout(Undefined, Undefined, Direction::LTR);

    assert_eq!(0, root.get_layout_left() as i32);
    assert_eq!(0, root.get_layout_top() as i32);
    assert_eq!(200, root.get_layout_width() as i32);
    assert_eq!(200, root.get_layout_height() as i32);

    assert_eq!(0, root_child0.get_layout_left() as i32);
    assert_eq!(0, root_child0.get_layout_top() as i32);
    assert_eq!(120, root_child0.get_layout_width() as i32);
    assert_eq!(50, root_child0.get_layout_height() as i32);

    assert_eq!(0, root_child1.get_layout_left() as i32);
    assert_eq!(50, root_child1.get_layout_top() as i32);
    assert_eq!(40, root_child1.get_layout_width() as i32);
    assert_eq!(150, root_child1.get_layout_height() as i32);

    root.calculate_layout(Undefined, Undefined, Direction::RTL);

    assert_eq!(0, root.get_layout_left() as i32);
    assert_eq!(0, root.get_layout_top() as i32);
    assert_eq!(200, root.get_layout_width() as i32);
    assert_eq!(200, root.get_layout_height() as i32);

    assert_eq!(80, root_child0.get_layout_left() as i32);
    assert_eq!(0, root_child0.get_layout_top() as i32);
    assert_eq!(120, root_child0.get_layout_width() as i32);
    assert_eq!(50, root_child0.get_layout_height() as i32);

    assert_eq!(160, root_child1.get_layout_left() as i32);
    assert_eq!(50, root_child1.get_layout_top() as i32);
    assert_eq!(40, root_child1.get_layout_width() as i32);
    assert_eq!(150, root_child1.get_layout_height() as i32);
}

#[test]
fn test_percentage_flex_basis_main_min_width() {
    let mut root = Node::new();
    root.set_flex_direction(FlexDirection::Row);
    root.set_width(Value::Point(200.0.into()));
    root.set_height(Value::Point(200.0.into()));

    let mut root_child0 = Node::new();
    root_child0.set_flex_grow(1.0);
    root_child0.set_flex_basis(Value::Percent(15.0.into()));
    root_child0.set_min_width(Value::Percent(60.0.into()));
    root.insert_child(&mut root_child0, 0);

    let mut root_child1 = Node::new();
    root_child1.set_flex_grow(4.0);
    root_child1.set_flex_basis(Value::Percent(10.0.into()));
    root_child1.set_min_width(Value::Percent(20.0.into()));
    root.insert_child(&mut root_child1, 1);
    root.calculate_layout(Undefined, Undefined, Direction::LTR);

    assert_eq!(0, root.get_layout_left() as i32);
    assert_eq!(0, root.get_layout_top() as i32);
    assert_eq!(200, root.get_layout_width() as i32);
    assert_eq!(200, root.get_layout_height() as i32);

    assert_eq!(0, root_child0.get_layout_left() as i32);
    assert_eq!(0, root_child0.get_layout_top() as i32);
    assert_eq!(120, root_child0.get_layout_width() as i32);
    assert_eq!(200, root_child0.get_layout_height() as i32);

    assert_eq!(120, root_child1.get_layout_left() as i32);
    assert_eq!(0, root_child1.get_layout_top() as i32);
    assert_eq!(80, root_child1.get_layout_width() as i32);
    assert_eq!(200, root_child1.get_layout_height() as i32);

    root.calculate_layout(Undefined, Undefined, Direction::RTL);

    assert_eq!(0, root.get_layout_left() as i32);
    assert_eq!(0, root.get_layout_top() as i32);
    assert_eq!(200, root.get_layout_width() as i32);
    assert_eq!(200, root.get_layout_height() as i32);

    assert_eq!(80, root_child0.get_layout_left() as i32);
    assert_eq!(0, root_child0.get_layout_top() as i32);
    assert_eq!(120, root_child0.get_layout_width() as i32);
    assert_eq!(200, root_child0.get_layout_height() as i32);

    assert_eq!(0, root_child1.get_layout_left() as i32);
    assert_eq!(0, root_child1.get_layout_top() as i32);
    assert_eq!(80, root_child1.get_layout_width() as i32);
    assert_eq!(200, root_child1.get_layout_height() as i32);
}

#[test]
fn test_percentage_flex_basis_cross_min_width() {
    let mut root = Node::new();
    root.set_width(Value::Point(200.0.into()));
    root.set_height(Value::Point(200.0.into()));

    let mut root_child0 = Node::new();
    root_child0.set_flex_grow(1.0);
    root_child0.set_flex_basis(Value::Percent(10.0.into()));
    root_child0.set_min_width(Value::Percent(60.0.into()));
    root.insert_child(&mut root_child0, 0);

    let mut root_child1 = Node::new();
    root_child1.set_flex_grow(4.0);
    root_child1.set_flex_basis(Value::Percent(15.0.into()));
    root_child1.set_min_width(Value::Percent(20.0.into()));
    root.insert_child(&mut root_child1, 1);
    root.calculate_layout(Undefined, Undefined, Direction::LTR);

    assert_eq!(0, root.get_layout_left() as i32);
    assert_eq!(0, root.get_layout_top() as i32);
    assert_eq!(200, root.get_layout_width() as i32);
    assert_eq!(200, root.get_layout_height() as i32);

    assert_eq!(0, root_child0.get_layout_left() as i32);
    assert_eq!(0, root_child0.get_layout_top() as i32);
    assert_eq!(200, root_child0.get_layout_width() as i32);
    assert_eq!(50, root_child0.get_layout_height() as i32);

    assert_eq!(0, root_child1.get_layout_left() as i32);
    assert_eq!(50, root_child1.get_layout_top() as i32);
    assert_eq!(200, root_child1.get_layout_width() as i32);
    assert_eq!(150, root_child1.get_layout_height() as i32);

    root.calculate_layout(Undefined, Undefined, Direction::RTL);

    assert_eq!(0, root.get_layout_left() as i32);
    assert_eq!(0, root.get_layout_top() as i32);
    assert_eq!(200, root.get_layout_width() as i32);
    assert_eq!(200, root.get_layout_height() as i32);

    assert_eq!(0, root_child0.get_layout_left() as i32);
    assert_eq!(0, root_child0.get_layout_top() as i32);
    assert_eq!(200, root_child0.get_layout_width() as i32);
    assert_eq!(50, root_child0.get_layout_height() as i32);

    assert_eq!(0, root_child1.get_layout_left() as i32);
    assert_eq!(50, root_child1.get_layout_top() as i32);
    assert_eq!(200, root_child1.get_layout_width() as i32);
    assert_eq!(150, root_child1.get_layout_height() as i32);
}

#[test]
fn test_percentage_multiple_nested_with_padding_margin_and_percentage_values() {
    let mut root = Node::new();
    root.set_width(Value::Point(200.0.into()));
    root.set_height(Value::Point(200.0.into()));

    let mut root_child0 = Node::new();
    root_child0.set_flex_grow(1.0);
    root_child0.set_flex_basis(Value::Percent(10.0.into()));
    root_child0.set_margin(Edge::Left, Value::Point(5.0.into()));
    root_child0.set_margin(Edge::Top, Value::Point(5.0.into()));
    root_child0.set_margin(Edge::Right, Value::Point(5.0.into()));
    root_child0.set_margin(Edge::Bottom, Value::Point(5.0.into()));
    root_child0.set_padding(Edge::Left, Value::Point(3.0.into()));
    root_child0.set_padding(Edge::Top, Value::Point(3.0.into()));
    root_child0.set_padding(Edge::Right, Value::Point(3.0.into()));
    root_child0.set_padding(Edge::Bottom, Value::Point(3.0.into()));
    root_child0.set_min_width(Value::Percent(60.0.into()));
    root.insert_child(&mut root_child0, 0);

    let mut root_child0_child0 = Node::new();
    root_child0_child0.set_margin(Edge::Left, Value::Point(5.0.into()));
    root_child0_child0.set_margin(Edge::Top, Value::Point(5.0.into()));
    root_child0_child0.set_margin(Edge::Right, Value::Point(5.0.into()));
    root_child0_child0.set_margin(Edge::Bottom, Value::Point(5.0.into()));
    root_child0_child0.set_padding(Edge::Left, Value::Percent(3.0.into()));
    root_child0_child0.set_padding(Edge::Top, Value::Percent(3.0.into()));
    root_child0_child0.set_padding(Edge::Right, Value::Percent(3.0.into()));
    root_child0_child0.set_padding(Edge::Bottom, Value::Percent(3.0.into()));
    root_child0_child0.set_width(Value::Percent(50.0.into()));
    root_child0.insert_child(&mut root_child0_child0, 0);

    let mut root_child0_child0_child0 = Node::new();
    root_child0_child0_child0.set_margin(Edge::Left, Value::Percent(5.0.into()));
    root_child0_child0_child0.set_margin(Edge::Top, Value::Percent(5.0.into()));
    root_child0_child0_child0.set_margin(Edge::Right, Value::Percent(5.0.into()));
    root_child0_child0_child0.set_margin(Edge::Bottom, Value::Percent(5.0.into()));
    root_child0_child0_child0.set_padding(Edge::Left, Value::Point(3.0.into()));
    root_child0_child0_child0.set_padding(Edge::Top, Value::Point(3.0.into()));
    root_child0_child0_child0.set_padding(Edge::Right, Value::Point(3.0.into()));
    root_child0_child0_child0.set_padding(Edge::Bottom, Value::Point(3.0.into()));
    root_child0_child0_child0.set_width(Value::Percent(45.0.into()));
    root_child0_child0.insert_child(&mut root_child0_child0_child0, 0);

    let mut root_child1 = Node::new();
    root_child1.set_flex_grow(4.0);
    root_child1.set_flex_basis(Value::Percent(15.0.into()));
    root_child1.set_min_width(Value::Percent(20.0.into()));
    root.insert_child(&mut root_child1, 1);
    root.calculate_layout(Undefined, Undefined, Direction::LTR);

    assert_eq!(0, root.get_layout_left() as i32);
    assert_eq!(0, root.get_layout_top() as i32);
    assert_eq!(200, root.get_layout_width() as i32);
    assert_eq!(200, root.get_layout_height() as i32);

    assert_eq!(5, root_child0.get_layout_left() as i32);
    assert_eq!(5, root_child0.get_layout_top() as i32);
    assert_eq!(190, root_child0.get_layout_width() as i32);
    assert_eq!(48, root_child0.get_layout_height() as i32);

    assert_eq!(8, root_child0_child0.get_layout_left() as i32);
    assert_eq!(8, root_child0_child0.get_layout_top() as i32);
    assert_eq!(92, root_child0_child0.get_layout_width() as i32);
    assert_eq!(25, root_child0_child0.get_layout_height() as i32);

    assert_eq!(10, root_child0_child0_child0.get_layout_left() as i32);
    assert_eq!(10, root_child0_child0_child0.get_layout_top() as i32);
    assert_eq!(36, root_child0_child0_child0.get_layout_width() as i32);
    assert_eq!(6, root_child0_child0_child0.get_layout_height() as i32);

    assert_eq!(0, root_child1.get_layout_left() as i32);
    assert_eq!(58, root_child1.get_layout_top() as i32);
    assert_eq!(200, root_child1.get_layout_width() as i32);
    assert_eq!(142, root_child1.get_layout_height() as i32);

    root.calculate_layout(Undefined, Undefined, Direction::RTL);

    assert_eq!(0, root.get_layout_left() as i32);
    assert_eq!(0, root.get_layout_top() as i32);
    assert_eq!(200, root.get_layout_width() as i32);
    assert_eq!(200, root.get_layout_height() as i32);

    assert_eq!(5, root_child0.get_layout_left() as i32);
    assert_eq!(5, root_child0.get_layout_top() as i32);
    assert_eq!(190, root_child0.get_layout_width() as i32);
    assert_eq!(48, root_child0.get_layout_height() as i32);

    assert_eq!(90, root_child0_child0.get_layout_left() as i32);
    assert_eq!(8, root_child0_child0.get_layout_top() as i32);
    assert_eq!(92, root_child0_child0.get_layout_width() as i32);
    assert_eq!(25, root_child0_child0.get_layout_height() as i32);

    assert_eq!(46, root_child0_child0_child0.get_layout_left() as i32);
    assert_eq!(10, root_child0_child0_child0.get_layout_top() as i32);
    assert_eq!(36, root_child0_child0_child0.get_layout_width() as i32);
    assert_eq!(6, root_child0_child0_child0.get_layout_height() as i32);

    assert_eq!(0, root_child1.get_layout_left() as i32);
    assert_eq!(58, root_child1.get_layout_top() as i32);
    assert_eq!(200, root_child1.get_layout_width() as i32);
    assert_eq!(142, root_child1.get_layout_height() as i32);
}

#[test]
fn test_percentage_margin_should_calculate_based_only_on_width() {
    let mut root = Node::new();
    root.set_width(Value::Point(200.0.into()));
    root.set_height(Value::Point(100.0.into()));

    let mut root_child0 = Node::new();
    root_child0.set_flex_grow(1.0);
    root_child0.set_margin(Edge::Left, Value::Percent(10.0.into()));
    root_child0.set_margin(Edge::Top, Value::Percent(10.0.into()));
    root_child0.set_margin(Edge::Right, Value::Percent(10.0.into()));
    root_child0.set_margin(Edge::Bottom, Value::Percent(10.0.into()));
    root.insert_child(&mut root_child0, 0);

    let mut root_child0_child0 = Node::new();
    root_child0_child0.set_width(Value::Point(10.0.into()));
    root_child0_child0.set_height(Value::Point(10.0.into()));
    root_child0.insert_child(&mut root_child0_child0, 0);
    root.calculate_layout(Undefined, Undefined, Direction::LTR);

    assert_eq!(0, root.get_layout_left() as i32);
    assert_eq!(0, root.get_layout_top() as i32);
    assert_eq!(200, root.get_layout_width() as i32);
    assert_eq!(100, root.get_layout_height() as i32);

    assert_eq!(20, root_child0.get_layout_left() as i32);
    assert_eq!(20, root_child0.get_layout_top() as i32);
    assert_eq!(160, root_child0.get_layout_width() as i32);
    assert_eq!(60, root_child0.get_layout_height() as i32);

    assert_eq!(0, root_child0_child0.get_layout_left() as i32);
    assert_eq!(0, root_child0_child0.get_layout_top() as i32);
    assert_eq!(10, root_child0_child0.get_layout_width() as i32);
    assert_eq!(10, root_child0_child0.get_layout_height() as i32);

    root.calculate_layout(Undefined, Undefined, Direction::RTL);

    assert_eq!(0, root.get_layout_left() as i32);
    assert_eq!(0, root.get_layout_top() as i32);
    assert_eq!(200, root.get_layout_width() as i32);
    assert_eq!(100, root.get_layout_height() as i32);

    assert_eq!(20, root_child0.get_layout_left() as i32);
    assert_eq!(20, root_child0.get_layout_top() as i32);
    assert_eq!(160, root_child0.get_layout_width() as i32);
    assert_eq!(60, root_child0.get_layout_height() as i32);

    assert_eq!(150, root_child0_child0.get_layout_left() as i32);
    assert_eq!(0, root_child0_child0.get_layout_top() as i32);
    assert_eq!(10, root_child0_child0.get_layout_width() as i32);
    assert_eq!(10, root_child0_child0.get_layout_height() as i32);
}

#[test]
fn test_percentage_padding_should_calculate_based_only_on_width() {
    let mut root = Node::new();
    root.set_width(Value::Point(200.0.into()));
    root.set_height(Value::Point(100.0.into()));

    let mut root_child0 = Node::new();
    root_child0.set_flex_grow(1.0);
    root_child0.set_padding(Edge::Left, Value::Percent(10.0.into()));
    root_child0.set_padding(Edge::Top, Value::Percent(10.0.into()));
    root_child0.set_padding(Edge::Right, Value::Percent(10.0.into()));
    root_child0.set_padding(Edge::Bottom, Value::Percent(10.0.into()));
    root.insert_child(&mut root_child0, 0);

    let mut root_child0_child0 = Node::new();
    root_child0_child0.set_width(Value::Point(10.0.into()));
    root_child0_child0.set_height(Value::Point(10.0.into()));
    root_child0.insert_child(&mut root_child0_child0, 0);
    root.calculate_layout(Undefined, Undefined, Direction::LTR);

    assert_eq!(0, root.get_layout_left() as i32);
    assert_eq!(0, root.get_layout_top() as i32);
    assert_eq!(200, root.get_layout_width() as i32);
    assert_eq!(100, root.get_layout_height() as i32);

    assert_eq!(0, root_child0.get_layout_left() as i32);
    assert_eq!(0, root_child0.get_layout_top() as i32);
    assert_eq!(200, root_child0.get_layout_width() as i32);
    assert_eq!(100, root_child0.get_layout_height() as i32);

    assert_eq!(20, root_child0_child0.get_layout_left() as i32);
    assert_eq!(20, root_child0_child0.get_layout_top() as i32);
    assert_eq!(10, root_child0_child0.get_layout_width() as i32);
    assert_eq!(10, root_child0_child0.get_layout_height() as i32);

    root.calculate_layout(Undefined, Undefined, Direction::RTL);

    assert_eq!(0, root.get_layout_left() as i32);
    assert_eq!(0, root.get_layout_top() as i32);
    assert_eq!(200, root.get_layout_width() as i32);
    assert_eq!(100, root.get_layout_height() as i32);

    assert_eq!(0, root_child0.get_layout_left() as i32);
    assert_eq!(0, root_child0.get_layout_top() as i32);
    assert_eq!(200, root_child0.get_layout_width() as i32);
    assert_eq!(100, root_child0.get_layout_height() as i32);

    assert_eq!(170, root_child0_child0.get_layout_left() as i32);
    assert_eq!(20, root_child0_child0.get_layout_top() as i32);
    assert_eq!(10, root_child0_child0.get_layout_width() as i32);
    assert_eq!(10, root_child0_child0.get_layout_height() as i32);
}

#[test]
fn test_percentage_absolute_position() {
    let mut root = Node::new();
    root.set_width(Value::Point(200.0.into()));
    root.set_height(Value::Point(100.0.into()));

    let mut root_child0 = Node::new();
    root_child0.set_position_type(PositionType::Absolute);
    root_child0.set_position(Edge::Left, Value::Percent(30.0.into()));
    root_child0.set_position(Edge::Top, Value::Percent(10.0.into()));
    root_child0.set_width(Value::Point(10.0.into()));
    root_child0.set_height(Value::Point(10.0.into()));
    root.insert_child(&mut root_child0, 0);
    root.calculate_layout(Undefined, Undefined, Direction::LTR);

    assert_eq!(0, root.get_layout_left() as i32);
    assert_eq!(0, root.get_layout_top() as i32);
    assert_eq!(200, root.get_layout_width() as i32);
    assert_eq!(100, root.get_layout_height() as i32);

    assert_eq!(60, root_child0.get_layout_left() as i32);
    assert_eq!(10, root_child0.get_layout_top() as i32);
    assert_eq!(10, root_child0.get_layout_width() as i32);
    assert_eq!(10, root_child0.get_layout_height() as i32);

    root.calculate_layout(Undefined, Undefined, Direction::RTL);

    assert_eq!(0, root.get_layout_left() as i32);
    assert_eq!(0, root.get_layout_top() as i32);
    assert_eq!(200, root.get_layout_width() as i32);
    assert_eq!(100, root.get_layout_height() as i32);

    assert_eq!(60, root_child0.get_layout_left() as i32);
    assert_eq!(10, root_child0.get_layout_top() as i32);
    assert_eq!(10, root_child0.get_layout_width() as i32);
    assert_eq!(10, root_child0.get_layout_height() as i32);
}

#[test]
fn test_percentage_width_height_undefined_parent_size() {
    let mut root = Node::new();

    let mut root_child0 = Node::new();
    root_child0.set_width(Value::Percent(50.0.into()));
    root_child0.set_height(Value::Percent(50.0.into()));
    root.insert_child(&mut root_child0, 0);
    root.calculate_layout(Undefined, Undefined, Direction::LTR);

    assert_eq!(0, root.get_layout_left() as i32);
    assert_eq!(0, root.get_layout_top() as i32);
    assert_eq!(0, root.get_layout_width() as i32);
    assert_eq!(0, root.get_layout_height() as i32);

    assert_eq!(0, root_child0.get_layout_left() as i32);
    assert_eq!(0, root_child0.get_layout_top() as i32);
    assert_eq!(0, root_child0.get_layout_width() as i32);
    assert_eq!(0, root_child0.get_layout_height() as i32);

    root.calculate_layout(Undefined, Undefined, Direction::RTL);

    assert_eq!(0, root.get_layout_left() as i32);
    assert_eq!(0, root.get_layout_top() as i32);
    assert_eq!(0, root.get_layout_width() as i32);
    assert_eq!(0, root.get_layout_height() as i32);

    assert_eq!(0, root_child0.get_layout_left() as i32);
    assert_eq!(0, root_child0.get_layout_top() as i32);
    assert_eq!(0, root_child0.get_layout_width() as i32);
    assert_eq!(0, root_child0.get_layout_height() as i32);
}

#[test]
fn test_percent_within_flex_grow() {
    let mut root = Node::new();
    root.set_flex_direction(FlexDirection::Row);
    root.set_width(Value::Point(350.0.into()));
    root.set_height(Value::Point(100.0.into()));

    let mut root_child0 = Node::new();
    root_child0.set_width(Value::Point(100.0.into()));
    root.insert_child(&mut root_child0, 0);

    let mut root_child1 = Node::new();
    root_child1.set_flex_grow(1.0);
    root.insert_child(&mut root_child1, 1);

    let mut root_child1_child0 = Node::new();
    root_child1_child0.set_width(Value::Percent(100.0.into()));
    root_child1.insert_child(&mut root_child1_child0, 0);

    let mut root_child2 = Node::new();
    root_child2.set_width(Value::Point(100.0.into()));
    root.insert_child(&mut root_child2, 2);
    root.calculate_layout(Undefined, Undefined, Direction::LTR);

    assert_eq!(0, root.get_layout_left() as i32);
    assert_eq!(0, root.get_layout_top() as i32);
    assert_eq!(350, root.get_layout_width() as i32);
    assert_eq!(100, root.get_layout_height() as i32);

    assert_eq!(0, root_child0.get_layout_left() as i32);
    assert_eq!(0, root_child0.get_layout_top() as i32);
    assert_eq!(100, root_child0.get_layout_width() as i32);
    assert_eq!(100, root_child0.get_layout_height() as i32);

    assert_eq!(100, root_child1.get_layout_left() as i32);
    assert_eq!(0, root_child1.get_layout_top() as i32);
    assert_eq!(150, root_child1.get_layout_width() as i32);
    assert_eq!(100, root_child1.get_layout_height() as i32);

    assert_eq!(0, root_child1_child0.get_layout_left() as i32);
    assert_eq!(0, root_child1_child0.get_layout_top() as i32);
    assert_eq!(150, root_child1_child0.get_layout_width() as i32);
    assert_eq!(0, root_child1_child0.get_layout_height() as i32);

    assert_eq!(250, root_child2.get_layout_left() as i32);
    assert_eq!(0, root_child2.get_layout_top() as i32);
    assert_eq!(100, root_child2.get_layout_width() as i32);
    assert_eq!(100, root_child2.get_layout_height() as i32);

    root.calculate_layout(Undefined, Undefined, Direction::RTL);

    assert_eq!(0, root.get_layout_left() as i32);
    assert_eq!(0, root.get_layout_top() as i32);
    assert_eq!(350, root.get_layout_width() as i32);
    assert_eq!(100, root.get_layout_height() as i32);

    assert_eq!(250, root_child0.get_layout_left() as i32);
    assert_eq!(0, root_child0.get_layout_top() as i32);
    assert_eq!(100, root_child0.get_layout_width() as i32);
    assert_eq!(100, root_child0.get_layout_height() as i32);

    assert_eq!(100, root_child1.get_layout_left() as i32);
    assert_eq!(0, root_child1.get_layout_top() as i32);
    assert_eq!(150, root_child1.get_layout_width() as i32);
    assert_eq!(100, root_child1.get_layout_height() as i32);

    assert_eq!(0, root_child1_child0.get_layout_left() as i32);
    assert_eq!(0, root_child1_child0.get_layout_top() as i32);
    assert_eq!(150, root_child1_child0.get_layout_width() as i32);
    assert_eq!(0, root_child1_child0.get_layout_height() as i32);

    assert_eq!(0, root_child2.get_layout_left() as i32);
    assert_eq!(0, root_child2.get_layout_top() as i32);
    assert_eq!(100, root_child2.get_layout_width() as i32);
    assert_eq!(100, root_child2.get_layout_height() as i32);
}

#[test]
fn test_percentage_container_in_wrapping_container() {
    let mut root = Node::new();
    root.set_justify_content(Justify::Center);
    root.set_align_items(Align::Center);
    root.set_width(Value::Point(200.0.into()));
    root.set_height(Value::Point(200.0.into()));

    let mut root_child0 = Node::new();
    root.insert_child(&mut root_child0, 0);

    let mut root_child0_child0 = Node::new();
    root_child0_child0.set_flex_direction(FlexDirection::Row);
    root_child0_child0.set_justify_content(Justify::Center);
    root_child0_child0.set_width(Value::Percent(100.0.into()));
    root_child0.insert_child(&mut root_child0_child0, 0);

    let mut root_child0_child0_child0 = Node::new();
    root_child0_child0_child0.set_width(Value::Point(50.0.into()));
    root_child0_child0_child0.set_height(Value::Point(50.0.into()));
    root_child0_child0.insert_child(&mut root_child0_child0_child0, 0);

    let mut root_child0_child0_child1 = Node::new();
    root_child0_child0_child1.set_width(Value::Point(50.0.into()));
    root_child0_child0_child1.set_height(Value::Point(50.0.into()));
    root_child0_child0.insert_child(&mut root_child0_child0_child1, 1);
    root.calculate_layout(Undefined, Undefined, Direction::LTR);

    assert_eq!(0, root.get_layout_left() as i32);
    assert_eq!(0, root.get_layout_top() as i32);
    assert_eq!(200, root.get_layout_width() as i32);
    assert_eq!(200, root.get_layout_height() as i32);

    assert_eq!(50, root_child0.get_layout_left() as i32);
    assert_eq!(75, root_child0.get_layout_top() as i32);
    assert_eq!(100, root_child0.get_layout_width() as i32);
    assert_eq!(50, root_child0.get_layout_height() as i32);

    assert_eq!(0, root_child0_child0.get_layout_left() as i32);
    assert_eq!(0, root_child0_child0.get_layout_top() as i32);
    assert_eq!(100, root_child0_child0.get_layout_width() as i32);
    assert_eq!(50, root_child0_child0.get_layout_height() as i32);

    assert_eq!(0, root_child0_child0_child0.get_layout_left() as i32);
    assert_eq!(0, root_child0_child0_child0.get_layout_top() as i32);
    assert_eq!(50, root_child0_child0_child0.get_layout_width() as i32);
    assert_eq!(50, root_child0_child0_child0.get_layout_height() as i32);

    assert_eq!(50, root_child0_child0_child1.get_layout_left() as i32);
    assert_eq!(0, root_child0_child0_child1.get_layout_top() as i32);
    assert_eq!(50, root_child0_child0_child1.get_layout_width() as i32);
    assert_eq!(50, root_child0_child0_child1.get_layout_height() as i32);

    root.calculate_layout(Undefined, Undefined, Direction::RTL);

    assert_eq!(0, root.get_layout_left() as i32);
    assert_eq!(0, root.get_layout_top() as i32);
    assert_eq!(200, root.get_layout_width() as i32);
    assert_eq!(200, root.get_layout_height() as i32);

    assert_eq!(50, root_child0.get_layout_left() as i32);
    assert_eq!(75, root_child0.get_layout_top() as i32);
    assert_eq!(100, root_child0.get_layout_width() as i32);
    assert_eq!(50, root_child0.get_layout_height() as i32);

    assert_eq!(0, root_child0_child0.get_layout_left() as i32);
    assert_eq!(0, root_child0_child0.get_layout_top() as i32);
    assert_eq!(100, root_child0_child0.get_layout_width() as i32);
    assert_eq!(50, root_child0_child0.get_layout_height() as i32);

    assert_eq!(50, root_child0_child0_child0.get_layout_left() as i32);
    assert_eq!(0, root_child0_child0_child0.get_layout_top() as i32);
    assert_eq!(50, root_child0_child0_child0.get_layout_width() as i32);
    assert_eq!(50, root_child0_child0_child0.get_layout_height() as i32);

    assert_eq!(0, root_child0_child0_child1.get_layout_left() as i32);
    assert_eq!(0, root_child0_child0_child1.get_layout_top() as i32);
    assert_eq!(50, root_child0_child0_child1.get_layout_width() as i32);
    assert_eq!(50, root_child0_child0_child1.get_layout_height() as i32);
}

#[test]
fn test_percent_absolute_position() {
    let mut root = Node::new();
    root.set_width(Value::Point(60.0.into()));
    root.set_height(Value::Point(50.0.into()));

    let mut root_child0 = Node::new();
    root_child0.set_flex_direction(FlexDirection::Row);
    root_child0.set_position_type(PositionType::Absolute);
    root_child0.set_position(Edge::Left, Value::Percent(50.0.into()));
    root_child0.set_width(Value::Percent(100.0.into()));
    root_child0.set_height(Value::Point(50.0.into()));
    root.insert_child(&mut root_child0, 0);

    let mut root_child0_child0 = Node::new();
    root_child0_child0.set_width(Value::Percent(100.0.into()));
    root_child0.insert_child(&mut root_child0_child0, 0);

    let mut root_child0_child1 = Node::new();
    root_child0_child1.set_width(Value::Percent(100.0.into()));
    root_child0.insert_child(&mut root_child0_child1, 1);
    root.calculate_layout(Undefined, Undefined, Direction::LTR);

    assert_eq!(0, root.get_layout_left() as i32);
    assert_eq!(0, root.get_layout_top() as i32);
    assert_eq!(60, root.get_layout_width() as i32);
    assert_eq!(50, root.get_layout_height() as i32);

    assert_eq!(30, root_child0.get_layout_left() as i32);
    assert_eq!(0, root_child0.get_layout_top() as i32);
    assert_eq!(60, root_child0.get_layout_width() as i32);
    assert_eq!(50, root_child0.get_layout_height() as i32);

    assert_eq!(0, root_child0_child0.get_layout_left() as i32);
    assert_eq!(0, root_child0_child0.get_layout_top() as i32);
    assert_eq!(60, root_child0_child0.get_layout_width() as i32);
    assert_eq!(50, root_child0_child0.get_layout_height() as i32);

    assert_eq!(60, root_child0_child1.get_layout_left() as i32);
    assert_eq!(0, root_child0_child1.get_layout_top() as i32);
    assert_eq!(60, root_child0_child1.get_layout_width() as i32);
    assert_eq!(50, root_child0_child1.get_layout_height() as i32);

    root.calculate_layout(Undefined, Undefined, Direction::RTL);

    assert_eq!(0, root.get_layout_left() as i32);
    assert_eq!(0, root.get_layout_top() as i32);
    assert_eq!(60, root.get_layout_width() as i32);
    assert_eq!(50, root.get_layout_height() as i32);

    assert_eq!(30, root_child0.get_layout_left() as i32);
    assert_eq!(0, root_child0.get_layout_top() as i32);
    assert_eq!(60, root_child0.get_layout_width() as i32);
    assert_eq!(50, root_child0.get_layout_height() as i32);

    assert_eq!(0, root_child0_child0.get_layout_left() as i32);
    assert_eq!(0, root_child0_child0.get_layout_top() as i32);
    assert_eq!(60, root_child0_child0.get_layout_width() as i32);
    assert_eq!(50, root_child0_child0.get_layout_height() as i32);

    assert_eq!(-60, root_child0_child1.get_layout_left() as i32);
    assert_eq!(0, root_child0_child1.get_layout_top() as i32);
    assert_eq!(60, root_child0_child1.get_layout_width() as i32);
    assert_eq!(50, root_child0_child1.get_layout_height() as i32);
}
