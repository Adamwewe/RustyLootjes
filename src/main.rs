// pub use lootjes::lib::foo;
use std::{io, os::windows::thread};
use rand::{seq::IteratorRandom, thread_rng, seq::SliceRandom};

mod lib;
use lib::Players;



fn pull(players: &Vec<String>) -> Vec<&String> {
    let mut rng = thread_rng();
    players.iter().choose_multiple(&mut rng, 1)
}



fn main() {

    let mut n = String::new();

    println!("Please enter the number of players");

    io::stdin()
        .read_line(&mut n)
        .expect("failed to read from stdin");
    let n = n.trim().parse::<i32>().expect("invalid input");

    let mut players = Players{number : n,
            names: vec![]};
    
    while players.names.len() < players.number.try_into().unwrap() {

        let mut name = String::new();

        io::stdin()
            .read_line(&mut name)
            .expect("failed to read from stdin");

        let len =name.len() - 2; // shitty hacky way to ommit last two chars ie: "\n"
        players.names.push(name[0..len].to_string()); 
        
    }   

    println!("{:?}", players.names);

    let mut rng = thread_rng();

    players.names.shuffle(&mut rng);
    

    println!("{:?}", &players.names);

}
