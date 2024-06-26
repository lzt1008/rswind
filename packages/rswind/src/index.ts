import process from 'node:process'
import { MUSL, familySync } from 'detect-libc'

export type * from './types'
export * from './binding.d'

type NativeBinding = typeof import('./binding')

function requireNative(): NativeBinding {
  const parts: string[] = [process.platform, process.arch]
  if (process.platform === 'linux') {
    if (familySync() === MUSL) {
      parts.push('musl')
    }
    else if (process.arch === 'arm') {
      parts.push('gnueabihf')
    }
    else {
      parts.push('gnu')
    }
  }
  else if (process.platform === 'win32') {
    parts.push('msvc')
  }

  try {
    // eslint-disable-next-line ts/no-require-imports -- .node must use require
    return require(`@rswind/binding-${parts.join('-')}`)
  }
  catch (err) {
    const binding = `./rswind.${parts.join('-')}.node`
    // explicitly extract `binding` to avoid glob import
    // eslint-disable-next-line ts/no-require-imports -- .node must use require
    return require(binding)
  }
}

const binding = requireNative()

export default binding

export const createGenerator = binding.createGenerator
export const Generator = binding.Generator
