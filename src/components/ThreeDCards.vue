<template>
  <div id="container" ref="containerRef"></div>

  <div id="menu">
    <button id="enter" :class="{ none: !(drawStatus===DRAW_STATUS.READY) }" @click="handleEnter">进入抽奖</button>
    <button class="lottery"  :class="{ none: !(drawStatus===DRAW_STATUS.DOWN) }" @click="drawing(DRAW_STATUS.DOWN)">开始抽奖</button>
    <button class="lottery"  :class="{ none: !(drawStatus===DRAW_STATUS.DRAWING) }" @click="drawing(DRAW_STATUS.DRAWING)">结束抽奖</button>
  </div>
</template>

<script setup lang="ts">
import {onMounted, onBeforeUnmount, ref} from 'vue';
import { CSS3DRenderer, CSS3DObject } from 'three/addons/renderers/CSS3DRenderer.js';
import {Tween} from '@tweenjs/tween.js';
import {CreateHeartHighlight, RemoveHighlight} from "@/hooks/useMatrix";
import {CardInfo, COLUMN_COUNT, DRAW_STATUS, ROW_COUNT, Target, WIDTH} from "@/types";
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

const drawStatus = ref(DRAW_STATUS.READY);

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

onMounted(() => {
  init();
  window.addEventListener("resize", ResieWindow, false);
  bindCloseBtn();
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
  transTableOrSphere(drawStatus.value);
  Animate();
  intervalId = ShineCard(threeDCards, allCardsArray, tempSelectedIndex, drawStatus.value);
}

const drawing = (status: string) => {
  if (status===DRAW_STATUS.DOWN) {
     ResetCard(targets, excludeIndex, threeDCards).then(() => {
       drawStatus.value = DRAW_STATUS.DRAWING;
        RotateBall((tween) => {
          rotateObj = tween; // 在这里设置全局变量rotateObj
        }).then(() => {
          console.log('Rotation complete');
          drawStatus.value = DRAW_STATUS.DOWN;
        }).catch(error => {
          console.error(error);
        });
    });
  } else if (status===DRAW_STATUS.DRAWING){
    rotateObj?.stop();
    if(allCardsArray.length === excludeIndex.length ) {
      console.log("抽奖人员全部抽取完毕")
      return
    }
    tempSelectedIndex.value = GetLuckNum(allCardsArray, excludeIndex, 10);
    excludeIndex = excludeIndex.concat(tempSelectedIndex.value)
    SelectCard(threeDCards, tempSelectedIndex.value, allCardsArray)
  }
}

const transTableOrSphere = (drawStatusProp:string):void => {
  if(drawStatusProp===DRAW_STATUS.READY){
    Transform("table", targets, threeDCards);
  } else if (drawStatusProp===DRAW_STATUS.DOWN){
    Transform("sphere", targets, threeDCards);
  }
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
  transTableOrSphere(DRAW_STATUS.DOWN)
  drawStatus.value = DRAW_STATUS.DOWN
  transTableOrSphere(DRAW_STATUS.DOWN)
}

</script>

<style scoped>
/* Your styles here */
</style>
