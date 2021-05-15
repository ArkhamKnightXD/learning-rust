pub fn run(){

    let mut count = 0;

    let mut count2 = 0;
    
    //infinite loop
    loop{

        count += 1;

        //De esta forma detenemos el loop sino seria infinito
        if count == 20{
            break;
        }
    }

    //while loop
    while count2 < 20 {
        
        println!("number: {}", count2);

        count2 += 1;
    }

    //for range en este indicaremos que su rango sea de 0 a < 100

    for number in 0..100 {
        
        println!("number for: {}", number);
    }
}