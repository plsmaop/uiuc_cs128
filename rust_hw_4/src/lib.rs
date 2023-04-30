/// TODO: Implement this function that passes a mutable Vector of Strings
/// You must iterate through the vector. 
/// If the word has an even number of characters, reverse the word by using the reverse function.
/// If the word has an odd number of characters, count the number of vowels in the word.
/// You should return a tuple containg a Vector of the reversed words 
/// and the total number of vowels from odd lengthed words.
pub fn vector_iter(words: &mut Vec<String>) -> (Vec<String>, i32) {
    words.iter_mut().map(|word| {
        match word.len() & 1 {
            0 => {
                reverse_word(word);
                (Some(word.clone()), 0)
            },
            _ => (None, count_vowels(word)),
        }
    }).fold((Vec::new(), 0), |(mut acc_reversed, acc_count), (reversed_word, count)| {
        match reversed_word {
            Some(reversed) => {
                acc_reversed.push(reversed);
                (acc_reversed, acc_count)
            },
            None => (acc_reversed, acc_count + count)
        }
    })
}

/// TODO: Implement this function that takes a String and reverses it.
/// This should directly modify the contents of the passed String.
pub fn reverse_word(word: &mut String) {
    let mut reversed = String::with_capacity(word.len());
    for c in word.chars().rev() {
        reversed.push(c);
    }

    *word = reversed;
}

/// TODO: Implement this function that takes a String and counts the number of vowels in it.
/// In this instance, we mean the vowels a, e, i, o, and u. Do not worry about y.
/// You will need to handle words with upper and lower case vowels.
/// You should not modify the contents of the passed String.
/// You should return the vowel count.
pub fn count_vowels(word: &str) -> i32 {
    word.chars().filter(|c| "aeiouAEIOU".contains(*c)).count() as i32
}


// You can test your code with the test cases we've provided by running `cargo test`
// We also encourage you to add more assert statements to test out your code further!
#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn test_vector_iter() {
        let words: &mut Vec<String> = &mut vec!["fish".to_string(), "cat".to_string(), "ardvaark".to_string(), "dog".to_string(), "bird".to_string(), "ant".to_string()];
        let (reversed_words, count) = vector_iter(words);
        assert_eq!(reversed_words, vec!["hsif".to_string(), "kraavdra".to_string() ,"drib".to_string()]);
        assert_eq!(count, 3);
    }
}
