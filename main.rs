use std::{io::{stdout, Write,Stdout}, thread, time};
use crossterm::{
    queue,
    ExecutableCommand, QueueableCommand,
    terminal, cursor, style::{self, Stylize, Color}, Result
};

use std::env;
use rand::Rng;

enum BranchType {Trunk, ShootLeft, ShootRight, Dying, Dead}

struct Config<'a>{
	live: u16,
	infinite: u16 ,
	screensaver: u16,
	print_tree: u16,
	verbosity: u16,
	life_start: u16,
	multiplier: u16,
	base_type: u16,
	seed: u16,
	leaves_size: u16,
	target_branch_count: u16,

	time_wait: f64,
	time_step: f64,

	message:&'a str,
	leaves:&'a str,
}

struct Counters {
    branches: u16,
    shoots: u16,
    shoot_counter: u16,   
}

fn roll(modular: i32) -> i32 {
    rand::thread_rng().gen::<i32>() % modular
}


fn set_deltas(branch_type: BranchType, life: u16, age: u16, multiplier: u16) -> (i32,i32) {
    let mut dx: i32 = 0;
    let mut dy: i32 = 0;
    let mut dice: i32;
    //enum BranchType {Trunk, ShootLeft, ShootRight, Dying, Dead}
    match branch_type {

      BranchType::Trunk => {
        if age <= 2 || life < 4 {
            dy = 0;
            dx = roll(2);
        } else if age < (multiplier * 3) {

            if (age % (multiplier / 2)) == 0 {dy = -1} 
                else {dy = 0} 
            
            dice = roll(10);
            if dice == 0 {dx = -2}
                else if dice >= 1 && dice <= 3 {dx = -1}
                else if dice >= 4 && dice <= 5 {dx = 0}
                else if dice >= 6 && dice <= 8 {dx = 1}
                else if dice >= 9 && dice <= 9 {dx = 2}
        } else {
            dice = roll(10);
            if dice > 2 {dy = -1}
                else {dy = 0}
            //i think rand(c) and roll(rust) have the same functionality 
            //if shit breaks (in the trunk) oopsie look here first
            dx = roll(3) - 1;
        }
      },
      BranchType::ShootLeft => {
        dice = roll(0);
		if dice >= 0 && dice <= 1 {dy = -1}
            else if dice >= 2 && dice <= 7 {dy = 0}
            else if dice >= 8 && dice <= 9 {dy = 1}

		dice = roll(10);
		if dice >= 0 && dice <=1 {dx = -2}
            else if dice >= 2 && dice <= 5 {dx = -1}
            else if dice >= 6 && dice <= 8 {dx = 0}
            else if dice == 9 {dx = 1}
      },
      BranchType::ShootRight => {
        dice = roll(10);
		if dice >= 0 && dice <= 1 {dy = -1}
            else if dice >= 2 && dice <= 7 {dy = 0}
            else if dice >= 8 && dice <= 9 {dy = 1}

		dice = roll(10);
		if dice >= 0 && dice <=1 {dx = 2}
            else if dice >= 2 && dice <= 5 {dx = 1}
            else if dice >= 6 && dice <= 8 {dx = 0}
            else if dice == 9 {dx = -1}
      },
      BranchType::Dying => {
        dice = roll( 10);
		if dice >= 0 && dice <=1 {dy = -1}
            else if dice >= 2 && dice <= 8 {dy = 0}
            else if dice == 9 {dy = 1}

        dice = roll( 15);
        if dice >= 0 && dice <=0 {dx = -3}
            else if dice >= 1 && dice <= 2 {dx = -2}
            else if dice >= 3 && dice <= 5 {dx = -1}
            else if dice >= 6 && dice <= 8 {dx = 0}
            else if dice >= 9 && dice <= 11 {dx = 1}
            else if dice >= 12 && dice <= 13 {dx = 2}
            else if dice == 14 {dx = 3}
      },
      BranchType::Dead => {
        dice = roll(10);
		if dice >= 0 && dice <= 2 {dy = -1}
		else if dice >= 3 && dice <= 6 {dy = 0}
		else if dice >= 7 && dice <= 9 {dy = 1}
        //might break or not, i think its like c's rand just reusing function for fun
		dx = roll(3) - 1;

      }
    }
    (dx,dy)
}

fn branch(config: &Config<'_>, mut stdout: Stdout, mut counters:Counters, mut y: u16, mut x: u16,branch_type: BranchType ,mut life: u16) {
    counters.branches+=1;
    let mut dx: u16 = 0;
    let mut dy: u16 = 0;
    let age: u16 = 0;
    let shoot_counter = config.multiplier;

    while life > 0 {
      life -= 1;
      let age = config.life_start - life;
      
      set_deltas(BranchType::Trunk, life, age, config.multiplier);

      let max_y = 25;
      if dy > 0 && y  > (max_y - 2) {dy -= 1}
      
      queue!(stdout, cursor::MoveTo(x,y)).unwrap();
      queue!(stdout, style::Print("%")).unwrap();

      x += dx;
	    y += dy;

    }

}

fn grow_tree(config: &Config, mut stdout: Stdout, mut counters: Counters) {
    let (max_y, max_x): (u16, u16) = (50, 25);
    let life_start: u16 = config.life_start;
    
    counters.shoots = 0;
    counters.branches = 0;

    //counters.shoot_counter = 
    stdout.flush().unwrap();
    branch(config, stdout, counters, max_y - 1, max_x / 2,BranchType::Trunk, life_start);
}   

fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(args);
    
    let config = Config{
        live: 0,
        infinite: 0 ,
        screensaver: 0,
        print_tree: 0,
        verbosity: 0,
        life_start: 32,
        multiplier: 5,
        base_type: 1,
        seed: 0,
        leaves_size: 0,
        target_branch_count: 0,

        time_wait: 4.0,
        time_step: 0.03,

        message: " ",
        leaves: " ",
    };    

    {
    let mut stdout = stdout();
    stdout.execute(terminal::Clear(terminal::ClearType::All)).unwrap();
    stdout.execute(cursor::Hide).unwrap();
    }
    loop {
    
        let mut stdout = stdout();
        
      
        let counters = Counters {
            branches: 0,
            shoots: 0,
            shoot_counter: 0,
        };

        queue!(stdout, cursor::MoveTo(20,20)).unwrap();
        queue!(stdout, style::Print("hi")).unwrap();
      
        grow_tree(&config, stdout, counters)
    }

}

//day1 0-95
//day2 95-100

//Notes
//use queue! macro with crossterm

//mvwprintw format
//mvwprintw(Window, Line, Column, Format, [Argument ...])
