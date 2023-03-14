use output::FunctionError;
use shopify_function::prelude::*;
use shopify_function::Result;
use graphql_client;
use serde::{Deserialize, Serialize};
generate_types!(
    query_path = "./input.graphql",
    schema_path = "./schema.graphql"
);

use crate::input::CountryCode;

#[derive(Serialize, Deserialize, Default, PartialEq)]
struct Config {}
#[shopify_function]
fn function(input: input::ResponseData) -> Result<output::FunctionResult> {
    let mut errors = Vec::new();
    let error = FunctionError {
        localized_message:
            "We do not ship to PO Boxes"
                .to_owned(),
        target: "cart".to_owned(),
    };
    // Parse the decimal (serialized as a string) into a float.
    let country_code = input.localization.country.iso_code;

    if country_code == CountryCode::CA {
        for x in input.cart.delivery_groups.iter() {
            if let Some(address) = &x.delivery_address {
                let customer_address = address.address1.clone().unwrap();
                if customer_address.contains("PO Box") {
                    errors.push(error.clone());
                }

            }
        }
    }

    Ok(output::FunctionResult { errors })
}
#[cfg(test)]
mod tests;
