var {decrypt, encrypt} = require('private-box')
var sodium = require('chloride/build/Release/sodium')

var keypair = sodium.crypto_box_keypair

var NUM_RECPIENTS = 255

var msg = Buffer.from('PIET123')

var keys = []

for (var i = 0; i < NUM_RECPIENTS; i++) {
  keys.push(keypair())
}

var pubKeys = keys.map(key => key.publicKey)

var cypherText = encrypt(msg, pubKeys, NUM_RECPIENTS)

var result = {
  keys: keys.map(key => {
    return {
      publicKey: key.publicKey.toString('base64'),
      secretKey: key.secretKey.toString('base64')
    }
  }),
  msg: msg.toString('base64'),
  cypherText: cypherText.toString('base64')
}

console.log(JSON.stringify(result))
