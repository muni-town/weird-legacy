<script lang="ts">
  import { onMount } from 'svelte'
  import { invoke } from '@tauri-apps/api/tauri'

  type Link = { text: string; url: string }

  let links: Array<Link> = []
  let url = ''
  let text = ''
  let name = ''
  let username = ''
  let description = ''
  let user_exists = true
  let loading = false
  let export_modal = false

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
    invoke('update_user', { name, username, description }).then(
      () => (user_exists = true)
    )
  }

  function toggle_preview() {
    invoke('toggle_preview_window')
  }
</script>

<main class="text-white bg-gray-800">
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
          bind:value={name}
          class="text-white bg-gray-700 rounded p-2"
          placeholder="Enter your name.."
        />
        <input
          required
          type="text"
          bind:value={username}
          class="text-white bg-gray-700 rounded p-2"
          placeholder="Enter you username.."
        />
        <textarea
          cols="20"
          rows="4"
          bind:value={description}
          placeholder="About you.."
          class="text-white bg-gray-700 rounded p-2"
        />
        <button
          class="bg-blue-600 hover:scale-105 active:scale-100 rounded p-2 mb-2"
          type="submit"
        >
          Save
        </button>
      </form>
    </div>
  {/if}

  {#if export_modal}
    <div class="absolute top-0 w-full transition h-full bg-gray-800 z-20">
      <button
        class="absolute top-5 right-5"
        on:click={() => (export_modal = false)}
      >
        <div class="text-white hover:scale-125 hover:text-red-300">
          <svg
            xmlns="http://www.w3.org/2000/svg"
            fill="none"
            viewBox="0 0 24 24"
            stroke-width="1.5"
            stroke="currentColor"
            class="w-6 h-6"
          >
            <path
              stroke-linecap="round"
              stroke-linejoin="round"
              d="M6 18L18 6M6 6l12 12"
            />
          </svg>
        </div>
      </button>
      <div class="flex flex-row flex-wrap justify-center p-10">
        <button
          on:click={export_zip}
          class="w-1/3 aspect-square m-3 bg-gray-300 hover:bg-blue-600 hover:scale-105 active:scale-100 rounded-md text-black hover:text-white text-center"
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
        <button
          class="w-1/3 aspect-square m-3 bg-gray-500 rounded-md text-black text-center"
        >
          <svg
            xmlns="http://www.w3.org/2000/svg"
            fill="none"
            viewBox="0 0 128 128"
            stroke-width="1.5"
            stroke="currentColor"
            class="w-1/2 h-1/2 mx-auto mt-3"
          >
            <g fill="currentColor"
              ><path
                fill-rule="evenodd"
                clip-rule="evenodd"
                d="M64 5.103c-33.347 0-60.388 27.035-60.388 60.388 0 26.682 17.303 49.317 41.297 57.303 3.017.56 4.125-1.31 4.125-2.905 0-1.44-.056-6.197-.082-11.243-16.8 3.653-20.345-7.125-20.345-7.125-2.747-6.98-6.705-8.836-6.705-8.836-5.48-3.748.413-3.67.413-3.67 6.063.425 9.257 6.223 9.257 6.223 5.386 9.23 14.127 6.562 17.573 5.02.542-3.903 2.107-6.568 3.834-8.076-13.413-1.525-27.514-6.704-27.514-29.843 0-6.593 2.36-11.98
        6.223-16.21-.628-1.52-2.695-7.662.584-15.98 0 0 5.07-1.623 16.61 6.19C53.7 35 58.867 34.327 64 34.304c5.13.023 10.3.694 15.127 2.033 11.526-7.813 16.59-6.19 16.59-6.19 3.287 8.317 1.22 14.46.593 15.98 3.872 4.23 6.215 9.617 6.215 16.21 0 23.194-14.127 28.3-27.574 29.796 2.167 1.874 4.097 5.55 4.097 11.183 0 8.08-.07 14.583-.07 16.572 0 1.607 1.088 3.49 4.148 2.897 23.98-7.994 41.263-30.622 41.263-57.294C124.388 32.14 97.35 5.104 64 5.104z"
              /><path
                d="M26.484
        91.806c-.133.3-.605.39-1.035.185-.44-.196-.685-.605-.543-.906.13-.31.603-.395 1.04-.188.44.197.69.61.537.91zm-.743-.55M28.93 94.535c-.287.267-.85.143-1.232-.28-.396-.42-.47-.983-.177-1.254.298-.266.844-.14 1.24.28.394.426.472.984.17 1.255zm-.575-.618M31.312 98.012c-.37.258-.976.017-1.35-.52-.37-.538-.37-1.183.01-1.44.373-.258.97-.025 1.35.507.368.545.368 1.19-.01 1.452zm0 0M34.573 101.373c-.33.365-1.036.267-1.552-.23-.527-.487-.674-1.18-.343-1.544.336-.366 1.045-.264 1.564.23.527.486.686
        1.18.333 1.543zm0 0M39.073 103.324c-.147.473-.825.688-1.51.486-.683-.207-1.13-.76-.99-1.238.14-.477.823-.7 1.512-.485.683.206 1.13.756.988 1.237zm0 0M44.016 103.685c.017.498-.563.91-1.28.92-.723.017-1.308-.387-1.315-.877 0-.503.568-.91 1.29-.924.717-.013 1.306.387 1.306.88zm0 0M48.614 102.903c.086.485-.413.984-1.126 1.117-.7.13-1.35-.172-1.44-.653-.086-.498.422-.997 1.122-1.126.714-.123 1.354.17 1.444.663zm0 0"
              /></g
            >
          </svg>
          Github Pages
        </button>
      </div>
    </div>
  {/if}

  <form
    on:submit|preventDefault={addLink}
    action="/"
    class="flex flex-col justify-start items-center gap-4 mt-8 text-white"
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
    <button
      class="bg-blue-600 hover:scale-105 active:scale-100 rounded p-2 mb-2"
      type="submit"
    >
      Add Link
    </button>
  </form>

  {#if links.length > 0}
    <div class="flex flex-col justify-start items-center">
      {#each links as link}
        <div class="flex-row w-3/4 p-3 m-2 bg-gray-700 rounded-md">
          <button
            class="relative float-right top-0 right-0 hover:scale-125 active:scale-100"
            on:click={() => removeLink(link.url, link.text)}
          >
            <div class="text-white hover:text-red-400">
              <svg
                xmlns="http://www.w3.org/2000/svg"
                fill="none"
                viewBox="0 0 24 24"
                stroke-width="1.5"
                stroke="currentColor"
                class="w-6 h-6"
              >
                <path
                  stroke-linecap="round"
                  stroke-linejoin="round"
                  d="M6 18L18 6M6 6l12 12"
                />
              </svg>
            </div>
          </button>
          <div>{link.text}</div>
          <div class="text-gray-500">{link.url}</div>
        </div>
      {/each}
    </div>
  {/if}

  <div class="absolute flex flex-row top-auto bottom-0.5 left-auto right-2">
    <button
      class="bg-gray-700 hover:bg-gray-600 hover:scale-105 active:scale-100 rounded-md p-3 mb-2 mr-4"
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
      class="float-right bg-blue-800 hover:scale-105 disabled:bg-gray-700 disabled:scale-100 disabled:text-gray-500 rounded-md p-3 mb-2"
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

<style lang="postcss">
  :root {
    @apply bg-gray-800;
  }
</style>
