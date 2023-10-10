pub mod ansi;
pub mod commands;
pub mod env;
pub mod prompts;

#[cfg(test)]
mod tests {
    use crate::commands;

    #[test]
    fn run_command() {
        let inputs: Vec<&str> = vec!["", "foo", "ls"];
        let results: Vec<bool> = vec![false, false, true];
        for i in 0..inputs.len() {
            let result: Result<(), &str> = commands::run(inputs[i]);
            assert_eq!(result.is_ok(), results[i]);
        }
    }

    #[test]
    fn run_command_silent() {
        let inputs: Vec<&str> = vec!["", "foo", "ls"];
        let results: Vec<bool> = vec![false, false, true];
        for i in 0..inputs.len() {
            let result: Result<(), &str> = commands::run_silent(inputs[i]);
            assert_eq!(result.is_ok(), results[i]);
        }
    }

    #[test]
    fn run_command_output() {
        let inputs: Vec<&str> = vec!["", "foo", "ls"];
        let results: Vec<bool> = vec![false, false, true];
        for i in 0..inputs.len() {
            let result: Result<String, &str> = commands::output(inputs[i]);
            assert_eq!(result.is_ok(), results[i]);
        }
    }
}
