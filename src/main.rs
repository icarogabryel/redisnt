
mod command;
mod database;
mod server;

fn main() {
    if let Err(error) = server::run("127.0.0.1:6379") {
        eprintln!("Server error: {error}");
    }
}
