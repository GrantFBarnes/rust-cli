use rust_cli::prompts;

fn main() {
    println!("---------- prompt::text ----------");
    let result: Result<String, std::io::Error> = prompts::prompt::text("Text:");
    if result.is_ok() {
        dbg!(result.unwrap());
    } else {
        println!("Text prompt failed.");
    }

    println!("---------- prompt::secret ----------");
    let result: Result<String, std::io::Error> = prompts::prompt::secret("Secret:");
    if result.is_ok() {
        dbg!(result.unwrap());
    } else {
        println!("Secret prompt failed.");
    }

    println!("---------- select::select ----------");
    let result: Result<String, std::io::Error> = prompts::select::select(
        "Select",
        vec![
            "one".to_string(),
            "two".to_string(),
            "three".to_string(),
            "four".to_string(),
            "five".to_string(),
            "six".to_string(),
            "seven".to_string(),
            "eight".to_string(),
            "nine".to_string(),
            "ten".to_string(),
            "eleven".to_string(),
            "twelve".to_string(),
        ],
        vec![
            "first".to_string(),
            String::new(),
            "third".to_string(),
            "fourth".to_string(),
        ],
    );
    if result.is_ok() {
        dbg!(result.unwrap());
    } else {
        println!("Select prompt failed.");
    }

    println!("---------- select::mutli_select ----------");
    let result: Result<Vec<String>, std::io::Error> = prompts::select::mutli_select(
        "Multi Select",
        vec![
            "one".to_string(),
            "two".to_string(),
            "three".to_string(),
            "four".to_string(),
            "five".to_string(),
            "six".to_string(),
            "seven".to_string(),
            "eight".to_string(),
            "nine".to_string(),
            "ten".to_string(),
            "eleven".to_string(),
            "twelve".to_string(),
        ],
        vec![
            "first".to_string(),
            String::new(),
            "third".to_string(),
            "fourth".to_string(),
        ],
    );
    if result.is_ok() {
        dbg!(result.unwrap());
    } else {
        println!("Multi Select prompt failed.");
    }
}
