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

const smartGridTest = () => {

  const options = {
    method: 'POST',
    headers: {
      accept: 'application/json',
      'x-guid': 'Wf3-OhFFSz3SSkxMK0-svQ',
      'x-api-key': '0Vk3vXr8TALtTUtCbck6Qwtt',
      'content-type': 'application/json'
    },
    body: JSON.stringify({
      email: 'chibbs+25@rothys.com',
      first_name: 'Cecily',
      last_name: 'Hibbs',
      opted_in: true
    })
  };
  
  fetch('https://loyalty.yotpo.com/api/v2/customers', options)
    .then(response => response.json())
    .then(response => console.log(response))
    .catch(err => console.error(err));
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
        <Button title="Enroll" type="primary" onPress={smartGridTest}>
        </Button>
      </Screen>
    </Navigator>
  );
}

render('pos.home.tile.render', () => <SmartGridTile />);
render('pos.home.modal.render', () => <SmartGridModal />);
