<script setup>
import { ref, onMounted } from 'vue';
import { my_project_backend } from 'declarations/my_project_backend/index';
let lastMsg = ref('');

onMounted(async () => {
  lastMsg.value = await getMessage();
})

async function handleSubmit(e) {
  e.preventDefault();
  const target = e.target;
  const message = target.querySelector('#message').value;
  await my_project_backend.set_msg(message);
  lastMsg.value = await getMessage();
}

async function getMessage() {
  const latestMesage = await my_project_backend.get_msg();
  return latestMesage;
}
</script>

<template>
  <main>
    <img src="/logo2.svg" alt="DFINITY logo" />
    <br />
    <br />
    <form action="#" @submit="handleSubmit">
      <label for="message">Enter your message: &nbsp;</label>
      <input id="message" alt="Message" type="text" />
      <button type="submit">Click Me!</button>
    </form>
    <p>{{lastMsg}}</p>
  </main>
</template>
