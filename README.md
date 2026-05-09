# fcb_cli

## When running `cargo run --bin dashboard`

🚁 HUMMINGBIRD AIO255 STATUS DASHBOARD
═══════════════════════════════════════
Board     : HUMMINGBIRD_AIO255_AT32F435
Firmware  : Betaflight 4.5.3

MCU Temp  :   53.0°C
Voltage   :  2.79 V
CPU Load  :  22.0%
Uptime    : 7879 seconds
I2C Errors: 9

Status    : ⚠️  Check I2C

## When running `cargo run --bin fcb_cli`

### Press Enter several times, type commands when you reach the # character

```
# status
MCU AT32F435 Clock=288MHz, Vref=3.27V, Core temp=52degC
Stack size: 2048, Stack address: 0x2002fff0
Configuration: CONFIGURED, size: 4104, max available: 16384
Devices detected: SPI:1, I2C:0
Gyros detected: gyro 1 locked dma
GYRO=ICM42688P, ACC=ICM42688P
OSD: MSP (53 x 20)
System Uptime: 7941 seconds
CPU:22%, cycle time: 125, GYRO rate: 8000, RX rate: 15, System rate: 9
Voltage: 278 * 0.01V (0S battery - NOT PRESENT)
I2C Errors: 9
FLASH: JEDEC ID=0x00efaa21 128M
GPS: NOT ENABLED
Arming disable flags: RXLOSS ANGLE CLI

# version
# Betaflight / AT32F435G (A435) 4.5.3 May 28 2025 / 13:06:58 (57c3471dc) MSP API: 1.46
# board: manufacturer_id: HUMMINGBIRD, board_name: HUMMINGBIRD_AIO255_AT32F435

# get gyro_hardware
gyro_hardware_lpf = NORMAL
Allowed values: NORMAL, OPTION_1, OPTION_2, EXPERIMENTAL

# diff

# version
# Betaflight / AT32F435G (A435) 4.5.3 May 28 2025 / 13:06:58 (57c3471dc) MSP API: 1.46

# start the command batch
batch start

board_name HUMMINGBIRD_AIO255_AT32F435
manufacturer_id HUMMINGBIRD

# feature
feature SERVO_TILT
feature RANGEFINDER
feature TELEMETRY
feature LED_STRIP
feature DISPLAY
feature OSD
feature CHANNEL_FORWARDING

profile 0

rateprofile 0

# end the command batch
batch end

```