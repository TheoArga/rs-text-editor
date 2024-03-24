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
        text.chars().nth(nb - 1).unwrap_or_default(),
        my_first_rope.fetch(nb)
    );

    let st1 = "JOHN FORTNITE IS";
    let st2 = "NOT A REAL PERSON";
    let binding = (st1.to_owned() + st2);
    let st3 = binding.as_str();

    let r1: Rope = Rope::from_text(st1, 3);
    let r2: Rope = Rope::from_text(st2, 6);
    let r3: Rope = Rope::concat(r1, r2);
    print!("Concatenated rope : {}", r3.to_text());

    print!("\n And the rope behind it : {}", r3.to_tree_print());

    let nb2: usize = 1;

    print!(
        "{}th element real : {} ; from rope : {} \n",
        nb2,
        st3.chars().nth(nb2 - 1).unwrap_or_default(),
        r3.fetch(nb2)
    );
}
