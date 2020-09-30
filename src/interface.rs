use crate::params::Params;
use crate::result::Result;

pub(crate) trait Interface {
    fn new(params: Params) -> Result<Self>
    where
        Self: Sized;
    fn name(&self) -> String;
}
