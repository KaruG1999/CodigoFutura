# Token CODEBIT - Guía Completa de Implementación 💻

## 🎯 Objetivo del Proyecto: DevPoints

**Problema:** El conocimiento y tiempo de developers en comunidades no es reconocido ni recompensado. Las revisiones de código, mentorías y ayuda técnica son "favores invisibles" sin registro ni valor tangible.

**Solución:** CODEBIT - Un token en Stellar donde **1 CODEBIT = 1 minuto de trabajo de desarrollo verificado**.

### ¿Por qué Stellar?
- ⚡ Transacciones en ~5 segundos
- 💸 Fees de ~$0.00001 (perfecto para micropagos)
- 🔒 Red probada y segura
- 🌍 Accesible globalmente

---

## 📋 Pre-requisitos

### Verificar instalaciones
```bash
# Verificar Rust (debe ser 1.74.0+)
rustc --version

# Verificar Stellar CLI (debe ser 20.0.0+)
stellar --version

# Agregar target WASM si no lo tienes
rustup target add wasm32-unknown-unknown
```

### Configurar cuenta en Testnet
```bash
# Generar nueva identidad (será el admin de DevPoints)
stellar keys generate --name devpoints_admin --network testnet

# Obtener la dirección pública
stellar keys address devpoints_admin

# Fondear cuenta con Friendbot
curl "https://friendbot.stellar.org?addr=TU_DIRECCION_PUBLICA_AQUI"
```

---

## 🚀 Paso 1: Crear el Proyecto

```bash
# Crear proyecto con Stellar CLI
stellar contract new token_codebit

# Navegar al directorio
cd token_codebit
```

**Estructura del proyecto:**
```
token_codebit/
├── Cargo.toml          # Dependencias y configuración
└── src/
    ├── lib.rs          # Contrato principal
    ├── storage.rs      # Tipos de almacenamiento
    ├── errors.rs       # Manejo de errores
    └── test.rs         # Tests unitarios
```

---

## 📦 Paso 2: Configurar Dependencias

### Editar `Cargo.toml`
```toml
[package]
name = "token_codebit"
version = "1.0.0"
edition = "2021"
authors = ["DevPoints Community"]
description = "CODEBIT - Token for valuing developer time and contributions"

[lib]
crate-type = ["cdylib"]

[dependencies]
soroban-sdk = "23.0.2"

[dev-dependencies]
soroban-sdk = { version = "23.0.2", features = ["testutils"] }

[profile.release]
opt-level = "z"           # Optimización de tamaño
overflow-checks = true    # Protección contra overflow
debug = 0
strip = "symbols"
debug-assertions = false
panic = "abort"
codegen-units = 1
lto = true                # Link-Time Optimization
```

**Optimizaciones clave:**
- `opt-level = "z"`: Minimiza tamaño del WASM (reduce fees de storage)
- `overflow-checks = true`: Seguridad contra desbordamientos numéricos
- `lto = true`: Optimización agresiva entre módulos

---

## 🗂️ Paso 3: Crear Tipos de Almacenamiento

### Crear `src/storage.rs`
```rust
use soroban_sdk::{contracttype, Address, String};

/// Claves para almacenar datos en el ledger
/// Diseñadas específicamente para el sistema DevPoints
#[contracttype]
pub enum DataKey {
    Balance(Address),              // CODEBIT balance de cada developer
    Allowance(Address, Address),   // Permisos para micro-bounties
    TotalSupply,                   // Supply total de CODEBIT
    Admin,                         // Admin de DevPoints (emisor inicial)
    TokenName,                     // "DevPoints CODEBIT"
    TokenSymbol,                   // "CODEBIT"
    Decimals,                      // 0 (1 CODEBIT = 1 minuto exacto)
    Initialized,                   // Flag de inicialización
}

/// Metadata del token CODEBIT
#[contracttype]
#[derive(Clone)]
pub struct TokenMetadata {
    pub name: String,
    pub symbol: String,
    pub decimals: u32,
}
```

**Decisión de diseño para DevPoints:**
- **Decimals = 0**: No queremos fracciones. 1 CODEBIT = 1 minuto completo de trabajo.
- **Balance por Address**: Cada developer tiene su propio balance verificable on-chain.
- **Allowance**: Permite que plataformas automaticen el pago de bounties.

---

## ⚠️ Paso 4: Definir Errores

### Crear `src/errors.rs`
```rust
use soroban_sdk::contracterror;

/// Errores específicos del sistema DevPoints
#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum CodebitError {
    AlreadyInitialized = 1,      // DevPoints ya fue inicializado
    InvalidAmount = 2,           // CODEBIT debe ser > 0
    InsufficientBalance = 3,     // No tienes suficientes CODEBIT
    InsufficientAllowance = 4,   // Allowance insuficiente para bounty
    NotInitialized = 5,          // Contrato sin inicializar
    InvalidDecimals = 6,         // Decimales debe ser 0
    OverflowError = 7,           // Overflow en operación
    InvalidRecipient = 8,        // No puedes transferirte a ti mismo
    InvalidMetadata = 9,         // Metadata inválido
}
```

**Por qué estos errores importan:**
- `InsufficientBalance`: Protege a developers de gastar más CODEBIT del que tienen
- `InvalidAmount`: Evita bounties de 0 minutos (spam)
- `OverflowError`: Seguridad crítica para evitar exploits numéricos

---

## 💻 Paso 5: Implementar el Contrato CODEBIT

### Crear `src/lib.rs`

