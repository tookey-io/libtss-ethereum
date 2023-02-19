const { privateKeyToPublicKey, privateKeyToEthereumAddress, publicKeyToEthereumAddress, encodeMessageSignature } = require('../index')
const { readFile } = require('fs')

readFile('examples/key1.json', {}, (error, key) => {
  const privateKey = key.toString()

  console.info('Public key default: ', privateKeyToPublicKey(privateKey).result)
  console.info('Public key compressed: ', privateKeyToPublicKey(privateKey, false).result)
  console.info('Address: ', privateKeyToEthereumAddress(privateKey).result)

  const sign =
    '{"r":{"curve":"secp256k1","scalar":[211,209,245,1,82,108,205,183,184,84,39,211,177,133,214,11,98,2,121,143,22,246,105,230,105,217,157,176,98,0,61,22]},"s":{"curve":"secp256k1","scalar":[66,69,95,208,244,171,134,203,247,127,151,227,68,19,155,28,255,197,176,24,148,124,161,165,2,202,142,154,43,110,121,178]},"recid":0}'
  const messageHash = '0x3ebe9335efa84698ea0083dcccdb4e45ac8f8b0d25fb545c64ad1b664674fd0e'
  const chainId = 31337

  console.log('Sign: ', encodeMessageSignature(messageHash, chainId, sign).result)
})


const { result: address } = publicKeyToEthereumAddress("02df6a1f98f0c2ba133f928de2f7cc4b0c595afbe6b5a4cd3e56f6c7a5bcd5f19f")
console.assert(address === "0x6cc93958DA6Bf5de40A15935A53eabB6695AaF9e");

console.info('Address: ', address)