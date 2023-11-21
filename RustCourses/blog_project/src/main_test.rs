fn main() {
    let x: i8 = 5; //variable inmutable
    print!("x old = {}\n", x);
    //x = 6; //error
    let mut y: i8 = 5; //variable mutable
    print!("y old = {}\n", y);
    y = 6;
    print!("y new= {}\n", y);

    sum(x, 6); //x se pasa por valor (se cambia la propiedad)
    sum_mut(&mut y, 6); //y se pasa por referencia (se mantiene la propiedad) borrowing
    sum_borrow(&x, &6); //x se pasa por referencia (se mantiene la propiedad) borrowing

    print!("x nuevo = {}\n", x);
    print!("y nuevo = {}\n", y);
}

fn sum( a: i8, b: i8) {
    // a = 18;
    let c = a + b;
    print!("{} + {} = {}\n", a, b, c);
}

fn sum_mut(a: &mut i8, b: i8) {
    *a = 18;
    let c = *a + b;
    print!("{} + {} = {}\n", a, b, c);
}

fn sum_borrow(a: &i8, b: &i8) {
    let c = a + b;
    //  a = &c;
    print!("{} + {} = {}\n", a, b, c);
}
