<script lang="ts">
  import { onMount } from 'svelte'
  import { invoke } from '@tauri-apps/api/tauri'
  import type { User } from '../bindings/User'
  import type { Link } from '../bindings/Link'
  import Icon from '../components/Icon.svelte'

  let links: Array<Link> = []
  let user: User
  let id = 1
  let url = ''
  let text = ''
  let user_exists = true
  let loading = false
  let export_modal = false

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
      .then((u: User) => (user = u))
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

  function export_zip() {
    invoke('export_zip')
      .catch((e) => console.error(e))
      .finally(() => (export_modal = true))
  }

  function generate() {
    loading = true
    invoke('generate_site')
      .then(() => {
        console.log('Site generated')
        export_modal = true
      })
      .catch((e) => {
        console.error(e)
      })
      .finally(() => (loading = false))
  }

  function addUser() {
    invoke('update_user', { user }).then(() => (user_exists = true))
  }

  function toggle_preview() {
    invoke('toggle_preview_window')
  }
</script>

<main>
  <header class="h-10 w-screen align-middle">
    <button
      on:click={() => (user_exists = false)}
      class="ml-auto p-2 text-center text-gray-400 hover:text-white hover:scale-105 active:scale-100"
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
    </button>
  </header>

  {#if !user_exists}
    <div class="absolute top-0 w-full transition h-full bg-gray-800 z-10">
      <form
        on:submit|preventDefault={addUser}
        action="/"
        class="flex flex-col justify-start items-center gap-4 mt-8 text-white"
      >
        <input
          required
          type="text"
          bind:value={user.name}
          class="text-white bg-gray-700 rounded p-2"
          placeholder="Enter your name.."
        />
        <input
          required
          type="text"
          bind:value={user.username}
          class="text-white bg-gray-700 rounded p-2"
          placeholder="Enter you username.."
        />
        <textarea
          cols="20"
          rows="4"
          bind:value={user.description}
          placeholder="About you.."
          class="text-white bg-gray-700 rounded p-2"
        />
        <button class="btn btn-primary" type="submit"> Save </button>
      </form>
    </div>
  {/if}

  {#if export_modal}
    <div class="absolute top-0 w-screen h-screen overflow-hidden">
      <div class="absolute top-0 w-full transition h-full bg-gray-800 z-20">
        <button
          class="absolute top-5 right-5"
          on:click={() => (export_modal = false)}
        >
          <div class="text-white transition hover:scale-125 hover:text-red-300">
            <Icon name="x" size={20} />
          </div>
        </button>
        <div class="flex flex-row flex-wrap justify-center p-10">
          <button
            on:click={export_zip}
            class="w-1/3 aspect-square m-3 bg-gray-300 hover:bg-blue-600 hover:scale-105 active:scale-100 rounded-md text-black transition-all hover:text-white text-center"
          >
            <svg
              xmlns="http://www.w3.org/2000/svg"
              fill="none"
              viewBox="0 0 24 24"
              stroke-width="1.5"
              stroke="currentColor"
              class="w-1/2 h-1/2 mx-auto mt-3"
            >
              <path
                stroke-linecap="round"
                stroke-linejoin="round"
                d="M21 7.5l-9-5.25L3 7.5m18 0l-9 5.25m9-5.25v9l-9 5.25M3 7.5l9 5.25M3 7.5v9l9 5.25m0-9v9"
              />
            </svg>
            Save .zip file
          </button>
          <button
            class="w-1/3 aspect-square m-3 bg-gray-500 rounded-md text-black text-center"
          >
            <svg
              xmlns="http://www.w3.org/2000/svg"
              fill="none"
              viewBox="0 0 24 24"
              stroke-width="1.5"
              stroke="currentColor"
              class="w-1/2 h-1/2 mx-auto mt-3"
            >
              <path
                stroke-linecap="round"
                stroke-linejoin="round"
                d="M12 16.5V9.75m0 0l3 3m-3-3l-3 3M6.75 19.5a4.5 4.5 0 01-1.41-8.775 5.25 5.25 0 0110.233-2.33 3 3 0 013.758 3.848A3.752 3.752 0 0118 19.5H6.75z"
              />
            </svg>
            Upload via ftp
          </button>
          <a
            href="publish/github"
            class="w-1/3 aspect-square m-3 bg-gray-300 hover:bg-blue-600 hover:scale-105 active:scale-100 rounded-md text-black transition-all hover:text-white text-center"
          >
            <Icon
              name="github"
              size={64}
              class="w-1/2 h-1/2 mx-auto mt-5 mb-2"
            />
            <div>Github Pages</div>
          </a>
        </div>
      </div>
    </div>
  {/if}

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
    <button class="btn mb-2 mr-4" on:click={toggle_preview}>
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
