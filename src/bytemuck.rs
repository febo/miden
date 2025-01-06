use bytemuck::{Pod, Zeroable};

/// A struct with padding.
///
/// Bytemuck "forces" the padding to be explicit.
///
/// The layout of this struct is:
/// [a 0 0 0 0 0 0 0 b b b b b b b b c 0 0 0 0 0 0 0]
#[repr(C)]
#[derive(Clone, Copy, Default, Pod, Zeroable)]
pub struct PaddedStruct {
    /// 1 byte
    pub a: u8,

    /// 7 bytes padding
    _padding_a: [u8; 7],

    /// 8 bytes
    pub b: u64,

    /// 1 byte
    pub c: u8,

    /// 7 bytes padding
    _padding_c: [u8; 7],
}

/// A struct with padding at the end.
///
/// Bytemuck "forces" the padding to be explicit.
///
/// The layout of this struct is:
/// [b b b b b b b b c 0 0 0 0 0 0 0]
#[repr(C)]
#[derive(Clone, Copy, Default, Pod, Zeroable)]
pub struct EndPaddedStruct {
    /// 8 bytes
    pub a: u64,

    /// 1 byte
    pub b: u8,

    /// 7 bytes padding
    _padding_b: [u8; 7],
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_size_of() {
        assert_eq!(core::mem::size_of::<PaddedStruct>(), 24);
        assert_eq!(core::mem::size_of::<EndPaddedStruct>(), 16);
    }

    #[test]
    fn test_try_from_bytes() {
        // PaddedStruct
        let data = [0u8; 24];
        let p = bytemuck::try_from_bytes::<PaddedStruct>(&data).unwrap();
        assert_eq!(p.a, 0);
        assert_eq!(p.b, 0);
        assert_eq!(p.c, 0);

        let data = [0u8; 16];
        let p = bytemuck::try_from_bytes::<PaddedStruct>(&data);
        assert!(p.is_err());

        // EndPaddedStruct
        let data = [0u8; 16];
        let p = bytemuck::try_from_bytes::<EndPaddedStruct>(&data).unwrap();
        assert_eq!(p.a, 0);
        assert_eq!(p.b, 0);

        let data = [0u8; 10];
        let p = bytemuck::try_from_bytes::<EndPaddedStruct>(&data);
        assert!(p.is_err());
    }

    #[test]
    fn try_from_bytes_mut() {
        // PaddedStruct
        let mut data = [0u8; 24];
        let p = bytemuck::try_from_bytes_mut::<PaddedStruct>(&mut data).unwrap();
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
        let p = bytemuck::try_from_bytes_mut::<EndPaddedStruct>(&mut data).unwrap();
        p.a = !0;
        p.b = 255;
        assert_eq!(
            data,
            [255, 255, 255, 255, 255, 255, 255, 255, 255, 0, 0, 0, 0, 0, 0, 0]
        );
    }
}
