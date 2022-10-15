extern crate proc_macro;

use crate::proc_macro::TokenStream;
use quote::quote;
use syn::{
    self, Data::Struct, DataStruct, DeriveInput, Field, Fields::Named, FieldsNamed
};

// #[proc_macro_derive(MyMacro)]
// pub fn my_macro(input:TokenStream) -> TokenStream {
//     let st = syn::parse(input).unwrap();

//     match do_expand(&st) {
//         Ok(token_stream) => token_stream.into(),
//         Err(e) => e.to_compile_error().into()
//     }
//     // impl_macro_fn(&st)
//     // TokenStream::new()
// }

// fn impl_macro_fn(st: &DeriveInput) -> TokenStream {
//     let name = &st.ident;
//     let gen = quote!{
//         impl #name {
//             fn hello_macro() {
//                 println!("my name is{}",stringify!(#name));
//             }
//         }
//     };
//     gen.into()
// }

// type StructFields = syn::punctuated::Punctuated<syn::Field,syn::Token![,]>;
// fn get_fields_from_derive_input(d: &syn::DeriveInput) -> syn::Result<&StructFields> {
//     if let syn::Data::Struct(syn::DataStruct {
//         fields: syn::Fields::Named(syn::FieldsNamed { ref named, .. }),
//         ..
//     }) = d.data{
//         return Ok(named)
//     }
//     Err(syn::Error::new_spanned(d, "Must define on a Struct, not Enum".to_string()))
// }
// fn generate_builder_struct_fields_def(fields: &StructFields) -> syn::Result<proc_macro2::TokenStream>{
//     let idents:Vec<_> = fields.iter().map(|f| {&f.ident}).collect();
//     let types:Vec<_> = fields.iter().map(|f| {&f.ty}).collect();

//     let token_stream = quote!{
//         #(#idents: std::option::Option<#types>),*
//     };
//     Ok(token_stream)
// }

// fn generate_builder_struct_factory_init_clauses(fields: &StructFields) -> syn::Result<Vec<proc_macro2::TokenStream>>{
//     let init_clauses: Vec<_> = fields.iter().map(|f| {
//         let ident = &f.ident;
//         quote!{
//             #ident: std::option::Option::None
//         }
//     }).collect();

//     Ok(init_clauses)
// }

// fn generate_struct_ident_init(fields: &StructFields) -> syn::Result<Vec<proc_macro2::TokenStream>> {
//     let init_cluase:Vec<_> = fields.iter().map(|f| {
//         let ident = &f.ident;
//         quote::quote!(
//             #ident: std::option::Option::None
//         )
//     }).collect();
//     Ok(init_cluase)
// }

// fn do_expand(st: &syn::DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
//     let struct_name_literal = st.ident.to_string();
//     let builder_name_literal = format!("{}Builder", struct_name_literal);
//     let builder_name_ident = syn::Ident::new(&builder_name_literal, st.span());

//     let struct_ident = &st.ident;

//     let fields = get_fields_from_derive_input(st)?;
//     let builder_struct_fields_def = generate_builder_struct_fields_def(fields)?;
//     // 下面这一行是新加的
//     let builder_struct_factory_init_clauses = generate_builder_struct_factory_init_clauses(fields)?;

//     let ret = quote! {
//         pub struct #builder_name_ident {
//             #builder_struct_fields_def
//         }
//         impl #struct_ident {
//             pub fn builder() -> #builder_name_ident {
//                 #builder_name_ident{
//                     // 下面这一行是新加的，注意我们在这里重复展开了每一个字段
//                     #(#builder_struct_factory_init_clauses),*
//                 }
//             }
//             pub fn idss() -> Vec<String> {
//                 // #builder_name_literal.to_string()
//                 vec!["a".to_string(),"b".to_string()]
//             }
//         }
//     };

//     return Ok(ret);
// }

#[derive(Debug)]
struct Entity {
    // name: String,
    fields: Vec<EntityField>,
}
#[derive(Debug)]
struct EntityField {
    name: String,
    // ty: String,
}

fn get_entity_field(field: &Field) -> Option<EntityField> {
    let ident = match &field.ident {
        Some(id) => Some(format!("{}", id)),
        None => {
            return None;
        }
    };
    // let ty_ident = match &field.ty {
    //     Type::Path(TypePath {
    //         path: Path { segments, .. },
    //         ..
    //     }) => segments.first().and_then(|s| Some(format!("{}", s.ident))),
    //     _ => {
    //         return None;
    //     }
    // };
    let entity_field = EntityField {
        name: ident.unwrap(),
        // ty: ty_ident.unwrap(),
    };
    Some(entity_field)
}

#[proc_macro_derive(Entity)]
pub fn derive(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, data, .. } = syn::parse(input).unwrap();
    let field = if let Struct(DataStruct {
        fields: Named(FieldsNamed { ref named, .. }),
        ..
    }) = data
    {
        named
    } else {
        panic!("This is not supported.")
    };
    let entity = Entity {
        // name: format!("{}", ident),
        fields: field.iter().filter_map(|f| get_entity_field(f)).collect(),
    };
    let fields: Vec<String> = entity.fields.iter().map(|f| f.name.to_string()).collect();
    let size = fields.len();
    let columns = fields.join(",");
    let select_string = format!("{}", &columns);
    let result = quote! {
        impl #ident {
            pub fn select() -> String {
                // println!("{:?}",#fields);
                ::std::string::String::from(#select_string)
                // #(#fields: std::vec::Vec::new()),*
                // #fields.to_vec()
            }
            pub fn size() -> usize {
                #size
            }
        }
    };
    result.into()
    // TokenStream::new()
}
