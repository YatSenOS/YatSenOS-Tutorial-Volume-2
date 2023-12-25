use core::fmt;

/// A port-mapped UART 16550 serial interface.
pub struct SerialPort;

impl SerialPort {
    pub const fn new(port: u16) -> Self {
        Self
    }

    /// Initializes the serial port.
    pub fn init(&self) {
        // FIXME: Initialize the serial port
    }

    /// Sends a byte on the serial port.
    pub fn send(&mut self, data: u8) {
        // FIXME: Send a byte on the serial port
    }

    /// Receives a byte on the serial port no wait.
    pub fn receive(&mut self) -> Option<u8> {
        // FIXME: Receive a byte on the serial port no wait
        None
    }
}

impl fmt::Write for SerialPort {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for byte in s.bytes() {
            self.send(byte);
        }
        Ok(())
    }
}
