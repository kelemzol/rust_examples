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

    if let dragon = Monsters::Dragon("Dragon".to_string()) { // Simpler, one line syntax to do the same thing as above
        println!("Not even close to bitbake");
    } else {
        println!("I'm running out of jokes"); //Else works with if let syntax
    }
}
