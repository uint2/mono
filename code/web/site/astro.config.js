import { defineConfig, fontProviders } from "astro/config"
import { readFileSync } from "node:fs"
import { parse } from "parse5"
import { join } from "node:path"

import { sans, mono } from "./src/config/fonts"
import { isTestedUrl } from "./src/config/urls"
import { random } from "./src/config/random"
import githubLight from "./src/styles/github-light.json"

import mdx from "@astrojs/mdx"
import solid from "@astrojs/solid-js"

const SITE = "https://nguyenvukhang.com/"

// has to match our theme's "bg-fg4".
githubLight.colors["editor.background"] = "var(--color-fg4)"

/**
 * This is purely for record-keeping.
 * @returns {import("@astrojs/markdown-remark").RemarkPlugin}
 */
function getLinks() {
  /** @param {import("@types/mdast").Root} tree */
  return function (_tree, _file) {
    // if (file.data?.astro?.frontmatter?.draft) {
    //   return
    // }
    // const stack = [tree]
    // while (stack.length != 0) {
    //   let node = stack.pop()
    //   if (node.type === "link") {
    //     console.log(node.url)
    //   }
    //   if (Array.isArray(node.children)) {
    //     node.children.forEach((node) => stack.push(node))
    //   }
    // }
  }
}

/**
 * Obtains all the href values in a html document.
 * @param  doc  The output of the parse5 parser.
 */
function getHrefs(doc) {
  const stack = [doc]
  /** @type {string[]} */
  const allHrefs = []
  while (stack.length != 0) {
    const node = stack.pop()
    const hrefs = (node.attrs || [])
      .filter((v) => v.name === "href")
      .map((v) => v.value)
    if (node.nodeName === "a" && hrefs.length > 0) {
      hrefs.forEach((v) => allHrefs.push(v))
    }
    if (Array.isArray(node.childNodes)) {
      node.childNodes.forEach((v) => stack.push(v))
    }
  }
  return allHrefs
}

/**
 * Obtains all the href values in a html document.
 * @param  doc  The output of the parse5 parser.
 */
function getHeaders(doc) {
  const stack = [doc]
  /** @type {string[]} */
  const allHeaders = []
  while (stack.length != 0) {
    const node = stack.pop()
    switch (node.tagName) {
      case "h1":
      case "h2":
      case "h3":
      case "h4":
      case "h5":
      case "h6":
        allHeaders.push(node)
        break
    }
    if (Array.isArray(node.childNodes)) {
      node.childNodes.forEach((v) => stack.push(v))
    }
  }
  return allHeaders
}

/**
 * Key is a string, representing the file URL. Value is the output of the parse5
 * parser.
 */
const urlToHtmlMap = {}

/** @returns {import("astro").AstroIntegration} */
function parseOutputs() {
  return {
    name: "parse",
    hooks: {
      "astro:build:done": ({ assets }) =>
        assets.forEach((urls, _) =>
          urls.forEach((url) => {
            const html = readFileSync(url.pathname, "utf-8")
            urlToHtmlMap[url] = parse(html, { scriptingEnabled: false })
          }),
        ),
    },
  }
}

/** @returns {import("astro").AstroIntegration} */
function urlChecks() {
  return {
    name: "urls",
    hooks: {
      "astro:build:done": ({ logger, assets }) => {
        const assetUrls = []
        assets.forEach((urls) => urls.forEach((url) => assetUrls.push(url)))
        /** @type Set<string> */
        const urlSet = new Set(
          assetUrls.flatMap((url) => getHrefs(urlToHtmlMap[url])),
        )
        let urls = Array.from(urlSet).sort()
        const httpUrls = urls.filter((url) => url.startsWith("http:"))
        if (httpUrls.length > 0) {
          httpUrls.forEach((url) => logger.warn(`http link found: ${url}`))
        }
        const externalUrls = urls
          .filter((url) => !url.startsWith("#"))
          .filter((url) => !url.startsWith("/"))
          .filter((url) => !url.startsWith("mailto"))
        logger.info(`${externalUrls.length} external url(s) found`)
        const untestedExternalUrls = externalUrls.filter((v) => !isTestedUrl(v))
        if (untestedExternalUrls.length > 0) {
          const n = untestedExternalUrls.length
          untestedExternalUrls.forEach((url) =>
            logger.warn(`Untested external link: ${url}`),
          )
          logger.warn(`Found ${n} untested external link(s)`)
        } else {
          logger.info("All external urls have been tested!")
        }
      },
    },
  }
}

/**
 * Disable all H3 headers and beyond.
 * @returns {import("astro").AstroIntegration}
 */
function noSmallHeaders() {
  return {
    name: "no-small-headers",
    hooks: {
      "astro:build:done": ({ assets }) => {
        assets.forEach((urls) =>
          urls.forEach((url) => {
            let doc = parse("")
            doc = urlToHtmlMap[url]
            getHeaders(doc).forEach((node) => {
              if (
                node.tagName === "h3" ||
                node.tagName === "h4" ||
                node.tagName === "h5" ||
                node.tagName === "h6"
              ) {
                delete node.parentNode
                console.error(node)
                throw new Error("Only h1 and h2 allowed.")
              }
            })
          }),
        )
      },
    },
  }
}

/** @param {{ variants: FontProps[] }} */
const font = ({ name, cssVariable, fallback, variants }) => ({
  provider: fontProviders.local(),
  name,
  cssVariable,
  fallbacks: [fallback],
  optimizedFallbacks: false,
  options: {
    variants: variants.map(({ f, w, s }) => ({
      src: [join("./src/assets/fonts", f)],
      weight: w,
      style: s,
    })),
  },
})

// https://astro.build/config
export default defineConfig({
  site: SITE,
  server: { port: 3000 },
  fonts: [
    font({
      name: "f" + random(39),
      cssVariable: "--font-sans",
      fallback: "sans-serif",
      variants: sans.noto,
    }),
    font({
      name: "f" + random(39),
      cssVariable: "--font-mono",
      fallback: "monospace",
      variants: mono.noto,
    }),
  ],
  markdown: {
    remarkPlugins: [getLinks],
    shikiConfig: { theme: githubLight, wrap: false },
  },
  integrations: [solid(), mdx(), parseOutputs(), urlChecks(), noSmallHeaders()],
})
