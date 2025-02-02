use std::io::{stdin, stdout, Write};

struct Commands {
    name: String,

    // ! string for now
    output: String,
}

impl Commands {
    fn new(i: &str) -> Self {
        Commands {
            name: i.to_string(),
            output: "test complete\r\n".to_string()
        }
    }

    fn spawn(self) -> std::io::Result<()> {

        match self.name.as_ref() {
            "hello" => stdout().write_all(self.output.as_bytes())?,
            _ => stdout().write_all(b"Invalid command\r\n")?,
        }

        // ! explicit return <3
        return Ok(());
    }
}

fn main() -> std::io::Result<()>{
    loop {
        print!("proompt: ");
        stdout().flush()?;

        let mut i = String::new();

        stdin().read_line(&mut i)?;

        let cmd = i.trim();

        Commands::new(cmd)
            .spawn()?;
    }
}
