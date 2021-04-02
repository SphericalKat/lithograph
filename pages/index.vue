<template>
  <div class="flex flex-col max-w-6xl m-auto h-full p-6">
    <Header />

    <div class="grid grid-cols-1 sm:grid-cols-4 gap-4 h-full">
      <div class="col-span-1 font-mono">
        <nuxt-img src="/images/sphericalkat.jpeg" loading="lazy" />
      </div>
      <div class="col-span-3">3</div>
    </div>

    <hr />

    <Footer />
  </div>
</template>

<script lang="ts">
import Vue from 'vue'

export default Vue.extend({})
</script>

<style></style>
