/*
2696. Minimum String Length After Removing Substrings

    Remove substrings "AB" and "CD"

    # Example 1
    Input s = "ABFCACDB"
    Output = 2
    ---------------
    1. Step => [AB]FCACDB   => FCACDB
    2. Step => FCA[CD]B     => FCAB
    3. Step => FC[AB]       => FC
    => Length 2

    # Example 2
    Input s = "ACBBD"
    Ouput = 5
    => no valid substring found
*/

fn main() {
    let example_one: String = String::from("ABFCACDB");
    let one = min_length(example_one);
    println!("{}", one);

    let two = min_length(String::from("ACBBD"));
    println!("{}", two);
}

fn min_length(s: String) -> i32 {
    let mut new_string = s;
    while new_string.contains("AB") || new_string.contains("CD") {
        new_string = remove_specified_substrings(new_string);
    }
    new_string.len().try_into().unwrap()
}

fn remove_specified_substrings(mut s: String) -> String {
    s = s.replace("AB", "");
    s = s.replace("CD", "");

    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_replace_one() {
        let example_one: String = String::from("ABFCACDB");
        assert_eq!(remove_specified_substrings(example_one), "FCAB");
    }
}
