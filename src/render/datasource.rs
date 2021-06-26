use super::super::parse::{self, *};
use super::config::RendererConfig;
use super::fields::*;
use super::fields::{field_is_method_or_member, ResolverType};
use super::sorter::sort_by_line_pos;
use super::typ::*;
use super::RenderContext;
use anyhow::Result;
use heck::SnakeCase;
use proc_macro2::TokenStream;
use quote::*;

pub fn empty_datasource_methods(
    schema: &StructuredSchema,
    render_config: &RendererConfig,
) -> Result<Vec<TokenStream>> {
    let mut objects: Vec<&Object> = schema.definitions.objects.values().into_iter().collect();
    if objects.is_empty() {
        return Ok(vec![]);
    }
    objects.sort_by(sort_by_line_pos);

    let custom_member_types = render_config.custom_member_types();

    let resolver_setting = render_config.resolver_setting();

    let mut result = Vec::<TokenStream>::new();
    for object in objects {
        let render_context = RenderContext {
            parent: TypeDef::Object(object),
        };
        let field_resolver = resolver_setting.get(&object.name);

        for field in object.fields.iter() {
            if let ResolverType::Method = field_is_method_or_member(
                &field,
                &schema,
                &render_context,
                &render_config,
                &field_resolver,
                &custom_member_types,
            )? {
                result.push(datasouerce_token_method(&field, &schema, &render_context)?);
            }
        }
    }

    Ok(result)
}

fn datasouerce_token_method(
    field: &parse::Field,
    schema: &StructuredSchema,
    context: &RenderContext,
) -> Result<TokenStream> {
    let parent_name = format_ident!("{}", context.parent_name());

    let resolver_name =
        format!("{}_{}", context.parent_name(), field.name_string()).to_snake_case();
    let resolver_method_name = format_ident!("{}", resolver_name);

    let typ = value_type_def_token(&field.typ, &schema)?;
    let typ: TokenStream = quote! {Result<#typ>};

    let (arg_defs, _) = args_defs_and_values(field, &schema)?;

    let q = quote! {
        pub async fn #resolver_method_name(&self, ctx: &Context<'_>, object: &#parent_name #arg_defs) -> #typ{
            unimplemented!("resolver {} is unimpemented yet", #resolver_name )
        }
    };
    Ok(q)
}
