//! ATA Bus
//!
//! reference: https://wiki.osdev.org/IDE
//! reference: https://wiki.osdev.org/ATA_PIO_Mode
//! reference: https://github.com/theseus-os/Theseus/blob/HEAD/kernel/ata/src/lib.rs

use super::consts::*;
use alloc::boxed::Box;
use x86_64::instructions::port::*;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Bus {
    id: u8,
    irq: u8,
    io_base: u16,
    ctrl_base: u16,
    data: Port<u16>,
    error: PortReadOnly<u8>,
    features: PortWriteOnly<u8>,
    sector_count: Port<u8>,
    /// Also used for sector_number
    lba_low: Port<u8>,
    /// Also used for cylinder_low
    lba_mid: Port<u8>,
    /// Also used for cylinder_high
    lba_high: Port<u8>,
    drive: Port<u8>,
    status: PortReadOnly<u8>,
    command: PortWriteOnly<u8>,
    alternate_status: PortReadOnly<u8>,
    control: PortWriteOnly<u8>,
    drive_blockess: PortReadOnly<u8>,
}

impl Bus {
    pub fn new(id: u8, irq: u8, io_base: u16, ctrl_base: u16) -> Self {
        Self {
            id,
            irq, // actually not used as we poll the status
            io_base,
            ctrl_base,
            data: Port::<u16>::new(io_base),
            error: PortReadOnly::<u8>::new(io_base + 1),
            features: PortWriteOnly::<u8>::new(io_base + 1),
            sector_count: Port::<u8>::new(io_base + 2),
            lba_low: Port::<u8>::new(io_base + 3),
            lba_mid: Port::<u8>::new(io_base + 4),
            lba_high: Port::<u8>::new(io_base + 5),
            drive: Port::<u8>::new(io_base + 6),
            status: PortReadOnly::new(io_base + 7),
            command: PortWriteOnly::new(io_base + 7),

            alternate_status: PortReadOnly::new(ctrl_base),
            control: PortWriteOnly::new(ctrl_base),
            drive_blockess: PortReadOnly::new(ctrl_base + 1),
        }
    }

    #[inline]
    fn read_data(&mut self) -> u16 {
        unsafe { self.data.read() }
    }

    #[inline]
    fn write_data(&mut self, data: u16) {
        unsafe { self.data.write(data) }
    }

    /// Also used for LBAmid
    #[inline]
    fn cylinder_low(&mut self) -> u8 {
        unsafe { self.lba_mid.read() }
    }

    /// Also used for LBAhi
    #[inline]
    fn cylinder_high(&mut self) -> u8 {
        unsafe { self.lba_high.read() }
    }

    /// Reads the `status` port and returns the value as an `AtaStatus` bitfield.
    /// Because some buses operate (change wire values) very slowly,
    /// this undergoes the standard procedure of reading the alternate status port
    /// and discarding it 4 times before reading the real status port value.
    /// Each read is a 100ns delay, so the total delay of 400ns is proper.
    #[inline]
    fn status(&mut self) -> AtaStatus {
        AtaStatus::from_bits_truncate(unsafe {
            // wait for 400ns
            self.alternate_status.read();
            self.alternate_status.read();
            self.alternate_status.read();
            self.alternate_status.read();
            // read the status
            self.status.read()
        })
    }

    /// Reads the `error` port and returns the value as an `AtaError` bitfield.
    #[inline]
    fn error(&mut self) -> AtaError {
        AtaError::from_bits_truncate(unsafe { self.error.read() })
    }

    /// Returns true if the `status` port indicates an error.
    #[inline]
    fn is_error(&mut self) -> bool {
        self.status().contains(AtaStatus::ERROR)
    }

    /// Polls the `status` port until the given bit is set to the given value.
    #[inline]
    fn poll(&mut self, bit: AtaStatus, val: bool) {
        let mut status = self.status();
        while status.intersects(bit) != val {
            if status.contains(AtaStatus::ERROR) {
                self.debug();
            }
            core::hint::spin_loop();
            status = self.status();
        }
    }

    /// Log debug information about the bus
    fn debug(&mut self) {
        warn!("ATA error register  : {:?}", self.error());
        warn!("ATA status register : {:?}", self.status());
    }

