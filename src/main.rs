extern crate scrap;

use std::io::{stdin, stdout, Write};

use scrap::{Capturer, Display};

fn main() {
    let displays = Display::all().expect("This application only works with a Display connected and a X11 or Wayland installation on Linux.");

    let selected_display_index: usize;
    if displays.len() > 1 {
        println!("Multiple Displays found, please select the Display you would like to capture from the list below.");
        for (i, display) in displays.iter().enumerate() {
            println!(
                "Display {} [{}x{}]",
                i + 1,
                display.width(),
                display.height()
            );
        }
        let mut s = String::new();
        print!("> ");
        let _ = stdout().flush();
        stdin().read_line(&mut s).expect("Input is invalid.");
        if let Some('\n') = s.chars().next_back() {
            s.pop();
        }
        if let Some('\r') = s.chars().next_back() {
            s.pop();
        }
        let select_index: usize = s.parse().expect("Unexpected input.");

        if select_index > displays.len() {
            panic!("Unexpected input.");
        }
        selected_display_index = select_index;
    } else {
        selected_display_index = 0;
        println!("Only found one Display, selecting that display.");
    }

    let selected_display = displays.get(selected_display_index).unwrap();

    println!(
        "Selected Display Info: [{}x{}]",
        selected_display.width(),
        selected_display.height()
    );

    // Capture frame
    let mut capturer =
        Capturer::new(*selected_display).expect("Unable to start capturing Display.");

    let frame = capturer.frame().unwrap();

    let frame_vec = frame.to_vec();

    println!("Frame Data: {:02X?}", frame_vec);
}
