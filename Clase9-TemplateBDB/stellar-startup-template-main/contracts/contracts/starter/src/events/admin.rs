use soroban_sdk::{contractevent, Address, Env, Symbol};

#[contractevent]
pub struct AdminChangedEvent {
    #[topic]
    pub name: Symbol,
    #[topic]
    pub old_admin: Address,
    pub new_admin: Address,
}

pub(crate) fn admin_changed(env: &Env, old_admin: &Address, new_admin: &Address) {
    // Construir el evento y publicarlo usando el nuevo tipo de evento.
    let ev = AdminChangedEvent {
        name: Symbol::new(env, "admin_changed"),
        old_admin: old_admin.clone(),
        new_admin: new_admin.clone(),
    };

    let topics = (Symbol::new(env, "admin_changed"), old_admin.clone());
    env.events().publish(topics, ev);
}
