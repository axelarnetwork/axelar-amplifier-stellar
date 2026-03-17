use core::fmt::Debug;

use soroban_sdk::{Env, Val, Vec};
#[cfg(any(test, feature = "testutils"))]
pub use testutils::*;

pub trait Event: Debug + PartialEq + Sized {
    fn emit(self, env: &Env);

    fn from_event(env: &Env, topics: Vec<Val>, data: Val) -> Self;

    fn schema(env: &Env) -> &'static str;
}

#[cfg(any(test, feature = "testutils"))]
mod testutils {
    use soroban_sdk::xdr;
    use soroban_sdk::{Address, Env, TryFromVal, Val, Vec};

    use crate::events::Event;
    use crate::testutils::Events;

    pub fn fmt_last_emitted_event<E>(env: &Env) -> std::string::String
    where
        E: Event,
    {
        fmt_emitted_event_at_idx::<E>(env, -1)
    }

    pub fn fmt_emitted_event_at_idx<E>(env: &Env, mut idx: i32) -> std::string::String
    where
        E: Event,
    {
        let events = env.events().all();
        let event_slice = events.events();

        if idx < 0 {
            idx += event_slice.len() as i32;
        }

        let event = &event_slice[idx as usize];

        let contract_id = event
            .contract_id
            .as_ref()
            .map(|id| {
                let sc_val = xdr::ScVal::Address(xdr::ScAddress::Contract(id.clone()));
                Address::try_from_val(env, &sc_val).expect("failed to parse contract_id")
            })
            .expect("event has no contract_id");

        let body = match &event.body {
            xdr::ContractEventBody::V0(v0) => v0,
        };

        let topics: Vec<Val> =
            Vec::try_from_val(env, &xdr::ScVal::Vec(Some(body.topics.clone().into())))
                .expect("failed to convert topics");
        let data: Val = Val::try_from_val(env, &body.data).expect("failed to convert data");

        let parsed = E::from_event(env, topics, data);
        std::format!("{:#?}\n\n{:?}\n\n{}", parsed, contract_id, E::schema(env))
    }
}

#[cfg(test)]
mod test {
    use core::fmt::Debug;

    use stellar_axelar_std::events::Event;
    use stellar_axelar_std::xdr::Int32;
    use stellar_axelar_std::{contract, BytesN, Env, String, Symbol};
    use stellar_axelar_std_derive::{contractimpl, IntoEvent};

    use crate as stellar_axelar_std;
    use crate::events::fmt_last_emitted_event;

    #[derive(Debug, PartialEq, Eq, IntoEvent)]
    struct TestEvent {
        topic1: Symbol,
        topic2: String,
        topic3: Int32,
        #[data]
        data1: String,
        #[data]
        data2: BytesN<32>,
    }

    #[contract]
    struct Contract;

    #[contractimpl]
    impl Contract {
        pub fn test(env: &Env) {
            TestEvent {
                topic1: Symbol::new(env, "topic1"),
                topic2: String::from_str(env, "topic2"),
                topic3: 10,
                data1: String::from_str(env, "data1"),
                data2: BytesN::from_array(env, &[3; 32]),
            }
            .emit(env);
        }
    }

    #[test]
    fn format_last_emitted_event() {
        let env = Env::default();

        let contract = env.register(Contract, ());
        let client = ContractClient::new(&env, &contract);

        client.test();
        goldie::assert!(fmt_last_emitted_event::<TestEvent>(&env));
    }
}
