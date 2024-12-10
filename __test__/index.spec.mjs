import fs from 'fs'
import path from 'path'
import { createRequire } from 'module'
import { fileURLToPath } from 'url'
import test from 'ava'

const __filename = fileURLToPath(import.meta.url)
const __dirname = path.dirname(__filename)
const require = createRequire(import.meta.url)
const { setFileTimes, getFileTimes } = require('../index')
const fixtures = path.join(__dirname, 'fixtures')
const dist = path.join(__dirname, 'dist')
const name = 'IMG_1341.jpg'
const filepath = path.join(fixtures, name)
const destpath = path.join(dist, name)

fs.rmSync(dist, { recursive: true, force: true })
fs.mkdirSync(dist, { recursive: true })
fs.copyFileSync(filepath, destpath)

test('get file times', (t) => {
  const stat = fs.statSync(destpath)
  const { error, btime, mtime, atime } = getFileTimes(destpath)

  t.is(error, undefined)
  t.is(btime, String(stat.birthtimeMs).slice(0, 10))
  t.is(mtime, String(stat.mtimeMs).slice(0, 10))
  t.is(atime, String(stat.atimeMs).slice(0, 10))
})

test('set file times', (t) => {
  const now = Date.now()
  const hour1 = 1000 * 60 * 60
  const hour1ago = now - hour1
  const hour2ago = now - hour1 * 2

  const res1 = setFileTimes({
    path: destpath,
    btime: String(hour1ago),
    mtime: String(hour1ago),
  })
  const res2 = setFileTimes({
    path: destpath,
    atime: String(hour2ago),
  })

  t.is(res1.error, undefined)
  t.is(res2.error, undefined)
  if (process.platform === 'darwin') {
    t.is(String(hour1ago).slice(0, 10), String(fs.statSync(destpath).birthtimeMs).slice(0, 10))
  }
  t.is(String(hour1ago).slice(0, 10), String(fs.statSync(destpath).mtimeMs).slice(0, 10))
  t.is(String(hour2ago).slice(0, 10), String(fs.statSync(destpath).atimeMs).slice(0, 10))
})
