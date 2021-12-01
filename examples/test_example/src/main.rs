use terminal_graphics_engine::GraphicsObject;
use terminal_graphics_engine::models::Position;
use terminal_graphics_engine::renderables::{Point, Rectangle};
use terminal_graphics_engine::renderers::terminal_renderer::run_terminal_renderer;

fn main() {
    let mut screen = Rectangle::graphics_object(
        Position { x: 0, y: 0 },
        10,
        11,
    );
    let mut point1 = Point::graphics_object(Position { x: 2, y: 2 });
    let mut point2 = Point::graphics_object(Position { x: 3, y: 3 });
    let mut point3 = Point::graphics_object(Position { x: 4, y: 4 });
    let mut point4 = Point::graphics_object(Position { x: 5, y: 5 });
    let mut point5 = Point::graphics_object(Position { x: 4, y: 6 });
    let mut point6 = Point::graphics_object(Position { x: 3, y: 7 });
    let mut point7 = Point::graphics_object(Position { x: 2, y: 8 });

    screen.add_child(point1);
    screen.add_child(point2);
    screen.add_child(point3);
    screen.add_child(point4);
    screen.add_child(point5);
    screen.add_child(point6);
    screen.add_child(point7);

    run_terminal_renderer(screen);
}
