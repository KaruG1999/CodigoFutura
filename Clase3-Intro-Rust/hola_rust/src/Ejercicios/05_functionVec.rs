pub fn contar_mayores(env: Env, numeros: Vec<u32>) -> u32 {
    let mut contador: u32 = 0;
    for numero in numeros.iter() {
        if *numero > 100 {
            contador += 1;
        }
    }
    contador
}

// Ejemplos:
// contar_mayores([50, 150, 200, 80]) → 2
// contar_mayores([10, 20, 30]) → 0