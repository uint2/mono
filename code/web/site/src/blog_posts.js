import { defineCollection, getCollection, z } from "astro:content"
import { glob } from "astro/loaders"
import { parse } from "path"

const firstUpper = new RegExp("^[A-Z]")
const firstUpperMsg = {
  message: "The first letter of the description must be capitalized.",
}
const DEV = import.meta.env.DEV

// get value of comparison
const cmp = (a, b) => {
  // sort by most recent first.
  if (a.data.pubDate != b.data.pubDate)
    return b.data.pubDate.localeCompare(a.data.pubDate)
  // sort by title in alphabetical order.
  return a.data.title.localeCompare(b.data.title)
}

const blogPosts = {}

blogPosts.defineCollection = defineCollection({
  loader: glob({
    pattern: "*.mdx",
    base: "./blog",
  }),
  // Type-check frontmatter using a schema
  schema: z.object({
    title: z.string().regex(firstUpper, firstUpperMsg),
    description: z
      .string()
      .endsWith(".")
      .regex(firstUpper, firstUpperMsg)
      .optional(),
    tags: z.array(z.string()).optional(),
    // Parses with YYYY-MM-DD format.
    // https://v3.zod.dev/?id=dates
    pubDate: z.string().date(),
    draft: z.boolean().optional(),
  }),
})

/** @typedef {import('astro:content').Project} Project */

/**
 * Returns an array of a type that is not accessible even from JSDocs.
 * https://docs.astro.build/en/reference/modules/astro-content/#collectionentry
 * @returns {Promise<CollectionEntry[]>}
 */
blogPosts.getCollection = (sorted = false) =>
  // The string passed into `getCollection` should match the key used in
  // `content.config.js`.
  getCollection("blog").then((blogPosts) => {
    if (sorted) {
      blogPosts.sort(cmp)
    }
    for (let i = 0; i < blogPosts.length; i++) {
      blogPosts[i].data.slug = parse(blogPosts[i].filePath).name
    }
    return DEV ? blogPosts : blogPosts.filter((e) => !e.data?.draft)
  })

// https://docs.astro.build/en/reference/routing-reference/#getstaticpaths
blogPosts.getStaticPaths = () =>
  blogPosts.getCollection().then((blogPosts) =>
    blogPosts.map((post) => {
      return {
        // The one and only key-value pair in `params` should have the same key
        // as the "[blog].astro" file located in `//src/pages`.
        params: { blog: post.data.slug },
        props: post,
      }
    }),
  )

export default blogPosts
