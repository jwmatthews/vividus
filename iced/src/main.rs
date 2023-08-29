mod gui;

use tracing::{info, error, trace, debug, Level};
use tracing_subscriber::FmtSubscriber;
use gui::App;
use iced::{Sandbox, Settings, Error, window };

fn main() -> Result<(), Error> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
    debug!("Test message from debug");
    trace!("Test message from trace");
    info!("Test message from info");
    error!("Test message from error");
    println!("Vividus started!");
    
    App::run(Settings {
        window: window::Settings {
            size: (800, 600),
            ..window::Settings::default()
        },
        ..Settings::default()
    })
}