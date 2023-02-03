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
            Action::Left
        } else {
            Action::Right
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
    use super::*;
    use tch::Tensor;

    #[test]
    fn to_action() {
        // Given
        let left_tensor = Tensor::from(&[1.0, 0.0][..]);
        let right_tensor = Tensor::from(&[0.0, 1.0][..]);

        // Assert
        assert_eq!(Action::from(left_tensor), Action::Left);
        assert_eq!(Action::from(right_tensor), Action::Right);
    }
}
