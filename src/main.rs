use anyhow::Result;
use chrono::{DateTime, Local, TimeZone};
use clap::Parser;
use enum_dispatch::enum_dispatch;

fn main() -> Result<()> {
    let cli = Cli::parse();
    cli.cmd.execute()
}

#[enum_dispatch]
pub trait CmdExecuter {
    fn execute(self) -> Result<()>;
}

#[derive(Debug, Parser)]
#[command(
    version,
    name = "timestamp",
    about = "datetime and timestamp convert tool"
)]
struct Cli {
    #[command(subcommand)]
    cmd: Subcommand,
}

#[derive(Debug, Parser)]
#[enum_dispatch(CmdExecuter)]
enum Subcommand {
    #[command(name = "now", about = "show now datetime and timestamp")]
    Now(NowOpt),
    #[command(name = "dt", about = "convert datetime to timestamp")]
    DateTime(DateTimeOpt),
    #[command(name = "st", about = "convert timestamp to datetime")]
    Timestamp(TimestampOpt),
}

#[derive(Debug, Parser)]
struct NowOpt;

impl CmdExecuter for NowOpt {
    fn execute(self) -> Result<()> {
        let now = Local::now();
        println!("now datetime: {}", now.format("%F %T%.3f"));
        println!(
            "now timestamp second: {} millisecond: {}",
            now.timestamp(),
            now.timestamp_millis()
        );
        Ok(())
    }
}

#[derive(Debug, Parser)]
struct DateTimeOpt {
    datetime: String,
}

impl CmdExecuter for DateTimeOpt {
    fn execute(self) -> Result<()> {
        let fmt = if self.datetime.len() == 19 {
            "%F %T %z"
        } else {
            "%F %T%.3f %z"
        };
        let dt = DateTime::parse_from_str(&format!("{} +0800", self.datetime), fmt)?;
        println!(
            "timestamp second: {} millisecond: {}",
            dt.timestamp(),
            dt.timestamp_millis()
        );
        Ok(())
    }
}

#[derive(Debug, Parser)]
struct TimestampOpt {
    timestamp: i64,
}

impl CmdExecuter for TimestampOpt {
    fn execute(self) -> Result<()> {
        let (dt, fmt) = if self.timestamp / 10000000000 > 0 {
            (Local.timestamp_millis_opt(self.timestamp), "%F %T%.3f")
        } else {
            (Local.timestamp_opt(self.timestamp, 0), "%F %T")
        };
        match dt {
            chrono::offset::LocalResult::Single(dt) => {
                println!("datetime: {}", dt.format(fmt));
            }
            _ => anyhow::bail!("Fail to convert timestamp"),
        }
        Ok(())
    }
}