```rust
#![no_std]

use soroban_sdk::{
    contract, contractimpl, Address, Env, String, 
    symbol_short, Symbol
};

mod storage;
mod errors;

use storage::{DataKey, TokenMetadata};
use errors::CodebitError;

/// Constantes del sistema DevPoints
const MAX_DECIMALS: u32 = 0;  // IMPORTANTE: 0 decimales (1 CODEBIT = 1 minuto)
const MAX_NAME_LENGTH: u32 = 100;
const MAX_SYMBOL_LENGTH: u32 = 32;

/// Trait que define la interfaz del token CODEBIT según CAP-46
pub trait CodebitTrait {
    /// Inicializa DevPoints con metadata del token
    fn initialize(
        env: Env, 
        admin: Address, 
        name: String, 
        symbol: String,
        decimals: u32
    ) -> Result<(), CodebitError>;
    
    /// Mintea CODEBIT (solo admin puede hacerlo inicialmente)
    /// Se usa para crear el supply inicial de la comunidad
    fn mint(env: Env, to: Address, amount: i128) -> Result<(), CodebitError>;
    
    /// Quema CODEBIT (cualquier holder puede quemar sus propios tokens)
    fn burn(env: Env, from: Address, amount: i128) -> Result<(), CodebitError>;
    
    /// Consulta balance de CODEBIT de un developer
    fn balance(env: Env, account: Address) -> i128;
    
    /// Transfiere CODEBIT entre developers (pago directo)
    fn transfer(
        env: Env, 
        from: Address, 
        to: Address, 
        amount: i128
    ) -> Result<(), CodebitError>;
    
    /// Aprueba a una plataforma/bot para mover CODEBIT (micro-bounties automáticos)
    fn approve(
        env: Env, 
        from: Address, 
        spender: Address, 
        amount: i128
    ) -> Result<(), CodebitError>;
    
    /// Consulta allowance (cuánto puede gastar un spender)
    fn allowance(env: Env, from: Address, spender: Address) -> i128;
    
    /// Transfiere CODEBIT en nombre de otro (usado por plataformas de bounties)
    fn transfer_from(
        env: Env, 
        spender: Address, 
        from: Address, 
        to: Address, 
        amount: i128
    ) -> Result<(), CodebitError>;
    
    // Métodos de consulta (metadata del token)
    fn name(env: Env) -> String;
    fn symbol(env: Env) -> String;
    fn decimals(env: Env) -> u32;
    fn total_supply(env: Env) -> i128;
    fn admin(env: Env) -> Address;
}

#[contract]
pub struct TokenCodebit;

#[contractimpl]
impl CodebitTrait for TokenCodebit {
    fn initialize(
        env: Env, 
        admin: Address, 
        name: String, 
        symbol: String,
        decimals: u32
    ) -> Result<(), CodebitError> {
        // 1. Verificar que DevPoints no esté inicializado
        if env.storage().instance().has(&DataKey::Initialized) {
            return Err(CodebitError::AlreadyInitialized);
        }
        
        // 2. CRÍTICO: Validar decimales = 0 (regla de DevPoints)
        if decimals != 0 {
            return Err(CodebitError::InvalidDecimals);
        }
        
        // 3. Validar metadatos
        if name.len() == 0 || name.len() > MAX_NAME_LENGTH {
            return Err(CodebitError::InvalidMetadata);
        }
        
        if symbol.len() == 0 || symbol.len() > MAX_SYMBOL_LENGTH {
            return Err(CodebitError::InvalidMetadata);
        }
        
        // 4. Guardar configuración de DevPoints en instance storage
        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::TokenName, &name);
        env.storage().instance().set(&DataKey::TokenSymbol, &symbol);
        env.storage().instance().set(&DataKey::Decimals, &decimals);
        env.storage().instance().set(&DataKey::TotalSupply, &0i128);
        env.storage().instance().set(&DataKey::Initialized, &true);
        
        // 5. Extender TTL para que DevPoints persista
        env.storage().instance().extend_ttl(100_000, 200_000);
        
        // 6. Emitir evento de lanzamiento de DevPoints
        env.events().publish(
            (symbol_short!("init"), admin.clone()),
            TokenMetadata {
                name: name.clone(),
                symbol: symbol.clone(),
                decimals,
            }
        );
        
        Ok(())
    }
    
    fn mint(env: Env, to: Address, amount: i128) -> Result<(), CodebitError> {
        // 1. Verificar que DevPoints esté inicializado
        if !env.storage().instance().has(&DataKey::Initialized) {
            return Err(CodebitError::NotInitialized);
        }
        
        // 2. Solo el admin puede mintear (control inicial del supply)
        let admin: Address = env.storage().instance()
            .get(&DataKey::Admin)
            .ok_or(CodebitError::NotInitialized)?;
        admin.require_auth();
        
        // 3. Validaciones: CODEBIT debe ser > 0
        if amount <= 0 {
            return Err(CodebitError::InvalidAmount);
        }
        
        // 4. Obtener balance actual y verificar overflow
        let balance = Self::balance(env.clone(), to.clone());
        let new_balance = balance.checked_add(amount)
            .ok_or(CodebitError::OverflowError)?;
        
        // 5. Actualizar balance con TTL extendido
        env.storage().persistent().set(
            &DataKey::Balance(to.clone()), 
            &new_balance
        );
        env.storage().persistent().extend_ttl(
            &DataKey::Balance(to.clone()),
            100_000,
            200_000
        );
        
        // 6. Actualizar total supply de CODEBIT
        let total: i128 = env.storage().instance()
            .get(&DataKey::TotalSupply)
            .unwrap_or(0);
        let new_total = total.checked_add(amount)
            .ok_or(CodebitError::OverflowError)?;
        env.storage().instance().set(
            &DataKey::TotalSupply, 
            &new_total
        );
        
        // 7. Emitir evento de mint (registro en blockchain)
        env.events().publish(
            (symbol_short!("mint"), to.clone()), 
            (amount, new_balance, new_total)
        );
        
        Ok(())
    }
    
    fn burn(env: Env, from: Address, amount: i128) -> Result<(), CodebitError> {
        // 1. Verificar inicialización
        if !env.storage().instance().has(&DataKey::Initialized) {
            return Err(CodebitError::NotInitialized);
        }
        
        // 2. Solo el dueño puede quemar sus propios CODEBIT
        from.require_auth();
        
        // 3. Validaciones
        if amount <= 0 {
            return Err(CodebitError::InvalidAmount);
        }
        
        let balance = Self::balance(env.clone(), from.clone());
        if balance < amount {
            return Err(CodebitError::InsufficientBalance);
        }
        
        // 4. Actualizar balance
        let new_balance = balance - amount;
        if new_balance == 0 {
            // Optimización: eliminar key si balance = 0
            env.storage().persistent().remove(&DataKey::Balance(from.clone()));
        } else {
            env.storage().persistent().set(
                &DataKey::Balance(from.clone()),
                &new_balance
            );
            env.storage().persistent().extend_ttl(
                &DataKey::Balance(from.clone()),
                100_000,
                200_000
            );
        }
        
        // 5. Reducir total supply
        let total: i128 = env.storage().instance()
            .get(&DataKey::TotalSupply)
            .unwrap_or(0);
        let new_total = total.checked_sub(amount)
            .ok_or(CodebitError::OverflowError)?;
        env.storage().instance().set(
            &DataKey::TotalSupply,
            &new_total
        );
        
        // 6. Emitir evento de burn
        env.events().publish(
            (symbol_short!("burn"), from),
            (amount, new_balance, new_total)
        );
        
        Ok(())
    }
    
    fn balance(env: Env, account: Address) -> i128 {
        env.storage().persistent()
            .get(&DataKey::Balance(account))
            .unwrap_or(0)
    }
    
    fn transfer(
        env: Env, 
        from: Address, 
        to: Address, 
        amount: i128
    ) -> Result<(), CodebitError> {
        // 1. Verificar que DevPoints esté activo
        if !env.storage().instance().has(&DataKey::Initialized) {
            return Err(CodebitError::NotInitialized);
        }
        
        // 2. Verificar que el sender autoriza la transferencia
        from.require_auth();
        
        // 3. Validaciones de negocio
        if amount <= 0 {
            return Err(CodebitError::InvalidAmount);
        }
        
        // No permitir transferirse a sí mismo (optimización)
        if from == to {
            return Err(CodebitError::InvalidRecipient);
        }
        
        let from_balance = Self::balance(env.clone(), from.clone());
        if from_balance < amount {
            return Err(CodebitError::InsufficientBalance);
        }
        
        // 4. Calcular nuevos balances con protección de overflow
        let new_from_balance = from_balance - amount;
        let to_balance = Self::balance(env.clone(), to.clone());
        let new_to_balance = to_balance.checked_add(amount)
            .ok_or(CodebitError::OverflowError)?;
        
        // 5. Actualizar balances atómicamente
        if new_from_balance == 0 {
            env.storage().persistent().remove(&DataKey::Balance(from.clone()));
        } else {
            env.storage().persistent().set(
                &DataKey::Balance(from.clone()),
                &new_from_balance
            );
            env.storage().persistent().extend_ttl(
                &DataKey::Balance(from.clone()),
                100_000,
                200_000
            );
        }
        
        env.storage().persistent().set(
            &DataKey::Balance(to.clone()),
            &new_to_balance
        );
        env.storage().persistent().extend_ttl(
            &DataKey::Balance(to.clone()),
            100_000,
            200_000
        );
        
        // 6. Emitir evento de transferencia (traza en blockchain)
        env.events().publish(
            (symbol_short!("transfer"), from, to), 
            (amount, new_from_balance, new_to_balance)
        );
        
        Ok(())
    }
    
    fn approve(
        env: Env, 
        from: Address, 
        spender: Address, 
        amount: i128
    ) -> Result<(), CodebitError> {
        // 1. Verificar inicialización
        if !env.storage().instance().has(&DataKey::Initialized) {
            return Err(CodebitError::NotInitialized);
        }
        
        // 2. Solo el dueño puede aprobar
        from.require_auth();
        
        // 3. Validación (permitir 0 para revocar)
        if amount < 0 {
            return Err(CodebitError::InvalidAmount);
        }
        
        // 4. Obtener allowance anterior
        let old_allowance = Self::allowance(env.clone(), from.clone(), spender.clone());
        
        // 5. Actualizar allowance
        if amount == 0 {
            env.storage().persistent().remove(
                &DataKey::Allowance(from.clone(), spender.clone())
            );
        } else {
            env.storage().persistent().set(
                &DataKey::Allowance(from.clone(), spender.clone()),
                &amount
            );
            env.storage().persistent().extend_ttl(
                &DataKey::Allowance(from.clone(), spender.clone()),
                100_000,
                200_000
            );
        }
        
        // 6. Emitir evento de approve
        env.events().publish(
            (symbol_short!("approve"), from, spender),
            (old_allowance, amount)
        );
        
        Ok(())
    }
    
    fn allowance(env: Env, from: Address, spender: Address) -> i128 {
        env.storage().persistent()
            .get(&DataKey::Allowance(from, spender))
            .unwrap_or(0)
    }
    
    fn transfer_from(
        env: Env, 
        spender: Address, 
        from: Address, 
        to: Address, 
        amount: i128
    ) -> Result<(), CodebitError> {
        // 1. Verificar inicialización
        if !env.storage().instance().has(&DataKey::Initialized) {
            return Err(CodebitError::NotInitialized);
        }
        
        // 2. Verificar que el spender (plataforma) está autorizado
        spender.require_auth();
        
        // 3. Validaciones
        if amount <= 0 {
            return Err(CodebitError::InvalidAmount);
        }
        
        if from == to {
            return Err(CodebitError::InvalidRecipient);
        }
        
        // 4. Verificar allowance (permiso delegado)
        let allowed = Self::allowance(env.clone(), from.clone(), spender.clone());
        if allowed < amount {
            return Err(CodebitError::InsufficientAllowance);
        }
        
        // 5. Verificar balance suficiente
        let from_balance = Self::balance(env.clone(), from.clone());
        if from_balance < amount {
            return Err(CodebitError::InsufficientBalance);
        }
        
        // 6. Calcular nuevos valores
        let new_from_balance = from_balance - amount;
        let to_balance = Self::balance(env.clone(), to.clone());
        let new_to_balance = to_balance.checked_add(amount)
            .ok_or(CodebitError::OverflowError)?;
        let new_allowance = allowed - amount;
        
        // 7. Actualizar estado atómicamente
        if new_from_balance == 0 {
            env.storage().persistent().remove(&DataKey::Balance(from.clone()));
        } else {
            env.storage().persistent().set(
                &DataKey::Balance(from.clone()),
                &new_from_balance
            );
            env.storage().persistent().extend_ttl(
                &DataKey::Balance(from.clone()),
                100_000,
                200_000
            );
        }
        
        env.storage().persistent().set(
            &DataKey::Balance(to.clone()),
            &new_to_balance
        );
        env.storage().persistent().extend_ttl(
            &DataKey::Balance(to.clone()),
            100_000,
            200_000
        );
        
        if new_allowance == 0 {
            env.storage().persistent().remove(
                &DataKey::Allowance(from.clone(), spender.clone())
            );
        } else {
            env.storage().persistent().set(
                &DataKey::Allowance(from.clone(), spender.clone()),
                &new_allowance
            );
            env.storage().persistent().extend_ttl(
                &DataKey::Allowance(from.clone(), spender.clone()),
                100_000,
                200_000
            );
        }
        
        // 8. Emitir evento completo
        env.events().publish(
            (symbol_short!("trnsfr_frm"), spender, from.clone(), to.clone()),
            (amount, new_from_balance, new_to_balance, new_allowance)
        );
        
        Ok(())
    }
    
    // Métodos de consulta
    fn name(env: Env) -> String {
        if !env.storage().instance().has(&DataKey::Initialized) {
            return String::from_str(&env, "");
        }
        
        env.storage().instance()
            .get(&DataKey::TokenName)
            .unwrap_or(String::from_str(&env, ""))
    }
    
    fn symbol(env: Env) -> String {
        if !env.storage().instance().has(&DataKey::Initialized) {
            return String::from_str(&env, "");
        }
        
        env.storage().instance()
            .get(&DataKey::TokenSymbol)
            .unwrap_or(String::from_str(&env, ""))
    }
    
    fn decimals(env: Env) -> u32 {
        if !env.storage().instance().has(&DataKey::Initialized) {
            return 0;
        }
        
        env.storage().instance()
            .get(&DataKey::Decimals)
            .unwrap_or(0)
    }
    
    fn total_supply(env: Env) -> i128 {
        env.storage().instance()
            .get(&DataKey::TotalSupply)
            .unwrap_or(0)
    }
    
    fn admin(env: Env) -> Address {
        env.storage().instance()
            .get(&DataKey::Admin)
            .expect("Admin not initialized")
    }
}
```

