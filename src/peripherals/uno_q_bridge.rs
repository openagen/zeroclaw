//! Arduino UNO R4 WiFi (Uno Q) Bridge — full peripheral tool surface.
//!
//! Provides 13 tools total:
//!   - 10 MCU tools via TCP socket to the Bridge app (GPIO, ADC, PWM, I2C, SPI, CAN, LED matrix, RGB LED)
//!   - 3 Linux tools for direct MPU access (camera capture, Linux RGB LED, system info)
//!
//! The Bridge app runs on the Uno Q board and exposes MCU peripherals over a local
//! TCP socket. Linux tools access sysfs and system commands directly.

use crate::tools::traits::{Tool, ToolResult};
use async_trait::async_trait;
use serde_json::{json, Value};
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

const BRIDGE_HOST: &str = "127.0.0.1";
const BRIDGE_PORT: u16 = 9999;
const MAX_DIGITAL_PIN: u64 = 21;
const PWM_PINS: &[u64] = &[3, 5, 6, 9, 10, 11];
const MAX_ADC_CHANNEL: u64 = 5;
const MIN_RGB_LED_ID: u64 = 3;
const MAX_RGB_LED_ID: u64 = 4;

// ---------------------------------------------------------------------------
// Validation helpers
// ---------------------------------------------------------------------------

fn is_valid_digital_pin(pin: u64) -> bool {
    pin <= MAX_DIGITAL_PIN
}

fn is_valid_pwm_pin(pin: u64) -> bool {
    PWM_PINS.contains(&pin)
}

fn is_valid_adc_channel(channel: u64) -> bool {
    channel <= MAX_ADC_CHANNEL
}

fn is_valid_rgb_led_id(id: u64) -> bool {
    (MIN_RGB_LED_ID..=MAX_RGB_LED_ID).contains(&id)
}

// ---------------------------------------------------------------------------
// Bridge communication helpers
// ---------------------------------------------------------------------------

/// Send a command to the Bridge app over TCP and return the response string.
async fn bridge_request(cmd: &str, args: &[String]) -> anyhow::Result<String> {
    let addr = format!("{}:{}", BRIDGE_HOST, BRIDGE_PORT);
    let mut stream = tokio::time::timeout(Duration::from_secs(5), TcpStream::connect(&addr))
        .await
        .map_err(|_| anyhow::anyhow!("Bridge connection timed out"))??;

    let msg = if args.is_empty() {
        format!("{}\n", cmd)
    } else {
        format!("{} {}\n", cmd, args.join(" "))
    };
    stream.write_all(msg.as_bytes()).await?;

    let mut buf = vec![0u8; 4096];
    let n = tokio::time::timeout(Duration::from_secs(3), stream.read(&mut buf))
        .await
        .map_err(|_| anyhow::anyhow!("Bridge response timed out"))??;
    let resp = String::from_utf8_lossy(&buf[..n]).trim().to_string();
    Ok(resp)
}

/// Convert a bridge response string into a `ToolResult`.
/// Responses prefixed with "error:" are treated as failures.
fn bridge_response_to_result(resp: &str) -> ToolResult {
    if resp.starts_with("error:") {
        ToolResult {
            success: false,
            output: resp.to_string(),
            error: Some(resp.to_string()),
        }
    } else {
        ToolResult {
            success: true,
            output: resp.to_string(),
            error: None,
        }
    }
}

/// Combined helper: send a bridge request and convert the response to a `ToolResult`.
async fn bridge_tool_request(cmd: &str, args: &[String]) -> ToolResult {
    match bridge_request(cmd, args).await {
        Ok(resp) => bridge_response_to_result(&resp),
        Err(e) => ToolResult {
            success: false,
            output: format!("Bridge error: {}", e),
            error: Some(e.to_string()),
        },
    }
}

// ===========================================================================
// MCU Tools (10) — via Bridge socket
// ===========================================================================

// ---------------------------------------------------------------------------
// 1. GPIO Read
// ---------------------------------------------------------------------------

/// Read a digital GPIO pin value (0 or 1) on the Uno Q MCU.
pub struct UnoQGpioReadTool;

#[async_trait]
impl Tool for UnoQGpioReadTool {
    fn name(&self) -> &str {
        "uno_q_gpio_read"
    }

