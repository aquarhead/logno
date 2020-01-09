use log::{Level, LevelFilter, Log, Metadata, Record};

static mut ERRORED: bool = false;

/// A logger that tracks whether there's any `Error` log,
/// remember to call `logno::exit()` at end of program.
pub struct TrackingLogger;

impl Log for TrackingLogger {
  fn enabled(&self, metadata: &Metadata) -> bool {
    metadata.level() <= Level::Info
  }

  fn log(&self, record: &Record) {
    if self.enabled(record.metadata()) {
      if record.level() == Level::Error {
        eprintln!("ERROR - {}", record.args());
        unsafe {
          if !ERRORED {
            ERRORED = true;
          }
        }
      } else {
        println!("{:5} - {}", record.level(), record.args());
      }
    }
  }

  fn flush(&self) {}
}

/// Set a global `TrackingLogger`
pub fn init() {
  log::set_boxed_logger(Box::new(TrackingLogger {})).expect("failed to set global logger");
  log::set_max_level(LevelFilter::Trace);
}

/// Exit with 1 if any error log occured. Otherwise exit with 0.
pub fn exit() {
  unsafe {
    if ERRORED {
      std::process::exit(1);
    }
  }
}
