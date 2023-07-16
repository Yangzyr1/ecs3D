<template>
    <q-btn color="primary"
           label="指定路径"
           @click="chooseFile"
    />
    {{csvFileList}}
</template>

<script lang="ts" setup>
import {ref} from "vue";
import { dialog } from '@tauri-apps/api'
const csvFileList = ref<string[]>([]);
const showList = [
    {
        name: 'name',
        required: true,
        label: '文件名',
        field: 'name'
    }
]
function chooseFile() {
    dialog.open({
        multiple: true,
        directory: true,
        filter: '*'
    })
        .then(result => {
            if (Array.isArray(result)) {
                csvFileList.value = result;
                // user selected multiple files
            } else if (result === null) {
                // user cancelled the selection
            } else {
                csvFileList.value.push(result)
                // user selected a single file
            }
        })
}
</script>

<style scoped>

</style>