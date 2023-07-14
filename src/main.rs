mod editor;
mod terminal;

use anyhow::Result;
use editor::Editor;
use flexi_logger::FileSpec;

fn main() -> Result<()> {
    flexi_logger::Logger::try_with_str("debug")?
        .log_to_file(FileSpec::default())
        .start()?;
    Editor::default()?.run()
}
