use std::{io::{stdout, Write,Stdout}, thread, time};
use crossterm::{
    ExecutableCommand, QueueableCommand,
    terminal, cursor, style::{self, Stylize, Color}, Result
};

struct Point {
    x: u16,
    y: u16,
}

fn main() -> Result<()> {
    let CSL_SIZE_X = 50;
    let CSL_SIZE_Y = 25;

    let mut ball = Point{x:10, y:10};
    let mut old_ball_pos = Point{x:10, y:10};

  
    let mut stdout = stdout();
    
    stdout.execute(terminal::Clear(terminal::ClearType::All))?;
    stdout.execute(cursor::Hide)?;
    
    draw_base(stdout,CSL_SIZE_Y,1);
  
    Ok(())
}

fn draw_base(mut stdout: Stdout,size_y: u16, base_type: u8) -> Result<()> {
  stdout
        .queue(cursor::MoveTo(0,size_y-4))?
        .queue(style::PrintStyledContent(":".with(Color::Rgb { r: (50), g: (50), b: (50) })))?
        .queue(style::PrintStyledContent("___________".with(Color::Rgb { r: (100), g: (200), b: (0) })))?
        .queue(style::PrintStyledContent("./~~~\\.".with(Color::Rgb { r: (100), g: (0), b: (0) })))?
        .queue(style::PrintStyledContent("___________".with(Color::Rgb { r: (100), g: (200), b: (0) })))?
        .queue(style::PrintStyledContent("\n \\                           / \n  \\_________________________/ \n  (_)                     (_)".with(Color::Rgb { r: (50), g: (50), b: (50) })))?;
  Ok(())
}