---

## 🧪 Paso 6: Crear Tests para DevPoints

### Crear `src/test.rs`

```rust
#![cfg(test)]

use super::*;
use soroban_sdk::{Env, String, testutils::Address as _};

#[test]
fn test_initialize_devpoints() {
    let env = Env::default();
    let contract_id = env.register_contract(None, TokenCodebit);
    let client = TokenCodebitClient::new(&env, &contract_id);
    
    let admin = Address::generate(&env);
    
    // Inicializar DevPoints con 0 decimales
    client.initialize(
        &admin,
        &String::from_str(&env, "DevPoints CODEBIT"),
        &String::from_str(&env, "CODEBIT"),
        &0  // CRÍTICO: 0 decimales
    );
    
    assert_eq!(client.symbol(&env), String::from_str(&env, "CODEBIT"));
    assert_eq!(client.decimals(&env), 0);
    assert_eq!(client.total_supply(&env), 0);
}

#[test]
#[should_panic(expected = "InvalidDecimals")]
fn test_reject_decimals() {
    let env = Env::default();
    let contract_id = env.register_contract(None, TokenCodebit);
    let client = TokenCodebitClient::new(&env, &contract_id);
    
    let admin = Address::generate(&env);
    
    // Debe fallar con decimales != 0
    client.initialize(
        &admin,
        &String::from_str(&env, "DevPoints"),
        &String::from_str(&env, "CODEBIT"),
        &7  // Incorrecto para DevPoints
    );
}

#[test]
fn test_mint_codebit_for_contribution() {
    let env = Env::default();
    let contract_id = env.register_contract(None, TokenCodebit);
    let client = TokenCodebitClient::new(&env, &contract_id);
    
    let admin = Address::generate(&env);
    let junior_dev = Address::generate(&env);
    
    env.mock_all_auths();
    
    client.initialize(
        &admin, 
        &String::from_str(&env, "DevPoints CODEBIT"), 
        &String::from_str(&env, "CODEBIT"), 
        &0
    );
    
    // Junior dev ayudó 30 minutos = 30 CODEBIT
    client.mint(&junior_dev, &30);
    
    assert_eq!(client.balance(&junior_dev), 30);
    assert_eq!(client.total_supply(&env), 30);
}

#[test]
fn test_code_review_payment() {
    let env = Env::default();
    let contract_id = env.register_contract(None, TokenCodebit);
    let client = TokenCodebitClient::new(&env, &contract_id);
    
    let admin = Address::generate(&env);
    let junior_dev = Address::generate(&env);
    let senior_dev = Address::generate(&env);
    
    env.mock_all_auths();
    
    client.initialize(
        &admin,
        &String::from_str(&env, "DevPoints CODEBIT"),
        &String::from_str(&env, "CODEBIT"),
        &0
    );
    
    // Junior tiene 1200 CODEBIT (20 horas de ayuda)
    client.mint(&junior_dev, &1200);
    
    // Junior paga 30 CODEBIT por revisión de código (30 min)
    client.transfer(&junior_dev, &senior_dev, &30);
    
    assert_eq!(client.balance(&junior_dev), 1170);
    assert_eq!(client.balance(&senior_dev), 30);
}

#[test]
fn test_bounty_system_with_approve() {
    let env = Env::default();
    let contract_id = env.register_contract(None, TokenCodebit);
    let client = TokenCodebitClient::new(&env, &contract_id);
    
    let admin = Address::generate(&env);
    let requester = Address::generate(&env);
    let solver = Address::generate(&env);
    let bounty_platform = Address::generate(&env);
    
    env.mock_all_auths();
    
    client.initialize(
        &admin,
        &String::from_str(&env, "DevPoints CODEBIT"),
        &String::from_str(&env, "CODEBIT"),
        &0
    );
    
    // Requester tiene 500 CODEBIT
    client.mint(&requester, &500);
    
    // Requester aprueba plataforma para gestionar micro-bounties
    client.approve(&requester, &bounty_platform, &100);
    assert_eq!(client.allowance(&requester, &bounty_platform), 100);
    
    // Plataforma paga automáticamente 45 CODEBIT al solver (45 min de trabajo)
    client.transfer_from(&bounty_platform, &requester, &solver, &45);
    
    assert_eq!(client.balance(&requester), 455);
    assert_eq!(client.balance(&solver), 45);
    assert_eq!(client.allowance(&requester, &bounty_platform), 55);
}

#[test]
fn test_burn_unused_codebit() {
    let env = Env::default();
    let contract_id = env.register_contract(None, TokenCodebit);
    let client = TokenCodebitClient::new(&env, &contract_id);
    
    let admin = Address::generate(&env);
    let developer = Address::generate(&env);
    
    env.mock_all_auths();
    
    client.initialize(
        &admin,
        &String::from_str(&env, "DevPoints CODEBIT"),
        &String::from_str(&env, "CODEBIT"),
        &0
    );
    
    client.mint(&developer, &1000);
    client.burn(&developer, &300);
    
    assert_eq!(client.balance(&developer), 700);
    assert_eq!(client.total_supply(&env), 700);
}

#[test]
#[should_panic(expected = "InsufficientBalance")]
fn test_cannot_spend_more_than_earned() {
    let env = Env::default();
    let contract_id = env.register_contract(None, TokenCodebit);
    let client = TokenCodebitClient::new(&env, &contract_id);
    
    let admin = Address::generate(&env);
    let junior_dev = Address::generate(&env);
    let senior_dev = Address::generate(&env);
    
    env.mock_all_auths();
    
    client.initialize(
        &admin,
        &String::from_str(&env, "DevPoints CODEBIT"),
        &String::from_str(&env, "CODEBIT"),
        &0
    );
    
    // Junior solo tiene 10 CODEBIT
    client.mint(&junior_dev, &10);
    
    // Intenta pagar 100 CODEBIT (debe fallar)
    client.transfer(&junior_dev, &senior_dev, &100);
}
```

