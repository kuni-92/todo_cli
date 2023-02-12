use std::io;
use std::io::{BufRead, Write};

static PROMPT_TEXT: &str = "cmd> ";

fn get_command(reader: &mut dyn BufRead, writer: &mut dyn Write) -> String {
    writer.write(PROMPT_TEXT.as_bytes())
        .expect("Error: Couldn't write prompt.");
    writer.flush()
        .expect("Error: Couldn't flush prompt.");

    let mut cmd = String::new();
    let _read_size = reader.read_line(&mut cmd)
        .expect("Error: Couldn't read command.");

    return cmd.trim_end().to_string();
}

fn main() {
    let mut stdin = io::stdin().lock();
    let mut stdout = io::stdout().lock();

    loop {
        let cmd = get_command(&mut stdin, &mut stdout);
        println!("command: {}", cmd);
    }
}