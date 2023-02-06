use rl_proto::app;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    app::subscriber::init_subscriber();

    let (sever, _) = app::run()?;

    sever.await
}