    fn description(&self) -> &str {
        "Read digital GPIO pin value (0 or 1) on Arduino UNO R4 WiFi MCU via Bridge."
    }

    fn parameters_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "pin": {
                    "type": "integer",
                    "description": "GPIO pin number (0-21)",
                    "minimum": 0,
                    "maximum": 21
                }
            },
            "required": ["pin"]
        })
    }

    async fn execute(&self, args: Value) -> anyhow::Result<ToolResult> {
        let pin = args
            .get("pin")
            .and_then(|v| v.as_u64())
            .ok_or_else(|| anyhow::anyhow!("Missing 'pin' parameter"))?;

        if !is_valid_digital_pin(pin) {
            return Ok(ToolResult {
                success: false,
                output: format!("Invalid pin: {}. Must be 0-{}.", pin, MAX_DIGITAL_PIN),
                error: Some(format!("Invalid pin: {}", pin)),
            });
        }

        Ok(bridge_tool_request("gpio_read", &[pin.to_string()]).await)
    }
}

// ---------------------------------------------------------------------------
// 2. GPIO Write
// ---------------------------------------------------------------------------

/// Write a digital GPIO pin value (0 or 1) on the Uno Q MCU.
pub struct UnoQGpioWriteTool;

#[async_trait]
impl Tool for UnoQGpioWriteTool {
    fn name(&self) -> &str {
        "uno_q_gpio_write"
    }

    fn description(&self) -> &str {
        "Set digital GPIO pin high (1) or low (0) on Arduino UNO R4 WiFi MCU via Bridge."
    }

    fn parameters_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "pin": {
                    "type": "integer",
                    "description": "GPIO pin number (0-21)",
                    "minimum": 0,
                    "maximum": 21
                },
                "value": {
                    "type": "integer",
                    "description": "0 for low, 1 for high",
                    "minimum": 0,
                    "maximum": 1
                }
            },
            "required": ["pin", "value"]
        })
    }

    async fn execute(&self, args: Value) -> anyhow::Result<ToolResult> {
        let pin = args
            .get("pin")
            .and_then(|v| v.as_u64())
            .ok_or_else(|| anyhow::anyhow!("Missing 'pin' parameter"))?;
        let value = args
            .get("value")
            .and_then(|v| v.as_u64())
            .ok_or_else(|| anyhow::anyhow!("Missing 'value' parameter"))?;

        if !is_valid_digital_pin(pin) {
            return Ok(ToolResult {
                success: false,
                output: format!("Invalid pin: {}. Must be 0-{}.", pin, MAX_DIGITAL_PIN),
                error: Some(format!("Invalid pin: {}", pin)),
            });
        }

        Ok(bridge_tool_request("gpio_write", &[pin.to_string(), value.to_string()]).await)
    }
}

// ---------------------------------------------------------------------------
// 3. ADC Read
// ---------------------------------------------------------------------------

/// Read an analog value from an ADC channel on the Uno Q MCU.
pub struct UnoQAdcReadTool;

#[async_trait]
impl Tool for UnoQAdcReadTool {
    fn name(&self) -> &str {
        "uno_q_adc_read"
    }

    fn description(&self) -> &str {
        "Read analog value from ADC channel (0-5) on Arduino UNO R4 WiFi MCU. WARNING: 3.3V max input on ADC pins."
    }

    fn parameters_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "channel": {
                    "type": "integer",
                    "description": "ADC channel number (0-5). WARNING: 3.3V max input.",
                    "minimum": 0,
                    "maximum": 5
                }
            },
            "required": ["channel"]
        })
    }

    async fn execute(&self, args: Value) -> anyhow::Result<ToolResult> {
        let channel = args
            .get("channel")
            .and_then(|v| v.as_u64())
            .ok_or_else(|| anyhow::anyhow!("Missing 'channel' parameter"))?;

        if !is_valid_adc_channel(channel) {
            return Ok(ToolResult {
                success: false,
                output: format!(
                    "Invalid ADC channel: {}. Must be 0-{}.",
                    channel, MAX_ADC_CHANNEL
                ),
                error: Some(format!("Invalid ADC channel: {}", channel)),
            });
        }

        Ok(bridge_tool_request("adc_read", &[channel.to_string()]).await)
    }
}

// ---------------------------------------------------------------------------
// 4. PWM Write
// ---------------------------------------------------------------------------

