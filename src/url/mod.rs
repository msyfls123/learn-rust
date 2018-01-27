extern crate url;

use self::url::{Url, ParseError};

fn url_to_str(test: &str) -> Result<String, ParseError> {

    let parsed = Url::parse(test)?;
    println!("The path part of the URL is: {}", parsed.path());

    Ok(parsed.as_str().to_string())
}

pub fn run() {
    let s = "https://github.com/rust-lang/rust/issues?labels=E-easy&state=open";
    let s1 = "";
    let url = url_to_str(s);
    let url1 = url_to_str(s1);
    match url {
        Ok(s) => println!("url:{}", s),
        Err(e) => println!("{}", e),
    }
    match url1 {
        Ok(s) => println!("url:{}", s),
        Err(e) => println!("{}", e),
    }
}