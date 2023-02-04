use goose::prelude::*;

async fn predict(user: &mut GooseUser) -> TransactionResult {
    let json = r#"{ "cart_position": 0.1, "cart_velocity": 50.0, "pole_angle": 0.13, "pole_angular_velocity": 0.1}"#;

    let _goose_metrics = user.post("/predict", json).await?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), GooseError> {
    GooseAttack::initialize()?
        .register_scenario(scenario!("predict").register_transaction(transaction!(predict)))
        .execute()
        .await?;

    Ok(())
}