### Ejecutar Tests
```bash
cargo test
```

**Resultado esperado:**
```
running 7 tests
test test_initialize_devpoints ... ok
test test_reject_decimals ... ok
test test_mint_codebit_for_contribution ... ok
test test_code_review_payment ... ok
test test_bounty_system_with_approve ... ok
test test_burn_unused_codebit ... ok
test test_cannot_spend_more_than_earned ... ok

test result: ok. 7 passed; 0 failed
```

---

## 🔨 Paso 7: Compilar el Contrato

### Usando Stellar CLI (recomendado)
```bash
stellar contract build
```

**Salida esperada:**
```
   Compiling soroban-sdk v23.0.2
   Compiling token_codebit v1.0.0
    Finished release [optimized] target(s) in 45.32s
```

**Archivo generado:**
```
target/wasm32-unknown-unknown/release/token_codebit.wasm
```

### Optimizar WASM (Opcional pero recomendado)
```bash
# Instalar wasm-opt (solo primera vez)
cargo install wasm-opt

# Optimizar para producción
wasm-opt -Oz \
  target/wasm32-unknown-unknown/release/token_codebit.wasm \
  -o target/wasm32-unknown-unknown/release/token_codebit_optimized.wasm
```

---

## 🚢 Paso 8: Deploy a Testnet

