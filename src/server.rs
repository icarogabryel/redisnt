use std::io::{self, BufRead, BufReader, BufWriter, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

use crate::command::{CommandError, CommandResult, execute, parse};
use crate::database::Database;

pub fn run(address: &str) -> io::Result<()> {
    let listener = TcpListener::bind(address)?;
    let database = Arc::new(Mutex::new(Database::new()));

    println!("Server listening on {address}");

    for stream in listener.incoming() {
        let stream = stream?;
        let database = Arc::clone(&database);

        thread::spawn(move || {
            if let Err(error) = handle_connection(stream, database) {
                eprintln!("Connection error: {error}");
            }
        });
    }

    Ok(())
}

fn handle_connection(stream: TcpStream, database: Arc<Mutex<Database>>) -> io::Result<()> {
    let reader_stream = stream.try_clone()?;

    let mut reader = BufReader::new(reader_stream);
    let mut writer = BufWriter::new(stream);

    let mut line = String::new();

    loop {
        line.clear();

        let bytes_read = reader.read_line(&mut line)?;

        if bytes_read == 0 {
            break;
        }

        let arguments: Vec<String> = line
            .trim_end_matches(['\r', '\n'])
            .split_whitespace()
            .map(String::from)
            .collect();

        let response = match parse(&arguments) {
            Ok(command) => {
                let mut database = database
                    .lock()
                    .map_err(|_| io::Error::other("Database lock poisoned"))?;

                format_result(execute(command, &mut database))
            }
            Err(error) => format_error(error),
        };

        writeln!(writer, "{response}")?;
        writer.flush()?;
    }

    Ok(())
}

fn format_result(result: CommandResult) -> String {
    match result {
        CommandResult::Pong => "PONG".to_string(),
        CommandResult::Ok => "OK".to_string(),
        CommandResult::Value(Some(value)) => value,
        CommandResult::Value(None) => "(nil)".to_string(),
        CommandResult::Deleted(deleted) => if deleted { "1" } else { "0" }.to_string(),
        CommandResult::Exists(exists) => if exists { "1" } else { "0" }.to_string(),
        CommandResult::Cleared => "OK".to_string(),
    }
}

fn format_error(error: CommandError) -> String {
    match error {
        CommandError::EmptyCommand => "ERR empty command".to_string(),
        CommandError::UnknownCommand(command) => {
            format!("ERR unknown command: {command}")
        }
        CommandError::InvalidArguments(message) => {
            format!("ERR {message}")
        }
    }
}
