mod rope;
use crate::rope::Rope;

fn main() {
    let text: String = String::from("The quick brown fox jumps over the lazy dog\n");

    let my_first_rope: Rope = Rope::from_text(&text, 5);

    let get_that_text_back: String = my_first_rope.to_text();

    print!("text gotten back = {}", get_that_text_back);

    print!(" \n tree structure : \n{}", my_first_rope.to_tree_print());

    let nb: usize = 10;

    print!(
        "{}th element real : {} ; from rope : {} \n",
        nb,
        text.chars().nth(nb).unwrap_or_default(),
        my_first_rope.fetch(nb)
    );
}
