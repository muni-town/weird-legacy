<script lang="ts">
  // Import the inner SVG from the icons dir.
  // Some icons are taken from https://icons.getbootstrap.com/
  const icons = import.meta.glob('./icons/*.path', { as: 'raw', eager: true })

  export let name
  export let size: number = 16

  let path: string
  $: {
    // Load the icon with the given name
    let key = `./icons/${name}.path`
    path = icons[key] as string
  }
</script>

<svg
  xmlns="http://www.w3.org/2000/svg"
  width={size}
  height={size}
  fill="currentColor"
  viewBox="0 0 16 16"
  {...$$restProps}
>
  {@html path}
</svg>