### 1. Deploy del Contrato CODEBIT
```bash
stellar contract deploy \
  --wasm target/wasm32-unknown-unknown/release/token_codebit.wasm \
  --source devpoints_admin \
  --network testnet
```

**Guarda el CONTRACT_ID que devuelve:**
```
CONTRACT_ID: CAEXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX
```

### 2. Guardar Contract ID
```bash
# Guardar en variable de entorno
export CODEBIT_CONTRACT_ID=CAEXXXXXX...

# Guardar en archivo
echo "CAEXXXXXX..." > codebit_contract_id.txt
```

### 3. Inicializar DevPoints
```bash
stellar contract invoke \
  --id $CODEBIT_CONTRACT_ID \
  --source devpoints_admin \
  --network testnet \
  -- initialize \
  --admin $(stellar keys address devpoints_admin) \
  --name "DevPoints CODEBIT" \
  --symbol "CODEBIT" \
  --decimals 0
```

**⚠️ IMPORTANTE:** `decimals` DEBE ser 0 (1 CODEBIT = 1 minuto exacto)

### 4. Crear Supply Inicial de DevPoints
```bash
# Mintear 100,000 CODEBIT iniciales para la comunidad (100,000 minutos = ~1,666 horas)
stellar contract invoke \
  --id $CODEBIT_CONTRACT_ID \
  --source devpoints_admin \
  --network testnet \
  -- mint \
  --to $(stellar keys address devpoints_admin) \
  --amount 100000
```

### 5. Verificar Balance
```bash
stellar contract invoke \
  --id $CODEBIT_CONTRACT_ID \
  --source devpoints_admin \
  --network testnet \
  -- balance \
  --account $(stellar keys address devpoints_admin)
```

### 6. Verificar en Stellar Expert
```bash
# Abrir en navegador
echo "https://stellar.expert/explorer/testnet/contract/$CODEBIT_CONTRACT_ID"
```

---

## 🧾 Paso 9: Script de Deploy Automatizado

