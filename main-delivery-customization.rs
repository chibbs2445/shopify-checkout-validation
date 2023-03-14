use shopify_function::prelude::*;
use shopify_function::Result;
use serde::{Serialize};
// Use the shopify_function crate to generate structs for the function input and output
generate_types!(
    query_path = "./input.graphql",
    schema_path = "./schema.graphql"
);

// Use the shopify_function crate to declare your function entrypoint
#[shopify_function]
fn function(input: input::ResponseData) -> Result<output::FunctionResult> {

    let no_removal = output::FunctionResult { operations: vec![] };

let targets: Vec<_> = input.cart.lines
.iter()
.filter_map(|line| match &line.merchandise {
    input::InputCartLinesMerchandise::ProductVariant(variant) => Some(variant),
    input::InputCartLinesMerchandise::CustomProduct => None,
})
.filter(|variant| variant.product.has_preorder)
.collect();

if targets.is_empty() {
    return Ok(no_removal)
}

let to_remove = input.cart.delivery_groups
    .iter()
    .flat_map(|options| {
        options.delivery_options
            .iter()
            .filter(|opt| {
                opt.title.clone().unwrap() == "Economy"
            })
    })
    .map(|option| output::HideOperation {
        delivery_option_handle: option.handle.to_string(),
    })
    .map(|hide| output::Operation {
        rename: None,
        hide: Some(hide),
        move_: None
    })
    .collect();

    Ok(output::FunctionResult { operations: to_remove })
}

#[cfg(test)]
mod tests;
