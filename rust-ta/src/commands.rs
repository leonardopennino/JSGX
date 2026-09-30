use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Clone, Eq, PartialEq, Deserialize)]
pub enum Command {
    Authenticate,
    AuthenticatedMethod(String, Value),
    Method(String, Value),
}
#[derive(Debug, Clone)]
pub enum ParsingError {
    InvalidJson,
    UnexpectedToken,
    MissingToken,
}

impl TryFrom<&str> for Command {
    type Error = ParsingError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut splits = value.split_whitespace();
        if let Some(elem) = splits.next() {
            match elem {
                "authenticate" => {
                    if splits.next().is_none() {
                        Ok(Command::Authenticate)
                    } else {
                        Err(ParsingError::InvalidJson)
                    }
                }
                "authmethod" => {
                    let method = splits.next().ok_or(ParsingError::MissingToken)?;
                    let values = splits.fold(String::new(), |a, b| format!("{a}{b}"));
                    let json =
                        serde_json::from_str(&values).map_err(|_| ParsingError::InvalidJson)?;
                    Ok(Command::AuthenticatedMethod(method.to_string(), json))
                }
                "method" => {
                    let method = splits.next().ok_or(ParsingError::MissingToken)?;
                    let values = splits.fold(String::new(), |a, b| format!("{a}{b}"));
                    let json =
                        serde_json::from_str(&values).map_err(|_| ParsingError::InvalidJson)?;
                    Ok(Command::Method(method.to_string(), json))
                }
                _ => Err(ParsingError::UnexpectedToken),
            }
        } else {
            Err(ParsingError::MissingToken)
        }
    }
}

pub fn parse_commands(unparsed_commands: &Vec<String>) -> Result<Vec<Command>, ParsingError> {
    let mut commands: Vec<Command> = vec![];
    for str in unparsed_commands {
        let command: Command = str.as_str().try_into()?;
        commands.push(command);
    }
    Ok(commands)
}

#[cfg(test)]
mod tests {
    use super::{parse_commands, Command};
    #[test]
    fn parser_works() {
        let json = r###"{"j" : "asd"}"###;
        let expected = vec![
            Command::Authenticate,
            Command::Method(
                "testendpoint".to_string(),
                serde_json::from_str(&json).unwrap(),
            ),
            Command::AuthenticatedMethod(
                "testauthendpoint".to_string(),
                serde_json::from_str(&json).unwrap(),
            ),
        ];
        let tokens = vec![
            "authenticate".into(),
            format!("method testendpoint {json}"),
            format!("authmethod testauthendpoint {json}"),
        ];
        assert_eq!(parse_commands(&tokens).unwrap(), expected);
    }
}
