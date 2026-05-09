use std::io::{self, Write};
use std::time::Duration;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚁 Flight Controller CLI (Rust Edition) - Echo Fixed");
    println!("Connected to /dev/ttyACM0 @ 115200 baud");
    println!("Press Ctrl + C to exit\n");

    let mut port = serialport::new("/dev/ttyACM0", 115200)
        .timeout(Duration::from_millis(50))
        .open()?;

    // Standard settings for flight controllers
    port.set_data_bits(serialport::DataBits::Eight)?;
    port.set_flow_control(serialport::FlowControl::None)?;
    port.set_parity(serialport::Parity::None)?;
    port.set_stop_bits(serialport::StopBits::One)?;

    enable_raw_mode()?;

    let mut buffer = [0u8; 512];

    loop {
        // Read everything from the flight controller
        if let Ok(n) = port.read(&mut buffer) {
            if n > 0 {
                print!("{}", String::from_utf8_lossy(&buffer[0..n]));
                io::stdout().flush()?;
            }
        }

        // Handle keyboard input
        if event::poll(Duration::from_millis(8))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('c') if key.modifiers == crossterm::event::KeyModifiers::CONTROL => {
                            break;
                        }
                        KeyCode::Enter => {
                            port.write_all(b"\r\n")?;
                            io::stdout().flush()?;
                        }
                        KeyCode::Backspace => {
                            // Send backspace sequence that works well with FCs
                            port.write_all(b"\x08 \x08")?;
                            io::stdout().flush()?;
                        }
                        KeyCode::Char(c) => {
                            // Send character to FC only (do NOT print locally)
                            port.write_all(&[c as u8])?;
                            io::stdout().flush()?;
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    disable_raw_mode()?;
    println!("\n\n👋 Disconnected.");
    Ok(())
}