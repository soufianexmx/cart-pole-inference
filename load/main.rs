use goose::prelude::*;

async fn predict(user: &mut GooseUser) -> TransactionResult {
    let observation: serde_json::Value = serde_json::from_str(
        r#"{ "cart_position": 0.1, "cart_velocity": 50.0, "pole_angle": 0.13, "pole_angular_velocity": 0.1}"#,
    ).expect("Couldn't parse observation json");

    let _goose_metrics = user.post_json("/predict", &observation).await?;

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
