# ZeroClaw Bridge — socket server for full MCU peripheral control
# SPDX-License-Identifier: MPL-2.0
#
# Bridge.call() must run on the main thread (not thread-safe).
# Socket accepts happen on a background thread, but each request
# is queued and processed in the main App.run() loop.

import queue
import socket
import sys
import threading
import traceback
from arduino.app_utils import *

ZEROCLAW_PORT = 9999

# Queue of (conn, data_str) tuples processed on the main thread.
request_queue = queue.Queue()


def process_request(data, conn):
    """Process a single bridge command on the main thread."""
    try:
        parts = data.split()
        if not parts:
            conn.sendall(b"error: empty command\n")
            return
        cmd = parts[0].lower()

        # ── GPIO ──────────────────────────────────────────────
        if cmd == "gpio_write" and len(parts) >= 3:
            Bridge.call("digitalWrite", int(parts[1]), int(parts[2]))
            conn.sendall(b"ok\n")

        elif cmd == "gpio_read" and len(parts) >= 2:
            val = Bridge.call("digitalRead", int(parts[1]))
            conn.sendall(f"{val}\n".encode())

        # ── ADC ───────────────────────────────────────────────
        elif cmd == "adc_read" and len(parts) >= 2:
            val = Bridge.call("analogRead", int(parts[1]))
            conn.sendall(f"{val}\n".encode())

        # ── PWM ───────────────────────────────────────────────
        elif cmd == "pwm_write" and len(parts) >= 3:
            result = Bridge.call("analogWrite", int(parts[1]), int(parts[2]))
            if result == -1:
                conn.sendall(b"error: not a PWM pin\n")
            else:
                conn.sendall(b"ok\n")

        # ── I2C ───────────────────────────────────────────────
        elif cmd == "i2c_scan":
            result = Bridge.call("i2cScan")
            conn.sendall(f"{result}\n".encode())

        elif cmd == "i2c_transfer" and len(parts) >= 4:
            result = Bridge.call("i2cTransfer", int(parts[1]), parts[2], int(parts[3]))
            conn.sendall(f"{result}\n".encode())

        # ── SPI ───────────────────────────────────────────────
        elif cmd == "spi_transfer" and len(parts) >= 2:
            result = Bridge.call("spiTransfer", parts[1])
            conn.sendall(f"{result}\n".encode())

        # ── CAN ───────────────────────────────────────────────
        elif cmd == "can_send" and len(parts) >= 3:
            result = Bridge.call("canSend", int(parts[1]), parts[2])
            if result == -2:
                conn.sendall(b"error: CAN not yet available\n")
            else:
                conn.sendall(b"ok\n")

        # ── LED Matrix ────────────────────────────────────────
        elif cmd == "led_matrix" and len(parts) >= 2:
            Bridge.call("ledMatrix", parts[1])
            conn.sendall(b"ok\n")

        # ── RGB LED ───────────────────────────────────────────
        elif cmd == "rgb_led" and len(parts) >= 5:
            result = Bridge.call("rgbLed", int(parts[1]), int(parts[2]), int(parts[3]), int(parts[4]))
            if result == -1:
                conn.sendall(b"error: invalid LED id (use 0 or 1)\n")
            else:
                conn.sendall(b"ok\n")

        # ── Capabilities ──────────────────────────────────────
        elif cmd == "capabilities":
            result = Bridge.call("capabilities")
            conn.sendall(f"{result}\n".encode())

        else:
            conn.sendall(b"error: unknown command\n")

    except Exception as e:
        print(f"[handle] ERROR: {e}", file=sys.stderr, flush=True)
        traceback.print_exc(file=sys.stderr)
        try:
            conn.sendall(f"error: {e}\n".encode())
        except Exception:
            pass
    finally:
        conn.close()


def accept_loop(server):
    """Background thread: accept connections and enqueue requests."""
    while True:
        try:
            conn, _ = server.accept()
            data = conn.recv(1024).decode().strip()
            if data:
                request_queue.put((conn, data))
            else:
                conn.close()
        except socket.timeout:
            continue
        except Exception:
            break


def loop():
    """Main-thread loop: drain the request queue and process via Bridge."""
    while not request_queue.empty():
        try:
            conn, data = request_queue.get_nowait()
            process_request(data, conn)
        except queue.Empty:
            break


def main():
    server = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    server.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)
    server.bind(("0.0.0.0", ZEROCLAW_PORT))
    server.listen(5)
    server.settimeout(1.0)
    print(f"[ZeroClaw Bridge] Listening on 0.0.0.0:{ZEROCLAW_PORT}", flush=True)
    t = threading.Thread(target=accept_loop, args=(server,), daemon=True)
    t.start()
    App.run(user_loop=loop)


if __name__ == "__main__":
    main()
