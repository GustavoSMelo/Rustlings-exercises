// iterators2.rs
// In this exercise, you'll learn some of the unique advantages that iterators
// can offer. Follow the steps to complete the exercise.
// Execute `rustlings hint iterators2` or use the `hint` watch subcommand for a hint.

// Step 1.
// Complete the `capitalize_first` function.
// "hello" -> "Hello"
pub fn capitalize_first(input: &str) -> String {
    let mut vec_chars = input.chars();

    if vec_chars.clone().count() > 0 {
        let mut result = String::new();
        let first_ch = vec_chars.next().unwrap();
        
        for ch in vec_chars {
            result += &ch.to_string();
        }
        result.replace_range(0..0, first_ch.to_string().to_uppercase().as_str());

        result
    } else {
        String::new()
    }

}

// Step 2.
// Apply the `capitalize_first` function to a slice of string slices.
// Return a vector of strings.
// ["hello", "world"] -> ["Hello", "World"]
pub fn capitalize_words_vector(words: &[&str]) -> Vec<String> {
    let mut support_words = words.iter();
    let mut result = Vec::new();

    for word in support_words {
        result.push(capitalize_first(word));
    }

    result
}

// Step 3.
// Apply the `capitalize_first` function again to a slice of string slices.
// Return a single string.
// ["hello", " ", "world"] -> "Hello World"
pub fn capitalize_words_string(words: &[&str]) -> String {
    let mut support_words = words.iter();
    let mut helper = Vec::new();
    let mut result = String::new();

    for word in support_words {
        if word.len() > 0 {
            helper.push(capitalize_first(word));
        }
    }

    let helper_iterator = helper.iter();

    for helper_words in helper_iterator {
        result += helper_words;
    }

    result.trim().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        assert_eq!(capitalize_first("hello"), "Hello");
    }

    #[test]
    fn test_empty() {
        assert_eq!(capitalize_first(""), "");
    }

    #[test]
    fn test_iterate_string_vec() {
        let words = vec!["hello", "world"];
        assert_eq!(capitalize_words_vector(&words), ["Hello", "World"]);
    }

    #[test]
    fn test_iterate_into_string() {
        let words = vec!["hello", " ", "world"];
        assert_eq!(capitalize_words_string(&words), "Hello World");
    }
}
