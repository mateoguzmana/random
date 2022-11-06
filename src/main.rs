mod logo;

fn main() {
    use rand::Rng;
    use terminal_menu::{button, label, menu, mut_menu, run};

    logo::print_logo();

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

    let random_index = rand::thread_rng().gen_range(0..2);

    if option == "Colors" {
        let colors = ["Green", "Yellow", "Red"];

        println!("Random color: {}", colors[random_index]);
    }

    if option == "Animals" {
        let animals = ["Cat", "Dog", "Bird"];

        println!("Random animal: {}", animals[random_index]);
    }

    if option == "Cities" {
        let cities = ["Medellín", "Amsterdam", "Athens"];

        println!("Random city: {}", cities[random_index]);
    }
}
