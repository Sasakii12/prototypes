use std::io;
use clap::Parser;

use reqwest::{Error};
use serde::{Deserialize, Serialize};
use serde_json::{Value, from_str};

fn input() -> String {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf)
    .expect("Error");

    buf
}

async fn get_pokemon(pokemon: &str) -> Result<String, Error> {
    let url = String::from("https://pokeapi.co/api/v2/pokemon/") + pokemon;
    let body = reqwest::get(url)
    .await?.text().await?;
    Ok(body)
}

#[tokio::main]
async fn main() {
    // let url = "https://pokeapi.co/api/v2/pokemon/ditto";
    // let pokemon = "Pikachu";
    // let js: Value = serde_json::from_str(get_pokemon(pokemon).await.unwrap().as_str()).unwrap();
    // println!("{:?}", js);
    // println!("Please enter a pokemon name");
    // let name = input();
    // let poke_str = get_pokemon(&name).await.unwrap();
    // let poke_json: Value = serde_json::from_str(&poke_str).unwrap();
    // let prim_type = &poke_json["types"][0]["type"]["name"];
    // println!("Your pokemon is an {} type", prim_type);
}

