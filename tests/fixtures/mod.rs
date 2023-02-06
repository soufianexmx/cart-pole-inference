use rl_proto::app;

pub fn spawn_app() -> actix_web::web::Data<app::env::AppEnv> {
    app::subscriber::init_subscriber();

    let (sever, env) = app::run().expect("failed to start sever!!!");

    tokio::spawn(sever);

    env
}
