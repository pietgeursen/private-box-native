var assert = require('assert');
var pull = require('pull-stream')
var marky = require('marky')
var privateBox = require('private-box')

var {decrypt} = require('../')

var test_data = require('./simple.json')

const NUM_DECRYPTIONS = 1E4

var secretKey = Buffer.from(test_data.keys[0].secretKey, 'base64')
var publicKey = Buffer.from(test_data.keys[0].publicKey, 'base64')
var cypherText = Buffer.from(test_data.cypherText, 'base64')
var msg = Buffer.from(test_data.msg, 'base64')

pull(
  pull.once(marky),
  pull.map((marky) => {
    marky.mark("private-box valid recipient")
    for (var i = 0; i < NUM_DECRYPTIONS; i++) {
      result = privateBox.decrypt(cypherText, secretKey); 
      assert.deepEqual(result, msg)
    }
    marky.stop("private-box valid recipient")
    return marky
  }),
  pull.map((marky) => {
    marky.mark("private-box invalid recipient")
    for (var i = 0; i < NUM_DECRYPTIONS; i++) {
      result = privateBox.decrypt(cypherText, publicKey); 
      assert.deepEqual(result, undefined)
    }
    marky.stop("private-box invalid recipient")
    return marky
  }),
  pull.map((marky) => {
    marky.mark("private-box-rs valid recipient")
    for (var i = 0; i < NUM_DECRYPTIONS; i++) {
      result = decrypt(cypherText, secretKey); 
      assert.deepEqual(result, msg)
    }
    marky.stop("private-box-rs valid recipient")
    return marky
  }),
  pull.map((marky) => {
    marky.mark("private-box-rs invalid recipient")
    for (var i = 0; i < NUM_DECRYPTIONS; i++) {
      result = decrypt(cypherText, publicKey); 
      assert.deepEqual(result, undefined)
    }
    marky.stop("private-box-rs invalid recipient")
    return marky
  }),
  pull.asyncMap((marky, cb) => {
    marky.mark("private-box-rs async valid key")
    for (i = 0; i < NUM_DECRYPTIONS; i++) {
      var decyptionCount = 0
      result = decrypt(cypherText, secretKey, (err, result)=> {
        assert.deepEqual(result, msg)
        decyptionCount ++
        if (decyptionCount == NUM_DECRYPTIONS - 1) {
          marky.stop("private-box-rs async valid key")
          cb(false, marky)
        }
      }); 
    }
  }),
  pull.asyncMap((marky, cb) => {
    marky.mark("private-box-rs async invalid key")
    for (i = 0; i < NUM_DECRYPTIONS; i++) {
      var decyptionCount = 0
      result = decrypt(cypherText, publicKey, (err, result)=> {
        assert.equal(result, undefined)
        decyptionCount ++
        if (decyptionCount == NUM_DECRYPTIONS - 1) {
          marky.stop("private-box-rs async invalid key")
          cb(false, marky)
        }
      }); 
    }
  }),
  pull.drain((marky) => {
    var entries = marky.getEntries()
    entries.map(entry => {
      console.log(entry.name, entry.duration);
    })
  })
)

