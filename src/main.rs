use rust_cli::prompts;

fn main() {
    let result = test_prompt();
    if result.is_err() {
        panic!("test_prompt failed");
    }

    let result = test_select();
    if result.is_err() {
        panic!("test_select failed");
    }
}

fn test_prompt() -> Result<(), std::io::Error> {
    println!("---------- prompt::text ----------");
    dbg!(prompts::prompt::text("Text:")?);

    println!("---------- prompt::secret ----------");
    dbg!(prompts::prompt::secret("Secret:")?);

    return Ok(());
}

fn test_select() -> Result<(), std::io::Error> {
    println!("---------- select::select ----------");
    let select = prompts::select::Select::new()
        .title("Select Value")
        .options(&vec![
            "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten",
        ])
        .option("eleven")
        .option("twelve")
        .details(&vec!["first", "", "third", "fourth"]);

    let select = select.max_rows_per_page(10);
    dbg!(select.prompt_for_value()?);

    let select = select.max_rows_per_page(5);
    dbg!(select.prompt_for_index()?);

    let select = select.allow_multi_select(true);
    dbg!(select.prompt_for_values()?);

    return Ok(());
}
