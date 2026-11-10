
use crate::database::Database;

#[derive(Debug, PartialEq)]
pub enum Command {
    Ping,
    Set { key: String, value: String },
    Get { key: String },
    Delete { key: String },
    Exists { key: String },
    Clear,
}

#[derive(Debug, PartialEq)]
pub enum CommandError {
    EmptyCommand,
    UnknownCommand(String),
    InvalidArguments(String),
}

#[derive(Debug, PartialEq)]
pub enum CommandResult {
    Pong,
    Ok,
    Value(Option<String>),
    Deleted(bool),
    Exists(bool),
    Cleared,
}

pub fn parse(args: &[String]) -> Result<Command, CommandError> {
    let Some(command) = args.first() else {
        return Err(CommandError::EmptyCommand);
    };

    let command = command.to_uppercase();

    match command.as_str() {
        "PING" if args.len() == 1 => Ok(Command::Ping),

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

        "PING" | "SET" | "GET" | "DELETE" | "EXISTS" | "CLEAR" => {
            Err(CommandError::InvalidArguments(command))
        }

        _ => Err(CommandError::UnknownCommand(command)),
    }
}

pub fn execute(command: Command, db: &mut Database) -> CommandResult {
    match command {
        Command::Ping => CommandResult::Pong,

        Command::Set { key, value } => {
            db.set(&key, &value);
            CommandResult::Ok
        }

        Command::Get { key } => {
            CommandResult::Value(db.get(&key).map(String::from))
        }

        Command::Delete { key } => {
            CommandResult::Deleted(db.delete(&key))
        }

        Command::Exists { key } => {
            CommandResult::Exists(db.exists(&key))
        }

        Command::Clear => {
            db.clear();
            CommandResult::Cleared
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_set() {
        let args = vec!["SET".into(), "name".into(), "john".into()];

        assert_eq!(
            parse(&args),
            Ok(Command::Set {
                key: "name".into(),
                value: "john".into(),
            })
        );
    }

    #[test]
    fn test_parse_get() {
        let args = vec!["GET".into(), "name".into()];

        assert_eq!(
            parse(&args),
            Ok(Command::Get { key: "name".into() })
        );
    }

    #[test]
    fn test_parse_delete() {
        let args = vec!["DELETE".into(), "name".into()];

        assert_eq!(
            parse(&args),
            Ok(Command::Delete { key: "name".into() })
        );
    }

    #[test]
    fn test_parse_exists() {
        let args = vec!["EXISTS".into(), "name".into()];

        assert_eq!(
            parse(&args),
            Ok(Command::Exists { key: "name".into() })
        );
    }

    #[test]
    fn test_parse_clear() {
        let args = vec!["CLEAR".into()];

        assert_eq!(parse(&args), Ok(Command::Clear));
    }

    #[test]
    fn test_parse_ping() {
        let args = vec!["PING".into()];

        assert_eq!(parse(&args), Ok(Command::Ping));
    }

    #[test]
    fn test_parse_empty_command() {
        let args: Vec<String> = vec![];

        assert_eq!(parse(&args), Err(CommandError::EmptyCommand));
    }

    #[test]
    fn test_parse_unknown_command() {
        let args = vec!["UNKNOWN".into()];

        assert_eq!(
            parse(&args),
            Err(CommandError::UnknownCommand("UNKNOWN".into()))
        );
    }

    #[test]
    fn test_parse_invalid_arguments() {
        let args = vec!["SET".into(), "name".into()];

        assert_eq!(
            parse(&args),
            Err(CommandError::InvalidArguments("SET".into()))
        );
    }

    #[test]
    fn test_parse_case_insensitive() {
        let args = vec!["set".into(), "name".into(), "john".into()];

        assert_eq!(
            parse(&args),
            Ok(Command::Set {
                key: "name".into(),
                value: "john".into(),
            })
        );
    }

    #[test]
    fn test_execute_set_and_get() {
        let mut db = Database::new();

        let set = Command::Set {
            key: "name".into(),
            value: "john".into(),
        };

        assert_eq!(execute(set, &mut db), CommandResult::Ok);

        let get = Command::Get { key: "name".into() };

        assert_eq!(
            execute(get, &mut db),
            CommandResult::Value(Some("john".into()))
        );
    }

    #[test]
    fn test_execute_delete() {
        let mut db = Database::new();

        db.set("name", "john");

        let delete = Command::Delete { key: "name".into() };

        assert_eq!(
            execute(delete, &mut db),
            CommandResult::Deleted(true)
        );

        assert_eq!(
            execute(Command::Get { key: "name".into() }, &mut db),
            CommandResult::Value(None)
        );
    }

    #[test]
    fn test_execute_delete_nonexistent() {
        let mut db = Database::new();

        let delete = Command::Delete {
            key: "missing".into(),
        };

        assert_eq!(
            execute(delete, &mut db),
            CommandResult::Deleted(false)
        );
    }

    #[test]
    fn test_execute_exists() {
        let mut db = Database::new();

        db.set("name", "john");

        assert_eq!(
            execute(Command::Exists { key: "name".into() }, &mut db),
            CommandResult::Exists(true)
        );

        assert_eq!(
            execute(Command::Exists { key: "missing".into() }, &mut db),
            CommandResult::Exists(false)
        );
    }

    #[test]
    fn test_execute_clear() {
        let mut db = Database::new();

        db.set("name", "john");

        assert_eq!(
            execute(Command::Clear, &mut db),
            CommandResult::Cleared
        );

        assert_eq!(
            execute(Command::Get { key: "name".into() }, &mut db),
            CommandResult::Value(None)
        );
    }

    #[test]
    fn test_execute_ping() {
        let mut db = Database::new();

        assert_eq!(
            execute(Command::Ping, &mut db),
            CommandResult::Pong
        );
    }

    #[test]
    fn test_execute_set_update() {
        let mut db = Database::new();

        execute(
            Command::Set {
                key: "name".into(),
                value: "john".into(),
            },
            &mut db,
        );

        execute(
            Command::Set {
                key: "name".into(),
                value: "mary".into(),
            },
            &mut db,
        );

        assert_eq!(
            execute(Command::Get { key: "name".into() }, &mut db),
            CommandResult::Value(Some("mary".into()))
        );
    }
}
