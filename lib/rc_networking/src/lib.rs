mod messaging;
mod config;
mod client;
mod server;

pub mod temp;
pub use temp::constants as constants;
pub use temp::protocol as protocol;
pub use temp::types as types;
pub use temp::client2::*;
pub use temp::server2::*;


pub use renet;

pub use config::*;

macro_rules! make_wrapper_struct {
    ($name: ident, $inner: ty) => {
        pub struct $name(pub $inner);

        impl bevy::prelude::Resource for $name { }
        impl std::ops::Deref for $name {
            type Target = $inner;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
        impl std::ops::DerefMut for $name {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }
    }
}

pub(crate) use make_wrapper_struct;

