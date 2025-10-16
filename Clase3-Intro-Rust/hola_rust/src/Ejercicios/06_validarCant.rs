pub fn validar_cantidad(cantidad: u32) -> Result<u32, String> {
    match cantidad {
// - Si cantidad == 0: Error "Cantidad no puede ser 0"
        0 => Err(String::from("Cantidad no puede ser 0")),
 // - Si 1 <= cantidad <= 1000: Ok(cantidad)
        1..=1000 => Ok(cantidad),
// - Si cantidad > 1000: Error "Cantidad máxima: 1000"
        _ => Err(String::from("Cantidad máxima: 1000")),
    }
    
} 

// Usar la validación
pub fn procesar_deposito(env: Env, cantidad: u32) {
    match validar_cantidad(cantidad) {
        Ok(monto_valido) => {
            println!("Depositando {} tokens", monto_valido);
        },
        Err(mensaje) => {
            panic!("Depósito rechazado: {}", mensaje);
        },
    }
}