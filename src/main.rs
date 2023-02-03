use rl_proto::app;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut model = tch::CModule::load("CartPole-v1.pt").expect("Couldn't load module");
    model.set_eval();

    app::run(model)?.await
}
