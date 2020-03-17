use crate::world::*;

/// A zuppa.
/// NB. Maybe these will be saved somewhere in the future.
#[derive(Default)]
pub struct Zuppa {
    pub author: CookKey,
    pub ingredients: Vec<String>,
}

impl Zuppa {
    pub fn empty() -> Self{
        Default::default()
    }
}
