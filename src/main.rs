use actix_web::{App, HttpServer};
use clap::{App as ClapApp, Arg};

mod validate;
mod version;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let matches = ClapApp::new("Email validator REST endpoint")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .arg(
            Arg::with_name("port")
                .short("p")
                .long("port")
                .takes_value(true)
                .help("Enter port number on which will server listens")
                .default_value("1234"),
        );

    let port = matches.get_matches().value_of("port").unwrap().to_owned();

    HttpServer::new(|| {
        App::new()
            .service(version::version_number)
            .service(validate::validate)
    })
    .bind(format!("127.0.0.1:{}", port))?
    .run()
    .await
}
