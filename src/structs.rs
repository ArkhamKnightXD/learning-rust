//las estructuras son muy importantes en rust, estas son parecidas  a las clases
//utilizadas para crear custom type data

//traditional struct
//se escriben en mayusculas como las clases
struct Color {

    //es necesario definir el tipo de dato
    red: u8,
    green: u8,
    blue: u8
}

//tuples structs, esta son mas simples
struct Color2(u8, u8, u8);


//Las estructuras al igual que las clases tambien pueden tener
//funciones dentro

struct Person {

    first_name: String,
    last_name: String,
}

//para poder poner funciones dentro de una estructura debemos 
//de hacerlo de esta manera
impl Person {

    //funcion constructor de la estructura person
    fn new_person(first: &str, last: &str) -> Person{
        
        //asignando los datos a Person
        Person{

            //es necesario hace el to string debido a &str
            first_name: first.to_string(),
            last_name: last.to_string()

        }
        
    }

    //segunda funcion para conseguir el nombre completo
    //self es el equivalente a this en otros lenguajes
    //en este caso self es la estructura person
    fn get_full_name(&self)-> String {
        
        // esta funcion nos puede llevar 2 o mas elementos a string
        //y le podemos dar varios formatos que deseemos
        format!("{} - {}", self.first_name, self.last_name)
    }

}

pub fn run(){

    //utilizando la estructura, esto es parecido a
    //instanciar un objeto
    let mut color = Color{

        red: 255,
        green: 0,
        blue: 0
    };


    //para aceder a los datos dentro de la estructura se hace
    // con . al igual que con una clase

    color.red = 200;
    color.green = 150;

    println!("{} {} {}", color.red, color.green, color.blue);

    //son mas simples de definir tambien
    let mut color2 = Color2(255,0,0);

    color2.1 = 100;

    //de esta forma accedemos a sus datos ya que no los nombramos
    //como en la primera struct
    println!("{} {} {}", color2.0, color2.1, color2.2);


    //Aqui utilizaremos el constructor del Person

    let mut person = Person::new_person("Karvin", "Jimenez");

    println!("Soy: {} {}", person.first_name, person.last_name);

    println!("{}", person.get_full_name());
    
}