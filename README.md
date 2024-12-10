# filetime

> Based on [File::set_times](https://doc.rust-lang.org/stable/std/fs/struct.File.html#method.set_times), written in Rust.

Rust addon to change the creation time (btime), modified time (mtime), and access time (atime) of files, directories, and symbolic links on Windows, macOS, and Linux.

[![NPM version](https://img.shields.io/npm/v/filetime.svg)](https://npmjs.org/package/filetime)
[![NPM Downloads](https://img.shields.io/npm/dm/filetime.svg?style=flat)](https://npmjs.org/package/filetime)

## Usage

```js
const { setFileTimes, getFileTimes } = require('filetime')

const now = Date.now()

const { error } = setFileTimes({
  path: '/xxx/filename',
  btime: String(now),
  mtime: String(now),
})

if (error) {
  console.log('✅ set file times failed:', error)
} else {
  console.log('❌ set file times success.')
}
```

## API <sub><sup>(Define)</sup></sub>

```ts
export interface SetFileTimesOptions {
  path: string
  btime?: string
  mtime?: string
  atime?: string
}

export interface SetFileTimesResult {
  error?: string
}

export declare function setFileTimes(options: SetFileTimesOptions): SetFileTimesResult
```
