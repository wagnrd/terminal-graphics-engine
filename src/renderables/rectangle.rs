use crate::{GraphicsObject, Renderable};
use crate::models::{Position, RenderFragment};

pub struct Rectangle {}

impl Renderable for Rectangle {
    fn render(&self, graphics_object: &GraphicsObject) -> RenderFragment {
        let full_row = vec!['#'; graphics_object.height as usize];
        let mut blank_middle_piece = vec!['\0'; (graphics_object.height - 2) as usize];
        let mut middle_row = vec!['#'];
        middle_row.append(&mut blank_middle_piece.clone());
        middle_row.push('#');

        let mut rows = vec![full_row.clone()];

        for x in 1..graphics_object.height - 1 {
            rows.push(middle_row.clone());
        }

        rows.push(full_row.clone());

        RenderFragment {
            content: rows
        }
    }
}

impl Rectangle {
    pub fn new() -> Rectangle {
        Rectangle {}
    }

    pub fn new_pointer() -> Box<Rectangle> {
        Box::new(Rectangle {})
    }

    pub fn graphics_object(position: Position, width: i32, height: i32) -> GraphicsObject {
        let mut graphics_object = GraphicsObject::new(position, width, height);
        graphics_object.set_renderable(Rectangle::new_pointer());

        return graphics_object;
    }
}

