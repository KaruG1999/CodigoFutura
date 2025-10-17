/* // Tests comprehensivos 
extern crate std; // Soroban no usa std, pero los tests sí

use soroban_sdk::{
    testutils::{Address as TestAddress},
    Address, Env, Symbol,
};

// 👇 Importá el contrato desde el crate principal
use hello_tiburona::{HelloContract, HelloContractClient};

// Test para `initialize`
#[test]
fn test_initialize() {
    let env = Env::default();

    // En Soroban 23.0.2: register_contract_with_id
    let contract_id = TestAddress::generate(&env);
    env.register_contract(&contract_id, HelloContract);
    let client = HelloContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);

    // Primera inicialización debe funcionar
    client.initialize(&admin);

    // Verificar contador en 0
    assert_eq!(client.get_contador(), 0);
}

// No inicializar dos veces
#[test]
#[should_panic(expected = "NoInicializado")]
fn test_no_reinicializar() {
    let env = Env::default();
    let contract_id = env.register_contract(None, HelloContract);
    let client = HelloContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);

    client.initialize(&admin);
    client.initialize(&admin); // Segunda vez debe fallar
}

// Hello con validaciones
#[test]
fn test_hello_exitoso() {
    let env = Env::default();
    let contract_id = env.register_contract(None, HelloContract);
    let client = HelloContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let usuario = Address::generate(&env);

    client.initialize(&admin);

    let nombre = Symbol::new(&env, "Ana");
    let resultado = client.hello(&usuario, &nombre);

    assert_eq!(resultado, Symbol::new(&env, "Hola"));
    assert_eq!(client.get_contador(), 1);
    assert_eq!(client.get_ultimo_saludo(&usuario), Some(nombre));
}

// Hello con nombre vacío
#[test]
#[should_panic(expected = "NombreVacio")]
fn test_nombre_vacio() {
    let env = Env::default();
    let contract_id = env.register_contract(None, HelloContract);
    let client = HelloContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let usuario = Address::generate(&env);

    client.initialize(&admin);

    let vacio = Symbol::new(&env, "");
    client.hello(&usuario, &vacio); // Debe fallar
}

// Reset solo admin
#[test]
fn test_reset_solo_admin() {
    let env = Env::default();
    let contract_id = env.register_contract(None, HelloContract);
    let client = HelloContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let usuario = Address::generate(&env);

    client.initialize(&admin);

    // Hacer saludos
    client.hello(&usuario, &Symbol::new(&env, "Test"));
    assert_eq!(client.get_contador(), 1);

    // Admin puede resetear
    client.reset_contador(&admin);
    assert_eq!(client.get_contador(), 0);
}

// Reset por no admin falla
#[test]
#[should_panic(expected = "NoAutorizado")]
fn test_reset_no_autorizado() {
    let env = Env::default();
    let contract_id = env.register_contract(None, HelloContract);
    let client = HelloContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let otro = Address::generate(&env);

    client.initialize(&admin);

    // Otro usuario intenta resetear
    client.reset_contador(&otro); // Debe fallar
}
 */