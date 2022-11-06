use quote::quote;

use crate::parse::SchemaNode;

pub(crate) fn generate_output(info: &syn::Ident, node: &SchemaNode) -> proc_macro2::TokenStream {
    match node {
        SchemaNode::Register(ident, nodes) => {
            let mut nodes_tokens = proc_macro2::TokenStream::new();
            for node in nodes {
                let inner = generate_output(info, node);
                nodes_tokens.extend(inner);
            }

            quote! {
                #info.references.#ident.insert(#info.version,
                    std::boxed::Box::leak(std::boxed::Box::new(|value: &mut quartz_nbt::NbtTag, from, to|{
                        #nodes_tokens
                        Ok(())
                    })));
            }
        }
        SchemaNode::All(nodes) => {
            let mut nodes_tokens = proc_macro2::TokenStream::new();
            for node in nodes {
                let inner = generate_output(info, node);
                nodes_tokens.extend(inner);
            }
            nodes_tokens
        }
        SchemaNode::Opt(val, node) => {
            let inner = generate_output(info, node);
            quote! {
                let opt_value: &mut quartz_nbt::NbtCompound = value.try_into()?;
                if let std::option::Option::Some(value) = opt_value.inner_mut().get_mut(#val) {
                    #inner
                }
            }
        }
        SchemaNode::Req(val, node) => {
            let inner = generate_output(info, node);
            quote! {
                let value: &mut quartz_nbt::NbtCompound = value.try_into()?;
                let value: &mut quartz_nbt::NbtTag = value.get_mut(#val)?;
                #inner
            }
        }
        SchemaNode::Reference(ident) => {
            let tokens = quote! {
                dfu_structures::types::convert(&crate::TYPES.#ident, <&mut quartz_nbt::NbtCompound>::try_from(value)?, from, to);
            };
            tokens
        }
        SchemaNode::List(node) => {
            let inner = generate_output(info, node);
            let tokens = quote! {
                let list: &mut quartz_nbt::NbtList = value.try_into()?;
                for value in list.iter_mut() {
                    #inner
                }
            };
            tokens
        }
        SchemaNode::MapValues(node) => {
            let inner = generate_output(info, node);
            let tokens = quote! {
                let compound: &mut quartz_nbt::NbtCompound = value.try_into()?;
                for (key, value) in compound.inner_mut().iter_mut() {
                    #inner
                }
            };
            tokens
        }
        SchemaNode::Custom(closure) => {
            quote! {
                dfu_structures::call_closure_with(#closure, value, from, to)?;
            }
        }
    }
}
