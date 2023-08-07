<script lang="ts">
  import '../app.css'
  import { onMount } from 'svelte'
  import { user, links, link_id } from '../store'
  import { invoke } from '@tauri-apps/api/tauri'
  import type { Link } from '../bindings/Link'
  import type { Profile } from '../bindings/Profile'

  onMount(() => {
    invoke('get_user')
      .then((u: Profile) => user.set(u))
      .catch((e) => console.error(e))
    invoke('get_links')
      .then((l: Array<Link>) => {
        links.set(l)
        if (l.length > 0) {
          link_id.set(l[l.length - 1].id + 1)
        }
      })
      .catch((e) => console.error(e))
  })
</script>

<div>
  <slot />
</div>
