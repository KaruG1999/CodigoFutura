#[contracttype]
pub enum ContractStatus {
    Active,
    Paused,
    Finalized,
}

let estado = ContractStatus::Active;

// Pattern matching para manejar cada caso
match estado {
    ContractStatus::Active => println!("Funcionando"),
    ContractStatus::Paused => println!("En pausa"),
    ContractStatus::Finalized => println!("Terminado"),
}