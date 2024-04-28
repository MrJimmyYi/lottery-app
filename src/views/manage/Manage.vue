<template>
  <el-container class="layout-container-demo" style="height: 100%">
    <el-aside width="200px">
      <el-scrollbar>
        <el-menu :default-openeds="['1', '2']">
          <el-sub-menu index="1">
            <template #title>
              <el-icon><setting /></el-icon>数据管理
            </template>
            <el-menu-item index="1-1">
              <RouterLink to="/pageManage/user">人员清单</RouterLink>
            </el-menu-item>
            <el-menu-item index="1-2">
              <RouterLink to="/pageManage/prize">抽奖设置</RouterLink>
            </el-menu-item>
          </el-sub-menu>

          <el-sub-menu index="2">
            <template #title>
              <el-icon><icon-menu /></el-icon>模式设置
            </template>
            <el-menu-item-group>
              <el-menu-item index="2-1">
                <RouterLink to="/pageManage/wedding">婚礼模式</RouterLink>
              </el-menu-item>

            </el-menu-item-group>
          </el-sub-menu>

        </el-menu>
      </el-scrollbar>
    </el-aside>

    <el-container>
      <el-main>
        <RouterView></RouterView>
      </el-main>
    </el-container>
  </el-container>
</template>

<script lang="ts" setup>
import { Menu as IconMenu, Setting } from '@element-plus/icons-vue'
</script>

<style scoped>
.layout-container-demo .el-header {
  position: relative;
  background-color: var(--el-color-primary-light-7);
  color: var(--el-text-color-primary);
}
.layout-container-demo .el-aside {
  color: var(--el-text-color-primary);
  background: var(--el-color-primary-light-8);
}
.layout-container-demo .el-menu {
  border-right: none;
}
.layout-container-demo .el-main {
  padding: 0;
}
.layout-container-demo .toolbar {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  height: 100%;
  right: 20px;
}
</style>
