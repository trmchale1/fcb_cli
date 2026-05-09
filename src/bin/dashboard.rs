use std::time::Duration;
use crossterm::{
    terminal::{self, Clear, ClearType},
    ExecutableCommand,
    style::{Color, Print, SetForegroundColor, ResetColor},
};
use serialport::SerialPort;
use std::io;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚁 Hummingbird Status Dashboard");
    println!("Press Ctrl + C to exit\n");

    let mut port = serialport::new("/dev/ttyACM0", 115200)
        .timeout(Duration::from_millis(100))
        .open()?;

    let mut buffer = [0u8; 4096];

    let mut stdout = io::stdout();
    stdout.execute(terminal::EnterAlternateScreen)?;
    stdout.execute(Clear(ClearType::All))?;

    loop {
        // Get fresh status
        let _ = port.write_all(b"status\r\n");
        let _ = port.flush();
        tokio::time::sleep(Duration::from_millis(300)).await;

        let mut status_text = String::new();
        if let Ok(n) = port.read(&mut buffer) {
            if n > 0 {
                status_text = String::from_utf8_lossy(&buffer[0..n]).to_string();
            }
        }

        // Parse useful values
        let mut temp = 0.0;
        let mut voltage = 0.0;
        let mut cpu = 0.0;
        let mut uptime = "0s".to_string();
        let mut i2c_errors = 0;

        for line in status_text.lines() {
            if line.contains("Core temp") {
                if let Some(v) = line.split('=').last().and_then(|s| s.trim().strip_suffix("degC")) {
                    temp = v.trim().parse().unwrap_or(0.0);
                }
            }
            if line.contains("Voltage:") {
                if let Some(v) = line.split('*').next().and_then(|s| s.split(':').last()) {
                    voltage = v.trim().parse().unwrap_or(0.0) / 100.0;
                }
            }
            if line.contains("CPU:") {
                if let Some(v) = line.split('%').next().and_then(|s| s.split(':').last()) {
                    cpu = v.trim().parse().unwrap_or(0.0);
                }
            }
            if line.contains("System Uptime") {
                uptime = line.split(':').last().unwrap_or("0s").trim().to_string();
            }
            if line.contains("I2C Errors") {
                if let Some(v) = line.split(':').last() {
                    i2c_errors = v.trim().parse().unwrap_or(0);
                }
            }
        }

        // Beautiful dashboard
        println!("\x1B[2J\x1B[H"); // Clear screen
        println!("🚁 HUMMINGBIRD AIO255 STATUS DASHBOARD");
        println!("═══════════════════════════════════════");
        println!("Board     : HUMMINGBIRD_AIO255_AT32F435");
        println!("Firmware  : Betaflight 4.5.3");
        println!();

        // Temperature
        let temp_color = if temp > 60.0 { Color::Red } else if temp > 45.0 { Color::Yellow } else { Color::Green };
        print!("MCU Temp  : ");
        stdout.execute(SetForegroundColor(temp_color))?;
        println!("{:6.1}°C", temp);
        stdout.execute(ResetColor)?;

        // Voltage
        let volt_color = if voltage < 3.5 { Color::Red } else if voltage < 3.8 { Color::Yellow } else { Color::Green };
        print!("Voltage   : ");
        stdout.execute(SetForegroundColor(volt_color))?;
        println!("{:5.2} V", voltage);
        stdout.execute(ResetColor)?;

        println!("CPU Load  : {:5.1}%", cpu);
        println!("Uptime    : {}", uptime);
        println!("I2C Errors: {}", i2c_errors);

        println!("\nStatus    : {}", if i2c_errors == 0 { "✅ Healthy" } else { "⚠️  Check I2C" });

        println!("\nRefresh every ~0.8s  |  Tilt the board to see live values update");

        tokio::time::sleep(Duration::from_millis(800)).await;
    }
}