//tasks1:
fn main() {
    let world:i32 = 1;
    println!("Hi interger {}",world)
}
//task2:
fn main() {
    //let hello = 1;
    // hello = 3; overwriting is not allowed in variables we need to usee mut(for mutabale) before on let
    let mut hello = 1;
    hello = 3;
    
    print!("Hello: {}",hello)
}
//task3:
const WORLD: i32 = 5;
fn main() {
    //constant
    let mut hello = 1;
    hello = 3;
    //const world = 5; addition of this const not work need to be pass type & we can use const as gloably 
    //  ^^^^^ help: convert the identifier to upper case: `WORLD`
    print!("Hello: {}",hello + WORLD)
}
//task4:
fn main() {
    let hello = 1;
    let hello = 3;
    //o/p: Hello: 3 ==> it will print only lastest variable
    
    println!("Hello: {}",hello)
  }