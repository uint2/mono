export const rangeInclusive = (i, j, d = 1) => {
  const x = []
  while (i <= j) {
    x.push(i)
    i += d
  }
  return x
}

export const range = (i, j, d = 1) => {
  const x = []
  while (i < j) {
    x.push(i)
    i += d
  }
  return x
}

export const PACES = rangeInclusive(200, 270, 5)

export const m = (s) => Math.floor(s / 60)
export const s = (s) => s % 60

export const timing = (s) => {
  s = Math.floor(s)
  const min = Math.floor(s / 60)
  const sec = s % 60
  if (sec < 10) {
    return `${min}'0${sec}"`
  } else {
    return `${min}'${sec}"`
  }
}

export const main = () => {
  console.log(PACES)
}
