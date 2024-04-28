<template>
  <el-container style="height:  100vh">
    <el-divider />
    <el-main style="height: 90%">
      <el-table :data="tableData" style="width: 100%;height: 100%" >
        <el-table-column prop="id"   v-if="false" />
        <el-table-column label="Id" type="index" width="100" :index="getIndex"/>
        <el-table-column prop="prizeName" label="Prize Name" width="100" />
        <el-table-column prop="prizeRange" label="Prize Range" width="100" />
        <el-table-column prop="winnerNum" label="User Num" width="100" />
        <el-table-column prop="winnerName" label="User Name" width="100" />
      </el-table>
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
import {invoke} from "@tauri-apps/api/tauri";
import {PageData, TauriResponse, Winner} from "@/types";

const tableData = ref<Winner[]>([])


// 当前页
const currentPage = ref(1);
// 每页条目数
const pageSize = ref(20);
// 总条目数，初始值可以设置为0，实际值应从后端获取
const totalItems = ref(0);

const getIndex = (index: number) => {
  return index + (currentPage.value-1)*(pageSize.value)+1;
}


onMounted(()=>{
  fetchData();
})

const fetchData = async () => {
  try {
    // 调用后端接口获取分页数据，传递当前页和每页条目数
    let res = await invoke('get_page_winners', {
      page: currentPage.value,
      pageSize: pageSize.value
    }) as TauriResponse<PageData<Winner>>;

    if(res.data) {
      let data = res.data
      // 更新表格数据
      tableData.value = data.data ?? []
      // 更新总条目数
      totalItems.value = <number>data.total
    }

  } catch (error) {
    console.error('Failed to fetch paged winners:', error);
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