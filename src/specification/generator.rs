use std::collections::HashMap;
use ndarray::Array2;

use crate::specification::{expression::Value, parser::ModelType};
pub struct Generator {
    model_type: ModelType,
    variable_map: HashMap<String, Value>,
    node_map: HashMap<String, i32>,
    transitions: Array2<f32>,
}