/// Write a PWM duty cycle to a PWM-capable pin on the Uno Q MCU.
pub struct UnoQPwmWriteTool;

#[async_trait]
impl Tool for UnoQPwmWriteTool {
    fn name(&self) -> &str {
        "uno_q_pwm_write"
    }

    fn description(&self) -> &str {
        "Write PWM duty cycle (0-255) to a PWM-capable pin on Arduino UNO R4 WiFi MCU. PWM pins: 3, 5, 6, 9, 10, 11."
    }

    fn parameters_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "pin": {
                    "type": "integer",
                    "description": "PWM-capable pin (3, 5, 6, 9, 10, 11)",
                    "enum": [3, 5, 6, 9, 10, 11]
                },
                "duty": {
                    "type": "integer",
                    "description": "PWM duty cycle (0-255)",
                    "minimum": 0,
                    "maximum": 255
                }
            },
            "required": ["pin", "duty"]
        })
    }

    async fn execute(&self, args: Value) -> anyhow::Result<ToolResult> {
        let pin = args
            .get("pin")
            .and_then(|v| v.as_u64())
            .ok_or_else(|| anyhow::anyhow!("Missing 'pin' parameter"))?;
        let duty = args
            .get("duty")
            .and_then(|v| v.as_u64())
            .ok_or_else(|| anyhow::anyhow!("Missing 'duty' parameter"))?;

        if !is_valid_pwm_pin(pin) {
            return Ok(ToolResult {
                success: false,
                output: format!(
                    "Pin {} is not PWM-capable. Valid PWM pins: {:?}.",
                    pin, PWM_PINS
                ),
                error: Some(format!("Pin {} is not PWM-capable", pin)),
            });
        }

        Ok(bridge_tool_request("pwm_write", &[pin.to_string(), duty.to_string()]).await)
    }
}

// ---------------------------------------------------------------------------
// 5. I2C Scan
// ---------------------------------------------------------------------------

/// Scan the I2C bus for connected devices on the Uno Q MCU.
pub struct UnoQI2cScanTool;

#[async_trait]
impl Tool for UnoQI2cScanTool {
    fn name(&self) -> &str {
        "uno_q_i2c_scan"
    }

    fn description(&self) -> &str {
        "Scan I2C bus for connected devices on Arduino UNO R4 WiFi MCU. Returns list of detected addresses."
    }

    fn parameters_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {},
            "required": []
        })
    }

    async fn execute(&self, _args: Value) -> anyhow::Result<ToolResult> {
        Ok(bridge_tool_request("i2c_scan", &[]).await)
    }
}

// ---------------------------------------------------------------------------
// 6. I2C Transfer
// ---------------------------------------------------------------------------

/// Perform an I2C read/write transfer on the Uno Q MCU.
pub struct UnoQI2cTransferTool;

#[async_trait]
impl Tool for UnoQI2cTransferTool {
    fn name(&self) -> &str {
        "uno_q_i2c_transfer"
    }

    fn description(&self) -> &str {
        "Perform I2C transfer on Arduino UNO R4 WiFi MCU. Write data and/or read bytes from a device address."
    }

    fn parameters_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "address": {
                    "type": "integer",
                    "description": "I2C device address (1-126)",
                    "minimum": 1,
                    "maximum": 126
                },
                "data": {
                    "type": "string",
                    "description": "Hex string of bytes to write (e.g. 'A0FF')"
                },
                "read_length": {
                    "type": "integer",
                    "description": "Number of bytes to read back",
                    "minimum": 0
                }
            },
            "required": ["address", "data", "read_length"]
        })
    }

    async fn execute(&self, args: Value) -> anyhow::Result<ToolResult> {
        let address = args
            .get("address")
            .and_then(|v| v.as_u64())
            .ok_or_else(|| anyhow::anyhow!("Missing 'address' parameter"))?;
        let data = args
            .get("data")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing 'data' parameter"))?;
        let read_length = args
            .get("read_length")
            .and_then(|v| v.as_u64())
            .ok_or_else(|| anyhow::anyhow!("Missing 'read_length' parameter"))?;

        if !(1..=126).contains(&address) {
            return Ok(ToolResult {
                success: false,
                output: format!("Invalid I2C address: {}. Must be 1-126.", address),
                error: Some(format!("Invalid I2C address: {}", address)),
            });
        }

        Ok(bridge_tool_request(
            "i2c_transfer",
            &[
                address.to_string(),
                data.to_string(),
                read_length.to_string(),
            ],
        )
        .await)
    }
}

