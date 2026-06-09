# Aquarium Solver

A solver for the [Aquarium puzzle](https://www.puzzle-aquarium.com/).

! Note: the problem set is compressed. Unzip it first before running.

## Problem Set Source

To obtain sample problems, use the
`https://aquarium2.vercel.app/api/get` API endpoint.

```
https://aquarium2.vercel.app/api/get?id=MDo4LDM0MCw5OTA=
```

The response will take this shape:

```json
{
  "id": "MDo2LDA3MCw1NzI=",
  "size": 6,
  "sums": {
    "cols": [2, 4, 5, 5, 4, 2],
    "rows": [4, 3, 4, 4, 2, 5]
  },
  "matrix": [
    [1, 2, 2, 2, 2, 3],
    [1, 1, 2, 4, 2, 3],
    [1, 2, 2, 4, 4, 3],
    [1, 2, 2, 2, 2, 3],
    [1, 5, 5, 5, 6, 3],
    [1, 1, 1, 5, 6, 3]
  ]
}
```

As specified in the [API endpoint](problemset/pages/api/get.ts)
