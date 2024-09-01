#![no_std]

// necesary crates
use sails_rs::{
    cell::RefCell, 
    gstd::{
        program,
        route,
        msg
    }, 
    prelude::*
};

// import our modules 
pub mod states;
pub mod services;

// import necesary data (CustomStruct state)
use states::{
    ping_state::PingState,
    signless_accounts_state::ContractSignlessAccounts
};


// Import service to be used for the program
use services::{
    ping_service::PingService,
    signless_service::SignlessService,
    query_service::QueryService
};


// Ping program struct to build the program (there can only be one per contract)
// Data is stored as a part of the program and passed to the services as Ref (query) 
// or RefMut (command), because services are instantiated for every incoming request
// message indicating that these services are stateless.
pub struct PingProgram {
    ping_state: RefCell<PingState>,
    signless_state: RefCell<ContractSignlessAccounts>
}

// Ping program, it host one or more services and it expose them to the 
// externar consumer.
// Only one program is allowed per application
#[program]
impl PingProgram {
    // Application constructor (it is an associated function)
    // It can be called once per application lifetime.
    pub fn new() -> Self {
        let ping_state = RefCell::new(PingState {
            last_who_call: msg::source()
        });
        let signless_state = RefCell::new(ContractSignlessAccounts::default());

        Self {
            ping_state,
            signless_state
        }
    }

    // Method working with "&self", havind no other parameters are treated as exposed
    // service constructors, and are called each time when an incoming request message 
    // needs be dispatched to a selected service
    // It has "message routing", This will change the way a service will be called 
    // (if omitted, the method name will be used, in this case TrafficLightSvc).
    #[route("Ping")]
    pub fn ping_svc(&self) -> PingService<'_> {
        PingService::new(
            self.ping_state.borrow_mut(), 
            self.signless_state.borrow()
        )
    }

    #[route("Signless")]
    pub fn signless_svc(&self) -> SignlessService<'_> {
        SignlessService::new(
            self.signless_state.borrow_mut()
        )
    }

    #[route("QueryService")]
    pub fn query_svc(&self) -> QueryService {
        QueryService::new(
            self.ping_state.borrow(),
            self.signless_state.borrow()
        )
    }
}