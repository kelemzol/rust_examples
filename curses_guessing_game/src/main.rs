extern crate rand;
extern crate pancurses;

use rand::Rng;
use std::cmp::Ordering;
use std::string::String;
use std::vec::Vec;

use pancurses::{initscr, endwin, Input, noecho, Window};

fn get_number(window: &Window, line: &mut String) -> bool {
    loop {
        match window.getch() {
            Some(Input::Character(c)) => {
                if c == '\n' {
                    break;
                }
                match std::char::from_u32(0x1b) {
                    Some(esc) => {
                        if c == esc {
                            return true;
                        }
                    },
                    None => continue,
                }
                if c.is_digit(10) {
                    window.addch(c);
                    line.push(c);
                }
            },
            Some(input) => { window.addstr(&format!("{:?}", input)); },
            None => ()
        }
    }
    return false;
}

fn main() {
    let window = initscr();
    window.keypad(true);
    noecho();
    let secret_number = rand::thread_rng().gen_range(0,101);
    let mut list: Vec<u32> = Vec::new();
    list.push(secret_number);
    
    loop {

        for element in list.iter() {
            if *element == secret_number {
                window.printw("  [ SECRET ]\n");
            } else {
                window.printw(&format!("- {}\n", element));
            }
        }

        let mut guess = String::new();
        window.printw("Type a number (ESC to give up): ");
        window.refresh();
        if get_number(&window, &mut guess) {
            break;
        }
        
        window.printw(&format!("\nYour guess: {}\n", guess));
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(err) => {
                window.printw(&format!("Error: {}\n", err));
                continue;
            }
        };

        list.push(guess);
        list.sort();
  
        match guess.cmp(&secret_number) {
            Ordering::Equal => {
                window.printw("You win\n");
                break;
            },
            Ordering::Greater => window.printw("Too big\n"), // not visible
            Ordering::Less => window.printw("Too small\n"),  // not visible
        };
        window.clear();
        window.refresh();
    }


    window.printw(&format!("Secret Number: {}\n", secret_number));
    window.printw("[Put any button to quit]");
    window.refresh();
    window.getch();
    endwin();

}