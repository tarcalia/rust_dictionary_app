use clap::{Arg, Command};
use reqwest::Error;
use serde_json::Value;

#[tokio::main]
async fn main(){
    let param_word = "word";

    let matches = Command::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about("Simple word definition application")
        .arg(Arg::new(param_word)
            .long("define")
            .short('d')
            .takes_value(true)
            .required(true)
            .help("Define a word for the word")
        )
        .get_matches();

    let search_word = matches.value_of(param_word).unwrap();

    println!("Searching for word: {}", search_word);
    get_word_description(search_word)
        .await.expect("Error when getting word description");
}

async fn get_word_description(word: &str) -> Result<String, Error> {
    let url = format!("https://api.dictionaryapi.dev/api/v2/entries/en/{}", word);

    let response = reqwest::get(url)
        .await?
        .text()
        .await?;

    let json: Value = match serde_json::from_str(&response) {
        Ok(val) => val,
        Err(_) => {
            println!("The response is not a valid JSON:\n{}", response);
            return Ok(response);
        }
    };

    println!("Definition of {}:\n{}",
             word, serde_json::to_string_pretty(&json).unwrap());

    Ok(response)
}
