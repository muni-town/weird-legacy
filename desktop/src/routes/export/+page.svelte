<script lang="ts">
  import { goto } from '$app/navigation'
  import { invoke } from '@tauri-apps/api/tauri'
  import Icon from 'components/Icon.svelte'

  function exportZip() {
    invoke('export_zip')
      .catch((e) => console.error(e))
      .finally(() => goto('/'))
  }
</script>

<div class="absolute top-0 w-screen h-screen overflow-hidden">
  <div class="absolute top-0 w-full transition h-full bg-gray-800 z-20">
    <button class="absolute top-5 right-5" on:click={() => goto('/')}>
      <div class="text-white transition hover:scale-125 hover:text-red-300">
        <Icon name="x" size={20} />
      </div>
    </button>
    <div class="flex flex-row flex-wrap justify-center p-10">
      <button
        on:click={exportZip}
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
        href="/publish/github"
        class="w-1/3 aspect-square m-3 bg-gray-300 hover:bg-blue-600 hover:scale-105 active:scale-100 rounded-md text-black transition-all hover:text-white text-center"
      >
        <Icon name="github" size={64} class="w-1/2 h-1/2 mx-auto mt-5 mb-2" />
        <div>Github Pages</div>
      </a>
    </div>
  </div>
</div>
