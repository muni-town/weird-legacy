<script lang="ts">
  import { onMount } from 'svelte'

  type Link = { text: string; url: string }

  let name = ''
  let username = ''
  let photo = 'https://unavatar.io/user'
  let links: Array<Link> = []

  onMount(async () => {
    let res = await fetch('/content.json')
    if (res.ok) {
      let data = await res.json()
      name = data.name
      links = data.links
      username = data.username
      photo = `https://unavatar.io/${username}`
    }
  })
</script>

<div
  class="bg-gray-200 font-sans h-screen w-full flex flex-row justify-center items-center"
>
  <div
    class="card w-full sm:w-2/3 lg:w-1/3 mx-auto bg-white md:m-5 shadow-md hover:shadow rounded-md"
  >
    <img
      class="w-32 mx-auto rounded-full mt-5 border-8 border-white"
      src={photo}
      alt=""
    />
    <div class="text-center mt-2 text-3xl font-medium">{name}</div>
    <div class="text-center mt-2 font-light text-sm">@{username}</div>
    <div class="text-center font-normal text-lg">Software Engineer</div>
    <div class="px-6 text-center mt-2 font-light text-sm">
      <p>
        Lorem, ipsum dolor sit amet consectetur adipisicing elit. Temporibus
        aliquam eveniet iste excepturi quia atque similique fugit repellendus
        facilis. Quae voluptatem ea voluptates accusantium fugit suscipit hic
        iure nobis ab!
      </p>
    </div>
    <hr class="mt-8" />
    <div class="flex p-4">
      <div class="w-1/2 text-center">
        <span class="font-bold">1.8 k</span> Followers
      </div>
      <div class="w-0 border border-gray-300" />
      <div class="w-1/2 text-center">
        <span class="font-bold">2.0 k</span> Following
      </div>
    </div>
    <div class="w-full text-center">
      {#each links as link}
        <div
          class="bg-gray-100 hover:bg-gray-200 w-5/6 ml-auto mr-auto px-5 py-2 my-2 rounded-lg text-center"
        >
          <a
            href={link.url}
            target="_blank"
            class="text-gray-800 text-center hover:text-gray-800"
          >
            <div class="text-gray-800">
              {link.text}
            </div>
            <div class="text-blue-500">
              {link.url}
            </div>
          </a>
        </div>
      {/each}
    </div>
  </div>
</div>
