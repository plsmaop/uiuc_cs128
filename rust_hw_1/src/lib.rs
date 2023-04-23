// Fill out the functions below according to each's specification...

// [COMPLETE THIS FUNCTION - DO NOT MODIFY THE GIVEN FUNCTION SIGNATURE]
pub fn class_number_status(class_number: i32) -> String {
    // Use the given class number to return a String commenting on that course.
    // If the class number is 128, return "the best class". Otherwise, if the course
    // number is in the 100s, return "100 level". Apply this same pattern for 200, 
    // 300, and 400 level courses. If the course is in the 500s, return "grad course". 
    // For any other course number, return "invalid course number".

    return match class_number {
        128 => "the best class".to_string(),
        level @ 100..=499 => {
            let hundred_digit: i32 = level / 100;
            format!("{}00 level", hundred_digit)
        },
        500..=599 => "grad course".to_string(),
        _ => "invalid course number".to_string(),
    }
}

// [COMPLETE THIS FUNCTION - DO NOT MODIFY THE GIVEN FUNCTION SIGNATURE]
pub fn return_3_point_5_as_double() -> f64 {
    // return the value 3.5 with double precision

    3.5
}

// [COMPLETE THIS FUNCTION - DO NOT MODIFY THE GIVEN FUNCTION SIGNATURE]
pub fn return_rustacean() -> char {
    // Return the rust crab emoji as a character. HINT: https://emojipedia.org/crab/
    
    '🦀'
}

pub fn match_item(item: &str) -> String {
    enum Category {
        Electronics,
        Food,
        Animal,
        Movie,
        Person
    }

    let category: Option<Category>= match item {
        "computer" | "television" | "microwave" => Some(Category::Electronics), 
        "pizza" | "bread" | "pancake" | "banana" | "spaghetti" => Some(Category::Food),
        "panda" | "giraffe" | "elephant" | "cat" => Some(Category::Animal),
        "Welby" | "Neil" | "Eustis" | "Ferris" => Some(Category::Person),
        "Interstellar" | "Spiderman" => Some(Category::Movie),
        _ => None,
    };

    match category {
        Some(Category::Animal) => "animal".to_string(),
        Some(Category::Electronics) => "electronics".to_string(),
        Some(Category::Food) => "food".to_string(),
        Some(Category::Movie) => "movie".to_string(),
        Some(Category::Person) => "person".to_string(),
        None => "invalid item".to_string(),
    }
}

// TODO: [INSERT LAST FUNCTION HERE]
// Your code will not compile until you have completed this function.

// You can test your code with the test cases we've provided by running `cargo test`
#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn test_class_number_status() {
        // Expected responses for valid course numbers:
        assert_eq!(class_number_status(128), String::from("the best class"));
        assert_eq!(class_number_status(100), String::from("100 level"));
        assert_eq!(class_number_status(196), String::from("100 level"));
        assert_eq!(class_number_status(124), String::from("100 level"));
        assert_eq!(class_number_status(200), String::from("200 level"));
        assert_eq!(class_number_status(299), String::from("200 level"));
        assert_eq!(class_number_status(225), String::from("200 level"));
        assert_eq!(class_number_status(300), String::from("300 level"));
        assert_eq!(class_number_status(399), String::from("300 level"));
        assert_eq!(class_number_status(374), String::from("300 level"));
        assert_eq!(class_number_status(400), String::from("400 level"));
        assert_eq!(class_number_status(499), String::from("400 level"));
        assert_eq!(class_number_status(421), String::from("400 level"));
        assert_eq!(class_number_status(500), String::from("grad course"));
        assert_eq!(class_number_status(573), String::from("grad course"));
        assert_eq!(class_number_status(599), String::from("grad course"));

        // Expected responses for invalid course numbers:
        assert_eq!(class_number_status(0), String::from("invalid course number"));
        assert_eq!(class_number_status(99), String::from("invalid course number"));
        assert_eq!(class_number_status(50), String::from("invalid course number"));

        assert_eq!(class_number_status(600), String::from("invalid course number"));
        assert_eq!(class_number_status(650), String::from("invalid course number"));
        assert_eq!(class_number_status(10000), String::from("invalid course number"));

        assert_eq!(class_number_status(-1), String::from("invalid course number"));
        assert_eq!(class_number_status(-50), String::from("invalid course number"));
        assert_eq!(class_number_status(-100), String::from("invalid course number"));
    }

    #[test]
    fn test_return_3_point_5_as_double() {
        let result: f64 = return_3_point_5_as_double();
        assert_eq!(result, 3.5);
    }

    #[test]
    fn test_return_rustacean() {
        let result: i32 = return_rustacean() as i32;
        assert_eq!(result, 0x1f980);
    }

    #[test]
    fn test_match_item() {
        assert_eq!(match_item("computer"), String::from("electronics"));
        assert_eq!(match_item("television"), String::from("electronics"));
        assert_eq!(match_item("microwave"), String::from("electronics"));
        assert_eq!(match_item("banana"), String::from("food"));
        assert_eq!(match_item("pizza"), String::from("food"));
        assert_eq!(match_item("bread"), String::from("food"));
        assert_eq!(match_item("pancake"), String::from("food"));
        assert_eq!(match_item("spaghetti"), String::from("food"));
        assert_eq!(match_item("cat"), String::from("animal"));
        assert_eq!(match_item("elephant"), String::from("animal"));
        assert_eq!(match_item("giraffe"), String::from("animal"));
        assert_eq!(match_item("panda"), String::from("animal"));
        assert_eq!(match_item("Spiderman"), String::from("movie"));
        assert_eq!(match_item("Interstellar"), String::from("movie"));
        assert_eq!(match_item("Eustis"), String::from("person"));
        assert_eq!(match_item("Neil"), String::from("person"));
        assert_eq!(match_item("Welby"), String::from("person"));
        assert_eq!(match_item("Ferris"), String::from("person"));
        assert_eq!(match_item("Apple"), String::from("invalid item"));
    }
}
