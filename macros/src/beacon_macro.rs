use proc_macro2::TokenStream;
use quote::{quote};
use syn::{Token,
    punctuated::Punctuated,
    Meta
};

pub fn impl_macro(args: Punctuated::<Meta, Token![,]>) -> TokenStream {
    let mut args_iter = args
        .iter()
        .map(|m| if let Meta::Path(path) = m { path } else { panic!("args should all be valid paths") });
    
    let beacon_name = args_iter.next().expect("args should include beacon name");
    let telemetry_definition_root_path = args_iter.next().expect("args should include tm definition path");
    let tm_definitions: Vec<_> = args_iter.collect();
    let tm_sizes = tm_definitions
        .iter()
        .map(|p| quote!{
            #telemetry_definition_root_path::#p::BYTE_SIZE
        });
    let bounds = tm_definitions
        .iter()
        .enumerate()
        .map(|(i, p)| quote!{
            #telemetry_definition_root_path::#p::ID => (Self::get_pos(#i), Self::SIZES[#i]),
        });
    let insertions = tm_definitions
        .iter()
        .enumerate()
        .map(|(i, p)| quote!{
            #telemetry_definition_root_path::#p::ID => {
                let mut bytes = [0u8; Self::SIZES[#i]];
                value.write(&mut bytes);
                self.insert_slice(telemetry_definition, &bytes)?;
            },
        });
    let tm_values_count = tm_definitions.len();
    
    quote! {
        struct #beacon_name {
            storage: [u8; Self::BYTE_SIZE],
        }
        impl #beacon_name {
            const SIZES: [usize; #tm_values_count] = [#(#tm_sizes),*];
            const BYTE_SIZE: usize = Self::get_pos(Self::SIZES.len());
            const fn get_pos(index: usize) -> usize {
                let mut len = 0;
                let mut i = 0;
                while i < index {
                    len += Self::SIZES[i];
                    i += 1;
                }
                len
            }

            pub fn new() -> Self {
                Self {
                    storage: [0u8; Self::BYTE_SIZE],
                }
            }
            
            pub fn from_bytes(bytes: &[u8]) -> Result<Self, <&[u8] as TryInto<[u8; Self::BYTE_SIZE]>>::Error> {
                Ok(Self {
                    storage: bytes.try_into()?
                })
            }
        }
        impl DynBeacon for #beacon_name {
            fn bytes(&self) -> &[u8] {
                &self.storage
            }
            fn flush(&mut self) {
                self.storage.fill(0);
            }
            fn get_bounds(&self, telemetry_definition: &dyn DynTelemetryDefinition) -> Result<(usize, usize), BoundsError> {
                Ok(match telemetry_definition.id() {
                    #(#bounds)*
                    _ => return Err(BoundsError)
                })
            }
            fn insert_slice(&mut self, telemetry_definition: &dyn DynTelemetryDefinition, data: &[u8]) -> InsrResult {
                let (pos, size) = self.get_bounds(telemetry_definition)?;
                assert_eq!(data.len(), size, "Length of bytestream does not match length of expected type");
                self.storage[pos..(pos + size)].copy_from_slice(&data);
                Ok(())
            }
            fn insert(&mut self, telemetry_definition: &dyn DynTelemetryDefinition, value: &dyn DynTMValue) -> InsrResult {
                Ok(match telemetry_definition.id() {
                    #(#insertions)*
                    _ => return Err(BoundsError)
                })
            }
            fn get_slice<'a>(&'a self, telemetry_definition: &dyn DynTelemetryDefinition) -> ExtrResult<'a> {
                let (pos, size) = self.get_bounds(telemetry_definition)?;
                Ok(&self.storage[pos..(pos + size)])
            }
        }
    }
}
