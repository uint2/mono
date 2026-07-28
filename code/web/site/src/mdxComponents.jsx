export const mdxComponents = {
  Image: ({ src, width, height }) => (
    <div style="display:flex;justify-content:center;">
      {width || height ? (
        <img alt="image" src={src} width={width} height={height} />
      ) : (
        <img alt="image" src={src} style="width=100%" />
      )}
    </div>
  ),
}
