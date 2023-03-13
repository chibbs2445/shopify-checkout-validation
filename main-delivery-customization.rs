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

// Map { iter: Iter([InputCartDeliveryGroups { delivery_address: Some(InputCartDeliveryGroupsDeliveryAddress { province_code: Some("IL") }), delivery_options: [InputCartDeliveryGroupsDeliveryOptions { handle: "b1c2e7620124ab4fbc9323bc64c4e92c-9e912b3dae9b0e04b6f1facf08aa7cb2", title: Some("Economy") }, InputCartDeliveryGroupsDeliveryOptions { handle: "b1c2e7620124ab4fbc9323bc64c4e92c-ee768830e386b87e4f230f4292c237a3", title: Some("Standard") }] }]) }
