use sails_rs::{
    gstd::{
        service,
        msg
    },
    cell::{RefMut, Ref},
    prelude::*
};

use crate::states::{
    signless_accounts_state::{
        ContractSignlessAccounts,
        SignlessError,
    },
    ping_state::PingState
};

pub type UserAddress = ActorId;
pub type NoWalletAccount = String;
pub type UserData = (Option<UserAddress>, Option<NoWalletAccount>);

// #[derive(Default)]
pub struct PingService<'a> {
    ping_state: RefMut<'a, PingState>,
    signless_state_ref: Ref<'a, ContractSignlessAccounts>
}

#[service]
impl<'a> PingService<'a> {
    pub fn new(
        ping_state: RefMut<'a, PingState>, 
        signless_state_ref: Ref<'a, ContractSignlessAccounts>
    ) -> Self {
        Self {
            ping_state,
            signless_state_ref
        }
    }

    // Remote call "ping" exposed to external consumers
    // Returns a struct that will be sent as a response to the user
    // Is treated as a command changing the state (&mut self)
    // It is used for "normal" calls or with vouchers.
    pub fn ping(&mut self) -> PingEvent{
        let caller = msg::source();
        self.handle_ping(caller)
    }

    // Remote call "ping_signless" exposed to external consumers
    // Returns a struct that will be sent as a response to the user
    // Is treated as a command changing the state (&mut self)
    // It is used for signless calls, receives the address of the user
    // who is owner of the signless account
    pub fn ping_signless(
        &mut self,
        user_address: ActorId
    ) -> PingEvent {
        let caller = msg::source();

        // Need to check if the signless addres is afiliated to the user address
        if let Err(signless_error) = self.signless_state_ref
            .check_signless_address_by_user_address(
                caller,
                user_address
            )
        {
            return PingEvent::SignlessError(signless_error);
        }
        
        // Call method with the use address as the caller
        self.handle_ping(user_address)
    }

    // Remote call "ping_no_wallet" exposed to external consumers
    // Returns a struct that will be sent as a response to the user
    // Is treated as a command changing the state (&mut self)
    // It is used for calls with no wallet, receives the name encoded 
    // string of the user who is owner of the signless account
    pub fn ping_no_wallet(
        &mut self,
        no_wallet_name_encoded: String
    ) -> PingEvent {
        let caller = msg::source();

        // Need to check if the signless addres is afiliated to the user encoded name
        if let Err(signless_error) = self.signless_state_ref
            .check_signless_address_by_no_wallet_account(
                caller,
                no_wallet_name_encoded
            )
        {
            return PingEvent::SignlessError(signless_error);
        }

        self.handle_ping(caller)
    }

    pub fn pong(&mut self) -> PingEvent{
        let caller = msg::source();
        self.handle_pong(caller)
    }

    pub fn pong_signless(
        &mut self,
        user_address: ActorId
    ) -> PingEvent {
        let caller = msg::source();

        if let Err(signless_error) = self.signless_state_ref
            .check_signless_address_by_user_address(
                caller,
                user_address
            )
        {
            return PingEvent::SignlessError(signless_error);
        }
            
        self.handle_pong(user_address)
    }

    pub fn pong_no_wallet(
        &mut self,
        no_wallet_name_encoded: String
    ) -> PingEvent {
        let caller = msg::source();

        if let Err(signless_error) = self.signless_state_ref
            .check_signless_address_by_no_wallet_account(
                caller,
                no_wallet_name_encoded
            )
        {
            return PingEvent::SignlessError(signless_error);
        }

        self.handle_pong(caller)
    }
}


// Methods to help service methods to handle common behaviors
impl<'a> PingService<'a> {
    fn handle_ping(
        &mut self, 
        caller: ActorId
    ) -> PingEvent {
        self
            .ping_state
            .last_who_call = caller;

        PingEvent::Pong
    }

    fn handle_pong(
        &mut self, 
        caller: ActorId
    ) -> PingEvent {
        self
            .ping_state
            .last_who_call = caller;

        PingEvent::Ping
    }
}

#[derive(Encode, Decode, TypeInfo)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub enum PingEvent {
    Ping,
    Pong,
    SignlessError(SignlessError),
}

#[derive(Encode, Decode, TypeInfo)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub enum TestEnum {
    First(String),
    Second(u64),
    Third(TestStruct),
    Four(Vec<String>),
    Five(Vec<(u64, String)>)
}

#[derive(Encode, Decode, TypeInfo)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub struct TestStruct {
    name: String,
    age: u64
}