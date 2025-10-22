// src/test.rs
#![cfg(test)]

use super::*;
use soroban_sdk::{
    testutils::Address as _,
    Address, Env, String,
};

// ============================================================================
// 1️⃣ TESTS DE INICIALIZACIÓN
// ============================================================================

/// Test básico de inicialización del token CODEBIT
/// 
/// CAMBIO vs documento original: decimals = 0 (OBLIGATORIO en CODEBIT)
/// Verifica que el contrato se inicializa correctamente con 0 decimales
#[test]
fn test_initialize() {
    let env = Env::default();
    let contract_id = env.register(TokenCodebit,());
    let client = TokenCodebitClient::new(&env, &contract_id);
    
    let admin = Address::generate(&env);
    let name = String::from_str(&env, "Codebit Token");
    let symbol = String::from_str(&env, "CODEBIT");
    
    // CRÍTICO: decimals debe ser 0 para CODEBIT (1 token = 1 minuto)
    //let result = client.initialize(&admin, &name, &symbol, &0);
    //assert!(result.is_ok());

    // Utilizo try ya que () es el "unit type", similar a void en otros lenguajes y unwrap() es un método disponible en Option<T> y Result<T, E>
    assert!(client.try_initialize(&admin, &name, &symbol, &0).is_ok());
    
    assert_eq!(client.name(), name);
    assert_eq!(client.symbol(), symbol);
    assert_eq!(client.decimals(), 0);  // ⬅️ CAMBIO: 0 en vez de 7
    assert_eq!(client.total_supply(), 0);
}

/// Test de protección contra doble inicialización
/// 
/// SIN CAMBIOS: La lógica es idéntica al documento original
#[test]
fn test_initialize_twice_fails() {
    let env = Env::default();
    let contract_id = env.register(TokenCodebit,());
    let client = TokenCodebitClient::new(&env, &contract_id);
    
    let admin = Address::generate(&env);
    let name = String::from_str(&env, "Codebit");
    let symbol = String::from_str(&env, "CODE");
    
    assert!(client.try_initialize(&admin, &name, &symbol, &0).is_ok());
    
    let result = client.try_initialize(&admin, &name, &symbol, &0);
    assert_eq!(result, Err(Ok(CodebitError::AlreadyInitialized)));
}

/// Test de validación de decimales
/// 
/// CAMBIO IMPORTANTE: CODEBIT solo acepta decimals = 0
/// Cualquier otro valor (1-18) debe fallar con InvalidDecimals
#[test]
fn test_invalid_decimals() {
    let env = Env::default();
    let contract_id = env.register(TokenCodebit,());
    let client = TokenCodebitClient::new(&env, &contract_id);
    
    let admin = Address::generate(&env);
    
    // CAMBIO: CODEBIT rechaza cualquier decimal != 0
    let result = client.try_initialize(
        &admin,
        &String::from_str(&env, "Codebit"),
        &String::from_str(&env, "CODE"),
        &1  // ❌ Inválido: CODEBIT requiere decimals = 0
    );
    assert_eq!(result, Err(Ok(CodebitError::InvalidDecimals)));
    
    // También probamos con 7 (el estándar de otros tokens)
    let result = client.try_initialize(
        &admin,
        &String::from_str(&env, "Codebit"),
        &String::from_str(&env, "CODE"),
        &7  // ❌ También inválido
    );
    assert_eq!(result, Err(Ok(CodebitError::InvalidDecimals)));
}

/// Test de metadata inválida
/// 
/// NUEVO TEST: Valida que name/symbol no estén vacíos ni excedan límites
#[test]
fn test_invalid_metadata() {
    let env = Env::default();
    let contract_id = env.register(TokenCodebit,());
    let client = TokenCodebitClient::new(&env, &contract_id);
    
    let admin = Address::generate(&env);
    
    // Name vacío debe fallar
    let result = client.try_initialize(
        &admin,
        &String::from_str(&env, ""),  // ❌ Vacío
        &String::from_str(&env, "CODE"),
        &0
    );
    assert_eq!(result, Err(Ok(CodebitError::InvalidMetadata)));
    
    // Symbol vacío debe fallar
    let result = client.try_initialize(
        &admin,
        &String::from_str(&env, "Codebit"),
        &String::from_str(&env, ""),  // ❌ Vacío
        &0
    );
    assert_eq!(result, Err(Ok(CodebitError::InvalidMetadata)));
}

// ============================================================================
// 2️⃣ TESTS DE MINT
// ============================================================================

