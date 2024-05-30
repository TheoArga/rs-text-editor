use std::sync::Arc;

#[derive(Clone)]
pub struct Rope {
    left: Option<Arc<Rope>>,
    right: Option<Arc<Rope>>,
    text: Option<String>,
    weight: usize,
}
/**
 * Characters are 0-indexed for all operations
 */
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
                left: Some(Arc::new(Rope::from_text(strings.0, max_length))),
                right: Some(Arc::new(Rope::from_text(strings.1, max_length))),
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

        return (text + " : ") + &size + "\n";
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

        return match (&self.left, &self.right, &self.text, ith >= self.weight) {
            (_, Some(right), _, true) => right.fetch(ith - self.weight),
            (Some(left), _, _, _) => left.fetch(ith),
            (None, None, Some(text), _) => text.chars().nth(ith).unwrap_or('F'),
            (_, _, _, _) => 'F',
        };
    }

    pub fn weight(&self) -> usize {
        return match (&self.left, &self.text) {
            (Some(left), _) => left._weight(),
            (_, Some(text)) => text.len(),
            (_, _) => 0,
        };
    }

    fn _weight(&self) -> usize {
        return match (&self.left, &self.right, &self.text) {
            (Some(left), Some(right), _) => left._weight() + right._weight(),
            (Some(left), _, _) => left._weight(),
            (_, _, Some(text)) => text.len(),
            (_, _, _) => 0,
        };
    }

    pub fn concat(r1: Rope, r2: Rope) -> Rope {
        Rope {
            weight: r1._weight(),
            left: Some(Arc::new(r1)),
            right: Some(Arc::new(r2)),
            text: None,
        }
    }

    pub fn split_node(&self, split_at: usize) -> (Rope, Rope) {
        assert!(<Option<String> as Clone>::clone(&self.text).is_some());
        assert!(<Option<String> as Clone>::clone(&self.text).unwrap().len() >= split_at);

        let t0: String = <Option<String> as Clone>::clone(&self.text).unwrap();
        let t_12: (_, _) = t0.split_at(split_at);
        let t1: String = t_12.0.to_owned();
        let t2: String = t_12.1.to_owned();
        let w1 = t1.len();
        let w2 = t2.len();

        (
            Rope {
                left: None,
                right: None,
                text: Some(t1),
                weight: w1,
            },
            Rope {
                left: None,
                right: None,
                text: Some(t2),
                weight: w2,
            },
        )
    }

    pub fn split(&self, index: usize) -> (Rope, Rope) {
        if index < self.weight() {
            if let (Some(left), Some(right), true) = (&self.left, &self.right, index < self.weight)
            {
                let split = left.split(index);
                let w = split.1._weight();
                return (
                    split.0,
                    Rope {
                        left: Some(Arc::new(split.1)),
                        right: Some(Arc::new(Rope {
                            left: right.left.clone(),
                            right: right.right.clone(),
                            text: right.text.clone(),
                            weight: right.weight.clone(),
                        })),
                        text: None,
                        weight: w,
                    },
                );
            }
        } else if index > self.weight() {
            if let (Some(left), Some(right), true) = (&self.left, &self.right, index < self.weight)
            {
                let split = right.split(index - self.weight);
                let w = split.0.weight;
                return (
                    Rope {
                        left: Some(left.clone()),
                        right: Some(Arc::new(split.0)),
                        text: None,
                        weight: w,
                    },
                    split.1,
                );
            }
        }

        let nl: Rope;
        let nr: Rope;

        match (&self.left, &self.right) {
            (Some(_left), Some(_right)) => {
                nl = (*self.left.clone().unwrap()).clone();
                nr = (*self.right.clone().unwrap()).clone();
            }
            (_, _) => {
                let s = self.split_node(index);
                nl = s.0;
                nr = s.1;
            }
        }

        return (nl, nr);
    }
}
