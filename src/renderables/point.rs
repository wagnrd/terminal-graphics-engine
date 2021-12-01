use crate::{GraphicsObject, Renderable};
use crate::models::{Position, RenderFragment};

pub struct Point {}

impl Renderable for Point {
    fn render(&self, graphics_object: &GraphicsObject) -> RenderFragment {
        RenderFragment {
            content: vec![vec!['#']]
        }
    }
}

impl Point {
    pub fn new() -> Point {
        Point {}
    }

    pub fn new_pointer() -> Box<Point> {
        Box::new(Point {})
    }

    pub fn graphics_object(position: Position) -> GraphicsObject {
        let mut graphics_object = GraphicsObject::new(position, 1, 1);
        graphics_object.set_renderable(Point::new_pointer());

        return graphics_object;
    }
}

