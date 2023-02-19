use proc_macro::TokenStream;
use syn::{DeriveInput, Data, Fields, Ident, Type};
use quote::quote;

#[proc_macro_derive(Codec)]
pub fn derive_codec(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as DeriveInput);

    let ident = input.ident;

    let fields = if let Data::Struct(data) = input.data {
        data.fields
    } else {
        panic!("Not supported!");
    };

    //Tuple struct support may be added if necessary
    match fields {
        Fields::Named(named_fields) => {
            let field_idents: Vec<Ident> = named_fields.named.iter().map(|x| x.ident.clone().unwrap()).collect();
            let field_types: Vec<Type> = named_fields.named.iter().map(|x| x.ty.clone()).collect();

            let expanded = quote!{
                impl crate::codec_data::codec::Codec for #ident {
                    fn decode(buf: &mut (impl std::io::Read + std::io::Seek)) -> crate::codec_data::codec::Result<Self> {
                        Ok(Self {
                            #(
                                #field_idents: <#field_types>::decode(buf)?
                            ),*
                        })
                    }
        
                    fn encode(&self, buf: &mut impl std::io::Write) -> crate::codec_data::codec::Result<()> {
                        #(
                            <#field_types>::encode(&self.#field_idents, buf)?;
                        )*
        
                        Ok(())
                    }
                }
            };

            TokenStream::from(expanded)
        },
        Fields::Unnamed(unnamed_fields) => {
            let field_idents: Vec<String> = (0..unnamed_fields.unnamed.len()).into_iter().map(|x| ["f", &x.to_string()].concat()).collect();
            let field_types: Vec<Type> = unnamed_fields.unnamed.iter().map(|x| x.ty.clone()).collect();

            let expanded = quote!{
                impl crate::codec_data::codec::Codec for #ident {
                    fn decode(buf: &mut (impl std::io::Read + std::io::Seek)) -> crate::codec_data::codec::Result<Self> {
                        Ok(Self(
                            #(
                                <#field_types>::decode(buf)?
                            ),*
                        ))
                    }
        
                    fn encode(&self, buf: &mut impl std::io::Write) -> crate::codec_data::codec::Result<()> {
                        let Self(#(#field_idents),*) = self;
                        
                        #(
                            <#field_types>::encode(#field_idents, buf)?;
                        )*
        
                        Ok(())
                    }
                }
            };

            TokenStream::from(expanded)
        },
        Fields::Unit => {
            let expanded = quote!{
                impl crate::codec_data::codec::Codec for #ident {
                    fn decode(buf: &mut (impl std::io::Read + std::io::Seek)) -> crate::codec_data::codec::Result<Self> {
                        Ok(Self)
                    }
        
                    fn encode(&self, buf: &mut impl std::io::Write) -> crate::codec_data::codec::Result<()> {
                        Ok(())
                    }
                }
            };

            TokenStream::from(expanded)
        },
    }    
}