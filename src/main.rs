use std::{env, io::Write};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

fn main() {
    // decided to make write_color a closure so I dont have to keep recomputing stdout or keep passing it as a value
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    let mut write_color = |text: &str, color: Color| {
        stdout.set_color(ColorSpec::new().set_fg(Some(color))).expect("Couldnt print red");
        write!(stdout, "{}", text).expect("Couldnt print text");
    };

    // this code probably isnt in very good practice but its shorter 
    let pwd = &format!("{}",env::current_dir().unwrap().display());
    let home = &format!("{}",homedir::my_home().unwrap().unwrap().display());
    
    write_color("[", Color::Red);
    if pwd.starts_with(home) {
        write_color(&pwd.replace(home, "~"), Color::Magenta);
    } else {
        write_color(pwd, Color::Magenta)
    }
    write_color("]", Color::Red);
    write_color(" ⮞ ", Color::White);
}