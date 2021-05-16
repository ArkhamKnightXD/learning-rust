//archivo encargado de los ejemplos de la command line arguments
//en este archivo coseguiremos los argumentos que  se pueden poner
//luego de hacer cargo run

use std::env;

pub fn run(){

    //buscaremos los args y utilizaremos el metodo collect
    //collect()` can take anything iterable, and turn it into a relevant
    // collection. 
    let args: Vec<String> = env::args().collect();

    //esto nos retorna el path donde se guarda nuestro exe a la hora
    //de compilar seguido de cualquier argumento que pongamos luego de run
    println!("Args: {:?}", args);


    //para conseguir el argumento que va despues de run debemos de
    // buscar en la posicion 1
    //si deseamos copiar el elemento de x posicion a una variable 
    //debemos de utilizar el metodo clone

    //si no pongo un args en consola el programa dara error
    //aqui concluyo con todo
    let command = args[1].clone();

    //println!("command: {}", command);

    if !command.is_empty() {

        println!("you write: {}", command);

    }

    else{

        println!("you didn't wrote anything");

    }
}