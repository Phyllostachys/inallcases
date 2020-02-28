use heck::*;

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

    println!("Orig:\t\t\t{}", input);
    println!("CamelCase:\t\t{}", input.to_camel_case());
    println!("snake_case:\t\t{}", input.to_snake_case());
    println!("kebab-case:\t\t{}", input.to_kebab_case());
    println!("SHOUTY_SNAKE_CASE:\t{}", input.to_shouty_snake_case());
    println!("mixedCase:\t\t{}", input.to_mixed_case());
    println!("Title Case:\t\t{}", input.to_title_case());
}
