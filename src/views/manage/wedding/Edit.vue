<template>
  <el-form
      ref="basicFormRef"
      style="max-width: 600px"
      :model="basicForm"
      label-width="auto"
      class="demo-ruleForm"
      :rules="rules"
  >
    <el-form-item label="Background Image" prop="bgImg">
      <el-upload
          class="avatar-uploader"
          :on-change="handleImgChange"
          list-type="picture-card"
          action="#"
          :limit="1"
          :auto-upload="false"
          :file-list="imgFileList"
      >
        <el-icon><Plus /></el-icon>
      </el-upload>
    </el-form-item>

    <el-form-item label="Lottery spin time (seconds):" prop="spinTime">
      <el-input-number v-model="basicForm.spinTime" :min="1" :max="10" />
    </el-form-item>

    <el-form-item label="Background Audio" prop="audio">
      <el-upload
          class="avatar-uploader"
          :on-change="handleAudioChange"
          action="#"
          :limit="1"
          :auto-upload="false"
          :file-list="audioFileList"
      >
        <el-icon><Plus /></el-icon>
      </el-upload>
    </el-form-item>

    <el-form-item>
      <el-button type="primary" @click="updateBasicModel(basicFormRef)">Submit</el-button>
      <el-button @click="cancelFn">Cancel</el-button>
    </el-form-item>
  </el-form>
</template>

<script setup lang="ts">
import {defineEmits, reactive, ref, onMounted, defineProps} from "vue";
import {ElMessage, FormInstance, UploadUserFile} from "element-plus";
import {ModelBasic, TauriResponse} from "@/types";
import {convertFileSrc, invoke} from "@tauri-apps/api/tauri";
import { Plus } from '@element-plus/icons-vue'

const emit = defineEmits(['close', "ok"]);

const selectedImg = ref<File | null>(null);
const basicFormRef = ref<FormInstance | null>(null);
const selectedAudio = ref<File | null>(null);

const imgFileList = ref<UploadUserFile[]>([])
const audioFileList = ref<UploadUserFile[]>([])

const basicForm = ref<ModelBasic>({
  id: 0,
  model: "wedding",
  bgImg: "",
  audioSrc: "",
  reDrawing: 1,
  spinTime: 10,
  remark1: "",
  remark2: ""
})

// 使用 defineProps 定义接收的 props
const props = defineProps({
  model: {
    type: String,
    required: true,
  },
});

// 表单验证的规则
const rules = reactive({})

const handleImgChange = async (file: any) => {
  selectedImg.value = file.raw;
  let res = await invoke('generate_imgstr', {fileName: file.name}) as TauriResponse<string>;
  basicForm.value.bgImg = res.data ?? '';
}

const handleAudioChange = async (file: any) => {
  selectedAudio.value = file.raw;
  let res = await invoke('generate_audiostr', {fileName: file.name}) as TauriResponse<string>;
  basicForm.value.audioSrc = res.data ?? '';
}


const cancelFn = () => {
  emit('close');
}

onMounted(async ()=>{
  let res = await invoke('get_model_basic', {model: props.model}) as TauriResponse<ModelBasic>
  basicForm.value = res.data ?? basicForm.value
  if (basicForm.value.bgImg) {
    let fileName = basicForm.value.bgImg.split(/[/\\]/).pop();
    if(fileName) {
      const assetUrl = convertFileSrc(basicForm.value.bgImg);
      imgFileList.value.push({url:assetUrl, name:fileName})
    }
  }

})

const updateBasicModel = (basicFormRef :FormInstance | null) => {
  if (!basicFormRef) {
    return;
  }
  basicFormRef.validate(async (isOk: boolean) => {
    if (isOk) {
      try {
        const imgArrayBuffer = await selectedImg.value?.arrayBuffer();
        let uint8ImgArray:number[] = []
        if(imgArrayBuffer) {
          uint8ImgArray = Array.from(new Uint8Array(imgArrayBuffer));
        }

        const audiArrayBuffer = await selectedAudio.value?.arrayBuffer();
        let uint8AudioArray:number[] = []
        if(audiArrayBuffer) {
          uint8AudioArray = Array.from(new Uint8Array(audiArrayBuffer));
        }

        await invoke('update_basic_model', {basicModel: basicForm.value, imgData: uint8ImgArray, audioData: uint8AudioArray});
        emit('ok')
        ElMessage.success('update basic model success');
      } catch (error) {
        console.log(error)
        ElMessage.error('update basic model fail');
      }
    }
  })
}

</script>

<style scoped>

</style>