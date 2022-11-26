use bevy::prelude::*;
use crate::client::Client;
use crate::server::Server;
use crate::messaging::serialize::make_deserializers;

make_deserializers![];