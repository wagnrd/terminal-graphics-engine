use crate::models::{Position, RenderFragment};

pub trait Renderable {
    fn render(&self, graphics_object: &GraphicsObject) -> RenderFragment;
}

pub struct GraphicsObject {
    pub position: Position,
    pub height: i32,
    pub width: i32,
    pub children: Vec<GraphicsObject>,
    pub renderable: Option<Box<dyn Renderable>>,
}

impl GraphicsObject {
    pub fn new(position: Position, width: i32, height: i32) -> GraphicsObject {
        GraphicsObject {
            position,
            height,
            width,
            children: vec![],
            renderable: Option::None,
        }
    }

    pub fn new_pointer(position: Position, width: i32, height: i32) -> Box<GraphicsObject> {
        let graphicsObject = GraphicsObject::new(position, width, height);

        Box::new(graphicsObject)
    }

    pub fn add_child(&mut self, graphics_object: GraphicsObject) {
        self.children.push(graphics_object);
    }

    pub fn set_renderable(&mut self, renderable: Box<dyn Renderable>) {
        self.renderable = Option::Some(renderable);
    }

    pub fn process(&self) -> RenderFragment {
        let mut base_render_fragment = match &self.renderable {
            Some(renderable) => renderable.render(self),
            None => RenderFragment { content: vec![vec!['\0'; self.width as usize]; self.height as usize] },
        };

        for child in &self.children {
            let render_fragment = child.process();

            // [column][row]
            for child_y in 0..child.height {
                for child_x in 0..child.width {
                    let character = render_fragment.content[child_y as usize][child_x as usize];

                    let base_x = child_x + child.position.x;
                    let base_y = child_y + child.position.y;

                    if (base_x >= 0 && base_x < self.width) && (base_y >= 0 && base_x < self.height) {
                        base_render_fragment.content[base_y as usize][base_x as usize] = character;
                    }
                }
            }
        }

        return base_render_fragment;
    }
}
