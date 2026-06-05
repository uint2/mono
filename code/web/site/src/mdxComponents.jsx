export const mdxComponents = {
  Sep: () => <span style="color:var(--color-ac1)">|</span>,
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
