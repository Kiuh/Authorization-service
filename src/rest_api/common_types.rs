use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Cell {
    pub parent_id: u64,
    pub own_id: u64,
    pub modules: Vec<Module>,
    pub intellect: Intellect,
}

#[derive(Serialize, Deserialize)]
pub struct Intellect {
    pub neurons_count: u64,
    pub gens_count: u64,
    pub input_neurons_count: u64,
    pub output_neurons_count: u64,
    pub neurons: Vec<Neuron>,
    pub gens: Vec<Gen>,
}

#[derive(Serialize, Deserialize)]
pub struct Neuron {
    pub bias: BigDecimal,
}

#[derive(Serialize, Deserialize)]
pub struct Gen {
    pub el_neur_number: u64,
    pub fin_neur_number: u64,
    pub weight: BigDecimal,
}

#[derive(Serialize, Deserialize)]
pub struct Module {
    pub name: String,
    pub value: Option<BigDecimal>,
}

#[derive(Serialize, Deserialize)]
pub struct Diff {
    pub cell_id: u64,
    pub name: String,
    pub value: BigDecimal,
}
