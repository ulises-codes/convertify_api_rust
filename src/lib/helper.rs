pub fn format_key(key: &str) -> String {
    key.to_lowercase().replace(" ", "_")
}

pub fn format_first_char(word: &str) -> String {
    let first_letter = word.chars().nth(0).unwrap().to_string().to_uppercase();

    let slice = word[1..].to_string();

    format!("{first_letter}{slice}")
}

pub fn remove_underscores(key: &str) -> String {
    print!("{key}");

    key.split("_")
        .map(|word| format_first_char(word))
        .collect::<Vec<String>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn replaces_underscores() {
        assert_eq!("Square Feet", remove_underscores("square_feet"));
        assert_eq!("Square Mile", remove_underscores("square_mile"));
        assert_eq!("Multiply By", remove_underscores("multiply_by"));
    }

    #[test]
    fn formats_first_char() {
        assert_eq!("Sloppy", format_first_char("sloppy"))
    }
}
