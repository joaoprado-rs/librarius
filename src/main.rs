use librarius::{debug, error, info, init, warn, Config, Level};

fn main() {
    let config = Config::new(Level::Error);
    init(config);

    info!("Mensagem de info com valor: {}", 42);
    warn!("Mensagem de warn com valor: {}", 42);
    error!("Mensagem de erro com valor: {}", 42);
    debug!("Mensagem de debug com valor: {}", 42);
}
