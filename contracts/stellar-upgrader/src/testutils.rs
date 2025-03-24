use stellar_axelar_std::Env;

use crate::{Upgrader, UpgraderClient};

pub fn setup_upgrader<'a>(env: &Env) -> UpgraderClient<'a> {
    let contract_id = env.register(Upgrader, ());

    UpgraderClient::<'a>::new(env, &contract_id)
}
