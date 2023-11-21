use regex::Regex;


fn main() {
    //regex
    let re_add = Regex::new(r"(\d+)\s?\+\s?(\d+)").unwrap();
    let re_mult = Regex::new(r"(\d+)\s?\*\s?(\d+)").unwrap();

    //traer los datos del usuario
    println!("Ingrese la expresion a calcular: ");

    let mut expresion = String::new();
    std::io::stdin().read_line(&mut expresion).unwrap();

    //multiplicacion
    loop {
        //aplicar las operaciones
        let caps = re_mult.captures(expresion.as_str());

        if caps.is_none() {
            break;
        }
        let caps = caps.unwrap();
        let cap_expression = caps.get(0).unwrap().as_str();
        let num1: i32 = caps.get(1).unwrap().as_str().parse().unwrap();
        let num2: i32 = caps.get(2).unwrap().as_str().parse().unwrap();
        let mult = num1 * num2;

        expresion = expresion.replace(cap_expression, mult.to_string().as_str());
    }
    //suma
    loop {
        //aplicar las operaciones
        let caps = re_add.captures(expresion.as_str());

        if caps.is_none() {
            break;
        }
        let caps = caps.unwrap();
        let cap_expression = caps.get(0).unwrap().as_str();
        let num1: i32 = caps.get(1).unwrap().as_str().parse().unwrap();
        let num2: i32 = caps.get(2).unwrap().as_str().parse().unwrap();
        let adition = num1 + num2;

        expresion = expresion.replace(cap_expression, adition.to_string().as_str());
    }

    //mostrar los resultados
    println!("El resultado es: {}", expresion);
}
