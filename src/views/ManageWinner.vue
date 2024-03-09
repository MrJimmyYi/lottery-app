<template>
  <el-table :data="tableData" border style="width: 100%">
    <el-table-column prop="range" label="奖品等级" width="180" />
    <el-table-column prop="name" label="奖品名称" width="180" />
    <el-table-column prop="winners" label="中奖名单" />
  </el-table>
  <el-button>导出结果</el-button>
</template>

<script setup lang="ts">
const tableData = [
  {
    range: '2016-05-03',
    name: 'Tom',
    winners: 'No. 189, Grove St, Los Angeles',
  },
  {
    range: '2016-05-02',
    name: 'Tom',
    winners: 'No. 189, Grove St, Los Angeles',
  },
  {
    range: '2016-05-04',
    name: 'Tom',
    winners: 'No. 189, Grove St, Los Angeles',
  },
  {
    range: '2016-05-01',
    name: 'Tom',
    winners: 'No. 189, Grove St, Los Angeles',
  },
]
</script>

<style scoped>

</style>