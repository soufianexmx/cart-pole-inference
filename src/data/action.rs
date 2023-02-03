use serde::Serialize;
use tch::Tensor;

#[derive(Serialize, Debug, PartialEq)]
#[serde(tag = "action")]
pub enum Action {
    Left,
    Right,
}

impl Action {
    pub fn new(left: f64) -> Self {
        if left.round() == 1.0 {
            Action::Right
        } else {
            Action::Left
        }
    }
}

impl From<Tensor> for Action {
    fn from(tensor: Tensor) -> Self {
        Action::new(tensor.double_value(&[0]))
    }
}

#[cfg(test)]
mod tests {
    use crate::action::Action;
    use tch::{Device, Kind};

    #[test]
    fn coercion() {
        let tensor = tch::Tensor::rand(&[2], (Kind::Float, Device::Cpu));

        let left = tensor.double_value(&[0]).round();

        if left == 0.0 {
            assert_eq!(Action::from(tensor), Action::Left)
        } else {
            assert_eq!(Action::from(tensor), Action::Right)
        }
    }
}
