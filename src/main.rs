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
}
