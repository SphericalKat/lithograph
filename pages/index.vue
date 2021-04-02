<template>
  <div class="flex flex-col max-w-5xl m-auto h-full">
    <Header />

    <div class="grid grid-cols-1 sm:grid-cols-4 gap-4 h-full">
      <div class="col-span-1 font-mono">2</div>
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
