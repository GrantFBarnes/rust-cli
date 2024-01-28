pub mod ansi;
pub mod commands;
pub mod prompts;

#[cfg(test)]
mod tests {
    use crate::commands;

    #[test]
    fn run_command() {
        let inputs: Vec<&str> = vec!["", "foo", "ls"];
        let results: Vec<bool> = vec![false, false, true];
        for i in 0..inputs.len() {
            assert_eq!(
                commands::Operation::new(inputs[i])
                    .current_dir("/")
                    .run()
                    .is_ok(),
                results[i]
            );
        }
    }
}
