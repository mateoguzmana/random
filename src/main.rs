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

    let binding = mut_menu(&menu);
    let option = binding.selected_item_name();

    println!("Selected: {}", option);

    if option == "Colors" {
        let colors = ["Green", "Yellow", "Red"];

        println!("Colors: {:?}", colors);
    }

    if option == "Animals" {
        let animals = ["Cat", "Dog", "Bird"];

        println!("Animals: {:?}", animals);
    }

    if option == "Cities" {
        let cities = ["Medellín", "Amsterdam", "Athens"];

        println!("Cities: {:?}", cities);
    }

    let random_index = rand::random::<u8>();

    println!("Random index: {}", random_index);
}
