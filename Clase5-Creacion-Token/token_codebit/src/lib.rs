#![no_std]

use soroban_sdk::{
    contract, contractimpl, Address, Env, String
};

mod storage;
mod errors;

use storage::DataKey;
use errors::TokenError as CodebitError;

/// Constantes del sistema DevPoints
const MAX_NAME_LENGTH: u32 = 100;
const MAX_SYMBOL_LENGTH: u32 = 32;
// const MAX_DECIMALS: u32 = 0;  // Nunca se usaba en el código


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
    
    /// Aprueba a una plataforma/bot para mover CODEBIT
    fn approve(
        env: Env, 
        from: Address, 
        spender: Address, 
        amount: i128
    ) -> Result<(), CodebitError>;
    
    /// Consulta allowance
    fn allowance(env: Env, from: Address, spender: Address) -> i128;
    
    /// Transfiere CODEBIT en nombre de otro
    fn transfer_from(
        env: Env, 
        spender: Address, 
        from: Address, 
        to: Address, 
        amount: i128
    ) -> Result<(), CodebitError>;
    
    // Métodos de consulta
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
        
        Ok(())
    }
    
    fn mint(env: Env, to: Address, amount: i128) -> Result<(), CodebitError> {
        // 1. Verificar que DevPoints esté inicializado
        if !env.storage().instance().has(&DataKey::Initialized) {
            return Err(CodebitError::NotInitialized);
        }
        
        // 2. Solo el admin puede mintear
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
        
        // 4. Actualizar allowance
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
        
        // 2. Verificar que el spender está autorizado
        spender.require_auth();
        
        // 3. Validaciones
        if amount <= 0 {
            return Err(CodebitError::InvalidAmount);
        }
        
        if from == to {
            return Err(CodebitError::InvalidRecipient);
        }
        
        // 4. Verificar allowance
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

#[cfg(test)]
mod test;
