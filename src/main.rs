extern crate learn;
use learn::url::run;
use learn::print;

fn main() {
    let s = "https://github.com/rust-lang/rust/issues?labels=E-easy&state=open";
    let s1 = "";
    let url = run(s);
    let url1 = run(s1);
    match url {
        Ok(s) => println!("url:{}", s),
        Err(e) => println!("{}", e),
    }
    match url1 {
        Ok(s) => println!("url:{}", s),
        Err(e) => println!("{}", e),
    }
    print::run()
}