// ---------------------------------------------------------------------------
// 7. SPI Transfer
// ---------------------------------------------------------------------------

/// Perform an SPI transfer on the Uno Q MCU.
pub struct UnoQSpiTransferTool;

#[async_trait]
impl Tool for UnoQSpiTransferTool {
    fn name(&self) -> &str {
        "uno_q_spi_transfer"
    }

    fn description(&self) -> &str {
        "Perform SPI transfer on Arduino UNO R4 WiFi MCU. Send and receive data bytes."
    }

    fn parameters_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "data": {
                    "type": "string",
                    "description": "Hex string of bytes to transfer (e.g. 'DEADBEEF')"
                }
            },
            "required": ["data"]
        })
    }

    async fn execute(&self, args: Value) -> anyhow::Result<ToolResult> {
        let data = args
            .get("data")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing 'data' parameter"))?;

        Ok(bridge_tool_request("spi_transfer", &[data.to_string()]).await)
    }
}

// ---------------------------------------------------------------------------
// 8. CAN Send
// ---------------------------------------------------------------------------

/// Send a CAN bus frame on the Uno Q MCU.
pub struct UnoQCanSendTool;

#[async_trait]
impl Tool for UnoQCanSendTool {
    fn name(&self) -> &str {
        "uno_q_can_send"
    }

    fn description(&self) -> &str {
        "Send a CAN bus frame on Arduino UNO R4 WiFi MCU. Standard 11-bit CAN ID (0-2047)."
    }

    fn parameters_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "id": {
                    "type": "integer",
                    "description": "CAN message ID (0-2047, standard 11-bit)",
                    "minimum": 0,
                    "maximum": 2047
                },
                "data": {
                    "type": "string",
                    "description": "Hex string of data bytes (up to 8 bytes, e.g. 'DEADBEEF')"
                }
            },
            "required": ["id", "data"]
        })
    }

    async fn execute(&self, args: Value) -> anyhow::Result<ToolResult> {
        let id = args
            .get("id")
            .and_then(|v| v.as_u64())
            .ok_or_else(|| anyhow::anyhow!("Missing 'id' parameter"))?;
        let data = args
            .get("data")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing 'data' parameter"))?;

        if id > 2047 {
            return Ok(ToolResult {
                success: false,
                output: format!("Invalid CAN ID: {}. Must be 0-2047.", id),
                error: Some(format!("Invalid CAN ID: {}", id)),
            });
        }

        Ok(bridge_tool_request("can_send", &[id.to_string(), data.to_string()]).await)
    }
}

// ---------------------------------------------------------------------------
// 9. LED Matrix
// ---------------------------------------------------------------------------

/// Control the 12x8 LED matrix on the Uno Q board.
pub struct UnoQLedMatrixTool;

#[async_trait]
impl Tool for UnoQLedMatrixTool {
    fn name(&self) -> &str {
        "uno_q_led_matrix"
    }

    fn description(&self) -> &str {
        "Set the 12x8 LED matrix bitmap on Arduino UNO R4 WiFi. Send 13 bytes (26 hex chars) as bitmap data."
    }

    fn parameters_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "bitmap": {
                    "type": "string",
                    "description": "Hex string bitmap for 12x8 LED matrix (26 hex chars = 13 bytes)"
                }
            },
            "required": ["bitmap"]
        })
    }

    async fn execute(&self, args: Value) -> anyhow::Result<ToolResult> {
        let bitmap = args
            .get("bitmap")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing 'bitmap' parameter"))?;

        if bitmap.len() != 26 {
            return Ok(ToolResult {
                success: false,
                output: format!(
                    "Invalid bitmap length: {} chars. Expected 26 hex chars (13 bytes).",
                    bitmap.len()
                ),
                error: Some(format!("Invalid bitmap length: {}", bitmap.len())),
            });
        }

        Ok(bridge_tool_request("led_matrix", &[bitmap.to_string()]).await)
    }
}

// ---------------------------------------------------------------------------
// 10. RGB LED (MCU-side, IDs 3-4)
// ---------------------------------------------------------------------------

