<template>
  <el-form
      style="max-width: 600px;max-height: 800px"
      label-width="auto"
      class="demo-ruleForm"
  >
    <el-form-item label="下载模板">
      <el-button @click="downloadUserTemplate" >点击下载</el-button>
    </el-form-item>

    <el-form-item label="批量导入">
      <el-upload
          ref="upload"
          class="upload-demo"
          :limit="1"
          :on-change="handleChange"
          :auto-upload="false"
      >
        <template #trigger>
          <el-button type="primary">选择文件</el-button>
        </template>
        <el-button class="ml-3" type="success" @click="submitUpload">
          开始导入
        </el-button>
        <template #tip>
          <div class="el-upload__tip text-red">
            请按照excel格式规范录入数据
          </div>
        </template>
      </el-upload>
    </el-form-item>

    <el-form-item label="导出数据">
      <el-button @click="exportData" >点击导出</el-button>
    </el-form-item>

  </el-form>

</template>

<script setup lang="ts">

import {invoke} from "@tauri-apps/api/tauri";
import {dialog, path} from "@tauri-apps/api";
import {writeBinaryFile} from "@tauri-apps/api/fs";
import {ElMessage} from "element-plus";
import { ref,defineEmits } from 'vue'

const emit = defineEmits(['import']);

const downloadUserTemplate = async () => {
  try {
    const fileName = 'userTemplate'
    const fileBytes = await  invoke('download_template_excel', {fileName});
    const basePath = await path.downloadDir() + `/${fileName}`
    let selPath = await dialog.save({
      title: `保存文件`,
      defaultPath: basePath,
      filters: [{
        name: fileName,
        extensions: ['xlsx']
      }]
    });
    console.log("selPath----", selPath);
    if(selPath) {
      await writeBinaryFile({ contents: fileBytes as any, path: `${selPath}` })
      console.log("download")
    }
  } catch (error) {
    console.error('Failed to download Excel template:', error);
  }
};

const upload = ref<File | null>(null);
const handleChange = async (file: any)=> {
  upload.value = file.raw;
}

const submitUpload = async () => {
  if (upload.value ) {
    const file = upload.value; // 获取原始File对象
    if (file) {
      try {
        const arrayBuffer = await upload.value?.arrayBuffer();
        if(arrayBuffer) {
          let uint8Array = Array.from(new Uint8Array(arrayBuffer));
          await invoke('batch_excel_operate', {fileName: file.name, fileContent: uint8Array});
          emit('import')
          ElMessage.success('用户信息录入成功');
        }
      } catch (error) {
        console.log(error)
        ElMessage.error('用户信息录入失败');
      }
    }
  } else {
    console.log("No file selected for upload.");
  }

}


const exportData = () => {

}
</script>

<style scoped>

</style>