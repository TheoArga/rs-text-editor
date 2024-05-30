mod rope;
mod editor_window;
use crate::rope::Rope;
use crate::editor_window::*;

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

    let st1 = "JOHN FORTNITE IS";
    let st2 = " NOT A REAL PERSON";
    let binding = st1.to_owned() + st2;
    let st3 = binding.as_str();

    let r1: Rope = Rope::from_text(st1, 3);
    let r2: Rope = Rope::from_text(st2, 6);
    let r3: Rope = Rope::concat(r1, r2);
    print!("Concatenated rope : {}", r3.to_text());

    print!("\n And the rope behind it : {}", r3.to_tree_print());

    let nb2: usize = 4;

    print!(
        "{}th element real : {} ; from rope : {} \n",
        nb2,
        st3.chars().nth(nb2).unwrap_or_default(),
        r3.fetch(nb2)
    );
    print!("Splitting (panic ensues ?) \n");

    let ind: usize = 2;

    let r45 = r3.split(ind);
    let r4 = r45.0;
    let r5 = r45.1;

    print!(
        "Split rope r3 (\"{}\") at {}, resumting in r4 and r5 ; \"{}\" + \"{}\" . \n ",
        r3.to_text(),
        ind,
        r4.to_text(),
        r5.to_text()
    );

    editor_window::init();


}
