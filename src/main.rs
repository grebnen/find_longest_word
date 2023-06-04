use std::*;

fn main() {
    let prompt = &"This program will find the longest word in a sentence.\n\nEnter a sentence now to begin!\n";
    let response = get_user_input(prompt);

    if response.chars().count() == 0 {
        println!("No input was entered by the user")
    }
    else {
        println!("The longest word in the sentence is: \'{}\'", find_longest_word(response));
    }
}

fn get_user_input (message: &'_ impl fmt::Display) -> String {
    println!("{}", message);
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).expect("Failed to read from stdin");
    user_input
}

fn find_longest_word(message: String) -> String {
    let mut longest = "";

    for word in message.split_whitespace(){
        if word.chars().count() > longest.chars().count(){
            longest = word;
        }
    }

    longest.to_string()
}

#[cfg(test)]
mod tests {
    // this brings everything from parent's scope into this scope
    use super::*;

    #[test]
    fn test_find_longest_word() {
        let mut test_string = "I can't believe the Celtics choked.";
        let mut test_result = find_longest_word(test_string.to_string());
        assert_eq!(test_result, "believe");

        test_string = "Code forever.";
        test_result = find_longest_word(test_string.to_string());
        assert_eq!(test_result, "forever.");

        //added an assert_ne! to show that the result of test is not finding the wrong value
        test_string = "Boss used TrenGPT for the last challenge!";
        test_result = find_longest_word(test_string.to_string());
        assert_ne!(test_result, "TrenGPT");

    }
}
