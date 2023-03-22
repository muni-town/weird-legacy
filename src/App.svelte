<script lang="ts">
  import { onMount } from "svelte";

  let todos = [];
  let currText = "";

  function addTodo() {
    todos = [...todos, currText];
    currText = "";
    updateStorage();
  }

  function removeTodo(text: string) {
    return () => {
      todos = todos.filter((t) => t != text);
      updateStorage();
    };
  }

  function updateStorage() {
    localStorage.setItem("todos", JSON.stringify(todos));
  }

  onMount(() => {
    todos = JSON.parse(localStorage.getItem("todos")) ?? [];
  });
</script>

<main class="flex flex-col justify-start items-center gap-4 text-white">
  <div class="mb-8" />
  <input type="text" bind:value={currText} class="text-black rounded p-2" />
  <button class="bg-blue-600 rounded p-2 mb-2" on:click={addTodo}
    >add todo</button
  >

  <div class="todo-items">
    {#each todos as todo}
      <p on:click={removeTodo(todo)} class="m-2 font-bold hover:line-through">
        {todo}
      </p>
    {/each}
  </div>
</main>

<style lang="postcss">
  :root {
    font-family: monospace;
    background-color: #292a32;
  }
</style>
