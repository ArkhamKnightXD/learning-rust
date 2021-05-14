//Archivo encargado del manejo de strings
// hay 2 versionmes de string


pub fn run(){

    //primitive string, estos son inmutables y tienen un tamaño limitado
    let test = "hello";


    //Estos son growabble heap-allocated, y estos son los que debemos de utilizar
    //si queremos crear un string capaz de ser modificable
    let mut hello = String::from("Hello 1");

    
    //operaciones con string

    //si queremos agregar un caracter a nuestro string utilizamos
    hello.push('K');

    //si queremos agregar un string utilizamos esto
    hello.push_str("arvin");
    
     //conseguir lenght del string
    println!("Tamaño string: {}", hello.len());

     //capacidad en bytes
    println!("Capacidad string: {}", hello.capacity());

     //Esta vacio?
    println!("isEmpty string: {}", hello.is_empty());

    //contiene
    println!("Contains string: {}", hello.contains("Karvin"));


    //loop thought string by whitespace
    for word in hello.split_whitespace() {
        
        println!("{}", word);
    }

    //create string with capacity
    let mut string1 = String::with_capacity(10);

    string1.push('a');
    string1.push('b');


    //ASSertion testing esto funciona igual que en java ambos valores deben ser iguales
    //para que la prueba pase, si la prueba pasa no pasara nada, pero si falla el programa dara error
    assert_eq!(2, string1.len());



    println!("string test: {}  hello: {}", test, hello);
}