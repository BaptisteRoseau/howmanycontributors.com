use crate::config::Config;

use super::errors::CacheError;

pub(crate) struct Cache{

}

impl TryFrom<&Config> for Cache{
    type Error = CacheError;
    fn try_from(_config: &Config) -> Result<Self, Self::Error> {
        Ok(Self{})
    }
}

impl Cache {
    pub(crate) async fn init(&self, _config: &Config) -> Result<(), CacheError> {
        Ok(())
    }
    
}