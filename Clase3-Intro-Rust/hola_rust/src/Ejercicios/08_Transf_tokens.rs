pub fn transferir(
    env: Env,
    from: Address,
    to: Address,
    amount: u128
) -> Result<(), String> { // -> Devuelve Ok(()) si todo sale bien, o Err(String) si ocurre algún error lógico.
    // Validación 1: Amount mayor a 0
    if amount == 0 {
        return Err("El monto debe ser mayor a 0".to_string());
    }
    
    // Leer balance del remitente
    let balance_from = env.storage()
        .instance()
        .get(&from)
        .unwrap_or(0);
    
    // Validación 2: Balance suficiente
    if balance_from < amount {
        return Err("Balance insuficiente".to_string());
    }
    
    // Leer balance del destinatario
    let balance_to = env.storage()
        .instance()
        .get(&to)
        .unwrap_or(0);
    
    // Restar de forma segura (prevenir underflow)
    let nuevo_balance_from = balance_from
        .checked_sub(amount)
        .ok_or("Error: underflow al restar")?;
    
    // Sumar de forma segura (prevenir overflow)
    let nuevo_balance_to = balance_to
        .checked_add(amount)
        .ok_or("Error: overflow al sumar")?;
    
    // Actualizar balances en storage
    env.storage().instance().set(&from, &nuevo_balance_from);
    env.storage().instance().set(&to, &nuevo_balance_to);
    
    // Emitir evento
    env.events().publish(
        (symbol_short!("transfer"),),
        (from, to, amount)
    );
    
    Ok(())
}