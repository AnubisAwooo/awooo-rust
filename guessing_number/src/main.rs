use std::io;
use rand::Rng;
use std::cmp::Ordering;

struct Tree {
    value: String,
    left: Option<Box<Tree>>,
    right: Option<Box<Tree>>
}

impl Tree {
    fn change(&mut self, value:String) {
        self.value = value;

    }
}

fn main() {

    let tree = Tree{value: String::from("123"), right:None, left:None};


    tree.value = String::from("456");

    tree.change(String::from("456"));

    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    // println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();
    
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");
    
        // let guess: u32 = guess.trim().parse()
        //     .expect("Please type a number!");
    
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        
        println!("You guessed: {}", guess);
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
    
}
