
pub trait DynTMValue {
    fn read(&mut self, bytes: &[u8]);
    fn write(&self, mem: &mut [u8]);
    fn type_name(&self) -> &str;
}

pub trait TMValue: DynTMValue + Default {
    const BYTE_SIZE: usize;
    fn from_bytes(bytes: [u8; Self::BYTE_SIZE]) -> Self {
        let mut value: Self = Self::default();
        Self::read(&mut value, &bytes);
        value
    }
    fn to_bytes(&self) -> [u8; Self::BYTE_SIZE] {
        let mut bytes = [0u8; Self::BYTE_SIZE];
        self.write(&mut bytes);
        bytes
    }
}

macro_rules! primitive_value {
    ($type:ident, $name:literal) => {
        impl DynTMValue for $type {
            fn read(&mut self, bytes: &[u8]) {
                *self = Self::from_le_bytes(bytes.try_into().expect("wrong memory provided"));
            }
            fn write(&self, mem: &mut [u8]) {
                let bytes = self.to_le_bytes();
                assert_eq!(bytes.len(), mem.len(), "wrong memory provided");
                mem.copy_from_slice(&bytes);
            }
            fn type_name(&self) -> &str {
                $name
            }
        }
        impl TMValue for $type {
            const BYTE_SIZE: usize = size_of::<Self>();
        }
    };
}

primitive_value!(u8, "uint8");
primitive_value!(u16, "uint16");
primitive_value!(u32, "uint32");
primitive_value!(u64, "uint64");
primitive_value!(u128, "uint128");

primitive_value!(i8, "int8");
primitive_value!(i16, "int16");
primitive_value!(i32, "int32");
primitive_value!(i64, "int64");
primitive_value!(i128, "int128");

primitive_value!(f32, "float32");
primitive_value!(f64, "float64");
