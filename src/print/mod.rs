use std::fmt;

#[derive(Debug)]
struct Complex {
    real: f64,
    imag: f64,
}

impl fmt::Display for Complex {
    // 这个 trait 要求 `fmt` 带有正确的标记
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let imag_part = if self.imag >= 0.0 { format!("+ {}", self.imag) } else { format!("- {}", self.imag.abs())};
        write!(f, "{} {}i", self.real, imag_part)
    }
}

#[derive(Debug)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "RGB({red}, {green}, {blue}) {red:<#04x}{green:02x}{blue:02x}",
          red=self.red, green=self.green, blue=self.blue)
    }
}

pub fn run() {
    let complex = Complex {
        real: 2.3,
        imag: 7.2,
    };
    println!("{}", complex);
    println!("{:?}", complex);
    println!("{}", Color{ red: 0, green: 1, blue: 255});
}