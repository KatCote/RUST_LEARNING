use std::io::{stdout, Write};

use crossterm::{
    execute,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    ExecutableCommand, event,
};

fn main() -> std::io::Result<()> {
    // using the macro
    execute!(
        stdout(),
        SetForegroundColor(Color::White),
        SetBackgroundColor(Color::Red),
        Print(" KatCote "),
        ResetColor
    )?;

    // or using functions
    stdout()
        .execute(SetForegroundColor(Color::White))?
        .execute(SetBackgroundColor(Color::Blue))?
        .execute(Print(" AltenCøre "))?
        .execute(ResetColor)?;    

    Ok(())
}
