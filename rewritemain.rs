use std::{io::{stdout, Write,Stdout}, thread, time};
use crossterm::{
    queue,
    ExecutableCommand, QueueableCommand,
    terminal, cursor, style::{self, Stylize, Color}, Result
};

use std::env;
use rand::Rng;

enum BranchType {Trunk, ShootLeft, ShootRight, Dying, Dead}

struct CanvasSize {
    x: i32,
    y: i32,
}

fn mod_hash(k:i32, m:i32) -> i32 {
    k*k%m
}

fn set_deltas(branch_type: BranchType, life: i32, age: i32, multiplier: i32, life_length:i32) -> (i32,i32) {
    let mut dx: i32 = 0;
    let mut dy: i32 = 0;
    //enum BranchType {Trunk, ShootLeft, ShootRight, Dying, Dead}
    match branch_type {

      BranchType::Trunk => {
        let mod_hash = mod_hash(life.try_into().unwrap(),multiplier.try_into().unwrap());
        
        if mod_hash < 50 {dx = -1}
        if mod_hash > 50 && mod_hash > 10 {dx = 1}
        if mod_hash > multiplier - multiplier/10 {dy = -1}

      },
      BranchType::ShootLeft => {
 
      },
      BranchType::ShootRight => {

      },
      BranchType::Dying => {

      },
      BranchType::Dead => {
      }
    }
    (dx,dy)
}


fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(args);

    let mut stdout = stdout();
    stdout.execute(terminal::Clear(terminal::ClearType::All)).unwrap();
    stdout.execute(cursor::Hide).unwrap();

    let canvas_size:CanvasSize = CanvasSize { x: {60}, y: (30) };

    let (mut dx, mut dy): (i32, i32);
    let (mut x, mut y): (i32, i32) = (canvas_size.x/2, canvas_size.y-4);

    let life_length = 98;
    let mut current_life = life_length;
    while current_life > 0 {
        if current_life < life_length && current_life > life_length - life_length/4 {
            (dx, dy) = set_deltas(BranchType::Trunk, current_life, life_length-current_life, 100,life_length);
        } else {
            (dx,dy)= (0,0)
        }
        x += dx; y += dy;
        current_life -= 1;

        queue!(stdout, cursor::MoveTo(x.try_into().unwrap(),y.try_into().unwrap())).unwrap();
        queue!(stdout, style::Print("/")).unwrap();
        
        stdout.flush().unwrap();
        thread::sleep(time::Duration::from_millis(50));
    }
}
