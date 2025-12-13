//! Constants and bitflags for the ATA driver.
//!
//! reference: https://github.com/theseus-os/Theseus/blob/HEAD/kernel/ata/src/lib.rs

use alloc::boxed::Box;

bitflags! {
    /// The possible error values found in an ATA drive's error port.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub(super) struct AtaError: u8 {
        const BAD_BLOCK              = 0x80;
        const UNCORRECTABLE_DATA     = 0x40;
        const MEDIA_CHANGED          = 0x20;
        const ID_MARK_NOT_FOUND      = 0x10;
        const MEDIA_CHANGE_REQUEST   = 0x08;
        const COMMAND_ABORTED        = 0x04;
        const TRACK_0_NOT_FOUND      = 0x02;
        const ADDRESS_MARK_NOT_FOUND = 0x01;
    }
}

bitflags! {
    /// The possible status values found in an ATA drive's status port.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub(super) struct AtaStatus: u8 {
        /// When set, the drive's port values are still changing, so ports shouldn't be accessed.
        const BUSY                 = 0x80;
        /// When set, the drive is on. When cleared, the drive is sleeping or "spun down".
        const DRIVE_READY          = 0x40;
        const DRIVE_WRITE_FAULT    = 0x20;
        const DRIVE_SEEK_COMPLETE  = 0x10;
        /// When **cleared**, the drive is ready for data to be read/written.
        /// When set, the drive is handling a data request and isn't ready for another command.
        const DATA_REQUEST_READY   = 0x08;
        const CORRECTED_DATA       = 0x04;
        const INDEX                = 0x02;
        const ERROR                = 0x01;
    }
}

#[repr(u8)]
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum AtaCommand {
    /// Read sectors using PIO (28-bit LBA)
    ReadPio = 0x20,
    /// Read sectors using PIO (48-bit LBA)
    ReadPioExt = 0x24,
    /// Read sectors using DMA (28-bit LBA)
    ReadDma = 0xC8,
    /// Read sectors using DMA (48-bit LBA)
    ReadDmaExt = 0x25,
    /// Write sectors using PIO (28-bit LBA)
    WritePio = 0x30,
    /// Write sectors using PIO (48-bit LBA)
    WritePioExt = 0x34,
    /// Write sectors using DMA (28-bit LBA)
    WriteDma = 0xCA,
    /// Write sectors using DMA (48-bit LBA)
    WriteDmaExt = 0x35,
    /// Flush the drive's bus cache (28-bit LBA).
    /// This is to be used after each write.
    CacheFlush = 0xE7,
    /// Flush the drive's bus cache (48-bit LBA).
    /// This is to be used after each write.
    CacheFlushExt = 0xEA,
    /// Sends a packet, for ATAPI devices using the packet interface (PI).
    Packet = 0xA0,
    /// Get identifying details of an ATAPI drive.
    IdentifyPacket = 0xA1,
    /// Get identifying details of an ATA drive.
    IdentifyDevice = 0xEC,
}

/// The possible types of drive devices that can be attached to an IDE controller via ATA.
pub(super) enum AtaDeviceType {
    /// A parallel ATA (PATA) drive, like a hard drive.
    /// This is the type previously known as just "ATA" before SATA existed.
    ///
    /// **which is the only type of drive that is supported by the current implementation.**
    Pata(Box<[u16; 256]>),
    /// A parallel ATA (PATA) drive that uses the packet interface,
    /// like an optical CD-ROM drive.
    PataPi,
    /// A serial ATA (SATA) drive that is operating in legacy IDE emulation mode,
    /// **not the standard AHCI interface for SATA**.
    /// Some systems refer to this as a `SEMB` (SATA Enclosure Management Bridge) device,
    /// which may or may not be attached through a port multiplier.
    Sata,
    /// A serial ATA (SATA) drive that that is operating in legacy IDE emulation mode
    /// and uses the packet interface.
    SataPi,
    /// The device type is unknown.
    None,
}
