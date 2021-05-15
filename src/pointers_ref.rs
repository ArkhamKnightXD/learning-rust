//Reference pointers, estos apuntan a un recurso de memoria

pub fn run(){

    //primitive array
    let arr1 = [1,2,3];
    let arr2 = arr1;


    //con valores primitivos a la hora de tu asignar una variable a otra,
    println!("{:?}", (arr1, arr2));
    

    //se le pasara el valor sin problema, pero cuando se trata de valores no primitivos
    //cuando se asigne el valor de la primera variable a una segunda
    //la primera variable perdera el valor, en este caso se necesitara una referencia (&)
    //para apuntar a ese recurso, para esto se utiliza el puntero


    //valores no primitivos
    //vectors
    //primitive array
    let vec1 = vec![1,2,3];

    //para conseguir el recurso en memoria del vector1 utilizamos & 
    //y asi tendremos el recurso y lo podremos asignar al otro vector
    let vec2 = &vec1;

    println!("{:?}", (&vec1, vec2));

    //este es basicamente el ejemplo mas basico para entender punteros
    //ya si deseo mas informacion debo buscar mas ejemplo, pero en muchos casos
    //voy a querer utilizar punteros para este caso en especificos
}