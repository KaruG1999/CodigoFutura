use soroban_sdk::Vec;

// Crear Vec vacío
let mut lista: Vec<u32> = Vec::new(&env);
//     ^^^ mut necesario para modificar

// Agregar elementos
lista.push_back(10);
lista.push_back(20);
lista.push_back(30);

// Acceder por índice
let primero = lista.get(0);   // Some(10)
let decimo = lista.get(10);   // None - no existe

// Obtener tamaño
let longitud = lista.len();   // 3

// Iterar
for numero in lista.iter() {
    // Hacer algo con cada número
}