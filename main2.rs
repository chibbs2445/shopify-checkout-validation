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

    // let delivery_options = input.cart.delivery_groups.delivery_options;
    
    // for x in delivery_options.iter() {
    //     eprintln!("{:?}", x.handle);
    //     eprintln!("{:?}", x.title);

    // }
    // let hide_payment_method = input.payment_methods
    // .iter()
    // .find(|&method| method.name.contains(&config.payment_method_name.to_string()))
    // .map(|method| output::HideOperation {
    //     payment_method_id: method.id.to_string()
    // });
    let hide_shipping_method = ""
        for group in input.cart.delivery_groups.iter() {
            hide_shipping_method = group.delivery_options
            .iter()
            .find(|&method| method.title.clone().unwrap() == "Standard")
            .map(|method| output::HideOperation {
                delivery_option_handle: method.handle
            });
        }


        // for group in input.cart.delivery_groups.iter() {
        //     for option in group.delivery_options.iter() {
        //         let title = option.title.clone().unwrap();
        //         let handle = option.handle.clone();
        //         if title == "Standard" {
        //             eprintln!("{:?}", handle);
        //         }
        //         // eprintln!("{:?}", option);
        //         // eprintln!("{:?}", option.handle);
        //         // eprintln!("{:?}", option.title.clone().unwrap());
                
        //     }

            // if let Some(address) = &x.delivery_address {
            //     let customer_address = address.address1.clone().unwrap();
            //     if customer_address.contains("PO Box") {
            //         errors.push(error.clone());
            //       }

            // }
        // }
    // }

    Ok(output::FunctionResult { operations: vec![
        output::Operation {
            hide: hide_shipping_method,
            move_: None,
            rename: None
        }
    ] })
}
#[cfg(test)]
mod tests;
