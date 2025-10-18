#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracterror, contracttype,
    Env, Symbol, Address 
};  

// Usar wee_alloc para optimizar el uso de memoria (Error en consola)
extern crate wee_alloc;
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

        // 4) Extender TTL del contrato
        env.storage()
            .instance()
            .extend_ttl(100, 100);
        

        Ok(())
    }

    pub fn hello(
        env: Env,
        usuario: Address,
        nombre: Symbol  
    ) -> Result<Symbol, Error> {
        usuario.require_auth(); // Requiere que el usuario firme la transacción

        // Validación del nombre (no vacio) -> Sale error con tostring 
        // let nombre_str= nombre.to_string(); 

        // if nombre_str.len() == 0 -> Error: función len() no disponible para Symbol
        if nombre == Symbol::new(&env, "") {
            return Err(Error::NombreVacio);
        } 
        
        // Validación del nombre (no muy largo) -> Sale error si es muy largo
        /* if nombre.len() > 32 {
            return Err(Error::NombreMuyLargo);
        }

 */
        // Incrementar el contador de saludos -> Lee el contador, lo incrementa y lo guarda
        let key_contador = DataKey::ContadorSaludos;

        let contador: u32 = env.storage()
            .instance()
            .get(&key_contador)
            .unwrap_or(0);
        
        env.storage()
            .instance()
            .set(&key_contador, &(contador + 1));

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

    // Funcion para obtener el último saludo de un usuario -> Option<Symbol> porque puede no existir
        pub fn get_ultimo_saludo(env: Env, usuario: Address) -> Option<Symbol> {
        env.storage()
            .persistent()
            .get(&DataKey::UltimoSaludo(usuario))
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

}

// Test ideal en otro archivo 

#[cfg(test)]
mod test {
    use super::*;
    // tira error
    use soroban_sdk::Env ;
    use soroban_sdk::testutils::Address as TestAddress;

    #[test]
    fn test_initialize() {
        let env = Env::default();
        let contract_id = env.register(HelloContract, ());
        let client = HelloContractClient::new(&env, &contract_id);
        
        let admin = Address::generate(&env); // funcion generate no asociada a Address
        
        // Primera inicialización debe funcionar
        client.initialize(&admin);
        
        // Verificar contador en 0
        assert_eq!(client.get_contador(), 0);
    }


    #[test]
    #[should_panic(expected = "NoInicializado")]
    fn test_no_reinicializar() {
        let env = Env::default();
        let contract_id = env.register(HelloContract, ());
        let client = HelloContractClient::new(&env, &contract_id);
        
        let admin = Address::generate(&env);
        
        client.initialize(&admin);
        client.initialize(&admin);  // Segunda vez debe fallar
    }



    #[test]
    fn test_hello_exitoso() {
        let env = Env::default();
        // register_contract Deprecated -> Soluciono con register 
        let contract_id = env.register(HelloContract, ()); 
        let client = HelloContractClient::new(&env, &contract_id);
        
        let admin = Address::generate(&env);
        let usuario = Address::generate(&env);
        
        client.initialize(&admin);
        
        let nombre = Symbol::new(&env, "Ana");
        let resultado = client.hello(&usuario, &nombre); // Acá arrastra error de fn hello
        
        assert_eq!(resultado, Symbol::new(&env, "Hola"));
        assert_eq!(client.get_contador(), 1);
        assert_eq!(client.get_ultimo_saludo(&usuario), Some(nombre));
    }




        #[test]
    #[should_panic(expected = "NombreVacio")]
    fn test_nombre_vacio() {
        let env = Env::default();
        let contract_id = env.register(HelloContract, ());
        let client = HelloContractClient::new(&env, &contract_id);
        
        let admin = Address::generate(&env);
        let usuario = Address::generate(&env);
        
        client.initialize(&admin);
        
        let vacio = Symbol::new(&env, ""); // Error de fn hello
        client.hello(&usuario, &vacio);  // Debe fallar
    }



        #[test]
    fn test_reset_solo_admin() {
        let env = Env::default();
        let contract_id = env.register(HelloContract, ());
        let client = HelloContractClient::new(&env, &contract_id);
        
        let admin = Address::generate(&env);
        let _otro = Address::generate(&env);
        let usuario = Address::generate(&env);
        
        client.initialize(&admin);
        
        // Hacer saludos
        client.hello(&usuario, &Symbol::new(&env, "Test"));
        assert_eq!(client.get_contador(), 1);
        
        // Admin puede resetear
        client.reset_contador(&admin);
        assert_eq!(client.get_contador(), 0);
    }



        #[test]
    #[should_panic(expected = "NoAutorizado")]
    fn test_reset_no_autorizado() {
        let env = Env::default();
        let contract_id = env.register(HelloContract, ());
        let client = HelloContractClient::new(&env, &contract_id);
        
        let admin = Address::generate(&env);
        let otro = Address::generate(&env);
        
        client.initialize(&admin);
        
        // Otro usuario intenta resetear
        client.reset_contador(&otro);  // Debe fallar
    }

}