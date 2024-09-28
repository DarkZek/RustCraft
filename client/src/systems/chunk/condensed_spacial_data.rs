use bevy::prelude::{warn};
use nalgebra::Vector3;
use serde::{Serialize, Deserialize};

// TODO: Allow partial bit usage

#[derive(Serialize, Deserialize)]
pub struct CondensedSpacialData<T: Sized + Copy> {
    data: Vec<T>,
    size: usize,
}

impl<T: Default + Sized + Copy> CondensedSpacialData<T> {
    pub fn new(size: usize) -> Self {
        CondensedSpacialData {
            data: vec![T::default(); size*size*size],
            size
        }
    }
}

impl<T: Sized + Copy> CondensedSpacialData<T> {
    pub fn from_slice<const S: usize>(spacial_data: [[[T; S]; S]; S]) -> Self {

        let size = spacial_data.len();
        let mut data = Vec::with_capacity(size * size * size);

        warn!("Dont use this in production it wasn't actually tested and doesnt send it in the right shape");

        for row in spacial_data {
            for col in row {
                for val in col {
                    data.push(val);
                }
            }
        }

        CondensedSpacialData {
            data,
            size
        }
    }
}

impl<T: Sized + Copy> CondensedSpacialData<T> {
    #[inline]
    pub fn get(&self, pos: Vector3<usize>) -> Option<&T> {
        let index =  pos.x * (self.size*self.size) + pos.y * (self.size) + pos.z;
        self.data.get(index)
    }

    #[inline]
    pub fn get_mut(&mut self, pos: Vector3<usize>) -> Option<&mut T> {
        let index =  pos.x * (self.size*self.size) + pos.y * (self.size) + pos.z;
        self.data.get_mut(index)
    }
}