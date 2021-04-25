use proc_macro::{
    self,
    TokenStream,
};
use quote::{
    quote,
};
use syn::{
    parse_macro_input,
    DataStruct,
    DeriveInput,
    Data,
    Fields,
    FieldsNamed,
};
use std::{
    env,
    path::{
        Path,
    },
    marker::{
        Sized,
    },
    fs::{
        read_to_string,
    },
};
use convert_case::{
    Case,
    Casing,
};
use csv::{
    Reader,
};
use serde::{
    Deserialize,
};
use proc_macro2::{
    Ident,
    Span,
};

#[proc_macro_derive(MasterData)]
pub fn derive_master_data(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(input);
    let current_dir = env::current_dir().expect("the current directory");
    let csv_path = Path::new(&current_dir)
        .join("csv")
        .join(format!("{}s.csv", ident.to_string().to_case(Case::Snake)));
    let raw_csv = read_to_string(csv_path.to_str().unwrap())
        .expect(format!("Expected a readable csv file at location: {:?}", csv_path).as_str());

    let output = quote! {
        impl amethyst_master_data::MasterData for #ident {
            fn preload(world: &mut amethyst::ecs::prelude::World) {
                #ident::all(world);
            }

            fn all(world: &mut amethyst::ecs::prelude::World) -> std::vec::Vec<Self> where
                Self: Sized,
            {
                world.entry::<Vec<#ident>>()
                    .or_insert_with(|| csv::Reader::from_reader(#raw_csv.as_bytes())
                        .deserialize()
                        .filter_map(Result::ok)
                        .collect::<Vec<#ident>>()
                    )
                    .to_vec()
            }

            fn find<F>(
                world: &mut amethyst::ecs::prelude::World,
                predicate: F,
            ) -> Option<Self> where
                F: Fn(&Self) -> bool,
                Self: Sized,
            {
                #ident::all(world)
                    .iter()
                    .find(|&item| predicate(&item))
                    .map(|item| item.clone())
            }

            fn filter<F>(
                world: &mut amethyst::ecs::prelude::World,
                predicate: F,
            ) -> Vec<Self> where
                F: Fn(&Self) -> bool,
                Self: Sized,
            {
                #ident::all(world)
                    .iter()
                    .filter(|&item| predicate(&item))
                    .map(|item| item.clone())
                    .collect()
            }
        }

        impl amethyst::ecs::Component for #ident {
            type Storage = amethyst::ecs::DenseVecStorage<Self>;
        }
    };

    output.into()
}
