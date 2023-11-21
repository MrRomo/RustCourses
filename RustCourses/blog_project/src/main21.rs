fn hacer_mutable(val: &mut &i32) {
    *val = &42; // Esto es válido y modifica la variable val en el ámbito de la función
}

fn main() {
    let x = 10;
    hacer_mutable(x); // Se transfiere la propiedad de x a val en hacer_mutable
    // x ya no es accesible aquí
    print!("x: {}\n", x); // Error! x ya no es accesible aquí (ya no existe

    let mut y = 20;
    hacer_mutable(&y); // Se transfiere la propiedad de y a val en hacer_mutable
    // y aún es accesible aquí
    println!("y: {}", y); // y ahora es 42
}