/// Control MCU-side RGB LEDs (IDs 3-4) on the Uno Q board.
pub struct UnoQRgbLedTool;

#[async_trait]
impl Tool for UnoQRgbLedTool {
    fn name(&self) -> &str {
        "uno_q_rgb_led"
    }

    fn description(&self) -> &str {
        "Set MCU-side RGB LED color on Arduino UNO R4 WiFi. LED IDs: 3 or 4. RGB values 0-255."
    }

    fn parameters_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "id": {
                    "type": "integer",
                    "description": "RGB LED ID (3 or 4)",
                    "enum": [3, 4]
                },
                "r": {
                    "type": "integer",
                    "description": "Red value (0-255)",
                    "minimum": 0,
                    "maximum": 255
                },
                "g": {
                    "type": "integer",
                    "description": "Green value (0-255)",
                    "minimum": 0,
                    "maximum": 255
                },
                "b": {
                    "type": "integer",
                    "description": "Blue value (0-255)",
                    "minimum": 0,
                    "maximum": 255
                }
            },
            "required": ["id", "r", "g", "b"]
        })
    }

    async fn execute(&self, args: Value) -> anyhow::Result<ToolResult> {
        let id = args
            .get("id")
            .and_then(|v| v.as_u64())
            .ok_or_else(|| anyhow::anyhow!("Missing 'id' parameter"))?;
        let r = args
            .get("r")
            .and_then(|v| v.as_u64())
            .ok_or_else(|| anyhow::anyhow!("Missing 'r' parameter"))?;
        let g = args
            .get("g")
            .and_then(|v| v.as_u64())
            .ok_or_else(|| anyhow::anyhow!("Missing 'g' parameter"))?;
        let b = args
            .get("b")
            .and_then(|v| v.as_u64())
            .ok_or_else(|| anyhow::anyhow!("Missing 'b' parameter"))?;

        if !is_valid_rgb_led_id(id) {
            return Ok(ToolResult {
                success: false,
                output: format!(
                    "Invalid LED ID: {}. Must be {} or {}.",
                    id, MIN_RGB_LED_ID, MAX_RGB_LED_ID
                ),
                error: Some(format!("Invalid LED ID: {}", id)),
            });
        }

        Ok(bridge_tool_request(
            "rgb_led",
            &[id.to_string(), r.to_string(), g.to_string(), b.to_string()],
        )
        .await)
    }
}

// ===========================================================================
// Linux Tools (3) — direct MPU access
// ===========================================================================

// ---------------------------------------------------------------------------
// 11. Camera Capture
// ---------------------------------------------------------------------------

/// Capture an image from the Uno Q on-board camera via GStreamer.
pub struct UnoQCameraCaptureTool;

#[async_trait]
impl Tool for UnoQCameraCaptureTool {
    fn name(&self) -> &str {
        "uno_q_camera_capture"
    }

    fn description(&self) -> &str {
        "Capture a photo from the USB camera on Arduino Uno Q. Returns the image path. Include [IMAGE:<path>] in your response to send it to the user."
    }

    fn parameters_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "width": {
                    "type": "integer",
                    "description": "Image width in pixels (default: 1280)"
                },
                "height": {
                    "type": "integer",
                    "description": "Image height in pixels (default: 720)"
                },
                "device": {
                    "type": "string",
                    "description": "V4L2 device path (default: /dev/video0)"
                }
            }
        })
    }

    async fn execute(&self, args: Value) -> anyhow::Result<ToolResult> {
        let width = args.get("width").and_then(|v| v.as_u64()).unwrap_or(1280);
        let height = args.get("height").and_then(|v| v.as_u64()).unwrap_or(720);
        let device = args
            .get("device")
            .and_then(|v| v.as_str())
            .unwrap_or("/dev/video0");
        let output_path = "/tmp/zeroclaw_capture.jpg";

        let fmt = format!("width={},height={},pixelformat=MJPG", width, height);
        let output = tokio::process::Command::new("v4l2-ctl")
            .args([
                "-d",
                device,
                "--set-fmt-video",
                &fmt,
                "--stream-mmap",
                "--stream-count=1",
                &format!("--stream-to={}", output_path),
            ])
            .output()
            .await;

        match output {
            Ok(out) if out.status.success() => Ok(ToolResult {
                success: true,
                output: format!(
                    "Photo captured ({}x{}) to {}. To send it to the user, include [IMAGE:{}] in your response.",
                    width, height, output_path, output_path
                ),
                error: None,
            }),
            Ok(out) => {
                let stderr = String::from_utf8_lossy(&out.stderr).to_string();
                Ok(ToolResult {
                    success: false,
                    output: format!("Camera capture failed: {}", stderr),
                    error: Some(stderr),
                })
            }
            Err(e) => Ok(ToolResult {
                success: false,
                output: format!("Failed to run v4l2-ctl: {}. Is v4l-utils installed?", e),
                error: Some(e.to_string()),
            }),
        }
    }
}

