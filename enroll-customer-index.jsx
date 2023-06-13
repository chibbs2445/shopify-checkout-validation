import React from 'react';
import {Tile, Text, Screen, Navigator, render, useExtensionApi, FormattedTextField, Button, useCartSubscription} from '@shopify/retail-ui-extensions-react';

const SmartGridTile = () => {
  const api = useExtensionApi();


  return (
    <Tile
      title="My app"
      subtitle="SmartGrid Extension"
      onPress={() => {
        api.smartGrid.presentModal();
      }}
      enabled
    />
  );
};

const smartGridTest = (customerName, customerEmail) => {
  const data = {
    name: customerName,
    email: customerEmail
  };

  fetch('https://webhook.site/9ee8dab6-2ed1-4571-9066-eb04c6391c56', {
  method: 'POST',
  headers: {
    'Content-Type': 'application/json' // Set the appropriate content-type if sending JSON data
  },
  body: JSON.stringify(data) // Replace with the data you want to send in the request body
})
  .then(response => {
    if (response.ok) {
      console.log('POST request succeeded');
    } else {
      console.error('POST request failed');
    }
  })
  .catch(error => {
    console.error('Error:', error);
  });
}

const SmartGridModal = () => {
  const cart = useCartSubscription();
  let customerEmail = cart.customer?.email
  let customerName = `${cart.customer?.firstName} ${cart.customer?.lastName}`
  const api = useExtensionApi();

  return (
    <Navigator>
      <Screen name="Enroll Customer" title="Enroll Customer">
        <Text>Enroll Customer: {customerEmail}</Text>
        <Button title="Enroll" type="primary" onPress={() => {
          smartGridTest(customerName, customerEmail)
          api.toast.show('Customer enrolled!')
        }
        }></Button>
      </Screen>
    </Navigator>
  );
}

render('pos.home.tile.render', () => <SmartGridTile />);
render('pos.home.modal.render', () => <SmartGridModal />);



