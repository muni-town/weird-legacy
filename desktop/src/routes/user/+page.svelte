<script lang="ts">
  import { goto } from '$app/navigation'
  import { invoke } from '@tauri-apps/api/tauri'
  import { user } from '../../store'

  function updateUser() {
    invoke('update_user', { user: $user }).then(() => goto('/'))
  }
</script>

<div class="absolute top-0 w-full transition h-full bg-gray-800 z-10">
  <form
    on:submit|preventDefault={updateUser}
    action="/"
    class="flex flex-col justify-start items-center gap-4 mt-8 text-white"
  >
    <input
      required
      type="text"
      bind:value={$user.name}
      class="text-white bg-gray-700 rounded p-2"
      placeholder="Enter your name.."
    />
    <input
      required
      type="text"
      bind:value={$user.username}
      class="text-white bg-gray-700 rounded p-2"
      placeholder="Enter you username.."
    />
    <input
      required
      type="text"
      bind:value={$user.title}
      class="text-white bg-gray-700 rounded p-2"
      placeholder="Enter you title.."
    />
    <input
      type="url"
      bind:value={$user.photo}
      class="text-white bg-gray-700 rounded p-2"
      placeholder="Enter url to your avatar.."
    />
    <textarea
      cols="20"
      rows="4"
      bind:value={$user.about}
      placeholder="About you.."
      class="text-white bg-gray-700 rounded p-2"
    />
    <button class="btn btn-primary" type="submit"> Save </button>
  </form>
</div>
