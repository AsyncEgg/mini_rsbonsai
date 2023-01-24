use std::{io::{stdout, Write,Stdout}, thread, time};
use crossterm::{
    queue,
    ExecutableCommand, QueueableCommand,
    terminal, cursor, style::{self, Stylize, Color}, Result
};

use std::env;

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

fn branch(config: &Config<'_>, stdout: Stdout, mut counters:Counters, max_y: u16, max_x: u16, branch_type: BranchType::Trunk ,mut life: u16) {
    counters.branches+=1;
    let dx: u16 = 0;
    let dy: u16 = 0;
    let age: u16 = 0;
    let shoot_counter = config.multiplier;

    while life > 0 {
      life -= 1;
      let age = config.life_start - life;

      //set_deltas()
      
    }

}

fn grow_tree(config: &Config, stdout: Stdout, mut counters: Counters) {
    let (max_y, max_x): (u16, u16) = (50, 25);
    let life_start: u16 = config.life_start;

    branch(config, stdout, counters, max_y, max_x life_start);
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
    
    loop {
    
        let mut stdout = stdout();

        let counters = Counters {
            branches: 0,
            shoots: 0,
            shoot_counter: 0,
        };

        grow_tree(&config, stdout, counters)
    }

}


//Notes
//use queue! macro with crossterm

//mvwprintw format
//mvwprintw(Window, Line, Column, Format, [Argument ...])
