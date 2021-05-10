//Archivo encargado de los ejemplos de las variables
//Notas de las variables en rust

//variables hold primitive data or references to data
//la varia son inmutable por defecto, esto quiere decir que una vez
//Que lke asignaste un valor a una variable, no puedes asignarle otro valor
//En otras palabras las variables son tratadas todas como constantes

//Rust is a block-scoped language esto quiere decir que cuando definimos una variable
//en una funcion estas variables solo le pertenecera y funcionara en esa funcion


pub fn run(){

    //definimos variables con let
    let name = "brad";

    //esta variable ya es mutable es decir que su valor puede cambiar sin problemas
    let mut age = 37;

    println!("My name is: {} and i am {}", name, age);

    //Si intentamos asignar un valor a age dara error, 
    //pues las variables son inmutables por defecto, pero si queremos que se pueda
    //reasignar valores a una variable utilizamos mut en la variable

    age = 38;

    println!("My name is: {} and i am {}", name, age);

    //Existe const pero no se usa tanto debido a la naturaleza de rust
    //ejemplo de const, a estas hay que definirles tipos de datos
    const ID: i32 = 001;

    println!("{}", ID);


    //Podemos tambien asignar multiples variables de una vez

    let ( my_name, my_age) = ("karvin", 26);


    println!("my name is: {} and my age is: {}", my_name, my_age);
}