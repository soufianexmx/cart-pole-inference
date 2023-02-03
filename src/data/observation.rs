use serde::{Deserialize, Serialize};
use tch::Tensor;

#[derive(Deserialize, Serialize, Clone, Debug)]
struct CartPosition(f32);

#[derive(Deserialize, Serialize, Clone, Debug)]
struct PoleAngle(f32);

#[derive(Deserialize, Serialize, Clone, Debug)]
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

#[macro_use]
#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::{quickcheck, Arbitrary, Gen};
    use tch::{Kind, Tensor};

    impl Arbitrary for Observation {
        fn arbitrary(g: &mut Gen) -> Observation {
            Observation::new(
                f32::arbitrary(g),
                f32::arbitrary(g),
                f32::arbitrary(g),
                f32::arbitrary(g),
            )
        }
    }

    quickcheck! {
      fn prop_tensor_kind_float(observation: Observation) -> bool {
          let tensor = Tensor::from(observation);

          tensor.kind() == Kind::Float
      }
    }
}
