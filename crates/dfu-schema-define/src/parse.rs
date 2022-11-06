use syn::parse::{Parse, ParseStream};
use syn::spanned::Spanned;
use syn::{braced, parenthesized, ExprClosure, ExprLit, Lit, Token};

pub(crate) struct SchemaDefinition {
    pub(crate) info: syn::Ident,
    pub(crate) root: SchemaTree,
}

impl Parse for SchemaDefinition {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let info: syn::Ident = input.parse()?;
        input.parse::<Token![,]>()?;
        let root: SchemaTree = input.parse()?;

        Ok(SchemaDefinition { info, root })
    }
}

#[derive(Debug)]
pub(crate) struct SchemaTree {
    pub nodes: Vec<SchemaNode>,
}

impl Parse for SchemaTree {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        braced!(content in input);

        let mut nodes = Vec::new();

        loop {
            let node: SchemaNode = content.parse()?;
            nodes.push(node);

            // if there is a comma, parse another
            let lookahead = content.lookahead1();
            if !lookahead.peek(Token![,]) {
                break;
            }
            content.parse::<Token![,]>()?; // consume the comma
        }

        Ok(Self { nodes })
    }
}

#[derive(Debug)]
pub(crate) enum SchemaNode {
    Register(syn::Ident, Vec<SchemaNode>),
    All(Vec<SchemaNode>),
    Opt(String, Box<SchemaNode>),
    Req(String, Box<SchemaNode>),
    Reference(syn::Ident),
    List(Box<SchemaNode>),
    MapValues(Box<SchemaNode>),
    Custom(ExprClosure),
}

impl Parse for SchemaNode {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ident: syn::Ident = input.parse()?;
        let node_ident = ident.to_string();
        let node_ident: &str = node_ident.as_str();
        match node_ident {
            "register" => {
                let registry: syn::Ident = input.parse()?;
                let content;
                braced!(content in input);

                let mut nodes = Vec::new();
                loop {
                    // parse a schema node
                    let node: SchemaNode = content.parse()?;
                    nodes.push(node);

                    // if there is a comma, parse another
                    let lookahead = content.lookahead1();
                    if !lookahead.peek(Token![,]) {
                        break;
                    }
                    content.parse::<Token![,]>()?; // consume the comma
                }

                Ok(SchemaNode::Register(registry, nodes))
            }
            "all" => {
                let content;
                parenthesized!(content in input);

                let mut nodes = Vec::new();
                loop {
                    // parse a schema node
                    let node: SchemaNode = content.parse()?;
                    nodes.push(node);

                    // if there is a comma, parse another
                    let lookahead = content.lookahead1();
                    if !lookahead.peek(Token![,]) {
                        break;
                    }
                    content.parse::<Token![,]>()?; // consume the comma
                }

                Ok(SchemaNode::All(nodes))
            }
            "req" => {
                let content;
                parenthesized!(content in input);
                let val: ExprLit = content.parse()?;
                content.parse::<Token![,]>()?;
                if let Lit::Str(s) = val.lit {
                    Ok(SchemaNode::Req(s.value(), Box::new(content.parse()?)))
                } else {
                    Err(syn::Error::new(val.span(), "expected string literal"))
                }
            }
            "opt" => {
                let content;
                parenthesized!(content in input);
                let val: ExprLit = content.parse()?;
                content.parse::<Token![,]>()?;
                if let Lit::Str(s) = val.lit {
                    Ok(SchemaNode::Opt(s.value(), Box::new(content.parse()?)))
                } else {
                    Err(syn::Error::new(val.span(), "expected string literal"))
                }
            }
            "refer" | "reference" => {
                let content;
                parenthesized!(content in input);
                let ident: syn::Ident = content.parse()?;
                Ok(SchemaNode::Reference(ident))
            }
            "list" => {
                let content;
                parenthesized!(content in input);
                let node: SchemaNode = content.parse()?;
                Ok(SchemaNode::List(Box::new(node)))
            }
            "map_values" => {
                let content;
                parenthesized!(content in input);
                let node: SchemaNode = content.parse()?;
                Ok(SchemaNode::MapValues(Box::new(node)))
            }
            "custom" => {
                let content;
                parenthesized!(content in input);
                let closure: ExprClosure = content.parse()?;
                Ok(SchemaNode::Custom(closure))
            }
            _ => Err(syn::Error::new(ident.span(), "unknown identifier")),
        }
    }
}
