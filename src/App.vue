<template>
  <StartFiled />
  <ThreeDCards v-model:showTable="showTable"/>
  <MenuBar v-model:showTable="showTable"/>
</template>

<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
//import Greet from "./components/Greet.vue";
import StartFiled from "@/components/StartFiled.vue"
import ThreeDCards from "@/components/ThreeDCards.vue"
import MenuBar from "@/components/MenuBar.vue"
import {ref} from "vue";
const showTable = ref(true);
</script>

<style scoped>
</style>
