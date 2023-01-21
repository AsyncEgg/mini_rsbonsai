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
    {
    let mut stdout = stdout();
    
    stdout.execute(terminal::Clear(terminal::ClearType::All))?;
    stdout.execute(cursor::Hide)?;
    
    draw_base(stdout,CSL_SIZE_Y,CSL_SIZE_X,1);
    }
    let mut stdout = stdout();
    stdout
      .queue(cursor::MoveTo(CSL_SIZE_X, CSL_SIZE_Y))?
      .queue(style::PrintStyledContent("@@".with(Color::Rgb { r: (255), g: (255), b: (255) })))?;
  Ok(())
}

fn draw_base(mut stdout: Stdout,screen_max_y: u16, screen_max_x: u16, base_type: u8) -> Result<()> {
  //calculate where to draw base X
  let base_pos_x = screen_max_x / 1;
  
  match base_type {
    1 => stdout
          .queue(cursor::MoveTo(2,screen_max_y-4))?
          .queue(style::PrintStyledContent(":".with(Color::Rgb { r: (50), g: (50), b: (50) })))?
          .queue(style::PrintStyledContent("___________".with(Color::Rgb { r: (100), g: (200), b: (0) })))?
          .queue(style::PrintStyledContent("./~~~\\.".with(Color::Rgb { r: (100), g: (0), b: (0) })))?
          .queue(style::PrintStyledContent("___________".with(Color::Rgb { r: (100), g: (200), b: (0) })))?
          .queue(style::PrintStyledContent(":".with(Color::Rgb { r: (50), g: (50), b: (50) })))?
          .queue(cursor::MoveTo(2,screen_max_y-3))?                                                     
          .queue(style::PrintStyledContent(" \\                           / \n  \\_________________________/ \n  (_)                     (_)"
              .with(Color::Rgb { r: (50), g: (50), b: (50) })))?, 
    2 => stdout.
      queue(cursor::MoveTo(base_pos_x,screen_max_y-4))?
      .queue(style::PrintStyledContent(":".with(Color::Rgb { r: (50), g: (50), b: (50) })))?
      .queue(style::PrintStyledContent("___________".with(Color::Rgb { r: (100), g: (200), b: (0) })))?
      .queue(style::PrintStyledContent("./~~~\\.".with(Color::Rgb { r: (100), g: (0), b: (0) })))?
      .queue(style::PrintStyledContent("___________".with(Color::Rgb { r: (100), g: (200), b: (0) })))?
      .queue(style::PrintStyledContent(":".with(Color::Rgb { r: (50), g: (50), b: (50) })))?
      .queue(style::PrintStyledContent(" (           ) \n  (_________)  "
          .with(Color::Rgb { r: (50), g: (50), b: (50) })))?, 
          
    _ => stdout.queue(cursor::MoveTo(0,0))?,
  };
  Ok(())
}

//fn draw_message(mut stdout: Stdout, message: &str)
//Links
//https://gitlab.com/jallbrit/cbonsai/-/blob/master/cbonsai.c
//https://www.cyberciti.biz/media/new/cms/2021/01/Linux-Desktop-Fun-Bonsai-tree-generator-for-CLI-lovers.jpg
