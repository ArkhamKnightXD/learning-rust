//Archivo encargado de los tipos de datos en rust

//bool : The boolean type.
//char : A character type.
//i8 : The 8-bit signed integer type.
//i16 : The 16-bit signed integer type.
//i32 : The 32-bit signed integer type.
//i64 : The 64-bit signed integer type.
//isize : The pointer-sized signed integer type.
//u8 : The 8-bit unsigned integer type.
//u16 : The 16-bit unsigned integer type.
//u32 : The 32-bit unsigned integer type.
//u64 : The 64-bit unsigned integer type.
//usize : The pointer-sized unsigned integer type.
//f32 : The 32-bit floating point type.
//f64 : The 64-bit floating point type.
//array : A fixed-size array, denoted [T; N], for the element type, T, and the non-negative compile-time constant size, N.
//slice : A dynamically-sized view into a contiguous sequence, [T].
//str : String slices.
//tuple : A finite heterogeneous sequence, (T, U, ..).

//unsigned significa que no hay valores negativos en este tipo de datos

//Rust es staticamente tipado, en resumen no es necesario definir el tipo de datoos de las variables
//ya que el compilador puede inferir esto basado en el valor de la variable, puede que a veces se queje el compilador

pub fn run(){

    //por defecto el tipo de dato es i32
    let x = 1;

    //f64
    let y = 2.5;

    //si queremos declarar el tipo de dato a una variable lo hacemos de esta forma
    let y64: i64 = 454545454;

    //mostrar el valor maximo que puede tener un i32
    //para conseguir este datos utilizamos std
    println!("Max i32: {}", std::i32::MAX);

    //Si Alguna vez deseamos utilizar un numero que sobrepase el limite i32, debemos especificar su tipo de dato con i64
    println!("Max i64: {}", std::i64::MAX);

    //boolean
    let is_active: bool = true;

    //get boolean from a expresion
    //evalua esta expresion y al final me guarda el valor de tru o false
    let is_greater = 10 > 5;


    //char en esto solamente se puede definir un caracter, si intento poner 2 tendre error
    let a1 = 'a';

    //con unicode se pueden poner mas de un caracter debido a que varios conforman un solo caracter
    let smiley_face = '\u{1F600}';

    //De esta forma imprimo todo
    println!("{:?}",( x, y, y64, is_active, is_greater, a1, smiley_face));
}