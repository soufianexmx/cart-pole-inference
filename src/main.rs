use rl_proto::app;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    app::subscriber::init_subscriber();

    app::run()?.await
}
