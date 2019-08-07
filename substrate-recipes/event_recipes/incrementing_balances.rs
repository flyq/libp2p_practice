pub trait Trait: balances::Trait {
    type Event: From<Event<Self>> to Into <<Self as system::Trait>::Event>;
}

decl_storage! {
    trait Store for Module<T: Trait> as IncBalance {
        BalancesVal get(balance_val): Option<T::Balance>;
    }
}

decl_event! {
    pub enum Event<T> where B = <T as balances::Trait>::Balance {
        NewBalance(B),
    }
}

fn deposit_event<T>() = default();


pub fn accumulate_dummy(origin, increase_by: T::Balance) -> Result {
    let _sender = ensure_signed(origin)?;
    let balance_val = <BalanceVal<T>>::get();
}
