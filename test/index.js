var test = require('tape')
var {decrypt} = require('../')

var test_data = require('./simple.json')

test('decrypts ok', function (t) {
  var secretKey = Buffer.from(test_data.keys[0].secretKey, 'base64')
  var cypherText = Buffer.from(test_data.cypherText, 'base64')
  var msg = Buffer.from(test_data.msg, 'base64')

  var result = decrypt(cypherText, secretKey)

  t.deepEqual(result, msg)
  t.end()
})

test('returns undefined when key is wrong', function (t) {
  var publicKey = Buffer.from(test_data.keys[0].publicKey, 'base64')
  var cypherText = Buffer.from(test_data.cypherText, 'base64')
  var msg = Buffer.from(test_data.msg, 'base64')

  var result = decrypt(cypherText, publicKey)

  t.false(result)
  t.end()
})

test('throws a type error if args are not buffers', function (t) {
  var cypherText = Buffer.from(test_data.cypherText, 'base64')
  var secretKey = Buffer.from(test_data.keys[0].secretKey, 'base64')
  var regex = new RegExp('ArgumentTypeError')
  t.throws(function () {
    decrypt(0, secretKey)
  }, regex, 'throws a type error')
  t.throws(function () {
    decrypt(cypherText, 0)
  }, regex, 'throws a type error')
  t.end()
})

test('throws an error if incorrect number of args', function (t) {
  var cypherText = Buffer.from(test_data.cypherText, 'base64')
  var secretKey = Buffer.from(test_data.keys[0].secretKey, 'base64')

  var regex = new RegExp('ArgumentTypeError')
  t.throws(function () {
    decrypt(0, secretKey)
  }, regex)
  t.throws(function () {
    decrypt(cypherText, 0)
  }, regex)
  t.end()
})

test('throws an error if secret key is incorrect', function (t) {
  var secretKey = Buffer.from(test_data.keys[0].secretKey, 'base64')
  var cypherText = Buffer.from(test_data.cypherText, 'base64')

  var regex = new RegExp('SecretKeyError')

  t.throws(function () {
    decrypt(cypherText, secretKey.slice(0, 16))
  }, regex)

  t.end()
})

test('decrypts ok async', function (t) {
  var secretKey = Buffer.from(test_data.keys[0].secretKey, 'base64')
  var cypherText = Buffer.from(test_data.cypherText, 'base64')
  var msg = Buffer.from(test_data.msg, 'base64')

  decrypt(cypherText, secretKey, (err, result) => {
    t.error(err)
    t.deepEqual(result, msg)
    t.end()
  })
})

test('calls back with undefined when not a recipient async', function (t) {
  var publicKey = Buffer.from(test_data.keys[0].publicKey, 'base64')
  var cypherText = Buffer.from(test_data.cypherText, 'base64')

  decrypt(cypherText, publicKey, (err, result) => {
    t.error(err)
    t.deepEqual(result, undefined)
    t.end()
  })
})

test('calls back with an error if args are not buffers async', function (t) {
  var secretKey = Buffer.from(test_data.keys[0].secretKey, 'base64')
  var regex = new RegExp('ArgumentTypeError')

  decrypt(0, secretKey, function (err, result) {
    t.false(result)
    t.true(regex.test(err.message))

    t.end()
  })
})

test.skip('calls back with an error if secret key is not valid async', function (t) {
  var cypherText = Buffer.from(test_data.cypherText, 'base64')
  var secretKey = Buffer.from(test_data.keys[0].secretKey, 'base64').slice(0, 16)
  var regex = new RegExp('SecretKeyError')

  decrypt(cypherText, secretKey, function (err, result) {
    t.false(result)
    console.log(err.message)
    t.true(regex.test(err.message))

    t.end()
  })
})
