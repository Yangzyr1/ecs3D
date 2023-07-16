<template>
    <q-btn color="secondary" label="启动Bevy程序" @click="handleLaunchBevy" />
</template>

<script setup lang="ts">
import { invoke } from '@tauri-apps/api/tauri'
function handleLaunchBevy(){
    invoke('start_bevy')
}
</script>

<style scoped>

</style>