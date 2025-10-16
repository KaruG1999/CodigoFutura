#![no_std]
use soroban_sdk::{contract, contractimpl, Env, Symbol, String, Address, Vec, Event};

// Contrato de ejemplo mostrando cuándo usar String o Symbol
#[contract]
pub struct EjemploStringSymbol;

#[contractimpl]
impl EjemploStringSymbol {
    // 1️⃣ Key para almacenar balance (Symbol)
    pub fn set_balance(env: Env, user: Address, balance: i128) {
        let key = Symbol::new("balance");
        env.storage().set((key, user), &balance);
    }

    // 2️⃣ Descripción de producto (String)
    pub fn set_descripcion(env: Env, descripcion: String) {
        env.storage().set(Symbol::new("descripcion"), &descripcion);
    }

    // 3️⃣ Estado del contrato: "active", "paused", "finalized" (Symbol)
    pub fn set_estado(env: Env, estado: Symbol) {
        env.storage().set(Symbol::new("estado"), &estado);
    }

    // 4️⃣ Comentario de usuario (String)
    pub fn add_comentario(env: Env, comentario: String) {
        let mut comentarios: Vec<String> = env.storage().get(Symbol::new("comentarios")).unwrap_or(Ok(Vec::new(&env))).unwrap();
        comentarios.push_back(comentario);
        env.storage().set(Symbol::new("comentarios"), &comentarios);
    }

    // 5️⃣ Topic de evento de transferencia (Symbol)
    pub fn emitir_evento_transfer(env: Env, from: Address, to: Address, amount: i128) {
        let topic = Symbol::new("transfer");
        env.events().publish((topic, from, to), amount);
    }

    // 6️⃣ Nombre de token largo (String)
    pub fn set_nombre_token(env: Env, nombre: String) {
        env.storage().set(Symbol::new("nombre_token"), &nombre);
    }

    // 7️⃣ Key para almacenar owner del contrato (Symbol)
    pub fn set_owner(env: Env, owner: Address) {
        env.storage().set(Symbol::new("owner"), &owner);
    }

    // 8️⃣ Mensaje de error personalizado (String)
    pub fn verificar_balance(balance: i128) {
        if balance < 0 {
            let mensaje = String::from_str("Error: el balance no puede ser negativo");
            panic!("{}", mensaje);
        }
    }
}
