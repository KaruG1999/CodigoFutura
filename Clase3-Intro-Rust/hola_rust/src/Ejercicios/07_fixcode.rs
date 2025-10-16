// Se debe usar referencias (usar &) para evitar copias innecesarias

struct TokenInfo {
    name: String,
    symbol: Symbol,
    total_supply: u128,
}

// Accedo a campos del struct por referencia
fn procesar_mal(info: &TokenInfo) -> u128 {
    verificar_nombre(&info.name);
    verificar_supply(info);
    info.total_supply // Retorno una copia del total_supply (u128 es Copy)
}

fn verificar_nombre(name: &String) {
    if name.len() == 0 {
        panic!("Nombre vacío");
    }
}

fn verificar_supply(info: &TokenInfo) {
    if info.total_supply == 0 {
        panic!("Supply no puede ser 0");
    }
}