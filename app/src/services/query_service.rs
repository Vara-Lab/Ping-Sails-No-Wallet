// necesary crates
use sails_rs::{
    prelude::*,
    gstd::service,
    cell::Ref
};

use crate::states::{
    signless_accounts_state::{
        ContractSignlessAccounts,
        SignlessAccount
    },
    ping_state::PingState
};

// Struct QueryService that will be used for all queries
// Data is passed to the service as Ref (query, does not change state)
pub struct QueryService<'a> {
    ping_state_ref: Ref<'a, PingState>,
    signless_state_ref: Ref<'a, ContractSignlessAccounts>
}

#[service]
impl<'a> QueryService<'a> {
    // Service constructor
    pub fn new(
        ping_state_ref: Ref<'a, PingState>,
        signless_state_ref: Ref<'a, ContractSignlessAccounts>
    ) -> Self {
        Self {
            ping_state_ref,
            signless_state_ref
        }
    }

    pub fn last_who_call(&self) -> QueryEvent {
        QueryEvent::LastWhoCall(self.ping_state_ref.last_who_call)
    }

    pub fn signless_address_from_user_address(
        &self,
        user_address: ActorId
    ) -> QueryEvent {
        let signless_address = self.signless_state_ref
            .signless_accounts_address_by_user_address
            .get(&user_address);

        QueryEvent::SignlessAccountAddress(signless_address.copied())
    }

    pub fn signless_address_from_no_wallet_account(
        &self,
        no_wallet_account: String
    ) -> QueryEvent {
        let signless_address = self.signless_state_ref
            .signless_accounts_address_by_no_wallet_name
            .get(&no_wallet_account);

        QueryEvent::SignlessAccountAddress(signless_address.copied())
    }

    pub fn signless_account_data(
        &self,
        signless_address: ActorId
    ) -> QueryEvent {
        let signless_data = self.signless_state_ref
            .signless_data_by_signless_address
            .get(&signless_address);

        let response = match signless_data {
            Some(data) => Some(data.clone()),
            None => None
        };

        QueryEvent::SignlessAccountData(response)
    }
}


#[derive(Encode, Decode, TypeInfo, Clone)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub enum QueryEvent {
    LastWhoCall(ActorId),
    SignlessAccountAddress(Option<ActorId>),
    SignlessAccountData(Option<SignlessAccount>),
}