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

  <div id="container" ref="containerRef"></div>

  <div id="menu">
    <button id="enter" :class="{ none: !(drawStatus===DRAW_STATUS.TABLE) }" @click="handleEnter">进入抽奖</button>
    <button class="lottery"  :class="{ none: !(drawStatus===DRAW_STATUS.READY || drawStatus===DRAW_STATUS.DOWN)}" @click="drawStatus = DRAW_STATUS.DRAWING">开始抽奖</button>
    <button class="lottery"  :class="{ none: !(drawStatus===DRAW_STATUS.DRAWING) }" @click="drawStatus = DRAW_STATUS.DOWN">结束抽奖</button>
  </div>
</template>

<script setup lang="ts">
import {onBeforeUnmount, onMounted, ref, watch} from 'vue';
import {CSS3DObject, CSS3DRenderer} from 'three/addons/renderers/CSS3DRenderer.js';
import {Tween} from '@tweenjs/tween.js';
import {CreateHeartHighlight, RemoveHighlight} from "@/hooks/useMatrix";
import {CardInfo, COLUMN_COUNT, DRAW_STATUS, Prize, ROW_COUNT, Target, WIDTH} from "@/types";
import {
  Animate,
  Create3D,
  GetRender,
  InitCards,
  ResetCard,
  ResieWindow,
  RotateBall,
  SelectCard,
  ShineCard,
  Transform
} from "@/hooks/useCard";
import {Euler} from "three";
import {GetLuckNum} from "@/utils/lottery";

const drawStatus = ref("");

const allCardsArray: Array<CardInfo> = [{"num":"000010","name": "伊万1", "img": "123"},{"num":"000011","name": "伊万2", "img": "123"}, {"num":"000012","name": "伊万3", "img": "123"},
  {"num":"000013","name": "伊万4", "img": "123"}, {"num":"000014","name": "伊万5", "img": "123"}, {"num":"000015","name": "伊万6", "img": "123"},
  {"num":"000016","name": "伊万7", "img": "123"}, {"num":"000017","name": "伊万8", "img": "123"}, {"num":"000018","name": "伊万9", "img": "123"},
  {"num":"000019","name": "伊万10", "img": "123"}, {"num":"000020","name": "伊万11", "img": "123"},
  {"num":"000010","name": "伊万1", "img": "123"},{"num":"000011","name": "伊万2", "img": "123"}, {"num":"000012","name": "伊万3", "img": "123"},
  {"num":"000013","name": "伊万4", "img": "123"}, {"num":"000014","name": "伊万5", "img": "123"}, {"num":"000015","name": "伊万6", "img": "123"},
  {"num":"000016","name": "伊万7", "img": "123"}, {"num":"000017","name": "伊万8", "img": "123"}, {"num":"000018","name": "伊万9", "img": "123"},
  {"num":"000019","name": "伊万10", "img": "123"}, {"num":"000020","name": "伊万11", "img": "123"},
  {"num":"000019","name": "伊万10", "img": "123"}, {"num":"000020","name": "伊万11", "img": "123"},
  {"num":"000019","name": "伊万10", "img": "123"}, {"num":"000020","name": "伊万11", "img": "123"},
  {"num":"000019","name": "伊万10", "img": "123"}, {"num":"000020","name": "伊万11", "img": "123"},
  {"num":"000019","name": "伊万10", "img": "123"}, {"num":"000020","name": "伊万11", "img": "123"},
  {"num":"000019","name": "伊万10", "img": "123"}, {"num":"000020","name": "伊万11", "img": "123"},
  {"num":"000019","name": "伊万10", "img": "123"}, {"num":"000020","name": "伊万11", "img": "123"},
  {"num":"000019","name": "伊万10", "img": "123"}, {"num":"000020","name": "伊万11", "img": "123"},
  {"num":"000019","name": "伊万10", "img": "123"}, {"num":"000020","name": "伊万11", "img": "123"},
  {"num":"000019","name": "伊万10", "img": "123"}, {"num":"000020","name": "伊万11", "img": "123"}];
const containerRef = ref<HTMLElement | null>(null);
let excludeIndex: Array<number> = [];
const tempSelectedIndex = ref<Array<number>>([]);
let intervalId: NodeJS.Timeout;
let threeDCards: Array<CSS3DObject> = [];
let targets: Target = { table: [], sphere: [] };
let rotateObj: Tween<Euler>;
let animationFrameId: number | null = null;
let renderer: CSS3DRenderer;

const prizesArray = ref<Array<Prize>>([])
const selectedPrize = ref<Prize | null>(null);

onMounted(() => {
  init();
  window.addEventListener("resize", ResieWindow, false);
  bindCloseBtn();
  fetchPrizes();
});

