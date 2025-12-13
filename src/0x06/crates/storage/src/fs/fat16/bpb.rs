//! Fat16 BIOS Parameter Block
//!
//! reference:
//! - <https://en.wikipedia.org/wiki/BIOS_parameter_block>
//! - <https://wiki.osdev.org/FAT#Boot_Record>

use crate::*;

/// Represents a Boot Parameter Block.
///
/// This is the first sector of a FAT 16 formatted partition,
/// and it describes various properties of the FAT 16 filesystem.
pub struct Fat16Bpb {
    data: [u8; 512],
}

impl Fat16Bpb {
    /// Attempt to parse a Boot Parameter Block from a 512 byte sector.
    pub fn new(data: &[u8]) -> FsResult<Fat16Bpb> {
        let data = data.try_into().unwrap();
        let bpb = Fat16Bpb { data };

        if bpb.data.len() != 512 || bpb.trail() != 0xAA55 {
            return Err(FsError::InvalidOperation);
        }

        Ok(bpb)
    }

    pub fn total_sectors(&self) -> u32 {
        if self.total_sectors_16() == 0 {
            self.total_sectors_32()
        } else {
            self.total_sectors_16() as u32
        }
    }

    // FIXME: define all the fields in the BPB
    //      - use `define_field!` macro
    //      - ensure you can pass the tests
    //      - you may change the field names if you want
}

impl core::fmt::Debug for Fat16Bpb {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Fat16 BPB")
            .field("OEM Name", &self.oem_name_str())
            .field("Bytes per Sector", &self.bytes_per_sector())
            .field("Sectors per Cluster", &self.sectors_per_cluster())
            .field("Reserved Sector Count", &self.reserved_sector_count())
            .field("FAT Count", &self.fat_count())
            .field("Root Entries Count", &self.root_entries_count())
            .field("Total Sectors", &self.total_sectors())
            .field("Media Descriptor", &self.media_descriptor())
            .field("Sectors per FAT", &self.sectors_per_fat())
            .field("Sectors per Track", &self.sectors_per_track())
            .field("Track Count", &self.track_count())
            .field("Hidden Sectors", &self.hidden_sectors())
            .field("Total Sectors", &self.total_sectors())
            .field("Drive Number", &self.drive_number())
            .field("Reserved Flags", &self.reserved_flags())
            .field("Boot Signature", &self.boot_signature())
            .field("Volume ID", &self.volume_id())
            .field("Volume Label", &self.volume_label_str())
            .field("System Identifier", &self.system_identifier_str())
            .field("Trail", &self.trail())
            .finish()
    }
}

