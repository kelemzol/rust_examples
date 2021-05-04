mod Heroes {
    fn fight() {
        println!("Argh!"); // callable with Heroes::fight();
    }
    mod Heroics{ // Mods can go inside mods
        fn win(){
            println!("I win"); // callable at Heroes::Heroics::win();
        }
    }
}

mod Monsters {
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
