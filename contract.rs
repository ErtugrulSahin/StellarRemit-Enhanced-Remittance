use soroban_sdk::{contractimpl, Address, Env, Symbol, contracttype, Vec};

pub struct Remittance;

#[contracttype]
pub struct Remit {
    pub sender: Address,
    pub receiver: Address,
    pub amount: i128,
    pub status: Symbol, // "pending", "completed"
}

#[contractimpl]
impl Remittance {
    fn remits<'a>(env: &'a Env) -> Vec<'a, Remit> {
        env.storage().instance().get::<Vec<Remit>>(Symbol::short("remits")).unwrap_or(Vec::new(&env))
    }

    pub fn send(env: Env, receiver: Address, amount: i128) {
        let sender = env.invoker();
        let mut remits = Self::remits(&env);
        remits.push_back(Remit { sender, receiver, amount, status: Symbol::short("pending") });
        env.storage().instance().set(Symbol::short("remits"), &remits);
    }

    pub fn complete(env: Env, index: u32) {
        let receiver = env.invoker();
        let mut remits = Self::remits(&env);
        let remit = &mut remits[index as usize];
        assert_eq!(remit.receiver, receiver, "Only receiver can complete");
        remit.status = Symbol::short("completed");
        env.storage().instance().set(Symbol::short("remits"), &remits);
    }
}