// ---------------------------------------------------------------------------
// 12. Linux RGB LED (sysfs, IDs 1-2)
// ---------------------------------------------------------------------------

/// Control Linux-side RGB LEDs (IDs 1-2) via sysfs on the Uno Q board.
pub struct UnoQLinuxRgbLedTool;

#[async_trait]
impl Tool for UnoQLinuxRgbLedTool {
    fn name(&self) -> &str {
        "uno_q_linux_rgb_led"
    }

    fn description(&self) -> &str {
        "Set Linux-side RGB LED color via sysfs on Uno Q. LED 1: user LEDs. LED 2: status LEDs. RGB values 0-255."
    }

    fn parameters_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "id": {
                    "type": "integer",
                    "description": "Linux RGB LED ID (1 or 2)",
                    "enum": [1, 2]
                },
                "r": {
                    "type": "integer",
                    "description": "Red value (0-255)",
                    "minimum": 0,
                    "maximum": 255
                },
                "g": {
                    "type": "integer",
                    "description": "Green value (0-255)",
                    "minimum": 0,
                    "maximum": 255
                },
                "b": {
                    "type": "integer",
                    "description": "Blue value (0-255)",
                    "minimum": 0,
                    "maximum": 255
                }
            },
            "required": ["id", "r", "g", "b"]
        })
    }

    async fn execute(&self, args: Value) -> anyhow::Result<ToolResult> {
        let id = args
            .get("id")
            .and_then(|v| v.as_u64())
            .ok_or_else(|| anyhow::anyhow!("Missing 'id' parameter"))?;
        let r = args
            .get("r")
            .and_then(|v| v.as_u64())
            .ok_or_else(|| anyhow::anyhow!("Missing 'r' parameter"))?;
        let g = args
            .get("g")
            .and_then(|v| v.as_u64())
            .ok_or_else(|| anyhow::anyhow!("Missing 'g' parameter"))?;
        let b = args
            .get("b")
            .and_then(|v| v.as_u64())
            .ok_or_else(|| anyhow::anyhow!("Missing 'b' parameter"))?;

        // LED 1: red:user / green:user / blue:user
        // LED 2: red:panic / green:wlan / blue:bt
        let (red_path, green_path, blue_path) = match id {
            1 => (
                "/sys/class/leds/red:user/brightness",
                "/sys/class/leds/green:user/brightness",
                "/sys/class/leds/blue:user/brightness",
            ),
            2 => (
                "/sys/class/leds/red:panic/brightness",
                "/sys/class/leds/green:wlan/brightness",
                "/sys/class/leds/blue:bt/brightness",
            ),
            _ => {
                return Ok(ToolResult {
                    success: false,
                    output: format!("Invalid Linux LED ID: {}. Must be 1 or 2.", id),
                    error: Some(format!("Invalid Linux LED ID: {}", id)),
                });
            }
        };

        // Use blocking write in spawn_blocking to avoid blocking the async runtime
        let r_str = r.to_string();
        let g_str = g.to_string();
        let b_str = b.to_string();
        let rp = red_path.to_string();
        let gp = green_path.to_string();
        let bp = blue_path.to_string();

        let result = tokio::task::spawn_blocking(move || -> anyhow::Result<()> {
            std::fs::write(&rp, &r_str)?;
            std::fs::write(&gp, &g_str)?;
            std::fs::write(&bp, &b_str)?;
            Ok(())
        })
        .await;

        match result {
            Ok(Ok(())) => Ok(ToolResult {
                success: true,
                output: format!("LED {} set to RGB({}, {}, {})", id, r, g, b),
                error: None,
            }),
            Ok(Err(e)) => Ok(ToolResult {
                success: false,
                output: format!("Failed to write LED sysfs: {}", e),
                error: Some(e.to_string()),
            }),
            Err(e) => Ok(ToolResult {
                success: false,
                output: format!("Task failed: {}", e),
                error: Some(e.to_string()),
            }),
        }
    }
}

