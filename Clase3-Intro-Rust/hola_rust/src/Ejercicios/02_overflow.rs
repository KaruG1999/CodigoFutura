// ¿Qué imprimiria en cada caso?

// CASO A
let x: u8 = 255;
match x.checked_add(1) {
    Some(valor) => println!("A: {}", valor),
    None => println!("A: Overflow"),   -> // Imprime esto : 256 fuera de rango para u8
}

// CASO B
let y: u32 = 100;
match y.checked_sub(50) {
    Some(valor) => println!("B: {}", valor), -> // Imprime esto : 50 dentro de rango para u32
    None => println!("B: Underflow"),
}

// CASO C
let z: u8 = 200;
match z.checked_add(100) {
    Some(valor) => println!("C: {}", valor),
    None => println!("C: Overflow"), -> // Imprime esto : 300 fuera de rango para u8
}