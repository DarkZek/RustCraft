<script lang="ts" setup>
import { useRouter } from 'vue-router'
import { login } from '../services/apiService'
import NavigationBar from '../components/NavigationBar.vue'
import RcSubmitButton from '../components/elements/RcSubmitButton.vue'

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
  <main>
    <navigation-bar />
    <form
      id="form"
      @submit.prevent="onSubmit"
    >
      <h1>Login</h1>
      <input placeholder="Username" name="username" required>
      <br>
      <rc-submit-button label="Submit" />
    </form>
  </main>
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

h1 {
  text-align: center;
  color: white;
  font-family: "Londrina Solid";
}

</style>