// ---------------------------------------------------------------------------
// 13. System Info
// ---------------------------------------------------------------------------

/// Read system information from the Uno Q Linux MPU.
pub struct UnoQSystemInfoTool;

#[async_trait]
impl Tool for UnoQSystemInfoTool {
    fn name(&self) -> &str {
        "uno_q_system_info"
    }

    fn description(&self) -> &str {
        "Read system information from the Uno Q Linux MPU: CPU temperature, memory, disk, and WiFi status."
    }

    fn parameters_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {},
            "required": []
        })
    }

    async fn execute(&self, _args: Value) -> anyhow::Result<ToolResult> {
        let mut info_parts: Vec<String> = Vec::new();

        // CPU temperature
        match tokio::fs::read_to_string("/sys/class/thermal/thermal_zone0/temp").await {
            Ok(temp_str) => {
                if let Ok(millideg) = temp_str.trim().parse::<f64>() {
                    info_parts.push(format!("CPU temp: {:.1}C", millideg / 1000.0));
                } else {
                    info_parts.push(format!("CPU temp raw: {}", temp_str.trim()));
                }
            }
            Err(e) => info_parts.push(format!("CPU temp: unavailable ({})", e)),
        }

        // Memory info (first 3 lines of /proc/meminfo)
        match tokio::fs::read_to_string("/proc/meminfo").await {
            Ok(meminfo) => {
                let lines: Vec<&str> = meminfo.lines().take(3).collect();
                info_parts.push(format!("Memory: {}", lines.join("; ")));
            }
            Err(e) => info_parts.push(format!("Memory: unavailable ({})", e)),
        }

        // Disk usage
        match tokio::process::Command::new("df")
            .args(["-h", "/"])
            .output()
            .await
        {
            Ok(out) if out.status.success() => {
                let stdout = String::from_utf8_lossy(&out.stdout).to_string();
                info_parts.push(format!("Disk:\n{}", stdout.trim()));
            }
            Ok(out) => {
                let stderr = String::from_utf8_lossy(&out.stderr).to_string();
                info_parts.push(format!("Disk: error ({})", stderr.trim()));
            }
            Err(e) => info_parts.push(format!("Disk: unavailable ({})", e)),
        }

        // WiFi status
        match tokio::process::Command::new("iwconfig")
            .arg("wlan0")
            .output()
            .await
        {
            Ok(out) if out.status.success() => {
                let stdout = String::from_utf8_lossy(&out.stdout).to_string();
                info_parts.push(format!("WiFi:\n{}", stdout.trim()));
            }
            Ok(out) => {
                let stderr = String::from_utf8_lossy(&out.stderr).to_string();
                info_parts.push(format!("WiFi: error ({})", stderr.trim()));
            }
            Err(e) => info_parts.push(format!("WiFi: unavailable ({})", e)),
        }

        Ok(ToolResult {
            success: true,
            output: info_parts.join("\n"),
            error: None,
        })
    }
}

