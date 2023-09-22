use anyhow::Result;
use flexi_logger::{
    default_format, detailed_format, Duplicate, FileSpec, Logger, LoggerHandle, WriteMode,
};

pub fn start_logger() -> LoggerHandle {
    try_start_logger().expect("failed to start logger")
}

pub fn try_start_logger() -> Result<LoggerHandle> {
    /*
    Error
    Warn
    Info
    Debug
    Trace
    */
    let level = std::env::var("LOGLEVEL").expect("no log level configured");
    Ok(Logger::try_with_str(level)?
        .log_to_file(FileSpec::default().directory("logs"))
        .write_mode(WriteMode::Direct)
        .duplicate_to_stdout(Duplicate::Info)
        .format_for_files(detailed_format)
        .format_for_stdout(default_format)
        .rotate(
            flexi_logger::Criterion::AgeOrSize(flexi_logger::Age::Day, 1024 * 1024 * 50),
            flexi_logger::Naming::Timestamps,
            flexi_logger::Cleanup::KeepLogFiles(10),
        )
        .print_message()
        .start()?)
}
