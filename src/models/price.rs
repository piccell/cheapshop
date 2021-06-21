use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, Eq, Ord, PartialEq, PartialOrd)]
pub struct Price {
    pub shop_id: String,
    pub value: usize, //en centimes pour 1 unité (Kg, L)
    pub unit: String,
}

#[derive(Debug, FromForm)]
pub struct PriceForm {
    pub shop_id: String,
    pub article_id: String,
    pub price: String,
    pub quantity: f32,
    pub unit: String,
}

#[derive(Debug, FromForm)]
pub struct PriceDeleteForm {
    pub shop_id: String,
    pub article_id: String,
}

pub fn price_per_unit(price: f32, weight: f32, unit: String) -> f32 {
    use crate::models::article;

    let units = Vec::from(article::UNITS);
    match units.iter().find(|x| x.0 == unit) {
        Some(u) => (price / weight) * u.1,
        _ => 0.0,
    }
}

impl Price {
    pub fn euros(&self) -> f32 {
        self.value as f32 / 100.0
    }
}
