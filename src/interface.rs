use crate::params::Params;
use crate::result::Result;

pub(crate) trait Interface {
    fn new(params: Params) -> Result<Self>
    where
        Self: Sized;
    fn name(&self) -> String;
}

pub(crate) fn new_interface<T: Interface>(params: Params) -> Result<T> {
    T::new(params)
}
