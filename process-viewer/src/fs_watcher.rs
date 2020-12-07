use color_eyre::eyre::Result;
use nix::sys::inotify;
use paris::Logger;

pub fn setup_watcher(path_str: &str) -> Result<bool> {
  let watcher = inotify::Inotify::init(inotify::InitFlags::empty())?;
  let watch = watcher.add_watch(path_str, inotify::AddWatchFlags::IN_ALL_EVENTS)?;

  let mut logger = Logger::new();
  let mut go = true;

  while go {
    logger.newline(1).loading("Waiting for events...");
    let events = watcher.read_events()?;
    logger.info(format!("Got {} events", events.len()));

    for event in events {
      let msg = format!("Event: {:?} for {:?}", event.mask, event.name);
      logger.indent(1).log(msg);
    }
  }

  watcher.rm_watch(watch)?;

  Ok(go)
}
