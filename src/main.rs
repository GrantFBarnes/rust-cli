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

    println!("---------- select::select_value ----------");
    let result: Result<String, std::io::Error> = prompts::select::select_value(
        "Select Value",
        &vec![
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
        &vec![
            "first".to_string(),
            String::new(),
            "third".to_string(),
            "fourth".to_string(),
        ],
        Some(5),
    );
    if result.is_ok() {
        dbg!(result.unwrap());
    } else {
        println!("Select value prompt failed.");
    }

    println!("---------- select::select_index ----------");
    let result: Result<usize, std::io::Error> = prompts::select::select_index(
        "Select Index",
        &vec![
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
        &vec![
            "first".to_string(),
            String::new(),
            "third".to_string(),
            "fourth".to_string(),
        ],
        Some(6),
    );
    if result.is_ok() {
        dbg!(result.unwrap());
    } else {
        println!("Select index prompt failed.");
    }

    println!("---------- select::mutli_select_values ----------");
    let result: Result<Vec<String>, std::io::Error> = prompts::select::mutli_select_values(
        "Multi Select Values",
        &vec![
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
        &vec![
            "first".to_string(),
            String::new(),
            "third".to_string(),
            "fourth".to_string(),
        ],
        Some(8),
    );
    if result.is_ok() {
        dbg!(result.unwrap());
    } else {
        println!("Multi Select values prompt failed.");
    }

    println!("---------- select::mutli_select_indexes ----------");
    let result: Result<Vec<usize>, std::io::Error> = prompts::select::mutli_select_indexes(
        "Multi Select Indexes",
        &vec![
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
        &vec![
            "first".to_string(),
            String::new(),
            "third".to_string(),
            "fourth".to_string(),
        ],
        Some(10),
    );
    if result.is_ok() {
        dbg!(result.unwrap());
    } else {
        println!("Multi Select indexes prompt failed.");
    }
}
