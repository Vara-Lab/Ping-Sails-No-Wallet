use sails_rs::prelude::*;

pub struct PingState {
    pub last_who_call: ActorId
}
