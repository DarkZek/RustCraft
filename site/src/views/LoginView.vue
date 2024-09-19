<script lang="ts" setup>
import { useRouter } from 'vue-router'
import { login } from '../services/apiService'

const router = useRouter()

async function onSubmit(e: any) {
  const username = e.target.elements.username.value

  e.preventDefault()

  try {
    const json = await login(username)
    localStorage.setItem("token", json.data.refresh_token)

    // Logged in!

    setTimeout(() => router.push({ name: 'play' }), 100)
  } catch (e) {
    alert('An error occurred')
    console.log(e)
  }

}

</script>

<template>
  <form
    id="form"
    @submit.prevent="onSubmit"
  >
    <h2>RustCraft Login</h2>
    <input placeholder="Username" name="username" required>
    <input type="submit" value="Submit">
  </form>
</template>

<style scoped lang="scss">

form {
  width: 400px;
  max-width: 90vw;
  display: flex;
  flex-direction: column;
  margin: auto;
  padding-top: 48px;
}

h2 {
  text-align: center;
  color: white;
}

</style>
