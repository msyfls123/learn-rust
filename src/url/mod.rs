extern crate url;

use self::url::{Url, ParseError};

pub fn run(test: &str) -> Result<String, ParseError> {

    let parsed = Url::parse(test)?;
    println!("The path part of the URL is: {}", parsed.path());

    Ok(parsed.as_str().to_string())
}