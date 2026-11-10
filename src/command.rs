pub enum Command {
    Ping,
    Set { key: String, value: String },
    Get { key: String },
    Delete { key: String },
    Exists { key: String },
    Clear,
}

pub fn parse(args: &[String]) -> Result<Command, String> {
    let Some(command) = args.first() else {
        return Err("No command provided".to_string());
    };

    match command.to_uppercase().as_str() {
        "PING" => {
            if args.len() == 1 {
                Ok(Command::Ping)
            } else {
                Err("PING command requires no arguments".to_string())
            }
        }
        "SET" if args.len() == 3 => Ok(Command::Set {
            key: args[1].clone(),
            value: args[2].clone(),
        }),
        "GET" if args.len() == 2 => Ok(Command::Get {
            key: args[1].clone(),
        }),
        "DELETE" if args.len() == 2 => Ok(Command::Delete {
            key: args[1].clone(),
        }),
        "EXISTS" if args.len() == 2 => Ok(Command::Exists {
            key: args[1].clone(),
        }),
        "CLEAR" if args.len() == 1 => Ok(Command::Clear),
        _ => Err(format!("Invalid command or arguments: {}", command)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_set() {
        let args = vec!["SET".to_string(), "name".to_string(), "john".to_string()];

        let command = parse(&args).unwrap();

        assert!(matches!(
            command,
            Command::Set { key, value }
                if key == "name" && value == "john"
        ));
    }
    #[test]
    fn test_parse_get() {
        let args = vec!["GET".to_string(), "name".to_string()];

        let command = parse(&args).unwrap();

        assert!(matches!(
            command,
            Command::Get { key }
                if key == "name"
        ));
    }
    #[test]
    fn test_parse_delete() {
        let args = vec!["DELETE".to_string(), "name".to_string()];

        let command = parse(&args).unwrap();

        assert!(matches!(
            command,
            Command::Delete { key }
                if key == "name"
        ));
    }

    #[test]
    fn test_parse_exists() {
        let args = vec!["EXISTS".to_string(), "name".to_string()];

        let command = parse(&args).unwrap();

        assert!(matches!(
            command,
            Command::Exists { key }
                if key == "name"
        ));
    }

    #[test]
    fn test_parse_clear() {
        let args = vec!["CLEAR".to_string()];

        let command = parse(&args).unwrap();

        assert!(matches!(command, Command::Clear));
    }

    #[test]
    fn test_parse_invalid_command() {
        let args = vec!["UNKNOWN".to_string()];

        assert!(parse(&args).is_err());
    }
}
