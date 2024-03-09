<template>
  <el-form
      ref="userFormRef"
      style="max-width: 600px"
      :model="userForm"
      label-width="auto"
      class="demo-ruleForm"
      :rules="rules"
  >
    <el-form-item label="编号" prop="num">
      <el-input v-model="userForm.num" />
    </el-form-item>
    <el-form-item label="名称" prop="name">
      <el-input v-model="userForm.name" />
    </el-form-item>
    <el-form-item label="图像" prop="img">
      <el-upload
          class="avatar-uploader"
          :on-change="handleImgChange"
          list-type="picture-card"
          action="#"
          :limit="1"
          :auto-upload="false"
          :file-list="fileList"
      >
        <el-icon><Plus /></el-icon>
      </el-upload>
    </el-form-item>
    <el-form-item label="备注1" prop="remark1">
      <el-input v-model="userForm.remark1" />
    </el-form-item>
    <el-form-item label="备注2" prop="remark2">
      <el-input v-model="userForm.remark2" />
    </el-form-item>
    <el-form-item>
      <el-button type="primary" @click="updateUser(userFormRef)">创建</el-button>
      <el-button @click="cancelFn">取消</el-button>
    </el-form-item>
  </el-form>
</template>

<script setup lang="ts">
import {defineEmits, reactive, ref, onMounted, defineProps} from "vue";
import {ElMessage, FormInstance, UploadUserFile} from "element-plus";
import {UserCard} from "@/types";
import {convertFileSrc, invoke} from "@tauri-apps/api/tauri";
import { Plus } from '@element-plus/icons-vue'

const emit = defineEmits(['close', "ok"]);

const selectedImg = ref<File | null>(null);
const userFormRef = ref<FormInstance | null>(null);

const fileList = ref<UploadUserFile[]>([])
const userForm = ref<UserCard>({
  id: 0,
  name: '',
  num: '',
  remark1: '',
  remark2: '',
  img: ''
})

// 使用 defineProps 定义接收的 props
const props = defineProps({
  id: {
    type: Number,
    required: true,
  },
});

// 表单验证的规则
const rules = reactive({
  "name": [
    { required: true, message: '请输入名车个', trigger: 'blur' }
  ],
  "num": [
    { required: true, message: '请输入编号', trigger: 'blur' }
  ],
  "img":  [
    { required: true, message: '请上传图片', trigger: 'blur' }
  ],
})

const handleImgChange = async (file: any) => {
  selectedImg.value = file.raw;
  const imgStr = await invoke('generate_imgstr', {fileName: file.name}) as string;
  userForm.value.img = imgStr;
}


const cancelFn = () => {
  emit('close');
}

onMounted(async ()=>{
  userForm.value = await invoke('get_user', {id: props.id})
  let fileName = userForm.value.img.split(/[/\\]/).pop();
  if(fileName) {
    const assetUrl = convertFileSrc(userForm.value.img);
    fileList.value.push({url:assetUrl, name:fileName})
  }
})

const updateUser = (userFormRef :FormInstance | null) => {
  if (!userFormRef) {
    return;
  }
  userFormRef.validate(async (isOk: boolean) => {
    if (isOk) {
      try {
        const arrayBuffer = await selectedImg.value?.arrayBuffer();
        if(arrayBuffer) {
          let uint8Array = Array.from(new Uint8Array(arrayBuffer));
          await invoke('update_user', {user: userForm.value, imgData: uint8Array});
          emit('ok')
          ElMessage.success('用户信息更新成功');
        }
      } catch (error) {
        console.log(error)
        ElMessage.error('用户信息更新失败');
      }
    }
  })
}

</script>

<style scoped>

</style>