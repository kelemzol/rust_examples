pub mod princess;

mod heroes {
    use crate::princess;
    
    fn fight() {
        println!("Argh!"); // callable with Heroes::fight();
        princess::save_me();
    }
    mod heroics{ // Mods can go inside mods
        fn win(){
            println!("I win"); // callable at Heroes::Heroics::win();
        }
    }
}

mod monsters {
    fn fight() { // each mod is a separate namespace
        println!("Argh!"); // callable with Monsters::fight();
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
