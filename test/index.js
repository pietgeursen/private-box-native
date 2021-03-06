var test = require('tape')
var {decrypt} = require('../')

var test_data = require('./simple.json')

test('decrypts ok', function(t) {
  var secretKey = Buffer.from(test_data.keys[0].secretKey, 'base64')
  var cypherText = Buffer.from(test_data.cypherText, 'base64')
  var msg = Buffer.from(test_data.msg, 'base64')

  result = decrypt(cypherText, secretKey); 

  t.deepEqual(result, msg)
  t.end()
})

test('returns undefined when key is wrong', function(t) {

  var publicKey = Buffer.from(test_data.keys[0].publicKey, 'base64')
  var cypherText = Buffer.from(test_data.cypherText, 'base64')
  var msg = Buffer.from(test_data.msg, 'base64')

  result = decrypt(cypherText, publicKey); 

  t.false(result)
  t.end()
})

test('decrypts ok async', function(t) {

  t.plan(1)
  var secretKey = Buffer.from(test_data.keys[0].secretKey, 'base64')
  var cypherText = Buffer.from(test_data.cypherText, 'base64')
  var msg = Buffer.from(test_data.msg, 'base64')

  decrypt(cypherText, secretKey, (err, result) => {
    t.deepEqual(result, msg)
    t.end()
  }); 

})
