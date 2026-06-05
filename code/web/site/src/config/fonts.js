/**
 * @typedef {Object} FontProps
 * @property {string} f font
 * @property {"normal" | number} w weight
 * @property {"normal" | "italic"} s style
 */

export const sans = {
  /** @type {FontProps[]} */
  source3: [
    { f: "SourceSans3-Regular.woff2", w: "normal", s: "normal" },
    { f: "SourceSans3-Italic.woff2", w: "normal", s: "italic" },
    { f: "SourceSans3-SemiBold.woff2", w: 600, s: "normal" },
    { f: "SourceSans3-SemiBoldItalic.woff2", w: 600, s: "italic" },
  ],
  /** @type {FontProps[]} */
  open: [
    { f: "OpenSans-Regular.woff2", w: "normal", s: "normal" },
    { f: "OpenSans-Italic.woff2", w: "normal", s: "italic" },
    { f: "OpenSans-SemiBold.woff2", w: 600, s: "normal" },
    { f: "OpenSans-SemiBoldItalic.woff2", w: 600, s: "italic" },
  ],
  /** @type {FontProps[]} */
  noto: [
    { f: "NotoSans-Regular.woff2", w: "normal", s: "normal" },
    { f: "NotoSans-Italic.woff2", w: "normal", s: "italic" },
    { f: "NotoSans-SemiBold.woff2", w: 600, s: "normal" },
    { f: "NotoSans-SemiBoldItalic.woff2", w: 600, s: "italic" },
  ],
  /** @type {FontProps[]} */
  fira: [
    { f: "FiraSans-Book.woff2", w: "normal", s: "normal" },
    { f: "FiraSans-Italic.woff2", w: "normal", s: "italic" },
    { f: "FiraSans-SemiBold.woff2", w: 600, s: "normal" },
    { f: "FiraSans-SemiBoldItalic.woff2", w: 600, s: "italic" },
  ],
  /** @type {FontProps[]} */
  archivo: [
    { f: "Archivo-Regular.woff2", w: "normal", s: "normal" },
    { f: "Archivo-Italic.woff2", w: "normal", s: "italic" },
    { f: "Archivo-SemiBold.woff2", w: 600, s: "normal" },
    { f: "Archivo-SemiBoldItalic.woff2", w: 600, s: "italic" },
  ],
}

export const mono = {
  /** @type {FontProps[]} */
  noto: [{ f: "NotoSansMono-Regular.woff2", w: "normal", s: "normal" }],
  /** @type {FontProps[]} */
  fira: [
    { f: "FiraMono-Regular.woff2", w: "normal", s: "normal" },
    { f: "FiraMono-Medium.woff2", w: 500, s: "normal" },
  ],
  /** @type {FontProps[]} */
  commit: [
    { f: "CommitMono-Regular.woff2", w: "normal", s: "normal" },
    { f: "CommitMono-Italic.woff2", w: "normal", s: "italic" },
    { f: "CommitMono-SemiBold.woff2", w: 600, s: "normal" },
    { f: "CommitMono-SemiBoldItalic.woff2", w: 600, s: "italic" },
  ],
}
