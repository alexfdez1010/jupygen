<script setup>
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { save } from '@tauri-apps/api/dialog';

let generating = ref(false);
const description = ref("");
const filePath = ref("");

async function generate() {
  generating.value = true;
  await invoke(
      "generate_notebook",
      {
          path: filePath.value,
          description: description.value,
      }
  );
  generating.value = false;
}

async function selectFile(){
  filePath.value = await save({
  filters: [{
    name: 'notebook',
    title: 'Select a path to save the Jupyter Notebook',
    extensions: ['ipynb']
  }]
});
}
</script>

<template>
  <div id="container-input">
      <div class="card">
        <label for="description">Description</label>
        <textarea v-model="description" rows="5" cols="40" placeholder="Enter the description for the Jupyter Notebook" />
      </div>
      <div class="card">
        <button id="path" type="button" @click="selectFile()">Select a path</button>
        <p>{{filePath}}</p>
      </div>
      <div class="card" v-if="generating">
        <button type="button" disabled>Generating...</button>
      </div>
      <div class="card" v-else-if="filePath !== '' && description !== ''">
        <button type="button" @click="generate()">Generate</button>
      </div>
      <div class="card" v-else>
        <button type="button" disabled>Generate</button>
      </div>
  </div>
</template>
