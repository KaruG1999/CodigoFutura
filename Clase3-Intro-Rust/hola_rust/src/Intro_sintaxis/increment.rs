pub fn increment(env: Env) -> u32 {
    // PASO 1: Leer del storage
    let mut contador: u32 = env.storage()
        .instance()
        .get(&symbol_short!("COUNTER"))
        .unwrap_or(0);
    
    // PASO 2: Incrementar
    contador += 1;
    
    // PASO 3: Guardar en storage
    env.storage().instance().set(
        &symbol_short!("COUNTER"),
        &contador
    );
    
    // PASO 4: Emitir evento
    env.events().publish(
        (symbol_short!("increment"),),
        contador
    );
    
    // PASO 5: Retornar
    contador
}