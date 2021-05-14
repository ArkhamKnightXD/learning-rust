//las tuplas son grupos de valores de diferentes tipos
// maximo 12 elementos, esto es diferente al array ya que en el
//array todos los valores deben de ser del mismo tipo

pub fn run(){

    // para las tuplas es necesario especifiar los tipos de datos
    let person: (&str, &str, i8) = ("karvin", "jamaica", 25);

    println!("{} is from {} and is {}", person.0, person.1, person.2);
}