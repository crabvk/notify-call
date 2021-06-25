use std::process::Command;
use std::{env, fmt, io, process, result};

#[derive(Debug)]
pub enum ParseActionError {
    InvalidFormat(String),
}

impl fmt::Display for ParseActionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseActionError::InvalidFormat(s) => {
                write!(f, "Invalid action syntax in \"{}\". Use COMMAND:LABEL.", s)
            }
        }
    }
}

impl std::error::Error for ParseActionError {}

#[derive(Debug)]
pub struct Action {
    command: String,
    pub label: String,
}

impl<'a> Action {
    pub fn from_str(s: &'a str) -> result::Result<Self, ParseActionError> {
        let cl: Vec<&'a str> = s.rsplitn(2, ':').collect();

        if cl.len() == 2 && cl.iter().all(|i| !i.trim().is_empty()) {
            Ok(Self {
                command: cl[1].to_string(),
                label: cl[0].to_string(),
            })
        } else {
            Err(ParseActionError::InvalidFormat(String::from(s)))
        }
    }

    pub fn default_from_str(cmd: &'a str) -> Self {
        Self {
            command: cmd.to_string(),
            label: String::new(),
        }
    }

    #[allow(unused)]
    pub fn invoke(&self) -> io::Result<process::Child> {
        let shell = env::var("SHELL").unwrap_or(String::from("sh"));
        Command::new(shell)
            .arg("-c")
            .arg(self.command.as_str())
            .spawn()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;
    use std::{env, error, fs};

    #[test]
    fn parsing_error_invalid_format() {
        let error = Action::from_str("test1").err().unwrap();
        let expect = "Invalid action syntax in \"test1\". Use COMMAND:LABEL.";
        assert_eq!(expect, format!("{}", error));
    }

    #[test]
    fn parse_and_invoke_echo() -> Result<(), Box<dyn error::Error>> {
        let tmpfile = env::temp_dir().join(Path::new("notify-call-test"));
        let tmpfile = tmpfile.to_str().unwrap();
        let cmd = format!("echo this:is_test > {}", tmpfile);
        let action = Action::from_str(format!("{}:check", cmd).as_str())?;

        assert_eq!("check", action.label);
        assert!(action.invoke()?.wait().is_ok());

        assert_eq!("this:is_test\n", fs::read_to_string(&tmpfile)?);
        assert!(fs::remove_file(tmpfile).is_ok());

        Ok(())
    }
}
