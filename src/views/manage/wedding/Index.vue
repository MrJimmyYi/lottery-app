<template>

<!--  <el-form-item label="Instant delivery">-->
<!--    <el-switch v-model="form.delivery" />-->
<!--  </el-form-item>-->
  <el-header style="height: 5%">
    <el-button @click="updateWedding">Edit</el-button>
  </el-header>

  <el-form :model="form" label-width="auto" style="max-width: 600px">

    <el-form-item label="Background Image:">
      <el-image style="width: 100px; height: 100px" :src="form.bgImg" fit="scale-down"/>
    </el-form-item>

    <el-form-item label="Lottery spin time (seconds):">
      <el-input-number v-model="form.spinTime" :min="1" :max="10" />
    </el-form-item>

    <el-form-item label="Background Music Path:">
      <el-input v-model="form.audioSrc" />
    </el-form-item>

  </el-form>

  <el-dialog v-model="editDialogVisible" title="修改模式"  :before-close="handleClose" :destroy-on-close=true>
    <BasicModelEdit :model="form.model" @ok="editOkFn" @close="editCloseFn" ></BasicModelEdit>
  </el-dialog>

</template>

<script setup lang="ts">
import {onMounted, ref} from 'vue'
import BasicModelEdit from "@/views/manage/wedding/Edit.vue";
import {convertFileSrc, invoke} from "@tauri-apps/api/tauri";
import {ModelBasic, TauriResponse} from "@/types";
import {ElMessageBox} from "element-plus";
const editDialogVisible = ref(false);

const form = ref<ModelBasic>({
  id: 1,
  model: "wedding",
  bgImg: "",
  audioSrc: "",
  reDrawing: 1,
  spinTime: 10,
  remark1: "",
  remark2: ""
})


onMounted(()=>{
  fetchData();
})

const updateWedding = ()=>{
  editDialogVisible.value=true;
}


const fetchData = async () => {
  try {
    // 调用后端接口获取分页数据，传递当前页和每页条目数
    let res = await invoke('get_model_basic', {
      model: form.value.model,
    }) as TauriResponse<ModelBasic>;

    if(res.data) {
      let data = res.data;
      form.value.bgImg = data.bgImg ? convertFileSrc(data.bgImg) : '';
      form.value.id = data.id;
      form.value.audioSrc = data.audioSrc;
      form.value.spinTime = data.spinTime;
    }
  } catch (error) {
    console.error('failed to fetch wedding model:', error);
  }
};

const editCloseFn=()=>{
  editDialogVisible.value = false;
}

const editOkFn=()=>{
  editCloseFn();
  fetchData();
}

const handleClose = (done: () => void) => {
  ElMessageBox.confirm('确定要关闭吗?')
      .then(() => {
        done()
      })
      .catch(() => {
        // catch error
      })
}

</script>

<style scoped>

</style>