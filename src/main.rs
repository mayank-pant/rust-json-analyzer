struct JsonAnalyzer<'a> {
    stack: Vec<&'a char>
}

impl JsonAnalyzer {
    fn accept_char(&self, char: &char) {
        if self.stack.is_empty() {
            if *char != '{' {
                eprint!("Not a valid json");
            } 
            self.stack.push(char);
        }
    }
}

fn main() {
    let argument = std::env::args().nth(1).unwrap_or_default();

    let json_analyzer = JsonAnalyzer {
        stack: Vec::new(),
    };

    for char in argument.chars() {
        json_analyzer.accept_char(&char);
    }
}
