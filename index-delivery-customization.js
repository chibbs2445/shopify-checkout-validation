// @ts-check
// Use JSDoc annotations for type safety
/**
* @typedef {import("../generated/api").InputQuery} InputQuery
* @typedef {import("../generated/api").FunctionResult} FunctionResult
* @typedef {import("../generated/api").Operation} Operation
*/

const NO_CHANGES = {
  operations: [],
};

// The @shopify/shopify_function package will use the default export as your function entrypoint
export default 
/**
* @param {InputQuery} input
* @returns {FunctionResult}
*/
  (input) => {

    const targets = input.cart.lines
    .filter(line => line.merchandise.__typename == "ProductVariant")
    .filter(line => { 
      const variant = /** @type {ProductVariant} */ (line.merchandise);
      return variant.product.hasPreorder
    })

    if (!targets.length) {
      // You can use STDERR for debug logs in your function
      return NO_CHANGES;
    }

    let toHide = input.cart.deliveryGroups
      // Collect the delivery options from these groups
      .flatMap(group => group.deliveryOptions)
      .filter(option => option.cost.amount == 0)
      // Construct a rename operation for each, adding the message to the option title
      .map(option => /** @type {Operation} */({
        hide: {
          deliveryOptionHandle: option.handle,
        }
      }));
      console.log('to hide', toHide)

    // The @shopify/shopify_function package applies JSON.stringify() to your function result
    // and writes it to STDOUT
    return {
      operations: toHide
    };
  };