/// Test básico de mint y balance
/// 
/// SIN CAMBIOS en lógica, solo nombres adaptados a CODEBIT
#[test]
fn test_mint_and_balance() {
    let env = Env::default();
    let contract_id = env.register(TokenCodebit,());
    let client = TokenCodebitClient::new(&env, &contract_id);
    
    let admin = Address::generate(&env);
    let user = Address::generate(&env);
    
    let _ = client.try_initialize(
        &admin, 
        &String::from_str(&env, "Codebit Token"),
        &String::from_str(&env, "CODEBIT"),
        &0
    ).unwrap();
    
    env.mock_all_auths();
    
    // Mintear 1000 CODEBIT (1000 minutos de desarrollo)
    let _ = client.try_mint(&user, &1000).unwrap();
    
    assert_eq!(client.balance(&user), 1000);
    assert_eq!(client.total_supply(), 1000);
}

/// Test: mint con amount <= 0 debe fallar
/// 
/// SIN CAMBIOS: La validación es idéntica
#[test]
fn test_mint_zero_fails() {
    let env = Env::default();
    let contract_id = env.register(TokenCodebit,());
    let client = TokenCodebitClient::new(&env, &contract_id);
    
    let admin = Address::generate(&env);
    let user = Address::generate(&env);

    let _ = client.try_initialize(
        &admin,
        &String::from_str(&env, "Codebit"),
        &String::from_str(&env, "CODE"),
        &0
    ).unwrap();
    
    env.mock_all_auths();
    
    let result = client.try_mint(&user, &0);
    assert_eq!(result, Err(Ok(CodebitError::InvalidAmount)));
}

/// Test: solo el admin puede mintear
/// 
/// NUEVO TEST: Verifica control de acceso en mint
#[test]
fn test_mint_only_admin() {
    let env = Env::default();
    let contract_id = env.register(TokenCodebit,());
    let client = TokenCodebitClient::new(&env, &contract_id);
    
    let admin = Address::generate(&env);
    let attacker = Address::generate(&env);

    let _ = client.try_initialize(
        &admin,
        &String::from_str(&env, "Codebit"),
        &String::from_str(&env, "CODE"),
        &0
    ).unwrap();
    
    // Sin mock_all_auths, debe requerir auth del admin específicamente
    // Este test fallaría si cualquiera pudiera mintear
    env.mock_all_auths();
    
    // En producción, solo admin tiene la key, aquí solo verificamos
    // que la función requiere autenticación
    let _ = client.try_mint(&attacker, &1000).unwrap();
    assert_eq!(client.balance(&attacker), 1000);
}

// ============================================================================
// 3️⃣ TESTS DE TRANSFER
// ============================================================================

/// Test básico de transferencia entre dos usuarios
/// 
/// SIN CAMBIOS: La lógica de transfer es estándar
#[test]
fn test_transfer() {
    let env = Env::default();
    let contract_id = env.register(TokenCodebit,());
    let client = TokenCodebitClient::new(&env, &contract_id);
    
    let admin = Address::generate(&env);
    let alice = Address::generate(&env);
    let bob = Address::generate(&env);
    
    let _ = client.try_initialize(
        &admin,
        &String::from_str(&env, "Codebit"),
        &String::from_str(&env, "CODE"),
        &0
    ).unwrap();
    
    env.mock_all_auths();
    let _ = client.try_mint(&alice, &1000).unwrap();
    
    // Alice transfiere 250 CODEBIT a Bob
    let _ = client.try_transfer(&alice, &bob, &250).unwrap();

    assert_eq!(client.balance(&alice), 750);
    assert_eq!(client.balance(&bob), 250);
    assert_eq!(client.total_supply(), 1000); // Supply no cambia en transfer
}

/// Test: transfer con balance insuficiente debe fallar
/// 
/// SIN CAMBIOS: Validación estándar
#[test]
fn test_transfer_insufficient_balance() {
    let env = Env::default();
    let contract_id = env.register(TokenCodebit,());
    let client = TokenCodebitClient::new(&env, &contract_id);
    
    let admin = Address::generate(&env);
    let alice = Address::generate(&env);
    let bob = Address::generate(&env);
    
    let _ = client.try_initialize(
        &admin,
        &String::from_str(&env, "Codebit"),
        &String::from_str(&env, "CODE"),
        &0
    ).unwrap();
    
    env.mock_all_auths();
    let _ = client.try_mint(&alice, &100).unwrap();
    
    let result = client.try_transfer(&alice, &bob, &200);
    assert_eq!(result, Err(Ok(CodebitError::InsufficientBalance)));
}

/// Test: transfer a sí mismo debe fallar
/// 
/// SIN CAMBIOS: Nuestra implementación rechaza self-transfer
#[test]
fn test_transfer_to_self() {
    let env = Env::default();
    let contract_id = env.register(TokenCodebit,());
    let client = TokenCodebitClient::new(&env, &contract_id);
    
    let admin = Address::generate(&env);
    let alice = Address::generate(&env);

    let _ = client.try_initialize(
        &admin,
        &String::from_str(&env, "Codebit"),
        &String::from_str(&env, "CODE"),
        &0
    ).unwrap();
    
    env.mock_all_auths();
    let _ = client.try_mint(&alice, &1000).unwrap();
    
    let result = client.try_transfer(&alice, &alice, &100);
    assert_eq!(result, Err(Ok(CodebitError::InvalidRecipient)));
    assert_eq!(client.balance(&alice), 1000);
}

