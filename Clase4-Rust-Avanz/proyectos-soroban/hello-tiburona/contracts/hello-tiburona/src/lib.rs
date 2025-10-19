#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracterror, contracttype,
    Env, Symbol, Address, String
};  

// Usar wee_alloc para optimizar el uso de memoria (Error en consola)
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// Definir errores 
#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum Error {
    NombreVacio = 1,
    NombreMuyLargo = 2,
    NoAutorizado = 3,
    NoInicializado = 4,
}

// Definir data keys
#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Admin,
    ContadorSaludos,
    UltimoSaludo(Address),
    ContadorPorUsuario(Address),  // RETO 1
    LimiteCaracteres,              // RETO 3
}

// Definir el contrato
#[contract]
pub struct HelloContract;

#[contractimpl]
impl HelloContract {
    // Acá las funciones

    pub fn initialize(env: Env, admin: Address) -> Result<(), Error> {
        // 1) Verificar si ya está inicializado
        if env.storage().instance().has(&DataKey::Admin) {
            return Err(Error::NoInicializado);
        }

        // 2) Guardar el administrador
        env.storage().instance().set(&DataKey::Admin, &admin);

        // 3) Inicializar el contador de saludos
        env.storage().instance().set(&DataKey::ContadorSaludos, &0u32);

        // RETO 3: Inicializar límite de caracteres por defecto (32)
        env.storage().instance().set(&DataKey::LimiteCaracteres, &32u32);

        // 4) Extender TTL del contrato
        env.storage()
            .instance()
            .extend_ttl(100, 100);
        
        Ok(())
    }

    pub fn hello(
        env: Env,
        usuario: Address,
        nombre: String
    ) -> Result<Symbol, Error> {
        usuario.require_auth(); 

        if nombre.len() == 0 { 
            return Err(Error::NombreVacio); 
        } 
        
        // RETO 3: Usar límite configurable en lugar de valor hardcoded
        let limite: u32 = env.storage()
            .instance()
            .get(&DataKey::LimiteCaracteres)
            .unwrap_or(32);
        
        if nombre.len() > limite {
            return Err(Error::NombreMuyLargo);
        }
       
        // Incrementar el contador de saludos -> Lee el contador, lo incrementa y lo guarda
        let key_contador = DataKey::ContadorSaludos;

        let contador: u32 = env.storage()
            .instance()
            .get(&key_contador)
            .unwrap_or(0);
        
        env.storage()
            .instance()
            .set(&key_contador, &(contador + 1));

        // RETO 1: Incrementar contador por usuario
        let key_contador_usuario = DataKey::ContadorPorUsuario(usuario.clone());
        let contador_usuario: u32 = env.storage()
            .persistent()
            .get(&key_contador_usuario)
            .unwrap_or(0);
        
        env.storage()
            .persistent()
            .set(&key_contador_usuario, &(contador_usuario + 1));

        // Extender TTL del contador de usuario
        env.storage()
            .persistent()
            .extend_ttl(&key_contador_usuario, 100, 100);

        // Guardar el último saludo del usuario -> Guarda el nombre con la key del usuario
        env.storage()
            .persistent()
            .set(&DataKey::UltimoSaludo(usuario.clone()), &nombre);

        // Extender TTL del contrato -> persistent y instance en ese orden 
        env.storage()
            .persistent()
            .extend_ttl(&DataKey::UltimoSaludo(usuario), 100, 100);
        
        env.storage()
            .instance()
            .extend_ttl(100, 100);

        // Retornar el saludo 
        Ok(Symbol::new(&env, "Hola"))
    }

    // 5) Funciones de consulta

    pub fn get_contador(env: Env) -> u32 {
        env.storage()
            .instance()
            .get(&DataKey::ContadorSaludos)
            .unwrap_or(0)
    }

    // Funcion para obtener el último saludo de un usuario -> Option<String> porque puede no existir
    pub fn get_ultimo_saludo(env: Env, usuario: Address) -> Option<String> {
        env.storage()
            .persistent()
            .get(&DataKey::UltimoSaludo(usuario))
    }

    // RETO 1: Función para obtener estadísticas por usuario
    pub fn get_contador_usuario(env: Env, usuario: Address) -> u32 {
        env.storage()
            .persistent()
            .get(&DataKey::ContadorPorUsuario(usuario))
            .unwrap_or(0)
    }

    // 6) Funcion administrativa

    pub fn reset_contador(env: Env, caller: Address) -> Result<(), Error> {
        // Obtener el administrador desde el almacenamiento 
        let admin: Address = env.storage()
            .instance()
            .get(&DataKey::Admin)
            .ok_or(Error::NoInicializado)?; // Recomendado el ? para propagar el error

        // Solo el admin puede resetear el contador
        if caller != admin {
            return Err(Error::NoAutorizado);
        }

        // Reiniciar el contador de saludos
        env.storage()
            .instance()
            .set(&DataKey::ContadorSaludos, &0u32); // &0u32 para especificar el tipo

        Ok(())
    }

    // RETO 2: Función para transferir ownership del contrato
    pub fn transfer_admin(
        env: Env,
        caller: Address,
        nuevo_admin: Address
    ) -> Result<(), Error> {
        // Verificar autenticación del caller
        caller.require_auth();

        // Obtener el administrador actual
        let admin_actual: Address = env.storage()
            .instance()
            .get(&DataKey::Admin)
            .ok_or(Error::NoInicializado)?;

        // Verificar que el caller sea el admin actual
        if caller != admin_actual {
            return Err(Error::NoAutorizado);
        }

        // Cambiar el admin al nuevo admin
        env.storage()
            .instance()
            .set(&DataKey::Admin, &nuevo_admin);

        // Extender TTL
        env.storage()
            .instance()
            .extend_ttl(100, 100);

        Ok(())
    }

    // RETO 3: Función para configurar el límite de caracteres
    pub fn set_limite(
        env: Env,
        caller: Address,
        limite: u32
    ) -> Result<(), Error> {
        // Verificar autenticación del caller
        caller.require_auth();

        // Obtener el administrador actual
        let admin: Address = env.storage()
            .instance()
            .get(&DataKey::Admin)
            .ok_or(Error::NoInicializado)?;

        // Solo el admin puede cambiar el límite
        if caller != admin {
            return Err(Error::NoAutorizado);
        }

        // Guardar el nuevo límite
        env.storage()
            .instance()
            .set(&DataKey::LimiteCaracteres, &limite);

        // Extender TTL
        env.storage()
            .instance()
            .extend_ttl(100, 100);

        Ok(())
    }

    // RETO 3: Función para consultar el límite actual
    pub fn get_limite(env: Env) -> u32 {
        env.storage()
            .instance()
            .get(&DataKey::LimiteCaracteres)
            .unwrap_or(32)
    }
}