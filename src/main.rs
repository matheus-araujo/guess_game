use std::io; //input/output library
use rand::Rng;//methods to random number generators
use std::cmp::Ordering; //compares two values 

fn main() {
        println!("Guess the number! ");//show the menssage

    //create a random number and store
        let secret_number = rand::thread_rng().gen_range(1..101);
   

        println!("The secret number is :{}", secret_number);
    loop{
            let mut guess = String::new();  

            io::stdin()
	        .read_line(&mut guess)
	        .expect("Failed to read line");

    //take only numerical values,
    //elimenate whitespaces and analyze if have some problem
        let guess: u32 = match guess.trim().parse(){
	    Ok(num)=>num,
	    Err(_)=> continue,
	};
    
            println!("Your guess: {}", guess);


            match guess.cmp(&secret_number) { // compare guess and secret_number
	        Ordering::Less => println!("Too small!"),
	        Ordering::Greater => println!("Too big!"),
	        Ordering::Equal => {
		    println!("You win");
		    break;
		}

            }
       }

}
