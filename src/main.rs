use dirs;
use glib::clone;
use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, Box, Button, Entry, Orientation, ScrolledWindow, TextView};
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;

fn create_file_path(filename: String) -> Option<PathBuf> {
    // Get path to home directory
    let mut file_path = match dirs::home_dir() {
        Some(path) => PathBuf::from(path),
        None => {
            eprintln!("Unable to get home directory");
            return None;
        }
    };

    file_path.push("Nextcloud2/Obsy"); // Append required directories
    file_path.push(filename); // Append filename
    
    Some(file_path)
}

fn write_to_file(file_path: PathBuf, text: String) {
    // Open the file in append mode (creates it if it does not exist)
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(&file_path)
        .unwrap();

    if let Err(e) = writeln!(file, "{}", &text) {
        eprintln!("Couldn't write to file: {}", e);
    }
}

fn main() {
    let application = Application::builder()
        .application_id("com.example.gtk-rust")
        .build();

    application.connect_activate(|app| {
        // Create main window
        let window = ApplicationWindow::builder()
            .application(app)
            .title("File Writer")
            .default_width(350)
            .default_height(200)
            .build();

        // Create Entry for filename
        let filename_entry = Entry::new();
        // Create TextView for file content
        let text_view = TextView::new();
        let scrolled_window = ScrolledWindow::builder().child(&text_view).build();

        // Create Button for appending content
        let append_button = Button::with_label("Append Contents");

        // Append Contents button clicked
        append_button.connect_clicked(
            clone!(@strong filename_entry, @strong text_view => move |_| {
                let filename = filename_entry.text();
                let filename = format!("{}.md", filename); // Add .md extension to filename

                if let Some(file_path) = create_file_path(filename) {
                    let buffer = text_view.buffer();
                    let start = buffer.start_iter();
                    let end = buffer.end_iter();
                    let text = buffer.text(&start, &end, false);

                    write_to_file(file_path, text.to_string());
                } 
            }),
        );

        // Pack widgets vertically
        let vbox = Box::builder().orientation(Orientation::Vertical).build();
        vbox.append(&filename_entry);
        vbox.append(&scrolled_window);
        vbox.append(&append_button);

        window.set_child(Some(&vbox));
        window.present();
    });

    application.run_with_args(&std::env::args().collect::<Vec<_>>());
}