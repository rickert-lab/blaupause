# blaupause 🗂️

_blaupause_ is a minimalistic tool to copy a directory.

<img src="./assets/gui.png" alt="GUI with macOS" width="566">  

Command-line tools like _rsync_ (Linux, macOS) and _ROBOCOPY_ (Windows) provide powerful options for data transfer and duplication. However, achieving specific outcomes often requires frequent reference to documentation or diligent note-taking to recall the correct parameters. Additionally, incorrect use of options like `--delete` (_rsync_) or `/PURGE` (_ROBOCOPY_) can result in unintended data loss.

_blaupause_ offers a sensible default configuration to optimize transfer speeds and deliver feedback during copy operations—all within a minimalistic, user-friendly interface. Rather than implementing a custom algorithm for data transfer and duplication, _blaupause_ leverages the aforementioned built-in command-line tools to copy directories and their contents. Progress and completion status are reported in the corresponding terminal (Linux, macOS) or command prompt (Windows).

>[!TIP]
Both _rsync_ and _ROBOCOPY_ support incremental updates: If a transfer is interrupted (e.g., due to connection loss or timeouts), simply rerun the operation to resume and complete it. Unchanged directories/files will be skipped, modified ones will be replaced, and missing items will be added.

>[!CAUTION]
If you enable the delete option (default: unchecked), any directories/files that exist on the target (receiving side) but are missing on the source (sending side) **will be deleted** — effectively synchronizing both directories. If the delete option remains unchecked, existing items on the target **will be updated but never deleted**.

_blaupause_ is built with [egui](https://github.com/emilk/egui) and [eframe](https://github.com/emilk/eframe) by Emil Ernerfeldt ([@emilk](https://github.com/emilk)). For licensing details, please refer to the respective copyright notices.
