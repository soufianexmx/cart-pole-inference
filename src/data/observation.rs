use serde::{Deserialize, Serialize};
use tch::Tensor;

#[derive(Deserialize, Serialize, Debug)]
struct CartPosition(f32);

#[derive(Deserialize, Serialize, Debug)]
struct PoleAngle(f32);

#[derive(Deserialize, Serialize, Debug)]
pub struct Observation {
    cart_position: CartPosition,
    cart_velocity: f32,
    pole_angle: PoleAngle,
    pole_angular_velocity: f32,
}

impl Observation {
    pub fn new(
        cart_position: f32,
        cart_velocity: f32,
        pole_angle: f32,
        pole_angular_velocity: f32,
    ) -> Self {
        Self {
            cart_position: CartPosition(cart_position),
            cart_velocity,
            pole_angle: PoleAngle(pole_angle),
            pole_angular_velocity,
        }
    }
}

impl From<Observation> for Tensor {
    fn from(obs: Observation) -> Self {
        let vec: Vec<f32> = vec![
            obs.cart_position.0,
            obs.cart_velocity,
            obs.pole_angle.0,
            obs.pole_angular_velocity,
        ];

        Tensor::from(&vec[..])
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use tch::{Kind, Tensor};

    #[test]
    fn to_tensor_float() {
        let observation = Observation::new(0.1, -0.5, 0.2, 1.5);

        let tensor = Tensor::from(observation);

        assert_eq!(tensor.kind(), Kind::Float)
    }
}
