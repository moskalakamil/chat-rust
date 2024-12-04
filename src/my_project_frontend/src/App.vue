<script setup>
import { ref, onMounted } from 'vue';
import { my_project_backend } from 'declarations/my_project_backend/index';
let chat = ref([]);

onMounted(async () => {
  await getChat();
})

async function handleSubmit(e) {
  e.preventDefault();
  const target = e.target;
  const message = target.querySelector('#message').value;
  await my_project_backend.set_msg(message);
  await getChat();
}

async function getChat() {
  const latestChat = await my_project_backend.get_chat();
  chat.value = latestChat;
  return latestChat;
}
</script>

<template>
  <main>
    <br />
    <br />
    <form action="#" @submit="handleSubmit">
      <label for="message">Enter your message: &nbsp;</label>
      <input id="message" alt="Message" type="text" />
      <button type="submit">Click Me!</button>
    </form>
    <p>{{chat.toString()}}</p>
  </main>
</template>
