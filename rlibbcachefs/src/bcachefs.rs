#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused)]

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
    TYPE, _: 4, 0;
}
use memoffset::offset_of;
impl bch_sb_field_crypt {
    pub fn scrypt_flags(&self) -> Option<bch_scrypt_flags> {
        let t = bch_crypt_flags(self.flags);
        if t.TYPE() != bch_kdf_types::BCH_KDF_SCRYPT as u64 {
            None
        } else {
            Some(bch_scrypt_flags(self.kdf_flags))
        }
    }
    pub fn key(&self) -> &bch_encrypted_key {
        &self.key
    }
}
impl bch_sb {
    pub fn crypt(&self) -> Option<&bch_sb_field_crypt> {
        unsafe {
            let ptr = bch2_sb_field_get(
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

    /// Get the nonce used to encrypt the superblock
    pub fn nonce(&self) -> nonce {
        use byteorder::{ReadBytesExt, LittleEndian};
        let mut internal_uuid = &self.uuid.b[..];
        let dword1 = internal_uuid.read_u32::<LittleEndian>().unwrap();
        let dword2 = internal_uuid.read_u32::<LittleEndian>().unwrap();
        nonce { d: [0, 0, dword1, dword2] }
    }
}
impl bch_sb_handle {
    pub fn sb(&self) -> &bch_sb {
        unsafe { &*self.sb }
    }
}