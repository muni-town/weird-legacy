<script lang="ts">
  import { onMount } from 'svelte'
  import { invoke } from '@tauri-apps/api/tauri'

  type Link = { text: string; url: string }

  let links: Array<Link> = []
  let url = ''
  let text = ''
  let loading = false

  function addLink() {
    links = [...links, { text, url }]
    invoke('add_link', { url, text }).catch((e) => {
      console.error(e)
    })
    url = ''
    text = ''
  }

  function removeLink(url: string, text: string) {
    invoke('remove_link', { url, text }).catch((e) => console.error(e))
  }

  onMount(() => {
    //  TODO: load saved urls.
  })

  function generate() {
    loading = true
    invoke('generate_site', { links })
      .then((v) => {
        console.log('Site generated')
        // TODO: prompt for deploy options.
      })
      .catch((e) => {
        console.error(e)
      })
      .finally(() => (loading = false))
  }

  function toggle_preview() {
    invoke('toggle_preview_window')
  }
</script>

<main class="text-white">
  <div class="mb-8" />

  <form
    on:submit|preventDefault={addLink}
    action="/"
    class="flex flex-col justify-start items-center gap-4 text-white"
  >
    <input
      type="text"
      bind:value={text}
      class="text-black rounded p-2"
      placeholder="Enter text.."
    />
    <input
      type="url"
      bind:value={url}
      class="text-black rounded p-2"
      placeholder="Enter url.."
    />
    <button class="bg-blue-600 rounded p-2 mb-2" type="submit">
      add url
    </button>
  </form>

  {#if links.length > 0}
    <div class="flex flex-col justify-start items-center">
      {#each links as link}
        <div class="flex-row w-2/4 p-3 m-2 bg-gray-700 rounded-md">
          <div>{link.text}</div>
          <div class="text-gray-500">{link.url}</div>
        </div>
      {/each}
    </div>
  {/if}

  <div class="absolute flex flex-row top-auto bottom-0.5 left-auto right-2">
    <button
      class="bg-gray-600 hover:bg-blue-600 rounded-md p-3 mb-2 mr-4"
      on:click={toggle_preview}
    >
      Toggle Preview
    </button>
    {#if loading}
      <div
        class="m-2 w-8 h-8 animate-spin rounded-full border-transparent border-2 border-y-white"
      />
    {/if}
    <button
      class="float-right bg-blue-800 rounded-md p-3 mb-2"
      disabled={loading}
      on:click={generate}
    >
      {#if loading}
        Generating...
      {:else}
        Generate site
      {/if}
    </button>
  </div>
</main>

<style lang="postcss">
  :root {
    font-family: monospace;
    background-color: #292a32;
  }
</style>
