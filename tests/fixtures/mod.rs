use rl_proto::app;

pub fn spawn_app() {
    app::subscriber::init_subscriber();

    let sever = app::run().expect("failed to start sever!!!");

    tokio::spawn(sever);
}

pub fn listen_addr() -> String {
    use rl_proto::app::CONFIG;

    format!("http://{}:{}", CONFIG.base_url(), CONFIG.port())
}
