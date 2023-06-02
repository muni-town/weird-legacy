<script>
  import Icon from 'components/Icon.svelte'
</script>

<div class="flex-col">
  <h1 class="text-3xl text-center my-4 flex items-center justify-center gap-3">
    Publish to GitHub <Icon name="github" size={25} />
  </h1>

  <div>
    <a href="/" class="text-white transition hover:scale-125 hover:text-red-400 absolute top-5 right-5">
      <Icon name="x" size={20} />
    </a>
  </div>

  <div class="m-4">
    <slot />
  </div>
</div>
