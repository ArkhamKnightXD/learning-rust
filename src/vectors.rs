//Es basicamente lo mismo que array, pero estos pueden aumentar en tamaño sin problema
//por esta razon son los que mas se utilizan
use std::mem;


pub fn run(){


    let mut numbers: Vec<i32> = vec![1,2,3,4,5];


    //en los arreglos no se pueden agregar mas elementos pero con los vectores si

    numbers.push(6);

    //quitar el ultimo elemento
    numbers.pop();

    //re asing value
    numbers[2] = 20;

    //get all Vector value
    println!("{:?}", numbers);

    //get single value
    println!("{}", numbers[0]);

    //get lengt
    println!("Vector lenght: {}", numbers.len());


    //get vector amount of memory
    println!("Vector occupies: {} bytes", mem::size_of_val(&numbers));

    
    let slices: &[i32] = &numbers[0..3];

    println!("Slice: {:?}", slices);



    //loop througt vector values
    //podria iterar tanto con iter como sin este
    for number in numbers.iter() {
        
        println!("value: {}", number);
    }


    //loop and mut vector values
    //en este caso multiplicare todo los elementos del array por 2
    //y los guardare dentro de este
    for number in numbers.iter_mut() {
        
    
        *number *= 2;
    }

    println!("Numbers: {:?}", numbers);

}