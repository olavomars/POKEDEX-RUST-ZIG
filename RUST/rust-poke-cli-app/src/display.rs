use crate::api::Pokemon;

pub fn show_pokemon_info(pokemon: &Pokemon) {
    println!("Pokemon ID: {}", pokemon.id);
    println!("Name: {}", pokemon.name);
    println!("Height: {} m", pokemon.height);
    println!("Weight: {} kg", pokemon.weight);
    println!("Types:");
    for poke_type in &pokemon.r#types {
        let name = &poke_type.r#type.name;
        println!("  - {}", name);
    }
    println!("Stats:");
    for stat in &pokemon.stats {
        println!("  {}: {}", stat.stat.name, stat.base_stat);
    }
}


