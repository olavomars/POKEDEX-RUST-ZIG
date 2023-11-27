use std::io::stdin;


const API_BASE_URL: &str = "https://pokeapi.co/api/v2/pokemon";

#[derive(serde::Deserialize, Debug)]
struct Pokemon {
    id: i32,
    name: String,
    r#types: Vec<PokemonType>

}

#[derive(serde::Deserialize, Debug)]
struct PokemonType {
    r#type: PokemonTypeStr
}

#[derive(serde::Deserialize, Debug)]
struct PokemonTypeStr {
    name: String,    
}

#[tokio::main]
async fn main() {
    println!("pokemon id:");
    
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();

    let pokemon_id: i32 = input.trim().parse().unwrap();

     let response: Pokemon = reqwest::get(format!("{API_BASE_URL}/{pokemon_id}"))
        .await.unwrap()
        .json()
        .await.unwrap();
    println!("{:#?}", response);



}   

// response.types.iter().map(|poke_type| {
//         let name = &poke_type.r#type.name;
//         print!("{}", name);