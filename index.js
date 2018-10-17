'use strict'

var binding = require('./build/Release/binding.node')

module.exports.decrypt = function decrypt (cypher, secret, cb) {
  var cypherAbuf = new ArrayBuffer(cypher.length)
  var secretAbuf = new ArrayBuffer(secret.length)

  cypher.copy(new Uint8Array(cypherAbuf))
  secret.copy(new Uint8Array(secretAbuf))

  if (cb) {
    binding.decryptAsync(cypherAbuf, secretAbuf, cb)
  } else {
    return binding.decrypt(cypherAbuf, secretAbuf)
  }
}
