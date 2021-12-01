pub struct RenderFragment {
    pub content: Vec<Vec<char>>,
}

impl RenderFragment {
    pub fn to_string(&self) -> String {
        let mut result = String::new();

        for row in &self.content {
            for mut character in row {
                if *character == '\0' {
                    result.push(' ')
                } else {
                    result.push(*character)
                }
            }

            result.push('\n');
        }

        return result;
    }
}