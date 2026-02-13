use reqwest::{get, Error};
use http::Response;

#[tokio::main]
async fn main() {
    let url = "https://pokeapi.co/api/v2/pokemon/ditto";
    let pokemon = "Pikachu";
    println!("{}", get_pokemon(pokemon).await.unwrap());
}


async fn get_pokemon(pokemon: &str) -> Result<String, Error> {
    let url = String::from("https://pokeapi.co/api/v2/pokemon/") + pokemon;
    let body = reqwest::get(url)
    .await?.text().await?;
    Ok(body)
}