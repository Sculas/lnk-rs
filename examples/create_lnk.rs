use std::io::Result;
use std::path::Path;

use encoding_rs::WINDOWS_1252;

fn main() -> Result<()> {
    pretty_env_logger::init();

    let shortcut = lnk::ShellLink::new_simple(Path::new(r"C:\Windows\System32\notepad.exe"))?;
    shortcut.save("np.lnk", WINDOWS_1252).expect("Failed to save shortcut!");
    Ok(())
}