    /// Selects the given drive (0 or 1) by writing to the `drive` port.
    ///
    /// - 0: Master
    /// - 1: Slave
    fn select_drive(&mut self, drive: u8) {
        debug_assert!(drive < 2);
        self.poll(AtaStatus::BUSY, false);
        self.poll(AtaStatus::DATA_REQUEST_READY, false);

        unsafe {
            // FIXME: select the drive
        }

        self.poll(AtaStatus::BUSY, false);
        self.poll(AtaStatus::DATA_REQUEST_READY, false);
    }

    /// Sets up the PIO mode for the given drive and block.
    ///
    /// reference: https://wiki.osdev.org/ATA_PIO_Mode#28_bit_PIO
    fn setup_pio(&mut self, drive: u8, block: u32, cmd: AtaCommand) {
        self.select_drive(drive);

        let bytes = block.to_le_bytes(); // a trick to convert u32 to [u8; 4]
        unsafe {
            // just 1 sector for current implementation
            self.sector_count.write(1);

            // FIXME: store the LBA28 address into four 8-bit registers
            //      - read the documentation for more information
            // FIXME: enable LBA28 mode
            // FIXME: write the command register (cmd as u8)
        }
    }

    /// Writes the given command
    fn write_command(&mut self, drive: u8, block: u32, cmd: AtaCommand) -> storage::Result<()> {
        self.setup_pio(drive, block, cmd);

        if self.status().is_empty() {
            // drive does not exist
            return Err(storage::DeviceError::UnknownDevice.into());
        }

        // FIXME: poll for the status to be not BUSY

        if self.is_error() {
            warn!("ATA error: {:?} command error", cmd);
            self.debug();
            return Err(storage::DeviceError::InvalidOperation.into());
        }

        // FIXME: poll for the status to be not BUSY and DATA_REQUEST_READY

        Ok(())
    }

    /// Identifies the drive at the given `drive` number (0 or 1).
    ///
    /// reference: <https://wiki.osdev.org/ATA_PIO_Mode#IDENTIFY_command>
    pub(super) fn identify_drive(&mut self, drive: u8) -> storage::Result<AtaDeviceType> {
        info!("Identifying drive {}", drive);

        // FIXME: use `AtaCommand::IdentifyDevice` to identify the drive
        //      - call `write_command` with `drive` and `0` as the block number
        //      - if the status is empty, return `AtaDeviceType::None`
        //      - else return `DeviceError::UnknownDevice` as `FsError`

        // FIXME: poll for the status to be not BUSY

        Ok(match (self.cylinder_low(), self.cylinder_high()) {
            // we only support PATA drives
            (0x00, 0x00) => AtaDeviceType::Pata(Box::new([0u16; 256].map(|_| self.read_data()))),
            // ignore the data as we don't support following types
            (0x14, 0xEB) => AtaDeviceType::PataPi,
            (0x3C, 0xC3) => AtaDeviceType::Sata,
            (0x69, 0x96) => AtaDeviceType::SataPi,
            _ => AtaDeviceType::None,
        })
    }

    /// Reads a block from the given drive and block number into the given buffer.
    ///
    /// reference: https://wiki.osdev.org/ATA_PIO_Mode#28_bit_PIO
    /// reference: https://wiki.osdev.org/IDE#Read.2FWrite_From_ATA_Drive
    pub(super) fn read_pio(
        &mut self,
        drive: u8,
        block: u32,
        buf: &mut [u8],
    ) -> storage::Result<()> {
        self.write_command(drive, block, AtaCommand::ReadPio)?;

        // FIXME: read the data from the data port into the buffer
        //      - use `buf.chunks_mut(2)`
        //      - use `self.read_data()`
        //      - ! pay attention to data endianness

        if self.is_error() {
            debug!("ATA error: data read error");
            self.debug();
            Err(storage::DeviceError::ReadError.into())
        } else {
            Ok(())
        }
    }

    /// Writes a block to the given drive and block number from the given buffer.
    ///
    /// reference: https://wiki.osdev.org/ATA_PIO_Mode#28_bit_PIO
    /// reference: https://wiki.osdev.org/IDE#Read.2FWrite_From_ATA_Drive
    pub(super) fn write_pio(&mut self, drive: u8, block: u32, buf: &[u8]) -> storage::Result<()> {
        self.write_command(drive, block, AtaCommand::WritePio)?;

        // FIXME: write the data from the buffer into the data port
        //      - use `buf.chunks(2)`
        //      - use `self.write_data()`
        //      - ! pay attention to data endianness

        if self.is_error() {
            debug!("ATA error: data write error");
            self.debug();
            Err(storage::DeviceError::WriteError.into())
        } else {
            Ok(())
        }
    }
}
