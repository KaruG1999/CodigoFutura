#[cfg(test)]

    use super::*;
    // Al hacerlo en otro archivo importo dependencias 
    use soroban_sdk::{Env, String, Symbol, testutils::Address};

    #[test]
    fn test_initialize() {
        let env = Env::default();
        // Warning register_contract deprecado -> Cambio a register e intercambio valores de parametros pasados
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
        let contract_id = env.register(HelloContract, ()); 
        let client = HelloContractClient::new(&env, &contract_id);
        
        let admin = Address::generate(&env);
        let usuario = Address::generate(&env);
        
        client.initialize(&admin);
        
        let nombre = String::from_str(&env, "Ana");  // Con String -> String:: from_str() en lugar de Symbol::new()
        let resultado = client.hello(&usuario, &nombre); 
        
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
        
        let vacio = String::from_str(&env, ""); // Error de fn hello
        client.hello(&usuario, &vacio);  
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
        client.hello(&usuario, &String::from_str(&env, "Test"));
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
        client.reset_contador(&otro); // Debe fallar 
    }

