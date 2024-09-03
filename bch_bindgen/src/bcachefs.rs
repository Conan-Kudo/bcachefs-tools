#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused)]

use crate::c;

include!(concat!(env!("OUT_DIR"), "/bcachefs.rs"));

use bitfield::bitfield;
bitfield! {
    pub struct bch_scrypt_flags(u64);
    pub N, _: 15, 0;
    pub R, _: 31, 16;
    pub P, _: 47, 32;
}
bitfield! {
    pub struct bch_crypt_flags(u64);
    pub TYPE, _: 4, 0;
}
use std::mem::offset_of;
impl bch_sb_field_crypt {
    pub fn scrypt_flags(&self) -> Option<bch_scrypt_flags> {
        use std::convert::TryInto;
        match bch_kdf_types(bch_crypt_flags(self.flags).TYPE().try_into().ok()?) {
            bch_kdf_types::BCH_KDF_SCRYPT => Some(bch_scrypt_flags(self.kdf_flags)),
            _ => None,
        }
    }
    pub fn key(&self) -> &bch_encrypted_key {
        &self.key
    }
}
impl PartialEq for bch_sb {
    fn eq(&self, other: &Self) -> bool {
        self.magic.b == other.magic.b
            && self.user_uuid.b == other.user_uuid.b
            && self.block_size == other.block_size
            && self.version == other.version
            && self.uuid.b == other.uuid.b
            && self.seq == other.seq
    }
}

impl bch_sb {
    pub fn crypt(&self) -> Option<&bch_sb_field_crypt> {
        unsafe {
            let ptr = bch2_sb_field_get_id(
                self as *const _ as *mut _,
                bch_sb_field_type::BCH_SB_FIELD_crypt,
            ) as *const u8;
            if ptr.is_null() {
                None
            } else {
                let offset = offset_of!(bch_sb_field_crypt, field);
                Some(&*((ptr.sub(offset)) as *const _))
            }
        }
    }
    pub fn uuid(&self) -> uuid::Uuid {
        uuid::Uuid::from_bytes(self.user_uuid.b)
    }

    pub fn number_of_devices(&self) -> u32 {
        unsafe { c::bch2_sb_nr_devices(self) }
    }

    /// Get the nonce used to encrypt the superblock
    pub fn nonce(&self) -> nonce {
        use byteorder::{LittleEndian, ReadBytesExt};
        let mut internal_uuid = &self.uuid.b[..];
        let dword1 = internal_uuid.read_u32::<LittleEndian>().unwrap();
        let dword2 = internal_uuid.read_u32::<LittleEndian>().unwrap();
        nonce {
            d: [0, 0, dword1, dword2],
        }
    }
}
impl bch_sb_handle {
    pub fn sb(&self) -> &bch_sb {
        unsafe { &*self.sb }
    }

    pub fn bdev(&self) -> &block_device {
        unsafe { &*self.bdev }
    }
}

// #[repr(u8)]
pub enum rhash_lock_head {}
pub enum srcu_struct {}
