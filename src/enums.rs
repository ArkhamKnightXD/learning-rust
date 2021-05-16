//mismo proposito que en los demas lenguajes
//enums son tipos los cuales tiene varios valores definidos

//se escriben en mayuscula al igual que las struct y  
// lo ideal es declararlas fuera de una funcion al igual que los struct
enum Movement{

    //sus valores deben de empezar con mayuscula
    Up,
    Down,
    Left,
    Right
}

//funcion encargada de utilizar el enum para mover nuestro caracter
//en este caso no indicamos tipo de dato enum sino el nombre de nuestro enum
fn move_character(movement: Movement){

    //match es el equivalente a switch case en rust
    //Aqui arriba se pondra la variable definida con el enum
    match movement {

        //aqui debajo se llama el enum directamente
        //se utiliza con arrow functions de las que hay en js
        Movement::Up => println!("Avatar moving Up"),
        Movement::Down => println!("Avatar moving Down"),
        Movement::Left => println!("Avatar moving Left"),
        Movement::Right => println!("Avatar moving Right")
    }

}

pub fn run(){

    move_character(Movement::Up);
    move_character(Movement::Down);
    move_character(Movement::Left);
    move_character(Movement::Right);


}