use std::{env, fs, io};
use zip;

// Entry point of the program, calls real_main and exits with its return value
fn main() {
    std::process::exit(real_main());
}

// The actual functionality of the program resides here
fn real_main() -> i32 {
    // Collect all arguments passed to the program
    let args: Vec<String> = env::args().collect();

    // Check for at least 2 arguments (program name and filename)
    if args.len() < 2 {
        println!("Usage: {} <filename>", args[0]);
        return 1; // Indicate error (incorrect usage)
    }

    // Extract the filename from the second argument (index 1)
    let file_name = std::path::Path::new(&*args[1]);

    // Open the ZIP file for reading
    let file = fs::File::open(&file_name).unwrap();

    // Create a ZipArchive object from the opened file
    let mut archive = zip::ZipArchive::new(file).unwrap();

    // Loop through all entries in the ZIP archive
    for i in 0..archive.len() {
        // Get the current Zip archive entry
        let mut file = archive.by_index(i).unwrap();

        // Extract the filename within the archive (if it exists)
        let out_path = match file.enclosed_name() {
            Some(path) => path.to_owned(),
            None => continue, // Skip entries without filenames
        };

        // Print the comment associated with the file, if any
        {
            let comment = file.comment();
            if !comment.is_empty() {
                println!("File {} comment:{}", i, comment)
            }
        }

        // Handle directories within the archive
        if (*file.name()).ends_with("/") {
            println!("File {} extracted to  \"{}\"", i, out_path.display());
            fs::create_dir_all(&out_path).unwrap(); // Create the directory
        } else { // Handle regular files
            println!(
                "File {} extracted to \"{}\" ({} bytes )",
                i,
                out_path.display(),
                file.size()
            );

            // Create parent directories if they don't exist
            if let Some(p) = out_path.parent() {
                if !p.exists() {
                    fs::create_dir_all(&p).unwrap();
                }
            }

            // Create the output file and copy the contents from the archive entry
            let mut out_file = fs::File::create(&out_path).unwrap();
            io::copy(&mut file, &mut out_file).unwrap();
        }

        // Set file permissions on extracted files (Unix-like systems only)
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;

            if let Some(mode) = file.unix_mode() {
                fs::set_permissions(&out_path, fs::Permissions::from_mode(mode)).unwrap();
            }
        }
    }

    // Program execution successful
    0
}