onBeforeUnmount(() => {
  console.log('onBeforeUnmount')
  if(animationFrameId){
    cancelAnimationFrame(animationFrameId);
    animationFrameId=null;
  }
  if (containerRef.value && renderer) {
    containerRef.value.removeChild(renderer.domElement);
  }
  if (intervalId) {clearInterval(intervalId);}
  window.onresize = null;
});

// Initialization function
const init = ():void => {

  document.body.style.height = "100%";
  document.body.style.background = "linear-gradient(to bottom, #131313 0%, #02101c 100%)";
  document.body.style.margin = "0";
  document.body.style.fontFamily = "Helvetica, sans-serif";
  document.body.style.overflow = "hidden";

  let isBold = false
  let index = 0
  let highlightCell = CreateHeartHighlight();
  let position = {
    x: (WIDTH * COLUMN_COUNT - 20) / 2,
    y: (180 * ROW_COUNT - 20) / 2
  };

  Create3D();
  renderer = GetRender();
  // 创建targets, threeDCards
  ({targets, threeDCards} = InitCards(isBold, index, position, allCardsArray, highlightCell));

  if (containerRef.value) containerRef.value.appendChild(renderer.domElement);
  Animate();
  intervalId = ShineCard(threeDCards, allCardsArray, tempSelectedIndex, drawStatus.value);
  drawStatus.value = DRAW_STATUS.TABLE;

}

watch(drawStatus, (newVal, oldVal) => {
  console.log("oldVal:", oldVal, "showTable updated to: ", newVal); // 当 showTable 更新时，这里会被调用
  if(newVal === DRAW_STATUS.DRAWING) {
    rotateBall();
  } else if (newVal === DRAW_STATUS.DOWN) {
    stopBall();
  } else if (newVal === DRAW_STATUS.TABLE){
    showTable();
  } else if (newVal === DRAW_STATUS.READY){
    showSphere();
  }
});


const rotateBall = ()=> {
  ResetCard(targets, excludeIndex, threeDCards).then(() => {
    RotateBall((tween) => {
      rotateObj = tween; // 在这里设置全局变量rotateObj
    }).then(() => {
      console.log('Rotation complete');
      drawStatus.value = DRAW_STATUS.DOWN;
    }).catch(error => {
      console.error(error);
    });
  });
}

const stopBall = () => {
  rotateObj?.stop();
  console.log(allCardsArray)
  console.log(excludeIndex)
  if((allCardsArray.length - excludeIndex.length) < 10) {
    console.log("抽奖人员不足，抽奖结束")
    return
  }
  tempSelectedIndex.value = GetLuckNum(allCardsArray, excludeIndex, 10);
  excludeIndex = excludeIndex.concat(tempSelectedIndex.value)
  SelectCard(threeDCards, tempSelectedIndex.value, allCardsArray)
}

const showTable = () => {
  Transform("table", targets, threeDCards);
}

const showSphere = () => {
  Transform("sphere", targets, threeDCards);
}

const bindCloseBtn = ()=> {
  document.getElementById('container')?.addEventListener('pointerdown', function(event) {
    // 检查是否点击了具有 "card-close-btn" 类的元素
    let target = event.target as HTMLElement;
    if (target.className === 'card-close-btn') {
      // 向上遍历DOM树找到包含卡片详细信息的元素
      let cardElement = target.closest('.element') as HTMLElement | null;
      if (cardElement) {
        // 假设details元素直接包含card.num值
        let id = parseInt(cardElement.id.split("-")[1])
        //如果点击的卡片是选择的卡片之外的卡片，则直接返回
        let n = tempSelectedIndex.value.indexOf(id)
        if (n ===-1) {
          return
        }
        //将删除的卡片放回原来位置
        ResetCard(targets, [id], threeDCards);
        // 如果排除的卡片不再tmp_exclude中则将id加入tmp_exclude中
        if (excludeIndex.indexOf(id) === -1)  {
          excludeIndex.push(id)
        }
        //重新抽取卡片
        let luckIndexArray = GetLuckNum(allCardsArray, excludeIndex,  1)
        let luckNewCardIndex = luckIndexArray[0]
        //将新抽取的卡片索引加入tmp_exclude
        excludeIndex.push(luckNewCardIndex)
        //计算当前卡片的x的坐标,y轴的坐标
        tempSelectedIndex.value[n] = luckNewCardIndex
        let x = -280
        let y = 0;
        if (tempSelectedIndex.value.length > 5){
          x = x+WIDTH*(n%5)
          y = Math.floor(n/5)<=0? -87: 87;
        }
        SelectCard(threeDCards, luckIndexArray, allCardsArray, [{x:x,y:y}])
      }
    }
  });
}

// 进入抽奖
const handleEnter = (): void => {
  RemoveHighlight()
  drawStatus.value = DRAW_STATUS.READY
}


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
