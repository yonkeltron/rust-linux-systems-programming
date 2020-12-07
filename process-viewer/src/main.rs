use color_eyre::eyre::Result;
use paris::Logger;

mod fs_watcher;
mod proc_viewer;

fn main() -> Result<()> {
  color_eyre::install()?;

  crate::proc_viewer::view_procs()?;

  let mut logger = Logger::new();

  if crate::fs_watcher::setup_watcher("target")? {
    logger.info("Watch: OK");
  } else {
    logger.error("Watch: NOK");
  }

  Ok(())
}