/// Test the `Fat16Bpb` struct
///
/// WARN: do not modify following test code unless you changed the field names
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fat16_bpb_1() {
        // Taken from a Raspberry Pi bootable SD-Card
        const DATA: [u8; 192] = hex_literal::hex!(
            "EB 3C 90 6D 6B 66 73 2E 66 61 74 00 02 10 01 00
        02 00 02 00 00 F8 20 00 3F 00 FF 00 00 00 00 00
        00 E0 01 00 80 01 29 BB B0 71 77 62 6F 6F 74 20
        20 20 20 20 20 20 46 41 54 31 36 20 20 20 0E 1F
        BE 5B 7C AC 22 C0 74 0B 56 B4 0E BB 07 00 CD 10
        5E EB F0 32 E4 CD 16 CD 19 EB FE 54 68 69 73 20
        69 73 20 6E 6F 74 20 61 20 62 6F 6F 74 61 62 6C
        65 20 64 69 73 6B 2E 20 20 50 6C 65 61 73 65 20
        69 6E 73 65 72 74 20 61 20 62 6F 6F 74 61 62 6C
        65 20 66 6C 6F 70 70 79 20 61 6E 64 0D 0A 70 72
        65 73 73 20 61 6E 79 20 6B 65 79 20 74 6F 20 74
        72 79 20 61 67 61 69 6E 20 2E 2E 2E 20 0D 0A 00"
        );
        
        let mut bpb_data = Vec::with_capacity(512);
        bpb_data.extend_from_slice(&DATA);
        bpb_data.resize(510, 0u8);
        bpb_data.extend_from_slice(&[0x55, 0xAA]);

        let bpb = Fat16Bpb::new(&bpb_data).unwrap();

        assert_eq!(bpb.oem_name(), b"mkfs.fat");
        assert_eq!(bpb.bytes_per_sector(), 512);
        assert_eq!(bpb.sectors_per_cluster(), 16);
        assert_eq!(bpb.reserved_sector_count(), 1);
        assert_eq!(bpb.fat_count(), 2);
        assert_eq!(bpb.root_entries_count(), 512);
        assert_eq!(bpb.total_sectors_16(), 0);
        assert_eq!(bpb.media_descriptor(), 0xf8);
        assert_eq!(bpb.sectors_per_fat(), 32);
        assert_eq!(bpb.sectors_per_track(), 63);
        assert_eq!(bpb.track_count(), 255);
        assert_eq!(bpb.hidden_sectors(), 0);
        assert_eq!(bpb.total_sectors_32(), 0x1e000);
        assert_eq!(bpb.drive_number(), 128);
        assert_eq!(bpb.reserved_flags(), 1);
        assert_eq!(bpb.boot_signature(), 0x29);
        assert_eq!(bpb.volume_id(), 0x7771b0bb);
        assert_eq!(bpb.volume_label(), b"boot       ");
        assert_eq!(bpb.system_identifier(), b"FAT16   ");

        assert_eq!(bpb.total_sectors(), 0x1e000);

        println!("{:#?}", bpb);
    }

    #[test]
    fn test_fat16_bpb_2() {
        // Taken from QEMU VVFAT
        const DATA: [u8; 64] = hex_literal::hex!(
            "EB 3E 90 4D 53 57 49 4E 34 2E 31 00 02 10 01 00
        02 00 02 00 00 F8 FC 00 3F 00 10 00 3F 00 00 00
        C1 BF 0F 00 80 00 29 FD 1A BE FA 51 45 4D 55 20
        56 56 46 41 54 20 46 41 54 31 36 20 20 20 00 00"
        );

        let mut bpb_data = Vec::with_capacity(512);
        bpb_data.extend_from_slice(&DATA);
        bpb_data.resize(510, 0u8);
        bpb_data.extend_from_slice(&[0x55, 0xAA]);

        let bpb = Fat16Bpb::new(&bpb_data).unwrap();

        assert_eq!(bpb.oem_name(), b"MSWIN4.1");
        assert_eq!(bpb.oem_name_str(), "MSWIN4.1");
        assert_eq!(bpb.bytes_per_sector(), 512);
        assert_eq!(bpb.sectors_per_cluster(), 16);
        assert_eq!(bpb.reserved_sector_count(), 1);
        assert_eq!(bpb.fat_count(), 2);
        assert_eq!(bpb.root_entries_count(), 512);
        assert_eq!(bpb.total_sectors_16(), 0);
        assert_eq!(bpb.media_descriptor(), 0xf8);
        assert_eq!(bpb.sectors_per_fat(), 0xfc);
        assert_eq!(bpb.sectors_per_track(), 63);
        assert_eq!(bpb.track_count(), 16);
        assert_eq!(bpb.hidden_sectors(), 63);
        assert_eq!(bpb.total_sectors_32(), 0xfbfc1);
        assert_eq!(bpb.drive_number(), 128);
        assert_eq!(bpb.reserved_flags(), 0);
        assert_eq!(bpb.boot_signature(), 0x29);
        assert_eq!(bpb.volume_id(), 0xfabe1afd);
        assert_eq!(bpb.volume_label(), b"QEMU VVFAT ");
        assert_eq!(bpb.volume_label_str(), "QEMU VVFAT ");
        assert_eq!(bpb.system_identifier(), b"FAT16   ");
        assert_eq!(bpb.system_identifier_str(), "FAT16   ");

        assert_eq!(bpb.total_sectors(), 0xfbfc1);

        println!("{:#?}", bpb);
    }
}
