fn main() {
    let s1 = String::from("Hola");

    let s2  = &s1;

    println!("El valor de s2 es: {}", s2);
    println!("Y el valor de s1 es: {}", s1);
}
