use std::io;

use crate::graphics_object::GraphicsObject;

pub fn run_terminal_renderer(root: GraphicsObject) {
    loop {
        let render_fragment = root.process();
        println!("{}", render_fragment.to_string());

        let std_in = io::stdin();
        let mut user_input = String::new();
        std_in.read_line(&mut user_input);
    }
}
