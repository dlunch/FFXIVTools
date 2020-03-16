use std::convert::TryInto;

pub trait SliceByteOrderExt {
    fn read_int_be<T>(&self) -> T
    where
        T: Integer;

    fn read_float_be<T>(&self) -> T
    where
        T: Float;
}

impl SliceByteOrderExt for &[u8] {
    fn read_int_be<T>(&self) -> T
    where
        T: Integer,
    {
        let sliced = &self[..std::mem::size_of::<T>()];

        T::from_be_bytes(sliced)
    }

    fn read_float_be<T>(&self) -> T
    where
        T: Float,
    {
        let sliced = &self[..std::mem::size_of::<T>()];

        T::from_be_bytes(sliced)
    }
}

pub trait Integer {
    fn from_be_bytes(bytes: &[u8]) -> Self;
}

impl Integer for u32 {
    fn from_be_bytes(bytes: &[u8]) -> Self {
        Self::from_be_bytes(bytes.try_into().unwrap())
    }
}

impl Integer for i32 {
    fn from_be_bytes(bytes: &[u8]) -> Self {
        Self::from_be_bytes(bytes.try_into().unwrap())
    }
}

impl Integer for u16 {
    fn from_be_bytes(bytes: &[u8]) -> Self {
        Self::from_be_bytes(bytes.try_into().unwrap())
    }
}

impl Integer for i16 {
    fn from_be_bytes(bytes: &[u8]) -> Self {
        Self::from_be_bytes(bytes.try_into().unwrap())
    }
}

impl Integer for u8 {
    fn from_be_bytes(bytes: &[u8]) -> Self {
        Self::from_be_bytes(bytes.try_into().unwrap())
    }
}

impl Integer for i8 {
    fn from_be_bytes(bytes: &[u8]) -> Self {
        Self::from_be_bytes(bytes.try_into().unwrap())
    }
}

pub trait Float {
    fn from_be_bytes(bytes: &[u8]) -> Self;
}

impl Float for f32 {
    fn from_be_bytes(bytes: &[u8]) -> Self {
        Self::from_be_bytes(bytes.try_into().unwrap())
    }
}
