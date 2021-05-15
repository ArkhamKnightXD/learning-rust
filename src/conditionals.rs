pub fn run(){
    
    let age: u8 = 22;

    let check_id: bool = true;


    //los condicionales siguen siendo iguales que en otro lenguajes 
    //la unica diferencia es que no se ponen parentesis
    //tambien hay else if
    // se sigue utilizando && y || como en los demas lenguajes
    if age >= 21 && check_id {

        println!("Que desea beber?");
    }

    else if age < 21 && check_id {

        println!("es menor de 21");
    }

    else{

        println!("deje ver id");
    }


    //shorthand if ejemplo de if corto
    let is_of_age = if age >= 21 {true} else {false};

    println!("is of age: {}", is_of_age);
}