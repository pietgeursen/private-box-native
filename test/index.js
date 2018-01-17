var test = require('tape')
var test_data = require('./simple.json')
var {decrypt} = require('../')

test('decrypts ok', function(t) {

  var secretKey = Buffer.from(test_data.keys[0].secretKey, 'base64')
  var cypherText = Buffer.from(test_data.cypherText, 'base64')
  var msg = Buffer.from(test_data.msg, 'base64')

  result = decrypt(cypherText, secretKey); 

  t.deepEqual(result, msg)
  t.end()
})
