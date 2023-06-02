<script lang="ts">
  import { onMount } from 'svelte'
  import { invoke } from '@tauri-apps/api/tauri'
  import { goto } from '$app/navigation'
  import { user } from '../store'
  import Icon from '../components/Icon.svelte'
  import type { User } from '../bindings/User'
  import type { Link } from '../bindings/Link'

  let links: Array<Link> = []
  let id = 1
  let url = ''
  let text = ''
  let loading = false

  function addLink() {
    let link: Link = { id, text, url }
    links = [...links, link]
    invoke('add_link', { link }).catch((e) => {
      console.error(e)
    })
    id += 1
    url = ''
    text = ''
  }

  function removeLink(id: number) {
    invoke('remove_link', { id }).catch((e) => console.error(e))
    links = links.filter((v) => v.id !== id)
  }

  onMount(() => {
    invoke('get_user')
      .then((u: User) => user.set(u))
      .catch((e) => console.error(e))
    invoke('get_links')
      .then((l: Array<Link>) => {
        links = l
        if (l.length > 0) {
          id = l[l.length - 1].id + 1
        }
      })
      .catch((e) => console.error(e))
  })

  function generate() {
    loading = true
    invoke('generate_site')
      .then(() => {
        console.log('Site generated')
        goto('/export')
      })
      .catch((e) => {
        console.error(e)
      })
      .finally(() => (loading = false))
  }

  function togglePreview() {
    invoke('toggle_preview_window')
  }
</script>

<main>
  <header class="h-10 w-screen align-middle">
    <a
      href="/user"
      class="hover:scale-105 active:scale-100"
    >
      <div
        class="ml-auto p-2 text-center text-gray-400 hover:text-white"
      >
        <svg
          xmlns="http://www.w3.org/2000/svg"
          fill="none"
          viewBox="0 0 24 24"
          stroke-width="1.5"
          stroke="currentColor"
          class="w-6 h-6 my-auto"
        >
          <path
            stroke-linecap="round"
            stroke-linejoin="round"
            d="M15.75 6a3.75 3.75 0 11-7.5 0 3.75 3.75 0 017.5 0zM4.501 20.118a7.5 7.5 0 0114.998 0A17.933 17.933 0 0112 21.75c-2.676 0-5.216-.584-7.499-1.632z"
          />
        </svg>
      </div>
    </a>
  </header>

  <form
    on:submit|preventDefault={addLink}
    action="/"
    class="flex flex-col justify-start items-center gap-4 mt-8 text-white mb-6"
  >
    <input
      required
      type="text"
      bind:value={text}
      class="text-white bg-gray-700 rounded p-2"
      placeholder="Enter text.."
    />
    <input
      required
      type="url"
      bind:value={url}
      class="text-white bg-gray-700 rounded p-2"
      placeholder="Enter url.."
    />
    <button class="btn btn-primary btn-sm" type="submit"> Add Link </button>
  </form>

  {#if links.length > 0}
    <div class="flex flex-col justify-start items-center">
      {#each links as link}
        <div class="flex-row w-3/4 p-3 m-2 bg-gray-700 rounded-md">
          <button
            class="relative transition float-right top-0 right-0 hover:scale-125 active:scale-100"
            on:click={() => removeLink(link.id)}
          >
            <div class="text-white hover:text-red-400">
              <Icon name="x" size={16} />
            </div>
          </button>
          <div>{link.text}</div>
          <div class="text-gray-500">{link.url}</div>
        </div>
      {/each}
    </div>
  {/if}

  <div class="absolute flex top-auto bottom-0.5 w-full px-2">
    <button class="btn mb-2 mr-4" on:click={togglePreview}>
      Toggle Preview
    </button>
    <div class="flex-grow" />
    {#if loading}
      <div
        class="m-2 w-8 h-8 animate-spin rounded-full border-transparent border-2 border-y-white"
      />
    {/if}
    <button
      class="btn btn-primary float-right mb-2"
      disabled={loading || links.length == 0}
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
