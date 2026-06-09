import axios from 'axios'
import type { NextApiRequest, NextApiResponse } from 'next'

type Problem = {
  id: string
  size: number
  sums: {
    cols: number[]
    rows: number[]
  }
  matrix: number[][]
}

export default async function handler(
  req: NextApiRequest,
  res: NextApiResponse
) {
  const id = (req.query.id as string) || 'MDo4LDM0MCw5OTA='
  return axios
    .get(`https://www.puzzle-aquarium.com/?e=${id}`)
    .then((r) => {
      const lines = (r.data as string).split('\n')
      const line = lines.find((v) => v.includes('var task'))
      if (!line) return ''
      const x = line.split('var task')[1]
      const a = x.indexOf("'")
      if (a === -1) return ''
      const b = x.indexOf("'", a + 1)
      if (b === -1) return ''
      return x.slice(a + 1, b)
    })
    .then((raw) => {
      if (!raw) return res.json({ id: 'INVALID', sums: [], frame: [], size: 0 })
      const [rawSums, rawGroups] = raw.split(';')
      const sums = rawSums.split('_').map((v) => parseInt(v))
      const size = sums.length / 2
      const [colSums, rowSums] = [
        sums.slice(0, size),
        sums.slice(size, size * 2),
      ]
      const groups = rawGroups.split(',').map((v) => parseInt(v))
      const matrix = [] as number[][]
      for (let i = 0; i < size; i++)
        matrix.push(groups.slice(i * size, (i + 1) * size))
      return res.json({
        id,
        size,
        sums: { cols: colSums, rows: rowSums },
        matrix,
        play: `https://www.puzzle-aquarium.com/?e=${id}`,
      } as Problem)
    })
}
