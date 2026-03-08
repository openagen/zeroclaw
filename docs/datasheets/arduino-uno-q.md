# Arduino UNO Q (ABX00162 / ABX00173)

## Pin Aliases

| alias       | pin | type  |
|-------------|-----|-------|
| builtin_led | 13  | gpio  |
| user_led    | 13  | gpio  |

## Overview

Arduino UNO Q is a dual-processor board: Qualcomm QRB2210 (quad-core Cortex-A53 @ 2.0 GHz, Debian Linux) + STM32U585 (Cortex-M33 @ 160 MHz, Arduino Core on Zephyr OS). They communicate via Bridge RPC.

Memory: 2/4 GB LPDDR4X + 16/32 GB eMMC.
Connectivity: Wi-Fi 5 (dual-band) + Bluetooth 5.1.

## Digital Pins (3.3V, MCU-controlled)

D0-D13 and D14-D21 (D20=SDA, D21=SCL). All 3.3V logic.

- D0/PB7: USART1_RX
- D1/PB6: USART1_TX
- D3/PB0: PWM (TIM3_CH3), FDCAN1_TX
- D4/PA12: FDCAN1_RX
- D5/PA11: PWM (TIM1_CH4)
- D6/PB1: PWM (TIM3_CH4)
- D9/PB8: PWM (TIM4_CH3)
- D10/PB9: PWM (TIM4_CH4), SPI2_SS
- D11/PB15: PWM (TIM1_CH3N), SPI2_MOSI
- D12/PB14: SPI2_MISO
- D13/PB13: SPI2_SCK, built-in LED
- D20/PB11: I2C2_SDA
- D21/PB10: I2C2_SCL

## ADC (12-bit, 0-3.3V, MCU-controlled)

6 channels: A0-A5. VREF+ = 3.3V. NOT 5V-tolerant in analog mode.

- A0/PA4: ADC + DAC0
- A1/PA5: ADC + DAC1
- A2/PA6: ADC + OPAMP2_INPUT+
- A3/PA7: ADC + OPAMP2_INPUT-
- A4/PC1: ADC + I2C3_SDA
- A5/PC0: ADC + I2C3_SCL

## PWM

Only pins marked ~: D3, D5, D6, D9, D10, D11. Duty cycle 0-255.

## I2C

- I2C2: D20 (SDA), D21 (SCL) — JDIGITAL header
- I2C4: Qwiic connector (PD13/SDA, PD12/SCL)

## SPI

SPI2 on JSPI header: MISO/PC2, MOSI/PC3, SCK/PD1. 3.3V.

## CAN

FDCAN1: TX on D3/PB0, RX on D4/PA12. Requires external CAN transceiver.

## LED Matrix

8x13 = 104 blue pixels, MCU-controlled. Bitmap: 13 bytes (one per column, 8 bits per column).

## MCU RGB LEDs (active-low)

- LED3: R=PH10, G=PH11, B=PH12
- LED4: R=PH13, G=PH14, B=PH15

## Linux RGB LEDs (sysfs)

- LED1 (user): /sys/class/leds/red:user, green:user, blue:user
- LED2 (status): /sys/class/leds/red:panic, green:wlan, blue:bt

## Camera

Dual ISPs: 13MP+13MP or 25MP@30fps. 4-lane MIPI-CSI-2. V4L2 at /dev/video*.

## ZeroClaw Tools

- `uno_q_gpio_read`: Read digital pin (0-21)
- `uno_q_gpio_write`: Set digital pin high/low (0-21)
- `uno_q_adc_read`: Read 12-bit ADC (channel 0-5, 0-3.3V)
- `uno_q_pwm_write`: PWM duty cycle (pins 3,5,6,9,10,11, duty 0-255)
- `uno_q_i2c_scan`: Scan I2C bus
- `uno_q_i2c_transfer`: I2C read/write (addr, hex data, read len)
- `uno_q_spi_transfer`: SPI exchange (hex data)
- `uno_q_can_send`: CAN frame (id, hex payload)
- `uno_q_led_matrix`: Set 8x13 LED matrix (hex bitmap)
- `uno_q_rgb_led`: Set MCU RGB LED 3 or 4 (r, g, b 0-255)
- `uno_q_camera_capture`: Capture image from MIPI-CSI camera
- `uno_q_linux_rgb_led`: Set Linux RGB LED 1 or 2 (sysfs)
- `uno_q_system_info`: CPU temp, memory, disk, Wi-Fi status

## Power

- USB-C: 5V / 3A (PD negotiation)
- DC input: 7-24V
- All headers: 3.3V logic (MCU), 1.8V (MPU). NOT 5V-tolerant on analog pins.
