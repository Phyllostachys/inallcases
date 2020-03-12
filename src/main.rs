use std::fmt::{Display, Error, Formatter};

use heck::*;

#[derive(PartialEq)]
enum CaseTy {
    Upper,
    Lower,
    Title,
    Mixed,
    Camel,
    Snake,
    ShoutySnake,
    Kebab,
    Not,
}

impl Display for CaseTy {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let name = match self {
            Self::Upper => "UPPER CASE",
            Self::Lower => "lower case",
            Self::Title => "Title Case",
            Self::Mixed => "mixedCase",
            Self::Camel => "CamelCase",
            Self::Snake => "snake_case",
            Self::ShoutySnake => "SHOUTY_SNAKE_CASE",
            Self::Kebab => "kebab-case",
            Self::Not => "",
        };
        write!(f, "{}", name)
    }
}

fn main() {
    let mut args = std::env::args();
    args.next();
    let mut input = String::new();
    for arg in args {
        input.push_str(&arg);
        input.push(' ');
    }

    if input.is_empty() {
        input.push_str("in all cases");
    }

    let case =

    if input.contains(&input.to_uppercase()) {
        CaseTy::Upper
    } else if input.contains(&input.to_lowercase()) {
        CaseTy::Lower
    } else if input.contains(&input.to_title_case()) {
        CaseTy::Title
    } else if input.contains(&input.to_mixed_case()) {
        CaseTy::Mixed
    } else if input.contains(&input.to_camel_case()) {
        CaseTy::Camel
    } else if input.contains(&input.to_snake_case()) {
        CaseTy::Snake
    } else if input.contains(&input.to_shouty_snake_case()) {
        CaseTy::ShoutySnake
    } else if input.contains(&input.to_kebab_case()) {
        CaseTy::Kebab
    } else {
        CaseTy::Not
    };

    if case == CaseTy::Not {
        println!("Orig :\t\t\t{}", input);
    } else {
        println!("Orig ({}):\t{}", case, input);
    }

    if case != CaseTy::Upper {
        println!("UPPER CASE:\t\t{}", input.to_uppercase());
    }

    if case != CaseTy::Lower {
        println!("lower case:\t\t{}", input.to_lowercase());
    }

    if case != CaseTy::Title {
        println!("Title Case:\t\t{}", input.to_title_case());
    }

    if case != CaseTy::Mixed {
        println!("mixedCase:\t\t{}", input.to_mixed_case());
    }

    if case != CaseTy::Camel {
        println!("CamelCase:\t\t{}", input.to_camel_case());
    }

    if case != CaseTy::Snake {
        println!("snake_case:\t\t{}", input.to_snake_case());
    }

    if case != CaseTy::ShoutySnake {
        println!("SHOUTY_SNAKE_CASE:\t{}", input.to_shouty_snake_case());
    }

    if case != CaseTy::Kebab {
        println!("kebab-case:\t\t{}", input.to_kebab_case());
    }
}
