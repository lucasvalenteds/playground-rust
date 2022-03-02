#[cfg(test)]
mod tests {

    #[test]
    fn looping_vector() {
        let words = vec!["Hello", "World", ":)"];
        let mut amount_of_words = 0;

        for _ in words.clone() {
            amount_of_words += 1;
        }

        assert_eq!(words.len(), amount_of_words)
    }

    #[test]
    fn looping_empty_vector() {
        let vector: Vec<String> = vec![];
        let mut amount_of_items = 0;

        for _ in vector.clone() {
            amount_of_items += 1;
        }

        assert_eq!(0, amount_of_items)
    }
}
