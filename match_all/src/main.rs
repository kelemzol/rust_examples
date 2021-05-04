enum Monsters{
  Goblin(String),
  Dragon(String),
  Imp(String),
  Drow(String),
  Vampire(String),
  Halfing(String),
  Troll(String),
  Beholder(String),
  Bitbake(String)
}

fn main() {
    let bitbake = Monsters::Bitbake("Bitbake".to_string());
    match bitbake {
        Monsters::Bitbake(_) => println!("This is too terrifying"),
        _ => println!("We can deal with this") //Match all syntax. Should always be the last in a match loop
    }
}
