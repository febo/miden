use zerocopy::{FromBytes, Immutable, KnownLayout};

/// A struct with padding.
///
/// Zerocopy makes the padding implicit.
///
/// The memory layout of this struct is:
/// [a 0 0 0 0 0 0 0 b b b b b b b b c 0 0 0 0 0 0 0]
#[repr(C)]
#[derive(KnownLayout, FromBytes, Immutable)]
pub struct PaddedStruct {
    /// 1 byte + 7 bytes padding
    pub a: u8,

    /// 8 bytes
    pub b: u64,

    /// 1 byte + 7 bytes padding
    pub c: u8,
}

/// A struct with padding at the end.
///
/// Zerocopy makes the padding implicit.
///
/// The memory layout of this struct is:
/// [b b b b b b b b c 0 0 0 0 0 0 0]
#[repr(C)]
#[derive(KnownLayout, FromBytes, Immutable)]
pub struct EndPaddedStruct {
    /// 8 bytes
    pub a: u64,

    /// 1 byte + 7 bytes padding
    pub b: u8,
}

#[cfg(test)]
mod tests {
    use zerocopy::TryFromBytes;

    use super::*;

    #[test]
    fn test_size_of() {
        assert_eq!(core::mem::size_of::<PaddedStruct>(), 24);
        assert_eq!(core::mem::size_of::<EndPaddedStruct>(), 16);
    }

    #[test]
    fn test_try_read_from_bytes() {
        // PaddedStruct
        let data = [0u8; 24];
        let p = PaddedStruct::try_read_from_bytes(&data).unwrap();
        assert_eq!(p.a, 0);
        assert_eq!(p.b, 0);
        assert_eq!(p.c, 0);

        let data = [0u8; 16];
        let p = PaddedStruct::try_read_from_bytes(&data);
        assert!(p.is_err());

        // EndPaddedStruct
        let data = [0u8; 16];
        let p = EndPaddedStruct::try_read_from_bytes(&data).unwrap();
        assert_eq!(p.a, 0);
        assert_eq!(p.b, 0);

        let data = [0u8; 10];
        let p = EndPaddedStruct::try_read_from_bytes(&data);
        assert!(p.is_err());
    }

    #[test]
    fn try_mut_from_bytes() {
        // PaddedStruct
        let mut data = [0u8; 24];
        let p = PaddedStruct::try_mut_from_bytes(&mut data).unwrap();
        p.a = 255;
        p.b = !0;
        p.c = 255;
        assert_eq!(
            data,
            [
                255, 0, 0, 0, 0, 0, 0, 0, 255, 255, 255, 255, 255, 255, 255, 255, 255, 0, 0, 0, 0,
                0, 0, 0
            ]
        );

        // EndPaddedStruct
        let mut data = [0u8; 16];
        let p = EndPaddedStruct::try_mut_from_bytes(&mut data).unwrap();
        p.a = !0;
        p.b = 255;
        assert_eq!(
            data,
            [255, 255, 255, 255, 255, 255, 255, 255, 255, 0, 0, 0, 0, 0, 0, 0]
        );
    }
}
