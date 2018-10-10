var {init} = require('../native')
init()

var {decrypt, decrypt_async } = require('../native')

function decrypt_extern (cypherText, secretKey, cb) {
  if (!cb) {
    return decrypt(cypherText, secretKey)
  }
  decrypt_async(cypherText, secretKey, cb)
}

module.exports = {
  decrypt: decrypt_extern
}