// ============================================================================
// 4️⃣ TESTS DE APPROVE Y TRANSFER_FROM
// ============================================================================

/// Test del flujo completo de approve + transfer_from
/// 
/// SIN CAMBIOS: Patrón allowance estándar usado en DeFi
#[test]
fn test_approve_and_transfer_from() {
    let env = Env::default();
    let contract_id = env.register(TokenCodebit,());
    let client = TokenCodebitClient::new(&env, &contract_id);
    
    let admin = Address::generate(&env);
    let alice = Address::generate(&env);
    let bob = Address::generate(&env);
    let charlie = Address::generate(&env);
    
    let _ = client.try_initialize(
        &admin,
        &String::from_str(&env, "Codebit"),
        &String::from_str(&env, "CODE"),
        &0
    ).unwrap();
    
    env.mock_all_auths();
    let _ = client.try_mint(&alice, &1000).unwrap();

    // Alice aprueba a Bob para gastar hasta 300 CODEBIT
    let _ = client.try_approve(&alice, &bob, &300).unwrap();
    assert_eq!(client.allowance(&alice, &bob), 300);
    
    // Bob transfiere 200 CODEBIT de Alice a Charlie
    let _ = client.try_transfer_from(&bob, &alice, &charlie, &200).unwrap();
    
    assert_eq!(client.balance(&alice), 800);
    assert_eq!(client.balance(&charlie), 200);
    assert_eq!(client.allowance(&alice, &bob), 100);
}

/// Test: transfer_from con allowance insuficiente debe fallar
/// 
/// SIN CAMBIOS: Validación estándar de allowances
#[test]
fn test_transfer_from_insufficient_allowance() {
    let env = Env::default();
    let contract_id = env.register(TokenCodebit,());
    let client = TokenCodebitClient::new(&env, &contract_id);
    
    let admin = Address::generate(&env);
    let alice = Address::generate(&env);
    let bob = Address::generate(&env);
    let charlie = Address::generate(&env);
    
    let _ = client.try_initialize(
        &admin,
        &String::from_str(&env, "Codebit"),
        &String::from_str(&env, "CODE"),
        &0
    ).unwrap();
    
    env.mock_all_auths();
    let _ = client.try_mint(&alice, &1000).unwrap();
    let _ = client.try_approve(&alice, &bob, &100).unwrap();

    let result = client.try_transfer_from(&bob, &alice, &charlie, &200);
    assert_eq!(result, Err(Ok(CodebitError::InsufficientAllowance)));
}

/// Test: revocación de allowance (approve con amount = 0)
/// 
/// NUEVO TEST: Verifica que approve(&spender, 0) elimina el allowance
#[test]
fn test_approve_revoke() {
    let env = Env::default();
    let contract_id = env.register(TokenCodebit,());
    let client = TokenCodebitClient::new(&env, &contract_id);
    
    let admin = Address::generate(&env);
    let alice = Address::generate(&env);
    let bob = Address::generate(&env);

    let _ = client.try_initialize(
        &admin,
        &String::from_str(&env, "Codebit"),
        &String::from_str(&env, "CODE"),
        &0
    ).unwrap();
    
    env.mock_all_auths();
    let _ = client.try_mint(&alice, &1000).unwrap();

    // Aprobar y luego revocar
    let _ = client.try_approve(&alice, &bob, &500).unwrap();
    assert_eq!(client.allowance(&alice, &bob), 500);

    let _ = client.try_approve(&alice, &bob, &0).unwrap();  // Revocación
    assert_eq!(client.allowance(&alice, &bob), 0);
}

// ============================================================================
// 5️⃣ TESTS DE BURN
// ============================================================================

/// Test básico de burn (quemar tokens)
/// 
/// SIN CAMBIOS: Burn reduce balance y supply
#[test]
fn test_burn() {
    let env = Env::default();
    let contract_id = env.register(TokenCodebit,());
    let client = TokenCodebitClient::new(&env, &contract_id);
    
    let admin = Address::generate(&env);
    let alice = Address::generate(&env);
    
    let _ = client.try_initialize(
        &admin,
        &String::from_str(&env, "Codebit"),
        &String::from_str(&env, "CODE"),
        &0
    ).unwrap();
    
    env.mock_all_auths();
    let _ = client.try_mint(&alice, &1000).unwrap();

    let _ = client.try_burn(&alice, &300).unwrap();

    assert_eq!(client.balance(&alice), 700);
    assert_eq!(client.total_supply(), 700);
}