### Crear `deploy_devpoints.sh`
```bash
#!/bin/bash

# Colores para output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
PURPLE='\033[0;35m'
NC='\033[0m' # No Color

echo -e "${PURPLE}🚀 Deploying DevPoints - CODEBIT Token...${NC}\n"

# 1. Build
echo -e "${GREEN}📦 Building contract...${NC}"
stellar contract build

if [ $? -ne 0 ]; then
    echo -e "${YELLOW}⚠️  Build failed. Please fix errors and try again.${NC}"
    exit 1
fi

# 2. Deploy
echo -e "${GREEN}🚢 Deploying to testnet...${NC}"
CONTRACT_ID=$(stellar contract deploy \
  --wasm target/wasm32-unknown-unknown/release/token_codebit.wasm \
  --source devpoints_admin \
  --network testnet)

if [ $? -ne 0 ]; then
    echo -e "${YELLOW}⚠️  Deploy failed.${NC}"
    exit 1
fi

echo -e "Contract ID: ${BLUE}$CONTRACT_ID${NC}"
echo "$CONTRACT_ID" > codebit_contract_id.txt

# 3. Initialize DevPoints
echo -e "${GREEN}⚙️  Initializing DevPoints...${NC}"
stellar contract invoke \
  --id $CONTRACT_ID \
  --source devpoints_admin \
  --network testnet \
  -- initialize \
  --admin $(stellar keys address devpoints_admin) \
  --name "DevPoints CODEBIT" \
  --symbol "CODEBIT" \
  --decimals 0

if [ $? -ne 0 ]; then
    echo -e "${YELLOW}⚠️  Initialization failed.${NC}"
    exit 1
fi

# 4. Mint initial community supply
echo -e "${GREEN}💰 Minting initial community supply (100,000 CODEBIT)...${NC}"
stellar contract invoke \
  --id $CONTRACT_ID \
  --source devpoints_admin \
  --network testnet \
  -- mint \
  --to $(stellar keys address devpoints_admin) \
  --amount 100000

if [ $? -ne 0 ]; then
    echo -e "${YELLOW}⚠️  Minting failed.${NC}"
    exit 1
fi

# 5. Verify
echo -e "${GREEN}✅ Verifying balance...${NC}"
BALANCE=$(stellar contract invoke \
  --id $CONTRACT_ID \
  --source devpoints_admin \
  --network testnet \
  -- balance \
  --account $(stellar keys address devpoints_admin))

echo -e "Community Balance: ${BLUE}$BALANCE CODEBIT${NC}"
echo -e "Equivalent to: ${BLUE}$(($BALANCE / 60)) hours${NC} of developer time"

echo -e "\n${PURPLE}🎉 DevPoints Deployment Complete!${NC}"
echo -e "Contract ID: ${GREEN}$CONTRACT_ID${NC}"
echo -e "Token Symbol: ${GREEN}CODEBIT${NC}"
echo -e "Decimals: ${GREEN}0 (1 CODEBIT = 1 minute)${NC}"
echo -e "Initial Supply: ${GREEN}100,000 CODEBIT (~1,666 hours)${NC}"
echo -e "\nExplorer: ${BLUE}https://stellar.expert/explorer/testnet/contract/$CONTRACT_ID${NC}"
echo -e "\n${PURPLE}💡 Next Steps:${NC}"
echo -e "1. Create developer accounts"
echo -e "2. Distribute CODEBIT for contributions"
echo -e "3. Test bounty system"
echo -e "4. Build frontend integration"
```

### Ejecutar script
```bash
chmod +x deploy_devpoints.sh
./deploy_devpoints.sh
```

---

## 🧪 Paso 10: Simular Casos de Uso de DevPoints

### Caso 1: Recompensar Ayuda en Discord

```bash
# 1. Crear cuenta para junior dev que ayudó
stellar keys generate --name alice_junior --network testnet
curl "https://friendbot.stellar.org?addr=$(stellar keys address alice_junior)"

# 2. Alice ayudó 45 minutos resolviendo dudas = 45 CODEBIT
stellar contract invoke \
  --id $CODEBIT_CONTRACT_ID \
  --source devpoints_admin \
  --network testnet \
  -- mint \
  --to $(stellar keys address alice_junior) \
  --amount 45

# 3. Verificar balance de Alice
stellar contract invoke \
  --id $CODEBIT_CONTRACT_ID \
  --source devpoints_admin \
  --network testnet \
  -- balance \
  --account $(stellar keys address alice_junior)
```

### Caso 2: Pagar por Code Review (30 minutos)

```bash
# 1. Crear cuenta de senior dev
stellar keys generate --name bob_senior --network testnet
curl "https://friendbot.stellar.org?addr=$(stellar keys address bob_senior)"

# 2. Alice acumuló 1,200 CODEBIT ayudando a otros (20 horas)
stellar contract invoke \
  --id $CODEBIT_CONTRACT_ID \
  --source devpoints_admin \
  --network testnet \
  -- mint \
  --to $(stellar keys address alice_junior) \
  --amount 1200

# 3. Alice paga 30 CODEBIT a Bob por revisión de código
stellar contract invoke \
  --id $CODEBIT_CONTRACT_ID \
  --source alice_junior \
  --network testnet \
  -- transfer \
  --from $(stellar keys address alice_junior) \
  --to $(stellar keys address bob_senior) \
  --amount 30

# 4. Verificar balances
echo "Alice balance:"
stellar contract invoke \
  --id $CODEBIT_CONTRACT_ID \
  --source alice_junior \
  --network testnet \
  -- balance \
  --account $(stellar keys address alice_junior)

echo "Bob balance:"
stellar contract invoke \
  --id $CODEBIT_CONTRACT_ID \
  --source alice_junior \
  --network testnet \
  -- balance \
  --account $(stellar keys address bob_senior)
```

### Caso 3: Sistema de Micro-Bounties Automático