// ===========================================================================
// Tests
// ===========================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // -- Pin/channel validation --

    #[test]
    fn valid_digital_pins_accepted() {
        for pin in 0..=21 {
            assert!(is_valid_digital_pin(pin), "pin {} should be valid", pin);
        }
    }

    #[test]
    fn invalid_digital_pins_rejected() {
        assert!(!is_valid_digital_pin(22));
        assert!(!is_valid_digital_pin(100));
    }

    #[test]
    fn valid_pwm_pins_accepted() {
        for pin in &[3, 5, 6, 9, 10, 11] {
            assert!(is_valid_pwm_pin(*pin), "pin {} should be PWM-capable", pin);
        }
    }

    #[test]
    fn non_pwm_pins_rejected() {
        for pin in &[0, 1, 2, 4, 7, 8, 12, 13] {
            assert!(
                !is_valid_pwm_pin(*pin),
                "pin {} should not be PWM-capable",
                pin
            );
        }
    }

    #[test]
    fn valid_adc_channels_accepted() {
        for ch in 0..=5 {
            assert!(is_valid_adc_channel(ch), "channel {} should be valid", ch);
        }
    }

    #[test]
    fn invalid_adc_channels_rejected() {
        assert!(!is_valid_adc_channel(6));
        assert!(!is_valid_adc_channel(100));
    }

    #[test]
    fn valid_rgb_led_ids() {
        assert!(is_valid_rgb_led_id(3));
        assert!(is_valid_rgb_led_id(4));
        assert!(!is_valid_rgb_led_id(1));
        assert!(!is_valid_rgb_led_id(5));
    }

    // -- Bridge response conversion --

    #[test]
    fn bridge_result_ok_response() {
        let result = bridge_response_to_result("ok");
        assert!(result.success);
        assert_eq!(result.output, "ok");
        assert!(result.error.is_none());
    }

    #[test]
    fn bridge_result_error_response() {
        let result = bridge_response_to_result("error: pin not found");
        assert!(!result.success);
        assert_eq!(result.output, "error: pin not found");
        assert!(result.error.is_some());
    }

    #[test]
    fn bridge_result_numeric_response() {
        let result = bridge_response_to_result("2048");
        assert!(result.success);
        assert_eq!(result.output, "2048");
        assert!(result.error.is_none());
    }

    // -- Tool schema validation --

    #[test]
    fn gpio_read_tool_schema() {
        let tool = UnoQGpioReadTool;
        assert_eq!(tool.name(), "uno_q_gpio_read");
        let schema = tool.parameters_schema();
        assert!(schema["properties"]["pin"].is_object());
    }

    #[test]
    fn adc_read_tool_schema() {
        let tool = UnoQAdcReadTool;
        assert_eq!(tool.name(), "uno_q_adc_read");
        let schema = tool.parameters_schema();
        assert!(schema["properties"]["channel"].is_object());
    }

    #[test]
    fn pwm_write_tool_schema() {
        let tool = UnoQPwmWriteTool;
        assert_eq!(tool.name(), "uno_q_pwm_write");
        let schema = tool.parameters_schema();
        assert!(schema["properties"]["pin"].is_object());
        assert!(schema["properties"]["duty"].is_object());
    }

    // -- Tool execute: input validation (no bridge needed) --

    #[tokio::test]
    async fn gpio_read_rejects_invalid_pin() {
        let tool = UnoQGpioReadTool;
        let result = tool.execute(json!({"pin": 99})).await.unwrap();
        assert!(!result.success);
        assert!(result.output.contains("Invalid pin"));
    }

    #[tokio::test]
    async fn pwm_write_rejects_non_pwm_pin() {
        let tool = UnoQPwmWriteTool;
        let result = tool.execute(json!({"pin": 2, "duty": 128})).await.unwrap();
        assert!(!result.success);
        assert!(result.output.contains("not PWM-capable"));
    }

    #[tokio::test]
    async fn adc_read_rejects_invalid_channel() {
        let tool = UnoQAdcReadTool;
        let result = tool.execute(json!({"channel": 7})).await.unwrap();
        assert!(!result.success);
        assert!(result.output.contains("Invalid ADC channel"));
    }

    #[tokio::test]
    async fn rgb_led_rejects_invalid_id() {
        let tool = UnoQRgbLedTool;
        let result = tool
            .execute(json!({"id": 1, "r": 255, "g": 0, "b": 0}))
            .await
            .unwrap();
        assert!(!result.success);
        assert!(result.output.contains("Invalid LED ID"));
    }

    #[tokio::test]
    async fn can_send_rejects_invalid_id() {
        let tool = UnoQCanSendTool;
        let result = tool
            .execute(json!({"id": 9999, "data": "DEADBEEF"}))
            .await
            .unwrap();
        assert!(!result.success);
        assert!(result.output.contains("Invalid CAN ID"));
    }

    #[tokio::test]
    async fn i2c_transfer_rejects_invalid_address() {
        let tool = UnoQI2cTransferTool;
        let result = tool
            .execute(json!({"address": 0, "data": "FF", "read_length": 1}))
            .await
            .unwrap();
        assert!(!result.success);
        assert!(result.output.contains("Invalid I2C address"));
    }
}
