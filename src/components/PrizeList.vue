<template>
  <div id="prizeBar">
    <div class="prize-mess" >正在抽取
      <span v-if="selectedPrize">
        <label id="prizeType" class="prize-shine">{{selectedPrize.range}}</label>
        <label id="prizeText" class="prize-shine">{{selectedPrize.title}}</label>， 剩余<label id="prizeLeft" class="prize-shine">{{selectedPrize.total-selectedPrize.count}}</label>个
      </span>
    </div>
    <ul class="prize-list">
      <li v-for="prize in prizesArray" :key="prize.id"
          :class="{ 'prize-item': true, 'shine': selectedPrize?selectedPrize.id === prize.id: false }"
          @click="updatePrizeInfo(prize)">
        <span></span><span></span><span></span><span></span>
        <div class="prize-img">
          <img :src="prize.imgSrc" :alt="prize.altText">
        </div>
        <div class="prize-text">
          <h5 class="prize-title">{{ prize.title }}</h5>
          <div class="prize-count">
            <div class="progress">
              <div :id="`prize-bar-${prize.id}`" class="progress-bar progress-bar-danger progress-bar-striped active" :style="{ width: (prize.count*100/prize.total).toString()+'%' }"></div>
            </div>
            <div :id="`prize-count-${prize.id}`" class="prize-count-left">{{ prize.count }}/{{ prize.total }}</div>
          </div>
        </div>
      </li>
    </ul>
  </div>

</template>

<script setup lang="ts">
import {onMounted, ref} from "vue";
import {Prize} from "@/types";
const prizesArray = ref<Array<Prize>>([])
const selectedPrize = ref<Prize | null>(null);

onMounted(()=>{
  fetchPrizes()
})

const fetchPrizes = () => {
  prizesArray.value = [
    {
      "id": 1,
      "range": "特等奖",
      "title": "神秘大礼",
      "imgSrc": "",
      "altText": "神秘大礼",
      "count": 1,
      "total": 2,
      "perCount": 2,
    },
    {
      "id": 2,
      "range": "一等奖",
      "title": "Mac Pro",
      "imgSrc": "",
      "altText": "Mac Pro",
      "count": 0,
      "perCount": 2,
      "total": 5
    },
    {
      "id": 3,
      "range": "二等奖",
      "title": "Book2",
      "imgSrc": "",
      "altText": "Book2",
      "count": 1,
      "total": 2,
      "perCount": 2,
    },
    {
      "id": 4,
      "range": "三等奖",
      "title": "Book3",
      "imgSrc": "",
      "altText": "Book3",
      "count": 1,
      "perCount": 2,
      "total": 2,
    },
    {
      "id": 5,
      "range": "四等奖",
      "title": "Book3",
      "imgSrc": "",
      "altText": "Book3",
      "count": 1,
      "perCount": 2,
      "total": 2,
    },
    {
      "id": 6,
      "range": "五等奖",
      "title": "Book4",
      "imgSrc": "",
      "altText": "Book4",
      "count": 1,
      "perCount": 2,
      "total": 2,
    },
    {
      "id": 7,
      "range": "六等奖",
      "title": "Book5",
      "imgSrc": "",
      "altText": "Book5",
      "count": 1,
      "perCount": 2,
      "total": 16,
    },
  ]
}

const updatePrizeInfo = (prize: Prize) => {
  selectedPrize.value = prize
}

</script>

<style scoped>

</style>