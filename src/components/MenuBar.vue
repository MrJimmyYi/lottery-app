<template>

  <div id="menu">
    <button id="enter" :class="{ none: !showTable }" @click="handleEnter">进入抽奖</button>
    <div id="lotteryBar" :class="{none: showTable }" >
      <button id="lottery" @click="handleLottery">{{ lotteryBtn }}</button>
<!--      <div class="fixed-bar">-->
<!--        <button id="result" class="fixed-btn" @click="showResult">查看获奖结果</button>-->
<!--        <button id="save" class="fixed-btn" @click="handleSave">导出抽奖结果</button>-->
<!--        <button id="rules" class="fixed-btn" @click="setRule">设置抽奖规则</button>-->
<!--      </div>-->
    </div>
  </div>
</template>
<script setup lang="ts">
import {RemoveHighlight} from "@/hooks/useMatrix";
import {defineProps, onMounted, ref} from "vue";
import {bus} from "@/utils/eventbus";

const props = defineProps<{
  showTable: boolean;
}>();
const emit = defineEmits(['update:showTable']);

const lotteryBtn = ref("开始抽奖")
const isLotting = ref(false)

onMounted(() => {
  //更新props.showTable的值
  emit("update:showTable", props.showTable)
})


// 进入抽奖
const handleEnter = (): void => {
  RemoveHighlight()
  emit("update:showTable", false)
}

const handleLottery = () => {
  if (isLotting.value) {
    lotteryBtn.value = "开始抽奖"
  } else {
    lotteryBtn.value = "结束抽奖"
  }
  isLotting.value = !isLotting.value;
  bus.emit("Drawing", isLotting.value)
}
</script>

<style scoped>
  /* Your styles here */
</style>