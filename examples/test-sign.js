const { sign } = require('../index')
const { readFile } = require('fs')

readFile('examples/key1.json', {}, (error, key) => {
  console.info('Started')
  let params = {
    roomId: 'default-signing',
    participantsIndexes: [1, 2],
    data: '0xbd621a5652a421f0b853d2a56609bfd26ae965709070708a34f7607f1ce97a60',
    key: key.toString(),
    relayAddress: 'http://localhost:8000',
    timeoutSeconds: 120,
  }

  sign(params).then((result) => {
    if (typeof result.result === 'undefined') {
      console.error('Got error: ', result.error)
    } else {
      console.info('Result: ', result.result)
    }
  })
})
