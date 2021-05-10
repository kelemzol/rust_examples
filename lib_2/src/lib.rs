pub mod troll;// Lib can read in folders as well. A similarly named folder can be read, and the content of mod.rs is visible here.

pub mod lib_test {
    use crate::troll;    

    pub fn print_success() {
    println!("Lib Tested");
    troll::troll();
}
}
