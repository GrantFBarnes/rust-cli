use rust_cli::prompts::*;

fn main() {
    let result = test_confirm();
    if result.is_err() {
        dbg!(result.err().unwrap());
    }

    let result = test_prompt();
    if result.is_err() {
        dbg!(result.err().unwrap());
    }

    let result = test_select();
    if result.is_err() {
        dbg!(result.err().unwrap());
    }
}

fn test_confirm() -> Result<(), std::io::Error> {
    println!("---------- confirm ----------");
    let confirm = Confirm::new().message("Confirm");
    dbg!(confirm.confirm()?);

    let confirm = confirm.default_no(true);
    dbg!(confirm.confirm()?);

    return Ok(());
}

fn test_prompt() -> Result<(), std::io::Error> {
    println!("---------- prompt ----------");
    let prompt = Text::new().message("Prompt:");
    dbg!(prompt.prompt()?);

    let prompt = prompt.secret(true).confirm(true).required(true);
    dbg!(prompt.prompt()?);

    return Ok(());
}

fn test_select() -> Result<(), std::io::Error> {
    println!("---------- select ----------");
    let select = Select::new()
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

    dbg!(select.prompt_for_values()?);

    return Ok(());
}
