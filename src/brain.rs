use darth_rust::DarthRust;

pub trait BrainTrait {
    fn learning();
}

#[derive(DarthRust, Debug, Clone)]
pub struct Brain {}

impl BrainTrait for Brain {
    fn learning() {}
}
