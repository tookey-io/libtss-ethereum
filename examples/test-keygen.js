const { keygen } = require('../index')
const { writeFile } = require('fs')

console.info('Started')

let params = {
  roomId: 'default-keygen',
  participantIndex: 1,
  participantsCount: 3,
  participantsThreshold: 1,
  relayAddress: 'http://localhost:8000',
  timeoutSeconds: 120,
}

keygen(params).then((result) => {
  if (typeof result.key === 'undefined') {
    console.error('Got error: ', result.error)
  } else {
    writeFile('examples/key1.json', result.key, {}, () => {})
    console.info('Wrote key')
  }
})
