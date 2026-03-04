fn main() {
    /*
     * Conceptos de Ownership y Scope
     let mut s1 = String::from("Hola");

    {

    let s2  = &s1;

    println!("El valor de s2 es: {}", s2);
    println!("Y el valor de s1 es: {}", s1);
    }
    s1.push_str(" Mundo");
    println!("s1 ahora es: {}",s1)*/

    let mi_nombre = String::from("Yoss");

    hacer_prestamo(&mi_nombre);
    tomar_propiedad(mi_nombre);

    //println!("Mi nombre es: {}", mi_nombre);

}

fn tomar_propiedad(texto: String) {
    println!("Muejeje. Me adueñe de: {}", texto)
}

fn hacer_prestamo(texto: &String) {
    println!("Hola, solo estoy leyendo: {}", texto)
}
