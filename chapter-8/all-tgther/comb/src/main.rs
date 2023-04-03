use std::collections::HashMap;
use std::io;
use comb::spell_workbench;

#[derive(Debug)]
struct User {

    name: String,
    coins: i32,
    
}

fn main() {

    /* Who is walking in? */
    println!("Hello, what is your name?");
    let mut traveler = String::new();

    // read_line returns a "Result" enum 
    // with variants 'Ok' and 'Err'
    io::stdin()
    .read_line(&mut traveler)
    .expect("failed to parse name...");

   /* Player structure */
   let player = User {
       name: traveler,
       coins: 0,
   };

   /* Last user action */
   let mut selection = String::new();

   println!("\nHi {} Welcome to backpacking!\n", player.name);

   /* Program */
   'main_program: loop {

        /* Landing Page UI */
       println!("What would you like to do?");
       println!("Current coins: {}\n", player.coins);
       println!("1. Check inventory");
       println!("2. Craft a spell");
       println!("3. Cast a spell");
       println!("4. View equipment");
       println!("5. Craft equipment");
       println!("6. Marketplace");
       println!("7. Check metrics");
       println!("8. Sleep");
       println!("9. Monster Park (scary, dangerous even)");
       println!("Selection: ");
       io::stdin()
       .read_line(&mut selection)
       .expect("failed to parse selection...");

       /* User selects action */
       let selection: u32 = match selection.trim().parse() {
           Ok(num) => num,
           Err(_) => continue,
       };

       match selection {
           
       }

       /* User is navigated to page for associated action */
       spell_workbench();

       break;

   }
}
