// @ts-check
// Use JSDoc annotations for type safety
/**
* @typedef {import("../generated/api").InputQuery} InputQuery
* @typedef {import("../generated/api").FunctionResult} FunctionResult
* @typedef {import("../generated/api").HideOperation} HideOperation
*/
/**
* @type {FunctionResult}
*/
const NO_CHANGES = {
  operations: [],
};
// The @shopify/shopify_function package will use the default export as your function entrypoint
export default /**
    * @param {InputQuery} input
    * @returns {FunctionResult}
    */
  (input) => {

const targets = input.cart.lines
    .filter(line => line.merchandise.__typename == "ProductVariant")
    .filter(line => { 
      const variant = /** @type {ProductVariant} */ (line.merchandise);
      return variant.product.id == "gid://shopify/Product/8171596185886"
    })

    if (!targets.length) {
      // You can use STDERR for debug logs in your function
      return NO_CHANGES;
    }

    // Find the payment method to hide
    const hidePaymentMethod = input.paymentMethods
      .find(method => method.name.includes("Cash on Delivery"));

    if (!hidePaymentMethod) {
      return NO_CHANGES;
    }

    // The @shopify/shopify_function package applies JSON.stringify() to your function result
    // and writes it to STDOUT
    return {
      operations: [{
        hide: {
          paymentMethodId: hidePaymentMethod.id
        }
      }]
    };
  };
