// BLOQUE 1: String
let s1 = String::from("rust");
let s2 = s1;
// println!("{}", s1);  // Línea A - ¿Error? -> // Error: s1 ya no es válido
println!("{}", s2);     // Línea B - ¿Error? -> // No hay error: s2 es una copia de s1

// BLOQUE 2: u32
let x: u32 = 10;
let y = x;
println!("{}, {}", x, y);  // Línea C - ¿Error? -> // No hay error: x y y son copias

// BLOQUE 3: Vec
let v1 = Vec::from([1, 2, 3]);
let v2 = v1;
// let v3 = v1;  // Línea D - ¿Error? -> // Error: v1 ya no es válido