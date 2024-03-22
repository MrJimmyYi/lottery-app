<template>
  <el-container style="height:  100vh">
    <el-header style="height: 5%">
      <el-button @click="addUser">添加</el-button>
      <el-button @click="batchOperate">批量操作</el-button>
    </el-header>
    <el-divider />
  <el-main style="height: 90%">
    <el-table :data="tableData" style="width: 100%;height: 100%" >
      <el-table-column prop="id"   v-if="false" />
      <el-table-column label="序号" type="index" width="100" :index="getIndex"/>
      <el-table-column prop="num" label="编号" width="100" />
      <el-table-column prop="name" label="名称" width="100" />
      <el-table-column  label="图片" width="120" >
        <template #default="scope">
          <div style="display: flex; align-items: center">
            <el-image style="width: 100px; height: 100px" :src="scope.row.img" fit="scale-down"/>
          </div>
        </template>
      </el-table-column>
      <el-table-column prop="remark" label="备注" width="100" />
      <el-table-column fixed="right" label="操作" width="120">
        <template #default="scope">
          <el-button link type="primary" size="small" @click="showEdit(scope.row.id)">
            编辑
          </el-button>
          <el-button link type="primary" size="small" @click.prevent="deleteUser(scope.row.id)">
            删除
          </el-button>
        </template>
      </el-table-column>
    </el-table>

    <el-dialog v-model="batchDialogVisible" title="批量操作" :before-close="handleClose" :destroy-on-close=true>
      <BatchOper @import="importFn"></BatchOper>
    </el-dialog>

    <el-dialog v-model="createDialogVisible" title="添加用户" :before-close="handleClose" :destroy-on-close=true>
      <UserAdd  @close="closeFn" @ok="okFn"></UserAdd>
    </el-dialog>

    <el-dialog v-model="editDialogVisible" title="修改用户"  :before-close="handleClose" :destroy-on-close=true>
      <UserEdit :id="editId" @ok="editOkFn" @close="editCloseFn" ></UserEdit>
    </el-dialog>
  </el-main>
    <el-footer style="height: 5%">
      <div class="example-pagination-block">
        <el-pagination
            layout="prev, pager, next"
            @current-change="handleCurrentChange"
            @size-change="handleSizeChange"
           :current-page="currentPage"
           :page-size="pageSize"
           :total="totalItems"
        />
      </div>
    </el-footer>
  </el-container>

</template>

<script setup lang="ts">
import {onMounted, ref} from "vue";
import {UserCard, TauriResponse, PageData} from "@/types";
import {invoke} from "@tauri-apps/api/tauri";
import {ElMessage, ElMessageBox} from "element-plus";
import {convertFileSrc} from "@tauri-apps/api/tauri";
import UserAdd from "@/views/ManageUser/add.vue";
import UserEdit from "@/views/ManageUser/edit.vue";
import BatchOper from "@/views/ManageUser/batchOper.vue";

const tableData = ref<UserCard[]>([])
const createDialogVisible = ref(false);
const editDialogVisible = ref(false);
const batchDialogVisible = ref(false);

// 当前页
const currentPage = ref(1);
// 每页条目数
const pageSize = ref(20);
// 总条目数，初始值可以设置为0，实际值应从后端获取
const totalItems = ref(0);

const getIndex = (index: number) => {
  return index + (currentPage.value-1)*(pageSize.value)+1;
}

const importFn = () => {
  batchDialogVisible.value = false
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
// 显示添加页面
const addUser= ()=>{
  createDialogVisible.value=true;
}


const closeFn=()=>{
  createDialogVisible.value = false;
}

const batchOperate = ()=> {
  batchDialogVisible.value = true
}

const okFn=()=>{
  closeFn();
  fetchData();
}

const editId = ref(0);
// 显示修改页面
const showEdit = (id: number)=>{
  editId.value = id;
  editDialogVisible.value=true;
}

const editCloseFn=()=>{
  editId.value = 0;
  editDialogVisible.value = false;
}

const editOkFn=()=>{
  editCloseFn();
  fetchData();
}

const deleteUser = async (id: number) => {
  try {
    await invoke('delete_user', {
      id: id,
    });
    await fetchData();
    ElMessage.success('选中用户信息删除成功');
  } catch (error) {
    ElMessage.error('选中用户信息删除失败');
  }
}

onMounted(()=>{
  fetchData();
})

const fetchData = async () => {
  try {
    // 调用后端接口获取分页数据，传递当前页和每页条目数
    let res = await invoke('get_page_users', {
      page: currentPage.value,
      pageSize: pageSize.value
    }) as TauriResponse<PageData<UserCard>>;

    if(res.data) {
      let data = res.data
      for(let e of data.data ) {
        if (e.img) {
          const assetUrl = convertFileSrc(e.img);
          e.img = assetUrl;
        }
      }
      // 更新表格数据
      tableData.value = data.data ?? [];
      // 更新总条目数
      totalItems.value = <number>data.total;
    }
  } catch (error) {
    console.error('Failed to fetch paged users:', error);
  }
};

const handleCurrentChange = (newPage: number) => {
  currentPage.value = newPage;
  fetchData();
};

const handleSizeChange = (newSize: number) => {
  pageSize.value = newSize;
  fetchData();
};

</script>

<style scoped>

</style>