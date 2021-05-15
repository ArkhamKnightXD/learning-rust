pub fn run(){

    greeting("Hello", "Karvin");

    let operation = add(4, 5);

    println!("{}", operation);

    //closure, esto son formas cortas de funciones
    // y estas nos permiten utilizar variables de afuera de la funcion

    let n3 = 10;

    let add_nums = |n1: i32, n2: i32| n1 + n2 +n3;

    println!("C sum: {}", add_nums(3,3));
}

//agregar parametros, a la hora de trabajar con funciones 
//debemos de especificar los tipos de datos de los parametros

fn greeting(greet: &str, name: &str){

    println!("{} {}", greet, name);
}

//A esta funcion le especificaremos el tipo de dato a retornar
//con -> 
fn add(number1: i32, number2: i32) -> i32 {

    //para retornar simplemente no agregamos ; en estos valores
    //y esto sera lo que se retorne
    number1+number2
}