```bash
# 1. Crear cuenta de plataforma de bounties
stellar keys generate --name bounty_platform --network testnet
curl "https://friendbot.stellar.org?addr=$(stellar keys address bounty_platform)"

# 2. Crear cuenta de solver
stellar keys generate --name charlie_solver --network testnet
curl "https://friendbot.stellar.org?addr=$(stellar keys address charlie_solver)"

# 3. Alice aprueba a la plataforma para gestionar 500 CODEBIT
stellar contract invoke \
  --id $CODEBIT_CONTRACT_ID \
  --source alice_junior \
  --network testnet \
  -- approve \
  --from $(stellar keys address alice_junior) \
  --spender $(stellar keys address bounty_platform) \
  --amount 500

# 4. Verificar allowance
stellar contract invoke \
  --id $CODEBIT_CONTRACT_ID \
  --source alice_junior \
  --network testnet \
  -- allowance \
  --from $(stellar keys address alice_junior) \
  --spender $(stellar keys address bounty_platform)

# 5. Plataforma paga automáticamente 60 CODEBIT al solver (1 hora de trabajo)
stellar contract invoke \
  --id $CODEBIT_CONTRACT_ID \
  --source bounty_platform \
  --network testnet \
  -- transfer_from \
  --spender $(stellar keys address bounty_platform) \
  --from $(stellar keys address alice_junior) \
  --to $(stellar keys address charlie_solver) \
  --amount 60

# 6. Verificar nuevo allowance (debería ser 440)
stellar contract invoke \
  --id $CODEBIT_CONTRACT_ID \
  --source alice_junior \
  --network testnet \
  -- allowance \
  --from $(stellar keys address alice_junior) \
  --spender $(stellar keys address bounty_platform)
```

### Caso 4: Verificar Portfolio On-Chain

```bash
# Ver historial completo de transacciones de Alice en Stellar Expert
echo "https://stellar.expert/explorer/testnet/account/$(stellar keys address alice_junior)/operations"

# Ver balance total ganado (su reputación verificable)
stellar contract invoke \
  --id $CODEBIT_CONTRACT_ID \
  --source alice_junior \
  --network testnet \
  -- balance \
  --account $(stellar keys address alice_junior)
```

---

## ✅ Checklist de Implementación DevPoints

### Contrato
- [x] ✅ Proyecto creado (`token_codebit`)
- [x] ✅ Cargo.toml configurado con soroban-sdk 23.0.2
- [x] ✅ storage.rs con DataKey para DevPoints
- [x] ✅ errors.rs con CodebitError
- [x] ✅ lib.rs con todas las funciones CAP-46
- [x] ✅ **Decimals configurado en 0** (crítico)
- [x] ✅ Tests ejecutados exitosamente

### Deploy
- [ ] ✅ Compilado sin warnings
- [ ] ✅ Desplegado en testnet
- [ ] ✅ CONTRACT_ID guardado
- [ ] ✅ Token inicializado con nombre "DevPoints CODEBIT"
- [ ] ✅ Symbol "CODEBIT" configurado
- [ ] ✅ Supply inicial minteado (100,000 CODEBIT)
- [ ] ✅ Visible en Stellar Expert

### Casos de Uso Probados
- [ ] ✅ Mint de CODEBIT por contribuciones
- [ ] ✅ Transfer directo (pago por code review)
- [ ] ✅ Approve para plataformas de bounties
- [ ] ✅ Transfer_from para pagos automáticos
- [ ] ✅ Verificación de portfolio on-chain

### Preparación Frontend
- [ ] ✅ Node.js 18+ instalado
- [ ] ✅ Freighter Wallet instalada
- [ ] ✅ Cuenta importada en Freighter
- [ ] ✅ CODEBIT disponible en cuenta de prueba

---

## 🎯 Métricas de Éxito de DevPoints

### Métricas On-Chain (verificables en blockchain)

```bash
# Total supply circulante
stellar contract invoke \
  --id $CODEBIT_CONTRACT_ID \
  --source devpoints_admin \
  --network testnet \
  -- total_supply

# Balance de un developer (su reputación)
stellar contract invoke \
  --id $CODEBIT_CONTRACT_ID \
  --source devpoints_admin \
  --network testnet \
  -- balance \
  --account $(stellar keys address alice_junior)
```

### Interpretación de Balances

| Balance CODEBIT | Equivalente | Significado |
|-----------------|-------------|-------------|
| 100 | 1.67 horas | Junior activo |
| 600 | 10 horas | Contributor regular |
| 1,200 | 20 horas | **Alice's goal** - Suficiente para mentoría Senior |
| 3,000 | 50 horas | Top contributor |
| 10,000+ | 166+ horas | Senior verificado con alto impacto |

---

## 🐛 Troubleshooting Específico DevPoints

### Error: "InvalidDecimals"
**Causa:** Intentaste inicializar con decimales != 0

**Solución:**
```bash
# SIEMPRE usar decimals 0 para DevPoints
-- decimals 0
```

### Error: No puedo transferir CODEBIT
**Causa:** Balance insuficiente

**Verificación:**
```bash
# Ver tu balance actual
stellar contract invoke \
  --id $CODEBIT_CONTRACT_ID \
  --source TU_CUENTA \
  --network testnet \
  -- balance \
  --account $(stellar keys address TU_CUENTA)
```

### Error: "InsufficientAllowance" en bounty
**Causa:** No aprobaste suficiente CODEBIT a la plataforma

**Solución:**
```bash
# Aumentar allowance
stellar contract invoke \
  --id $CODEBIT_CONTRACT_ID \
  --source TU_CUENTA \
  --network testnet \
  -- approve \
  --from $(stellar keys address TU_CUENTA) \
  --spender $(stellar keys address PLATAFORMA) \
  --amount CANTIDAD_NECESARIA
```

---

## 📊 Roadmap Post-Deploy

### Fase 1: MVP Testnet (Actual) ✅
- [x] Token CODEBIT desplegado
- [x] Sistema de mint/transfer/approve funcional
- [x] Tests de casos de uso completados

