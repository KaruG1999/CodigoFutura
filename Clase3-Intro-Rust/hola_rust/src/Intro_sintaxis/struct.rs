#[contracttype]
pub struct Usuario {
    pub nombre: String,
    pub balance: u128,
    pub activo: bool,
}

// Crear instancia
let alice = Usuario {
    nombre: String::from_str(&env, "Alice"),
    balance: 1_000_000,
    activo: true,
};

// Acceder a campos
let nombre = alice.nombre;
let balance = alice.balance;