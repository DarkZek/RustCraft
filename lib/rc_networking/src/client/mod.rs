use bevy::prelude::*;
use renet::{RenetClient, RenetError};

pub struct ClientPlugin;

impl Plugin for ClientPlugin {
    fn build(&self, app: &mut App) {
        todo!()
    }
}

crate::make_wrapper_struct!(Client, RenetClient);