### Fase 2: Frontend (Próxima Clase)
- [ ] Dashboard para ver balance de CODEBIT
- [ ] Interfaz para crear/resolver bounties
- [ ] Integración con Freighter Wallet
- [ ] Historial de transacciones visualizado
- [ ] Perfil de developer con reputación

### Fase 3: Comunidad Beta (1-2 meses)
- [ ] Invitar 50 developers a testnet
- [ ] Sistema de verificación de contribuciones
- [ ] Bot de Discord para distribuir CODEBIT
- [ ] Leaderboard de top contributors

### Fase 4: Mainnet (3-6 meses)
- [ ] Auditoría de seguridad completa
- [ ] Deploy a mainnet de Stellar
- [ ] Integración con plataformas de learning
- [ ] Partnerships con bootcamps

---

## 💡 Ideas de Expansión DevPoints

### 1. Bot de Discord para Distribución Automática
```bash
# Concepto: Bot que detecta ayuda y mintea CODEBIT automáticamente
# Ejemplo: /devpoints reward @alice 30 "Helped debug React issue"
```

### 2. Badges NFT por Hitos
- 100 CODEBIT ganados → "Helper" Badge
- 1,000 CODEBIT → "Contributor" Badge
- 10,000 CODEBIT → "Mentor" Badge

### 3. Integración con GitHub
- Pull request mergeado → CODEBIT automático
- Code review completado → CODEBIT al reviewer
- Issue resuelto → CODEBIT al solver

### 4. Marketplace de Servicios
- "Pago 50 CODEBIT por revisión de portfolio"
- "Ofrezco mentoría 1:1 por 120 CODEBIT/hora"
- "Necesito ayuda con smart contracts - 200 CODEBIT"

---

## 🔐 Seguridad y Mejores Prácticas

### Para Developers que Ganan CODEBIT
1. **Guarda tu secret key segura** - Es tu portfolio verificable
2. **No compartas tu cuenta** - Tu balance es tu reputación
3. **Verifica transacciones** - Usa Stellar Expert para auditar
4. **Reporta bugs** - Si encuentras exploits, contacta al admin

### Para Admin de DevPoints
1. **Control de mint:** Solo mintear por contribuciones verificadas
2. **Rate limiting:** Implementar verificación manual de grandes amounts
3. **Transparencia:** Publicar logs de todos los mints
4. **Governance:** Considerar DAO para decisiones comunitarias

### Prevención de Abuse
```bash
# Implementar en frontend:
# - Verificación de identidad (GitHub, Discord)
# - Límite máximo de CODEBIT por día
# - Sistema de reportes de abuse
# - Multisig para mints grandes
```

---

## 📚 Recursos Adicionales

### Documentación Técnica
- **CAP-46 Spec:** https://stellar.org/protocol/cap-46
- **Soroban Docs:** https://soroban.stellar.org
- **Stellar SDK:** https://docs.rs/soroban-sdk/23.0.2

### Herramientas
- **Stellar Expert:** https://stellar.expert
- **Freighter Wallet:** https://www.freighter.app
- **Friendbot:** https://friendbot.stellar.org

### Comunidad
- **Stellar Discord:** https://discord.gg/stellardev
- **DevPoints Telegram:** [Próximamente]
- **GitHub Repo:** [Próximamente]

---

## 🎓 Conceptos Clave Aprendidos

### 1. Tokenomics de DevPoints
- **1 CODEBIT = 1 minuto** (decimals = 0)
- Supply controlado por contribuciones verificadas
- Transferible peer-to-peer sin intermediarios
- Historial inmutable en blockchain

### 2. Casos de Uso Blockchain Real
- Micropagos instantáneos ($0.00001 fee)
- Portfolio verificable on-chain
- Sistema de reputación descentralizado
- Incentivos económicos para comunidades

### 3. Patrones de Smart Contracts
- Authorization-first security
- Overflow protection (checked operations)
- Event-driven architecture
- TTL management para persistencia

### 4. Integración Web3
- Wallets como identidad (Freighter)
- Transacciones firmadas por usuarios
- Frontend conectado a blockchain
- UX de Web2 con beneficios de Web3

---

## 🦈 Mensaje Final: Tu Portfolio Blockchain

**¡Felicitaciones!** Has creado más que un token. Has creado un **sistema de reputación verificable** que resuelve un problema real.

### Lo que construiste:
✅ Un token fungible en Stellar (CODEBIT)  
✅ Un sistema de incentivos para comunidades dev  
✅ Un portfolio on-chain inmutable  
✅ Una economía descentralizada funcional  

### Tu impacto potencial:
- 🌍 **Democratizar acceso a mentoría** (sin barreras económicas)
- 💼 **Nuevo tipo de CV** (verificado en blockchain)
- 🤝 **Comunidades más colaborativas** (trabajo recompensado)
- 🚀 **Oportunidades laborales** (reclutadores ven tu CODEBIT balance)

### Próximos pasos:
1. **Prueba tu token** con amigos en testnet
2. **Documenta casos de uso** reales
3. **Construye frontend** en Clase 6
4. **Comparte tu visión** en redes sociales

### Recuerda:
> "El mejor momento para plantar un árbol fue hace 20 años. El segundo mejor momento es ahora. Tu CODEBIT balance es la prueba verificable de que estás plantando árboles en la comunidad dev."

**¡Tu portfolio blockchain comienza hoy! 💻🌟**

---

## 📞 Soporte DevPoints

**¿Preguntas sobre el proyecto?**  
Telegram: [Tu canal]

**¿Encontraste un bug?**  
GitHub Issues: [Tu repo]

**¿Quieres contribuir?**  
Pull requests bienvenidos!

---

**Versión:** 1.0.0  
**Deploy Date:** Octubre 2025  
**Network:** Stellar Testnet  
**Token:** CODEBIT  
**Smart Contract:** CAP-46 Compatible  

---

# 🚀 ¡Bienvenida a DevPoints - Donde tu conocimiento tiene valor verificable!