<script lang="ts">
  import type { Link } from '../../../src/bindings/Link'

  let name = 'User Name'
  let username = 'user'
  let photo = 'https://unavatar.io/user'
  let title = 'Programmer'
  let links: Array<Link> = [
    { id: 1, url: 'https://link1.example.com', text: 'Link 1' },
    { id: 2, url: 'https://link2.example.com', text: 'Link 2' },
  ]
</script>

<div
  class="bg-gray-200 font-sans h-screen w-full flex flex-row justify-center items-center"
>
  <div
    class="card w-full sm:w-2/3 lg:w-1/3 mx-auto bg-white md:m-5 shadow-md hover:shadow rounded-md"
  >
    <img
      id="photo"
      class="w-32 mx-auto rounded-full mt-5 border-8 border-white"
      src={photo}
      alt=""
    />
    <div id="name" class="text-center mt-2 text-3xl font-medium">{ name }</div>
    <div id="username" class="text-center mt-2 font-light text-sm">@{ username }</div>
    <div id="title" class="text-center font-normal text-lg">{ title }</div>
    <div id="about" class="px-6 text-center mt-2 font-light text-sm">
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
    <div id="links" class="w-full text-center">
      {#each links as link}
        <div
          class="link"
        >
          <a
            href={link.url}
            target="_blank"
          >
            <div class="link-text">
              {link.text}
            </div>
            <div class="link-url">
              {link.url}
            </div>
          </a>
        </div>
      {/each}
    </div>
  </div>
</div>
