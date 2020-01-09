use log::{error, info};

fn main() {
  logno::init();

  info!("info");
  error!("error");

  logno::exit();
}
