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
  <main class="chat-container">
    <h1>Chat Room</h1>
    <form action="#" @submit="handleSubmit" class="chat-form">
      <label for="message">Enter your message: &nbsp;</label>
      <input id="message" alt="Message" type="text" class="message-input" />
      <button type="submit" class="submit-btn">Send</button>
    </form>
    <ul class="chat-list">
      <li v-for="(message, index) in chat" :key="index" class="chat-message">
        {{ message }}
      </li>
    </ul>
  </main>
</template>

<style scoped>
.chat-container {
  max-width: 600px;
  margin: 0 auto;
  padding: 20px;
  font-family: Arial, sans-serif;
  background-color: #f9f9f9;
  border: 1px solid #ccc;
  border-radius: 8px;
  box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
}

.chat-form {
  display: flex;
  gap: 10px;
  margin-bottom: 20px;
}

.message-input {
  flex: 1;
  padding: 8px;
  border: 1px solid #ccc;
  border-radius: 4px;
  font-size: 16px;
}

.submit-btn {
  padding: 8px 16px;
  font-size: 16px;
  background-color: #007bff;
  color: white;
  border: none;
  border-radius: 4px;
  cursor: pointer;
}

.submit-btn:hover {
  background-color: #0056b3;
}

.chat-list {
  list-style: none;
  padding: 0;
  margin: 0;
}

.chat-message {
  padding: 10px;
  margin: 5px 0;
  background-color: #e6f7ff;
  border: 1px solid #91d5ff;
  border-radius: 4px;
}
</style>
