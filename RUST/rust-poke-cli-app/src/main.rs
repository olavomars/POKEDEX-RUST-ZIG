use std::io::stdin;
mod api;
mod display;

#[tokio::main]
async fn main() {
    loop {
        println!("Enter Pokemon ID (1-1292) or type 'exit' to quit:");

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input.eq_ignore_ascii_case("exit") {
            println!("Exiting the program.");
            break;
        }

        if let Ok(pokemon_id) = input.parse::<i32>() {
            if (1..=1292).contains(&pokemon_id) {
                let response = api::get_pokemon(pokemon_id).await;
                display::show_pokemon_info(&response);
            } else {
                println!("Please enter a valid Pokemon ID between 1 and 1292.");
            }
        } else {
            println!("Invalid input. Please enter a valid Pokemon ID or 'exit'.");
        }
    }
}
