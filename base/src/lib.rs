use colored::Colorize;
use flexi_logger::Logger;

mod queue;
pub use queue::{QueueWithList, QueueWithVec};

mod stack;
pub use stack::{StackWithList, StackWithVec};

pub fn init_logger(log_level: impl AsRef<str>) -> anyhow::Result<()> {
    Logger::try_with_env_or_str(log_level)?
        .format(|w, now, record| {
            let level = match record.level() {
                log::Level::Error => record.level().to_string().red(),
                log::Level::Warn => record.level().to_string().yellow(),
                log::Level::Info => record.level().to_string().green(),
                log::Level::Debug => record.level().to_string().blue(),
                log::Level::Trace => record.level().to_string().purple(),
            };
            write!(
                w,
                "[{}] {} [{}] {}",
                now.format("%Y-%m-%d %H:%M:%S%.3f"),
                level,
                record.module_path().unwrap_or("<unnamed>"),
                &record.args()
            )
        })
        .start()?;

    Ok(())
}
