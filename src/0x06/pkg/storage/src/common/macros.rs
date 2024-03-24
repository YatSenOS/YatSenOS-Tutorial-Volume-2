/// Used to define fields in a struct
///
/// # Example
///
/// ```rust, ignore
/// struct Example {
///     data: [u8; 10],
/// }
///
/// impl Example {
///     define_field!(u8, 0, field1);
///     define_field!(u16, 1, field2);
///     define_field!(u32, 3, field3);
///     define_field!([u8; 3], 7, field4);
/// }
///
/// impl Debug for Example {
///     fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
///         f.debug_struct("Example")
///             .field("field1", &self.field1())
///             .field("field2", &self.field2())
///             .field("field3", &self.field3())
///             .field("field4", &self.field4_str()) // to get str
///             .finish()
///     }
/// }
/// ```
macro_rules! define_field {
    (u8, $offset:expr, $name:ident) => {
        paste::item! {
                #[doc = "Get u8 from the " $name " field"]
            pub fn $name(&self) -> u8 {
                self.data.get($offset).unwrap_or(&0).clone()
            }
        }
    };

    (u16, $offset:expr, $name:ident) => {
        paste::item! {
            #[doc = "Get u16 from the " $name " field"]
            pub fn $name(&self) -> u16 {
                u16::from_le_bytes(self.data[$offset..$offset + 2].try_into().unwrap_or([0; 2]))
            }
        }
    };

    (u32, $offset:expr, $name:ident) => {
        paste::item! {
                #[doc = "Get u32 from the " $name " field"]
            pub fn $name(&self) -> u32 {
                u32::from_le_bytes(self.data[$offset..$offset + 4].try_into().unwrap_or([0; 4]))
            }
        }
    };

    ([u8; $len:expr], $offset:expr, $name:ident) => {
        paste::item! {
            #[doc = "Get `&[u8]` from the " $name " field"]
            pub fn $name(&self) -> &[u8; $len] {
                (&self.data[$offset..$offset + $len])
                    .try_into()
                    .unwrap_or(&[0; $len])
            }

            #[doc = "Get `&str` from the " $name " field"]
            pub fn [<$name _str>](&self) -> &str {
                core::str::from_utf8(&self.data[$offset..$offset+$len]).unwrap_or("")
            }
        }
    };
}
