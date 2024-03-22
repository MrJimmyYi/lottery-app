<template>
  <el-form
      ref="prizeFormRef"
      style="max-width: 600px"
      :model="prizeForm"
      label-width="auto"
      class="demo-ruleForm"
      :rules="rules"
  >
    <el-form-item label="奖品等级" prop="range">
      <el-input v-model="prizeForm.range" />
    </el-form-item>
    <el-form-item label="奖品名称" prop="name">
      <el-input v-model="prizeForm.name" />
    </el-form-item>
    <el-form-item label="奖品图像" prop="img">
      <el-upload
          class="avatar-uploader"
          :on-change="handleImgChange"
          list-type="picture-card"
          action="#"
          :limit="1"
          :auto-upload="false"
      >
        <el-icon><Plus /></el-icon>
      </el-upload>
    </el-form-item>
    <el-form-item label="奖品总数" prop="total">
      <el-input v-model.number="prizeForm.total" type="number"/>
    </el-form-item>
    <el-form-item label="当前已抽数" prop="count">
      <el-input v-model.number="prizeForm.count" type="number" :disabled="true"/>
    </el-form-item>
    <el-form-item label="单次抽奖数" prop="perDraw">
      <el-input v-model.number="prizeForm.perDraw" type="number"/>
    </el-form-item>
    <el-form-item>
      <el-button type="primary" @click="createPrize(prizeFormRef)">创建</el-button>
      <el-button @click="cancelFn">取消</el-button>
    </el-form-item>
  </el-form>
</template>

<script setup lang="ts">
import { defineEmits,reactive,ref } from 'vue';
import {invoke} from "@tauri-apps/api/tauri";
import {ElMessage, FormInstance} from "element-plus";
import { Plus } from '@element-plus/icons-vue'
import {Prize, TauriResponse} from "@/types";

const emit = defineEmits(['close', "ok"]);

const selectedImg = ref<File | null>(null);
const prizeFormRef = ref<FormInstance | null>(null);

const prizeForm = ref<Prize>({
  id: 0,
  range: '',
  name: '',
  img: '',
  count: 0,
  total: 0,
  perDraw: 0
})

// 表单验证的规则
const rules = reactive({
  "range": [
    { required: true, message: '请输入奖品等级', trigger: 'blur' }
  ],
  "name": [
    { required: true, message: '请输入奖品名称', trigger: 'blur' }
  ],
  "img":  [
    { required: true, message: '请上传图片', trigger: 'blur' }
  ],
  "total": [
    { required: true, message: '请输入奖品总数', trigger: 'blur' },
    { type: 'number', message: 'must be a number' }
  ],
  "perDraw":  [
    { required: true, message: '每次抽奖个数', trigger: 'blur' },
    { type: 'number', message: 'must be a number' }
  ],
})

const createPrize= (prizeFormRef :FormInstance | null) => {
  if (!prizeFormRef) {
    return;
  }
  prizeFormRef.validate(async (isOk: boolean) => {
    if (isOk) {
      try {
        const arrayBuffer = await selectedImg.value?.arrayBuffer();
        if(arrayBuffer) {
          let uint8Array = Array.from(new Uint8Array(arrayBuffer));
          await invoke('create_prize', {prize: prizeForm.value, imgData: uint8Array});
          emit('ok')
          ElMessage.success('奖品信息录入成功');
        }
      } catch (error) {
        console.log(error)
        ElMessage.error('奖品信息录入失败');
      }
    }
  })
}

const handleImgChange = async (file: any) => {
  selectedImg.value = file.raw;
  let res = await invoke('generate_imgstr', {fileName: file.name}) as TauriResponse<string>;
  prizeForm.value.img = res.data ?? '';
}

const cancelFn = () => {
  emit('close');
}

</script>

<style scoped>

</style>