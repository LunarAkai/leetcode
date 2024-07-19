use std::ops::Deref;

fn main() {
    println!("{}", match_entity("&amp;"));  
    println!("{}", match_entity("&quot;"));  
    println!("{}", match_entity("&test;"));  
}

fn match_entity(word: &str) -> &str {
    match word {
        "&amp;" => return "&",
        "&quot;" => return "\"",
        "&apos;" => return "'",
        "&gt;" => return ">",
        _ => word
    }
}

fn entity_parser(text: String) -> String {
    println!("{:?}", text.as_str().split(" ").collect::<Vec<&str>>());
    let mut vec: Vec<&str> = Vec::new();
    let mut new = "";
    for mut word in text.as_str().split(" ").collect::<Vec<&str>>().iter() {
        new = &match_entity(word);
        vec.push(new);
    }
    println!("new: {:?}", vec);
    vec.join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        let sentence: String = String::from("&amp; is an HTML entity but &ambassador; is not.");
        assert_eq!(entity_parser(sentence), String::from("& is an HTML entity but &ambassador; is not."));
    }

    #[test]
    fn test_two() {
        let sentence: String = String::from("and I quote: &quot;...&quot;");
        assert_eq!(entity_parser(sentence), String::from("and I quote: \"...\""));
    }
}