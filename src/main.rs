fn main() {

    // this is how you declare a variable wher "u" is for positive number (unsigned) and  8 is the quantity of bytes
    // in this way the vaiables are inmutable
    // for mutables variables put expression "mut" after let
    // let mut age: u8 = 44
    let age: u8 = 44;
    // declaracion de entero i es integer y 16 es la cantidad de bytes
    let weight: i16 = 478;

    // declaracion de string
    let name: &str = "Jose"; 

    println!("Hi i am {} and i'm {} years old, my weigth is: {} pounds", name, age, weight);

    // getting info fron keyborad user
    println!("Please what is your last name: ");
    let mut last_name : String = String::new();
    std::io::stdin().read_line(&mut last_name).unwrap();
    // adding trim to drop the line break
    last_name = last_name.trim().to_string();

    println!("Please what is your favorite number: ");
    let mut favorite_number : String = String::new();
    std::io::stdin().read_line(&mut favorite_number).unwrap();
    
    // adding trim to drop the line break and parser to change to a number 
    let favorite_number_int : u16 = favorite_number.trim().parse().unwrap();

    // Condition
    if favorite_number_int >= 18 {
        println!("Your number big");
    } else {
        println!("Your number is small");
    }

    println!("Welcome {}, your number is: {}", last_name, favorite_number_int);

}
