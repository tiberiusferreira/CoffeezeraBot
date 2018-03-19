use super::super::models::CoffeezeraUser;

pub enum UpdateImpact {
    TurnOff,
    TurnOn,
    AddPicpayAccount{
        user: CoffeezeraUser,
        picpay_name: String
    },
    RemovePicpayAccount{
        user: CoffeezeraUser,
    },
    DoNothing
}

