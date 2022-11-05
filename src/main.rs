use std::io;
use terminal_menu::{list, menu, mut_menu, run};

fn main() {
    ask_for_random_type();
}

fn ask_for_random_type() {
    let menu = menu(vec![list(
        "Please choose one of the following",
        vec!["Cats", "Dogs", "Guapas"],
    )]);

    run(&menu);

    println!(
        "My Lists Value: {}",
        mut_menu(&menu).selection_value("My Lists Name")
    );

    println!("How many do you want to generate?");

    let mut generate_option = String::new();

    io::stdin()
        .read_line(&mut generate_option)
        .expect("Failed to read line");

    println!("You generated: {generate_option}");
}
