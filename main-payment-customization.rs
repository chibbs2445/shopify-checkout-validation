use shopify_function::prelude::*;
use shopify_function::Result;
use serde::{Serialize};
// Use the shopify_function crate to generate structs for the function input and output
generate_types!(
    query_path = "./input.graphql",
    schema_path = "./schema.graphql"
);

// Hide afterpay on certain products (gift card)

// Use the shopify_function crate to declare your function entrypoint
#[shopify_function]
fn function(input: input::ResponseData) -> Result<output::FunctionResult> {
    let no_changes = output::FunctionResult { operations: vec![] };

    let targets: Vec<_> = input.cart.lines
    .iter()
    .filter_map(|line| match &line.merchandise {
        input::InputCartLinesMerchandise::ProductVariant(variant) => Some(variant),
        input::InputCartLinesMerchandise::CustomProduct => None,
    })
    .filter(|variant| {
       // TO DO: Figure out how to do this without hardcoding it - maybe have something through the UI? 
       // Or we could also use product type
        variant.product.id == "gid://shopify/Product/8171596448030"
        })
    .collect();


    if targets.is_empty() {
        eprintln!("All products in can show afterpay.");
        return Ok(no_changes);
    }

    // Find the payment method to hide, and create a hide output operation from it
    // (this will be None if not found)
    let hide_payment_method = input.payment_methods
        .iter()
        .find(|&method| method.name.contains(&"Cash on Delivery".to_string()))
        .map(|method| output::HideOperation {
            payment_method_id: method.id.to_string()
        });

    // The shopify_function crate serializes your function result and writes it to STDOUT
    Ok(output::FunctionResult { operations: vec![
        output::Operation {
            hide: hide_payment_method,
            move_: None,
            rename: None
        }
    ] })
}
#[cfg(test)]
mod tests;
