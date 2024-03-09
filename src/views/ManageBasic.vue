<template>

  <el-row  gutter=20 justify="center">
    <el-col :span="12">重抽单张卡片:</el-col>
    <el-col :span="6">
      <el-switch
          v-model="reDrawSingle"
          class="ml-2"
          active-text="Open"
          inactive-text="Close"
      />
    </el-col>
  </el-row>

  <el-row  gutter=20 justify="center">
    <el-col :span="12">背景图片上传:</el-col>
    <el-col :span="6">
      <el-upload
          ref="upload"
          class="upload-demo"
          :on-preview="handlePreviewImg"
          :limit="1"
          :on-exceed="handleExceed"
          list-type="picture"
          :auto-upload="false"
      >
        <el-button type="primary">Click to upload</el-button>
      </el-upload>
    </el-col>
  </el-row>

  <el-row  gutter=20 justify="center">
    <el-col :span="12">抽奖转动时间(秒):</el-col>
    <el-col :span="6">
      <el-input-number v-model="rotateSecond" :min="1" :max="10" @change="handleChange" />
    </el-col>
  </el-row>

  <el-row  gutter=20 justify="center">
    <el-col :span="12">背景音乐上传:</el-col>
    <el-col :span="6">
      <el-upload
          ref="upload"
          class="upload-demo"
          :limit="1"
          :on-exceed="handleExceed"
          :auto-upload="false"
      >
        <el-button type="primary">上传背景音乐</el-button>
      </el-upload>
    </el-col>
  </el-row>


</template>

<script setup lang="ts">
import { ref } from 'vue'
import {genFileId, UploadInstance, UploadProps, UploadRawFile} from "element-plus";
const reDrawSingle = ref(true)
const rotateSecond = ref(1)


const handleChange = (rotateSecond: number) => {
  console.log(rotateSecond)
}

const upload = ref<UploadInstance>()

const handlePreviewImg: UploadProps['onPreview'] = (file) => {
  console.log(file)
}


const handleExceed: UploadProps['onExceed'] = (files) => {
  upload.value!.clearFiles()
  const file = files[0] as UploadRawFile
  file.uid = genFileId()
  upload.value!.handleStart(file)
}

</script>

<style scoped>

</style>