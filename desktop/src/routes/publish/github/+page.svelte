<script lang="ts">
  import { browser } from '$app/environment'
  import type { Octokit } from '@octokit/rest'
  import Spinner from 'components/Spinner.svelte'
  import { getOctokit } from 'utils'

  let user:
    | Awaited<ReturnType<Octokit['users']['getAuthenticated']>>['data']
    | null = null

  if (browser) {
    const octokit = getOctokit()
    ;(async () => {
      const userResp = await octokit.users.getAuthenticated()
      user = userResp.data
    })()
  }
</script>

{#if user}
  <div class="flex gap-4 justify-center items-center mt-4">
    <img
      alt="avatar"
      src={user.avatar_url}
      width="45"
      height="45"
      class="rounded-full"
    />
    {user.login}
  </div>
{:else}
  <div class="flex justify-center mt-6">
    <Spinner size={40} />
  </div>
{/if}
