<script>
  import Icon from 'components/Icon.svelte'
</script>

<div class="flex-col">
  <h1 class="text-2xl text-center mt-4 flex items-center justify-center gap-3">
    Publish to GitHub <Icon name="github" size={25} />
  </h1>

  <div class="m-4">
    <slot />
  </div>

  <a class="btn mx-auto absolute bottom-2 left-4" href="/">Back</a>
</div>
