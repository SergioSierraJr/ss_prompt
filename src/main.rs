use std::{env, io::Write};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

fn main() {
    let binding = env::current_dir().expect("Couldnt get PWD");
    let pwd = match binding.to_str() {
        Some(x) => x,
        None => return,
    };

    let binding = homedir::my_home().expect("Couldnt get homedir");
    let home = match binding {
        Some(x) => x,
        None => return,
    };

    let home = match home.to_str() {
        Some(x) => x,
        None => return,
    };

    write_color("[", Color::Red);
    if pwd.starts_with(home) {
        write_color(&pwd.replace(home, "~"), Color::Magenta);
    } else {
        write_color(pwd, Color::Magenta)
    }
    write_color("]", Color::Red);
    write_color(" ⮞ ", Color::White);
}

fn write_color(text: &str, color: Color) {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    stdout.set_color(ColorSpec::new().set_fg(Some(color))).expect("Couldnt print red");
    write!(&mut stdout, "{}", text).expect("Couldnt print text");
}
