var assert = require('assert')
var pull = require('pull-stream')
var marky = require('marky')
var privateBox = require('private-box')

var {decrypt} = require('../')

var testData = require('./big.json')

const NUM_DECRYPTIONS = 1E4

var secretKey = Buffer.from(testData.keys[testData.keys.length - 1].secretKey, 'base64')
var publicKey = Buffer.from(testData.keys[0].publicKey, 'base64')
var cypherText = Buffer.from(testData.cypherText, 'base64')
var msg = Buffer.from(testData.msg, 'base64')

pull(
  pull.once(marky),
  pull.map((marky) => {
    marky.mark('private-box valid recipient')
    for (var i = 0; i < NUM_DECRYPTIONS; i++) {
      var result = privateBox.decrypt(cypherText, secretKey, testData.keys.length)
      assert.deepEqual(result, msg)
    }
    marky.stop('private-box valid recipient')
    return marky
  }),
  pull.map((marky) => {
    marky.mark('private-box invalid recipient')
    for (var i = 0; i < NUM_DECRYPTIONS; i++) {
      var result = privateBox.decrypt(cypherText, publicKey, testData.keys.length)
      assert.deepEqual(result, undefined)
    }
    marky.stop('private-box invalid recipient')
    return marky
  }),
  pull.map((marky) => {
    marky.mark('private-box-rs valid recipient')
    for (var i = 0; i < NUM_DECRYPTIONS; i++) {
      var result = decrypt(cypherText, secretKey)
      assert.deepEqual(result, msg)
    }
    marky.stop('private-box-rs valid recipient')
    return marky
  }),
  pull.map((marky) => {
    marky.mark('private-box-rs invalid recipient')
    for (var i = 0; i < NUM_DECRYPTIONS; i++) {
      var result = decrypt(cypherText, publicKey)
      assert.deepEqual(result, undefined)
    }
    marky.stop('private-box-rs invalid recipient')
    return marky
  }),
  pull.asyncMap((marky, cb) => {
    marky.mark('private-box-rs async valid key')
    for (var i = 0; i < NUM_DECRYPTIONS; i++) {
      var decyptionCount = 0
      var result = decrypt(cypherText, secretKey, (err, result) => {
        assert.deepEqual(result, msg)
        decyptionCount++
        if (decyptionCount === NUM_DECRYPTIONS - 1) {
          marky.stop('private-box-rs async valid key')
          cb(false, marky)
        }
      })
    }
  }),
  pull.asyncMap((marky, cb) => {
    marky.mark('private-box-rs async invalid key')
    for (var i = 0; i < NUM_DECRYPTIONS; i++) {
      var decyptionCount = 0
      var result = decrypt(cypherText, publicKey, (err, result) => {
        assert.equal(result, undefined)
        decyptionCount++
        if (decyptionCount == NUM_DECRYPTIONS - 1) {
          marky.stop('private-box-rs async invalid key')
          cb(false, marky)
        }
      })
    }
  }),
  pull.drain((marky) => {
    var entries = marky.getEntries()
    entries.map(entry => {
      console.log(entry.name, entry.duration)
    })
  })
)