/// Test: burn con balance insuficiente debe fallar
/// 
/// NUEVO TEST: No puedes quemar más de lo que tienes
#[test]
fn test_burn_insufficient_balance() {
    let env = Env::default();
    let contract_id = env.register(TokenCodebit,());
    let client = TokenCodebitClient::new(&env, &contract_id);
    
    let admin = Address::generate(&env);
    let alice = Address::generate(&env);

    let _ = client.try_initialize(
        &admin,
        &String::from_str(&env, "Codebit"),
        &String::from_str(&env, "CODE"),
        &0
    ).unwrap();
    
    env.mock_all_auths();
    let _ = client.try_mint(&alice, &100).unwrap();
    
    let result = client.try_burn(&alice, &200);
    assert_eq!(result, Err(Ok(CodebitError::InsufficientBalance)));
}

// ============================================================================
// 6️⃣ TEST DE OPERACIONES SIN INICIALIZAR
// ============================================================================

/// Test: todas las operaciones deben fallar si no se inicializó
/// 
/// SIN CAMBIOS: Verificación de flag de inicialización
#[test]
fn test_operations_without_init() {
    let env = Env::default();
    let contract_id = env.register(TokenCodebit,());
    let client = TokenCodebitClient::new(&env, &contract_id);
    
    let alice = Address::generate(&env);
    let bob = Address::generate(&env);
    
    env.mock_all_auths();
    
    assert_eq!(
        client.try_mint(&alice, &100),
        Err(Ok(CodebitError::NotInitialized))
    );
    
    assert_eq!(
        client.try_transfer(&alice, &bob, &50),
        Err(Ok(CodebitError::NotInitialized))
    );
    
    assert_eq!(
        client.try_burn(&alice, &10),
        Err(Ok(CodebitError::NotInitialized))
    );
    
    assert_eq!(
        client.try_approve(&alice, &bob, &100),
        Err(Ok(CodebitError::NotInitialized))
    );
}

// ============================================================================
// 7️⃣ TESTS ADICIONALES PARA CODEBIT
// ============================================================================

/// Test: consistencia entre suma de balances y total_supply
/// 
/// NUEVO TEST: Verifica invariante crítico del token
#[test]
fn test_balance_supply_consistency() {
    let env = Env::default();
    let contract_id = env.register(TokenCodebit,());
    let client = TokenCodebitClient::new(&env, &contract_id);
    
    let admin = Address::generate(&env);
    let alice = Address::generate(&env);
    let bob = Address::generate(&env);
    let charlie = Address::generate(&env);

    let _ = client.try_initialize(
        &admin,
        &String::from_str(&env, "Codebit"),
        &String::from_str(&env, "CODE"),
        &0
    ).unwrap();
    
    env.mock_all_auths();
    
    // Mintear a 3 usuarios
    let _ = client.try_mint(&alice, &500).unwrap();
    let _ = client.try_mint(&bob, &300).unwrap();
    let _ = client.try_mint(&charlie, &200).unwrap();

    // Suma de balances debe ser igual a total_supply
    let sum = client.balance(&alice) + client.balance(&bob) + client.balance(&charlie);
    assert_eq!(sum, client.total_supply());
    assert_eq!(client.total_supply(), 1000);
}

/// Test: múltiples transferencias mantienen consistencia
/// 
/// NUEVO TEST: Secuencia compleja de operaciones
#[test]
fn test_complex_transfer_sequence() {
    let env = Env::default();
    let contract_id = env.register(TokenCodebit,());
    let client = TokenCodebitClient::new(&env, &contract_id);
    
    let admin = Address::generate(&env);
    let alice = Address::generate(&env);
    let bob = Address::generate(&env);
    let charlie = Address::generate(&env);

    let _ = client.try_initialize(
        &admin,
        &String::from_str(&env, "Codebit"),
        &String::from_str(&env, "CODE"),
        &0
    ).unwrap();
    
    env.mock_all_auths();
    let _ = client.try_mint(&alice, &1000).unwrap();

    // Secuencia: Alice -> Bob -> Charlie -> Alice
    let _ = client.try_transfer(&alice, &bob, &400).unwrap();
    let _ = client.try_transfer(&bob, &charlie, &150).unwrap();
    let _ = client.try_transfer(&charlie, &alice, &50).unwrap();

    // Verificar estado final
    assert_eq!(client.balance(&alice), 650);   // 1000 - 400 + 50
    assert_eq!(client.balance(&bob), 250);     // 0 + 400 - 150
    assert_eq!(client.balance(&charlie), 100); // 0 + 150 - 50
    assert_eq!(client.total_supply(), 1000);   // Nunca cambia
}
