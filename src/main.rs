fn main() {
    use terminal_menu::{button, label, menu, mut_menu, run};
    let menu = menu(vec![
        label("----------------------"),
        label("Please select a random option"),
        label("this will generate a more random answer for you"),
        label("-----------------------"),
        button("Colors"),
        button("Animals"),
        button("Cities"),
    ]);
    run(&menu);

    // you can get the selected buttons name like so:
    let binding = mut_menu(&menu);
    let option = binding.selected_item_name();

    println!("Selected: {}", option);
}
