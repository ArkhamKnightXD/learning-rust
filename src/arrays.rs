//arrays ara fixed list where elements are the same data types
//si nos vemos en la necesidad de utilizar el standard library
//es recomendable definirlo aqui arriba, para evitar colocar std a cada rato

use std::mem;


pub fn run(){

    //Para estos hay que definir el tipo de datoy el lenght, si defino
    //que el lenght del array es 5 debo de obligatoriamente poner 5 elementos sino falla
    let mut numbers: [i32; 5] = [1,2,3,4,5];


    //re asing value
    numbers[2] = 20;

    //get all array value
    println!("{:?}", numbers);

    //get single value
    println!("{}", numbers[0]);

    //get lengt
    println!("arrays lenght: {}", numbers.len());


    //get array amount of memory
    println!("arrays occupies: {} bytes", mem::size_of_val(&numbers));

    //get slices con esto podemos partir el arreglo
    //el codigo de abajo me dara los 3 primeros elementos del arreglo
    let slices: &[i32] = &numbers[0..3];

    println!("Slice: {:?}", slices);




}