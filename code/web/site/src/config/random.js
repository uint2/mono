import { createHash } from "node:crypto"

/**
 * Generates a random hex string of length `len`.
 * @param {number} len
 */
export const random = (len) => {
  const hash = createHash("sha256")
  const now = () => performance.now().toString()
  let s = ""
  while (s.length < len) s += hash.update(now()).digest("hex")
  return s.slice(0, len)
}
