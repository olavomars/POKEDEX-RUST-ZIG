use serde::Deserialize;

const API_BASE_URL: &str = "https://pokeapi.co/api/v2/pokemon";

#[derive(Deserialize, Debug)]
pub struct Pokemon {
    pub id: i32,
    pub name: String,
    pub height: i32,
    pub weight: i32,
    pub r#types: Vec<PokemonType>,
    pub stats: Vec<PokemonStat>,
}

#[derive(Deserialize, Debug)]
pub struct PokemonType {
    pub r#type: PokemonTypeStr,
}

#[derive(Deserialize, Debug)]
pub struct PokemonTypeStr {
    pub name: String,
}

#[derive(Deserialize, Debug)]
pub struct PokemonStat {
    pub base_stat: i32,
    pub stat: PokemonStatInfo,
}

#[derive(Deserialize, Debug)]
pub struct PokemonStatInfo {
    pub name: String,
}

pub async fn get_pokemon(pokemon_id: i32) -> Pokemon {
    reqwest::get(format!("{}/{}", API_BASE_URL, pokemon_id))
        .await.unwrap()
        .json()
        .await.unwrap()
}
