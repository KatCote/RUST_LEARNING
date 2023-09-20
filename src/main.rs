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
        SetForegroundColor(Color::Black),
        SetBackgroundColor(Color::Red),
        Print(" Powered by "),
        ResetColor
    )?;

    // or using functions
    stdout()
        .execute(SetForegroundColor(Color::Red))?
        .execute(SetBackgroundColor(Color::Black))?
        .execute(Print(" CastleCore "))?
        .execute(ResetColor)?;    

    Ok(())
}
