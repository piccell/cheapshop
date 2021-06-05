use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, Eq, Ord, PartialEq, PartialOrd)]
pub struct Price {
    pub store_id: String,
    pub price: usize, //en centimes pour 1 unité (Kg, L)
}