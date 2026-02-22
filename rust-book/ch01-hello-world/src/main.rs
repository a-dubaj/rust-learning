fn greeting() -> String {
    String::from("Hello, world!")
}

fn main() {
    println!("{}", greeting());
}

#[cfg(test)]
mod tests;