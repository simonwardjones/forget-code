pub enum ShakeFlavour {
    Chocolate,
    Vanilla,
    Strawberry,
}
pub struct Shake {
    pub flavour: ShakeFlavour,
    pub volume: u32,
    price: u32, // this field is private
}

impl Shake {
    fn new(flavour: ShakeFlavour) -> Shake {
        Shake {
            flavour,
            volume: 500,
            price: 650,
        }
    }
    pub fn chocolate_shake() -> Shake {
        Shake::new(ShakeFlavour::Chocolate)
    }
    pub fn price(&self) -> u32 {
        self.price
    }
}

// define a public submodule
pub mod reshape;
