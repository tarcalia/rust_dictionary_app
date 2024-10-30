use clap::{Arg, Command};
use reqwest::Error;

#[tokio::main]
async fn main(){
    let matches = Command::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about("Simple world definition application")
        .arg(Arg::new("word")
            .long("define")
            .short('d')
            .takes_value(true)
            .required(true)
            .help("Define a word for the word")
        )
        .get_matches();
    let word = matches.value_of("word").unwrap();
    println!("{:?}", word);
    get_word_description(word).await.expect("Error when getting word description");
}

async fn get_word_description(word: &str) -> Result<String, Error> {
    let url = format!("https://api.dictionaryapi.dev/api/v2/entries/en/{}", word);

    let response = reqwest::get(url)
        .await?
        .text()
        .await?;

    println!("Searched word is: {}\nResponse is:\n{:?}", word, response);

    Ok(response)
}
