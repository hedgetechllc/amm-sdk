#![no_std]

extern crate alloc;

use alloc::vec::Vec;
use proc_macro::TokenStream;
use quote::{format_ident, quote};

fn serialize_enum_json(enum_type: &syn::Ident, data: &syn::DataEnum) -> TokenStream {
  // Add a match arm for all possible enum variants based on their type
  let mut enum_arms: Vec<proc_macro2::TokenStream> = Vec::new();
  for variant in &data.variants {
    let variant_type = &variant.ident;
    match &variant.fields {
      syn::Fields::Named(named_fields) => {
        let (mut fields, mut values) = (Vec::new(), Vec::new());
        let mut key = alloc::format!("{{{{\"_type\":\"{variant_type}\",");
        for (idx, field) in named_fields.named.iter().enumerate() {
          let field_name = field.ident.as_ref().unwrap();
          match &field.ty {
            syn::Type::Path(type_path) => {
              fields.push(field_name);
              key += alloc::format!(
                "\"{field_name}\":{{}}{}",
                if idx + 1 < named_fields.named.len() { "," } else { "}}" }
              )
              .as_str();
              match &type_path.path.segments.first().unwrap().ident {
                field_type if field_type == "Vec" => {
                  values.push(quote! { format!("[{}]", #field_name.iter().map(|el| el.serialize_json()).collect::<Vec<_>>().join(",")) });
                }
                field_type if field_type == "Option" => {
                  values.push(quote! { #field_name.map(|el| el.serialize_json()).unwrap_or(String::from("\"\"")) });
                }
                _ => values.push(quote! { #field_name.serialize_json() }),
              }
            }
            _ => panic!("Unknown AMM Enum field type"),
          }
        }
        enum_arms.push(quote! { #enum_type::#variant_type { #(#fields),* } => format!(#key, #(#values),*) });
      }
      syn::Fields::Unnamed(unnamed_fields) => match &unnamed_fields.unnamed.first().unwrap().ty {
        syn::Type::Path(type_path) => {
          let field_details = type_path.path.segments.first().unwrap();
          match &field_details.ident {
            field_type if field_type == "u8" => {
              let variant_type_string = alloc::format!("\"{variant_type}");
              enum_arms.push(quote! { #enum_type::#variant_type(el) => #variant_type_string.to_string() + "-" + el.serialize_json().as_str() + "\"" });
            }
            _ => {
              enum_arms.push(quote! { #enum_type::#variant_type(el) => el.serialize_json() });
            }
          }
        }
        _ => panic!("Unknown AMM Enum field type"),
      },
      syn::Fields::Unit => {
        let variant_type_string = alloc::format!("\"{variant_type}\"");
        enum_arms.push(quote! { #enum_type::#variant_type => #variant_type_string.to_string() });
      }
    }
  }

  // Generate the actual serialization function
  TokenStream::from(quote! {
    impl JsonSerializer for #enum_type {
      fn serialize_json(&self) -> String {
        match self { #(#enum_arms),* }
      }
    }
  })
}

fn deserialize_enum_json(enum_type: &syn::Ident, data: &syn::DataEnum) -> TokenStream {
  // Add a match arm for all possible enum variants based on their type
  let mut enum_arms: Vec<proc_macro2::TokenStream> = Vec::new();
  let mut unit_enum_arms: Vec<proc_macro2::TokenStream> = Vec::new();
  for variant in &data.variants {
    let variant_type = &variant.ident;
    let variant_type_string = alloc::format!("{variant_type}");
    match &variant.fields {
      syn::Fields::Named(named_fields) => {
        let mut fields = Vec::new();
        for field in &named_fields.named {
          let field_name = field.ident.as_ref().unwrap();
          let field_name_string = alloc::format!("{field_name}");
          match &field.ty {
            syn::Type::Path(type_path) => {
              let field_details = type_path.path.segments.first().unwrap();
              match &field_details.ident {
                field_type if field_type == "Vec" => {
                  if let syn::PathArguments::AngleBracketed(details) = &field_details.arguments {
                    if let syn::GenericArgument::Type(syn::Type::Path(vec_path)) = details.args.first().unwrap() {
                      let content_type = &vec_path.path.segments.first().unwrap().ident;
                      fields.push(quote! { #field_name: struct_fields.get(#field_name_string).ok_or(format!("Missing AMM enum field: \"{}\"", #field_name_string))?.split(',').map(|x| #content_type::deserialize_json(x).unwrap_or_default()).collect() });
                    }
                  }
                }
                field_type if field_type == "Option" => {
                  if let syn::PathArguments::AngleBracketed(details) = &field_details.arguments {
                    if let syn::GenericArgument::Type(syn::Type::Path(option_path)) = details.args.first().unwrap() {
                      let content_type = &option_path.path.segments.first().unwrap().ident;
                      fields.push(quote! {
                        #field_name: match struct_fields.get(#field_name_string) {
                          Some(value) => { if value.is_empty() { None } else { Some(#content_type::deserialize_json(value).unwrap_or_default()) } },
                          None => None,
                        }
                      });
                    }
                  }
                }
                _ => {
                  fields.push(quote! { #field_name: #type_path::deserialize_json(struct_fields.get(#field_name_string).ok_or(format!("Missing AMM enum field: \"{}\"", #field_name_string))?)? });
                }
              }
            }
            _ => panic!("Unknown AMM Enum field type"),
          }
        }
        enum_arms.push(quote! {
          #variant_type_string => {
            let mut value;
            let mut struct_fields = BTreeMap::new();
            let (mut data, mut key) = json_next_key(json);
            while !key.is_empty() {
              (data, value) = json_next_value(data);
              struct_fields.insert(key, value);
              (data, key) = json_next_key(data);
            }
            Self::#variant_type { #(#fields),* }
          }
        });
      }
      syn::Fields::Unnamed(unnamed_fields) => match &unnamed_fields.unnamed.first().unwrap().ty {
        syn::Type::Path(type_path) => {
          let field_details = type_path.path.segments.first().unwrap();
          match &field_details.ident {
            field_type if field_type == "u8" => {
              let variant_type_string_dash = variant_type_string.clone() + "-";
              unit_enum_arms.push(quote! { x if x.contains(#variant_type_string_dash) => {
                  match json.find('-') {
                    Some(idx) => Self::#variant_type(#type_path::deserialize_json(&json[idx+1..])?),
                    None => Self::#variant_type(1),
                  }
                }
              });
            }
            _ => {
              enum_arms
                .push(quote! { #variant_type_string => Self::#variant_type(#variant_type::deserialize_json(json)?) });
            }
          }
        }
        _ => panic!("Unknown AMM Enum field type"),
      },
      syn::Fields::Unit => unit_enum_arms.push(quote! { #variant_type_string => Self::#variant_type }),
    }
  }
  unit_enum_arms.push(quote! { _ => Err(alloc::format!("Unknown enum field: {}", json))? });

  // Generate the actual deserialization function
  if enum_arms.is_empty() {
    TokenStream::from(quote! {
      impl JsonDeserializer for #enum_type {
        fn deserialize_json(json: &str) -> Result<Self, String> {
          Ok(match json { #(#unit_enum_arms),* })
        }
      }
    })
  } else {
    TokenStream::from(quote! {
      impl JsonDeserializer for #enum_type {
        fn deserialize_json(json: &str) -> Result<Self, String> {
          Ok(match json_get_type(json) {
            #(#enum_arms),*,
            _ => match json { #(#unit_enum_arms),* },
          })
        }
      }
    })
  }
}

fn serialize_struct_json(struct_type: &syn::Ident, fields: &syn::FieldsNamed) -> TokenStream {
  // Serialize each struct field based on its type
  let struct_type_string = alloc::format!("{struct_type}");
  let mut serialized_fields: Vec<proc_macro2::TokenStream> = Vec::new();
  for (idx, field) in fields.named.iter().enumerate() {
    let field_name = field.ident.as_ref().unwrap();
    match &field.ty {
      syn::Type::Path(type_path) => {
        let field_details = type_path.path.segments.first().unwrap();
        match &field_details.ident {
          field_type if field_type == "Vec" || field_type == "BTreeSet" => {
            let key = alloc::format!(
              "\"{}\":[{{}}]{}",
              format_ident!("{field_name}"),
              if idx + 1 < fields.named.len() { "," } else { "" }
            );
            serialized_fields.push(quote! { format!(#key, self.#field_name.iter().map(|el| el.serialize_json()).collect::<Vec<_>>().join(",")).as_str() });
          }
          field_type if field_type == "Option" => {
            let key = alloc::format!(
              "\"{}\":{{}}{}",
              format_ident!("{field_name}"),
              if idx + 1 < fields.named.len() { "," } else { "" }
            );
            serialized_fields.push(quote! { format!(#key, self.#field_name.as_ref().map(|el| el.serialize_json()).unwrap_or(String::from("\"\""))).as_str() });
          }
          field_type if field_type == "BTreeMap" => {
            let key = alloc::format!(
              "\"{}\":{{{{{{}}}}}}{}",
              format_ident!("{field_name}"),
              if idx + 1 < fields.named.len() { "," } else { "" }
            );
            serialized_fields.push(quote! { format!(#key, self.#field_name.iter().map(|(k, v)| format!("\"{}\":\"{}\"", k, v)).collect::<Vec<_>>().join(",")).as_str() });
          }
          _ => {
            let key = alloc::format!(
              "\"{}\":{{}}{}",
              format_ident!("{field_name}"),
              if idx + 1 < fields.named.len() { "," } else { "" }
            );
            serialized_fields.push(quote! { format!(#key, self.#field_name.serialize_json()).as_str() });
          }
        }
      }
      _ => panic!("Unknown AMM Struct field type"),
    }
  }

  // Generate the actual serialization function
  TokenStream::from(quote! {
    impl JsonSerializer for #struct_type {
      fn serialize_json(&self) -> String {
        String::from("{\"_type\":\"") + #struct_type_string + "\"," + #(#serialized_fields)+* + "}"
      }
    }
  })
}

fn deserialize_struct_json(struct_type: &syn::Ident, fields: &syn::FieldsNamed) -> TokenStream {
  // Deserialize each struct field based on its type
  let mut serialized_fields: Vec<proc_macro2::TokenStream> = Vec::new();
  for field in &fields.named {
    let field_name = field.ident.as_ref().unwrap();
    let field_name_string = alloc::format!("{field_name}");
    match &field.ty {
      syn::Type::Path(type_path) => {
        let field_details = type_path.path.segments.first().unwrap();
        match &field_details.ident {
          field_type if field_type == "Vec" => {
            if let syn::PathArguments::AngleBracketed(details) = &field_details.arguments {
              if let syn::GenericArgument::Type(syn::Type::Path(vec_path)) = details.args.first().unwrap() {
                let content_type = &vec_path.path.segments.first().unwrap().ident;
                serialized_fields.push(quote! { #field_name_string => {
                  let mut subdata = value;
                  (subdata, value) = json_next_value(subdata);
                  while !value.is_empty() {
                    parsed.#field_name.push(#content_type::deserialize_json(value)?);
                    (subdata, value) = json_next_value(subdata);
                  }
                }});
              }
            }
          }
          field_type if field_type == "BTreeSet" => {
            if let syn::PathArguments::AngleBracketed(details) = &field_details.arguments {
              if let syn::GenericArgument::Type(syn::Type::Path(vec_path)) = details.args.first().unwrap() {
                let content_type = &vec_path.path.segments.first().unwrap().ident;
                serialized_fields.push(quote! { #field_name_string => {
                  let mut subdata = value;
                  (subdata, value) = json_next_value(subdata);
                  while !value.is_empty() {
                    parsed.#field_name.insert(#content_type::deserialize_json(value)?);
                    (subdata, value) = json_next_value(subdata);
                  }
                }});
              }
            }
          }
          field_type if field_type == "Option" => {
            if let syn::PathArguments::AngleBracketed(details) = &field_details.arguments {
              if let syn::GenericArgument::Type(syn::Type::Path(option_path)) = details.args.first().unwrap() {
                let content_type = &option_path.path.segments.first().unwrap().ident;
                serialized_fields.push(
                  quote! { #field_name_string => parsed.#field_name = if value.is_empty() { None } else { Some(#content_type::deserialize_json(value)?) } },
                );
              }
            }
          }
          field_type if field_type == "BTreeMap" => {
            serialized_fields.push(quote! { #field_name_string => {
              let mut subdata = value;
              (subdata, key) = json_next_key(subdata);
              while !key.is_empty() {
                (subdata, value) = json_next_value(subdata);
                parsed.#field_name.insert(String::deserialize_json(key)?, String::deserialize_json(value)?);
                (subdata, key) = json_next_key(subdata);
              }
            }});
          }
          field_type => {
            serialized_fields
              .push(quote! { #field_name_string => parsed.#field_name = #field_type::deserialize_json(value)? });
          }
        }
      }
      _ => panic!("Unknown AMM Struct field type"),
    }
  }

  // Generate the actual deserialization function
  TokenStream::from(quote! {
    impl JsonDeserializer for #struct_type {
      fn deserialize_json(json: &str) -> Result<Self, String> {
        let mut value;
        let mut parsed = Self::default();
        let (mut data, mut key) = json_next_key(json);
        while !key.is_empty() {
          (data, value) = json_next_value(data);
          match key {
            #(#serialized_fields),*,
            _ => (),
          }
          (data, key) = json_next_key(data);
        }
        Ok(parsed)
      }
    }
  })
}

#[allow(clippy::missing_panics_doc)]
#[proc_macro_derive(JsonSerialize)]
pub fn json_serialize(tokens: TokenStream) -> TokenStream {
  if let Ok(ast) = syn::parse::<syn::DeriveInput>(tokens) {
    match &ast.data {
      syn::Data::Struct(data) => match &data.fields {
        syn::Fields::Named(named_fields) => serialize_struct_json(&ast.ident, named_fields),
        _ => panic!("Unit and tuple structs are not supported in AMM objects"),
      },
      syn::Data::Enum(data) => serialize_enum_json(&ast.ident, data),
      syn::Data::Union(_) => panic!("Union types are not supported in AMM objects"),
    }
  } else {
    panic!("Invalid input for AMM object serialization");
  }
}

#[allow(clippy::missing_panics_doc)]
#[proc_macro_derive(JsonDeserialize)]
pub fn json_deserialize(tokens: TokenStream) -> TokenStream {
  if let Ok(ast) = syn::parse::<syn::DeriveInput>(tokens) {
    match &ast.data {
      syn::Data::Struct(data) => match &data.fields {
        syn::Fields::Named(named_fields) => deserialize_struct_json(&ast.ident, named_fields),
        _ => panic!("Unit and tuple structs are not supported in AMM objects"),
      },
      syn::Data::Enum(data) => deserialize_enum_json(&ast.ident, data),
      syn::Data::Union(_) => panic!("Union types are not supported in AMM objects"),
    }
  } else {
    panic!("Invalid input for AMM object deserialization");
  }
}

#[allow(clippy::missing_panics_doc)]
#[proc_macro_derive(ModOrder)]
pub fn modification_order(tokens: TokenStream) -> TokenStream {
  if let Ok(ast) = syn::parse::<syn::DeriveInput>(tokens) {
    match &ast.data {
      syn::Data::Enum(data) => {
        let mut enum_value: usize = 0;
        let enum_type = &ast.ident;
        let mut enum_arms: Vec<proc_macro2::TokenStream> = Vec::new();
        for variant in &data.variants {
          let variant_type = &variant.ident;
          match &variant.fields {
            syn::Fields::Named(_) => enum_arms.push(quote! { Self::#variant_type { .. } => #enum_value }),
            _ => enum_arms.push(quote! { Self::#variant_type => #enum_value }),
          }
          enum_value += 1;
        }
        TokenStream::from(quote! {
          impl #enum_type {
            fn get_unique_value(&self) -> usize {
              match self { #(#enum_arms),* }
            }
          }

          impl Ord for #enum_type {
            fn cmp(&self, other: &Self) -> core::cmp::Ordering {
              self.get_unique_value().cmp(&other.get_unique_value())
            }
          }

          impl PartialOrd for #enum_type {
            fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
              Some(self.cmp(other))
            }
          }
        })
      }
      _ => panic!("Only enums are supported for AMM modification ordering"),
    }
  } else {
    panic!("Invalid input for AMM modification ordering");
  }
}
