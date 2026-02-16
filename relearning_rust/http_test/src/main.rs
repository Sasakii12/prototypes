use reqwest::{Error};
use serde::{Deserialize, Serialize};
use serde_json::{Value, from_str};

#[tokio::main]
async fn main() {
    let url = "https://pokeapi.co/api/v2/pokemon/ditto";
    let pokemon = "Pikachu";
    let js: Value = serde_json::from_str(get_pokemon(pokemon).await.unwrap().as_str()).unwrap();
    println!("{:?}", js);
}


async fn get_pokemon(pokemon: &str) -> Result<String, Error> {
    let url = String::from("https://pokeapi.co/api/v2/pokemon/") + pokemon;
    let body = reqwest::get(url)
    .await?.text().await?;
    Ok(body)
}