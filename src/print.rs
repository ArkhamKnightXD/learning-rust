//Archivo encargado de los distintos ejemplos de la funcion print!

pub fn run(){

    //Si queremos imprimir una variable, ya sea un string o int, etc.. 
    //lo hacemos con este formato
    println!("{} is from: {}", "Karvin", "DR");


    //positional arguments a veces queremos imprimir argumentos que se repiten
    //con esto indicamos el argumento exacto que queremos que vaya en cierto lugar
    // la posicion empieza en 0
    println!("{0} is from {1} and {0} likes to {2}", "karvin", "DR", "Code");

    //named arguments esto es otra forma de trabajar los datos en print
    //primero se especifica el nombre de los parametros y luego se inicializan

    println!("{name} likes to play {activity}", name= "Karvin", activity = "Baseball");

    //Placeholder traits
    println!("Binary: {:b} Hex: {:o} Octal: {:o}", 10, 10, 10);

    //placeholder for debug trait, uno de los usos que puede tener
    // esto es el de imprimir un arreglo completo, con el ? puedo poner multimple valores y imprimirlos
    println!("{:?}", ("kar", 12, true));

    //basic math, el resultado sera pues en el parentesis despues del =
    println!("10 + 10 = {}", 10 + 10);
    println!("10 - 10 = {}", 10 - 10);
    println!("10 * 10 = {}", 10 * 10);
    println!("10 / 10 = {}", 10 / 10);
}