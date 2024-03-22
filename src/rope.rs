pub struct Rope {
    left: Option<Box<Rope>>,
    right: Option<Box<Rope>>,
    text: Option<String>,
    weight: usize,
}

impl Rope {
    pub fn new() -> Rope {
        Rope {
            left: None,
            right: None,
            text: None,
            weight: 0,
        }
    }

    pub fn from_str(string: &str) -> Rope {
        Rope {
            left: None,
            right: None,
            text: Some(String::from(string)),
            weight: string.len(),
        }
    }

    pub fn from_text(string: &str, max_length: usize) -> Rope {
        if string.len() <= max_length {
            return Rope::from_str(string);
        } else {
            let strings: (&str, &str) = string.split_at(string.len() / 2);

            Rope {
                left: Some(Box::new(Rope::from_text(strings.0, max_length))),
                right: Some(Box::new(Rope::from_text(strings.1, max_length))),
                text: None,
                weight: strings.0.len(),
            }
        }
    }

    pub fn to_text(&self) -> String {
        if let Some(txt) = &self.text {
            return txt.clone();
        }

        return match (&self.left, &self.right) {
            (Some(lr), Some(rr)) => lr.to_text() + &rr.to_text(),
            (Some(lr), None) => lr.to_text(),
            (None, Some(rr)) => rr.to_text(),
            (None, None) => String::new(),
        };
    }

    pub fn to_node_print(&self) -> String {
        let size: String = self.weight.to_string();
        let text: String;
        if let Some(txt) = &self.text {
            text = String::from("leaf node with text = \"") + &txt.replace("\n", "\\n") + "\"";
        } else {
            text = String::from("branch node no text");
        }

        return (text + " : ") + size.as_str() + "\n";
    }

    pub fn to_tree_print(&self) -> String {
        return self._to_tree_print(0);
    }

    fn _to_tree_print(&self, level: usize) -> String {
        let tree = String::new();

        return match (&self.left, &self.right) {
            (Some(lr), Some(rr)) => {
                tree + &" | ".repeat(level)
                    + &self.to_node_print()
                    + &lr._to_tree_print(level + 1)
                    + &rr._to_tree_print(level + 1)
            }
            (Some(lr), None) => {
                tree + &" | ".repeat(level) + &self.to_node_print() + &lr._to_tree_print(level + 1)
            }
            (None, Some(rr)) => {
                tree + &" | ".repeat(level) + &self.to_node_print() + &rr._to_tree_print(level + 1)
            }
            (None, None) => tree + &" | ".repeat(level) + " └─" + &self.to_node_print(),
        };
    }

    pub fn fetch(&self, ith: usize) -> char {
        print!("fetching {} in node {}", ith, self.to_node_print());

        return match (&self.left, &self.right, &self.text, ith > self.weight) {
            (_, Some(right), _, true) => right.fetch(ith - self.weight),
            (Some(left), _, _, _) => left.fetch(ith),
            (None, None, Some(text), _) => text.chars().nth(ith).unwrap_or('F'),
            (_, _, _, _) => 'F',
        };
    }
}