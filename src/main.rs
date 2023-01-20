fn plus_one(number:i32) -> i32 {
    let result: i32 = number + 1;
    return result;
}

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

    // loop
    // be carefull with the scope of loop vaiables

    loop {

        println!("Please what is your favorite number: ");
        let mut favorite_number : String = String::new();
        std::io::stdin().read_line(&mut favorite_number).unwrap();
        
        // adding trim to drop the line break and parser to change to a number 
        let favorite_number_int : u16 = favorite_number.trim().parse().unwrap();

        // Condition
        if favorite_number_int >= 18 {
            println!("Your number big");
        } 
        else if favorite_number_int == 0{
            println!("Your number is zero");
        }else {
            println!("Your number is small");
            break;
        }
    }

    println!("Welcome {}", last_name);

    // Vectors
    // You can access to position vector[index]

    let mut names: Vec<String> = Vec::new();

    // for loop

    for _i in 0..3 {
        println!("Please what is your name: ");
        let mut the_name : String = String::new();
        std::io::stdin().read_line(&mut the_name).unwrap();

        names.push(the_name);
    }
    println!("{:?}", names);

    // call function
    let the_number = plus_one(5);
    println!("{}", the_number);

}
