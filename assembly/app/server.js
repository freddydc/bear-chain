import express from 'express'
import path from 'path'

const port = 3000
const app = express()

const dirname = path.resolve()

app.use('/', express.static(path.join(dirname, 'app')))

app.listen(port, () => {
  console.log(`Server at http://localhost:${port}`)
})
