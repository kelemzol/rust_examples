mod monster; // Use mod where you want to use another file. No need to use mod in those files. 
mod princess; // Cargo reads up mods based on file names. Princess.rs is read in as a mod here, from src.

use lib::lib_test;

fn main() {
    

    monster::monster();
    princess::princess();
    lib_test::print_success();
}