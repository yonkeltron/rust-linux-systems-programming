use color_eyre::eyre::Result;
use paris::Logger;

fn main() -> Result<()> {
  color_eyre::install()?;

  let mut logger = Logger::new();

  logger.info("Starting up!").newline(1).log("Processes:");

  procfs::process::all_processes()?
    .into_iter()
    .map(|process| {
      format!(
        "{}: {} - {} bytes",
        process.pid, process.stat.comm, process.stat.vsize
      )
    })
    .for_each(|process_message| {
      logger.indent(1).info(process_message);
    });

  Ok(())
}
