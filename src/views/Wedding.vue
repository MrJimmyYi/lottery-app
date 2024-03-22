<template>
  <div id="prizeBar">
    <div class="prize-mess" >正在抽取
      <span v-if="selectedPrize">
        <label id="prizeType" class="prize-shine">{{selectedPrize.range}}</label>
        <label id="prizeText" class="prize-shine">{{selectedPrize.name}}</label>， 剩余<label id="prizeLeft" class="prize-shine">{{selectedPrize.total-selectedPrize.count}}</label>个
      </span>
    </div>
    <ul class="prize-list">
      <li v-for="prize in prizesArray" :key="prize.id"
          :class="{ 'prize-item': true, 'shine': selectedPrize?selectedPrize.id === prize.id: false }"
          @click="updatePrizeInfo(prize)">
        <span></span><span></span><span></span><span></span>
        <div class="prize-img">
          <img :src="prize.img" >
        </div>
        <div class="prize-text">
          <h5 class="prize-title">{{ prize.name }}</h5>
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

  <div id="cards-container" ref="containerRef"></div>

  <div id="menu">
    <button id="enter" :class="{ none: !(drawStatus===DRAW_STATUS.ENTER) }" @click="handleToDraw">进入抽奖</button>
    <button class="lottery"  :class="{ none: !(drawStatus===DRAW_STATUS.READY )}" @click="startDrawing">开始抽奖</button>
    <button class="lottery"  :class="{ none: !(drawStatus===DRAW_STATUS.DRAWING) }" >正在抽奖</button>
    <button class="lottery"  :class="{ none: !(drawStatus===DRAW_STATUS.DOWN) }" @click="stopDrawing">结束抽奖</button>
  </div>

</template>

<script setup lang="ts">
import {onBeforeUnmount, onMounted, ref, watch} from 'vue';
import {CSS3DObject, CSS3DRenderer} from 'three/addons/renderers/CSS3DRenderer.js';
import {Tween} from '@tweenjs/tween.js';
import {CreateHeartHighlight, RemoveHighlight} from "@/hooks/Matrix";
import {COLUMN_COUNT, DRAW_STATUS, Prize, ROW_COUNT, Target, TauriResponse, UserCard, WIDTH} from "@/types";
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
} from "@/hooks/userCard";
import {Euler} from "three";
import {GetLuckNum} from "@/utils/lottery";
import {convertFileSrc, invoke} from "@tauri-apps/api/tauri";

const drawStatus = ref("");

const allCardsArray = ref<Array<UserCard>>([]);
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

onMounted( async () => {
  fetchPrizes();
  await fetchUserCards();
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
  ({targets, threeDCards} = InitCards(isBold, index, position, allCardsArray.value, highlightCell));
  if (containerRef.value) containerRef.value.appendChild(renderer.domElement);
  Animate();
  drawStatus.value = DRAW_STATUS.ENTER;
  intervalId = ShineCard(threeDCards, allCardsArray.value, tempSelectedIndex, drawStatus.value);
}

watch(drawStatus, (newVal, oldVal) => {
  console.log("oldVal:", oldVal, "showTable updated to: ", newVal); // 当 showTable 更新时，这里会被调用
  if(newVal === DRAW_STATUS.DRAWING) {
    rotateBall();
  } else if (newVal === DRAW_STATUS.DOWN) {
    stopBall();
  } else if (newVal === DRAW_STATUS.ENTER){
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
  if((allCardsArray.value.length - excludeIndex.length) < 10) {
    console.log("抽奖人员不足，抽奖结束")
    return
  }
  const p = selectedPrize.value
  if (p) {
    console.log("+++++")
    console.log(p.perDraw)
    tempSelectedIndex.value = GetLuckNum(allCardsArray.value, excludeIndex, p.perDraw ?? 0);
    // updatePrizeCount(p?.id)
    // fetchPrizes()
    console.log(tempSelectedIndex.value)
    excludeIndex = excludeIndex.concat(tempSelectedIndex.value)
    SelectCard(threeDCards, tempSelectedIndex.value, allCardsArray.value)
  }
}


// const updatePrizeCount = async (id: number) => {
//   try {
//     await invoke('update_prize_count', {
//       id: id
//     });
//   } catch (error) {
//
//   }
// }

const showTable = () => {
  Transform("table", targets, threeDCards);
}

const showSphere = () => {
  Transform("sphere", targets, threeDCards);
}

const bindCloseBtn = ()=> {
  document.getElementById('cards-container')?.addEventListener('pointerdown', function(event) {
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
        let luckIndexArray = GetLuckNum(allCardsArray.value, excludeIndex,  1)
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
        SelectCard(threeDCards, luckIndexArray, allCardsArray.value, [{x:x,y:y}])
      }
    }
  });
}

// 进入抽奖
const handleToDraw = (): void => {
  RemoveHighlight()
  drawStatus.value = DRAW_STATUS.READY
}

const startDrawing = (): void => {
  drawStatus.value = DRAW_STATUS.DRAWING
}

const stopDrawing = (): void => {

  drawStatus.value = DRAW_STATUS.READY
}

const fetchPrizes = async () => {
  console.log("fetchPrizes")
  try {
    // 调用后端接口获取分页数据，传递当前页和每页条目数
    let res = await invoke('get_all_prizes') as TauriResponse<Prize[]>;
    if(res.data) {
      let data = res.data
      for(let e of data) {
        if (e.img) {
          const assetUrl = convertFileSrc(e.img);
          e.img = assetUrl;
        }
      }
      prizesArray.value = data ?? []
    }

  } catch (error) {
    console.error('Failed to fetch paged users:', error);
  }
};

const fetchUserCards = async () => {
  try {
    // 调用后端接口获取分页数据，传递当前页和每页条目数
    let res = await invoke('get_all_users') as TauriResponse<UserCard[]>;
    if(res.data) {
      let data = res.data
      for(let e of data) {
        if (e.img) {
          const assetUrl = convertFileSrc(e.img);
          e.img = assetUrl;
        }
      }
      allCardsArray.value = data ?? []

    }

  } catch (error) {
    console.error('Failed to fetch paged users:', error);
  }
};

const updatePrizeInfo = (prize: Prize) => {
  selectedPrize.value = prize
}

</script>

<style >
html, body {
  height: 100%;
}

body {
  margin: 0;
  font-family: Helvetica, sans-serif;
  overflow: hidden;
}


.none {
  display: none;
}

#cards-container {
  z-index: 3;
  position: relative;
  margin: 0 15vh;
}

#menu {
  z-index: 4;
  margin-left: 15vh;
}

.canvas-box {
  position: fixed;
  left: 0;
  top: 0;
  z-index: -1;
}

canvas {
  width: 100%;
  height: 100%;
}

#info {
  position: absolute;
  width: 100%;
  color: #ffffff;
  padding: 5px;
  font-family: Monospace;
  font-size: 13px;
  font-weight: bold;
  text-align: center;
  z-index: 1;
}

#menu {
  position: absolute;
  bottom: 2vh;
  width: 100%;
  text-align: center;
}

.element {
  position: relative;
  width: 12vh;
  height: 16vh;
  box-shadow: 0 0 12px rgba(0, 255, 255, 0.5);
  border: 1px solid rgba(127, 255, 255, 0.25);
  text-align: center;
  cursor: default;
  transition: background-color 0.3s ease-in;
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
}


/* 叉叉按钮的样式 */
.element .card-close-btn {
  display: none; /* 默认不显示 */
  position: absolute;
  right: 0;
  top: 0;
  cursor: pointer;
  padding: 1px;
  background-color: red; /* 可以根据需要调整样式 */
  color: white; /* 可以根据需要调整样式 */
}

/* 当鼠标悬停在.element上时，显示叉叉按钮 */
.element:hover .card-close-btn {
  display: block;
}

.element:hover {
  box-shadow: 0 0 12px rgba(0, 255, 255, 0.75);
  border: 1px solid rgba(127, 255, 255, 0.75);
}

.element .card-num {
  position: absolute;
  top: 0;
  left: 50%;
  transform: translateX(-50%);
  width: 100%;
  font-size: 2vh;
  color: rgba(127, 255, 255, 0.75);
}

.element .card-img {
  position: absolute;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
  width: 90px; /* 或者使用 vh 单位 */
  height: 90px; /* 或者使用 vh 单位 */
}

.element .card-name {
  position: absolute;
  top: calc(50% + 45px); /* 根据 .card-img 的高度加上期望的间距调整 */
  left: 50%;
  transform: translateX(-50%);
  width: 100%;
  font-size: 1.6vh;
  color: rgba(127, 255, 255, 0.75);
}

.element .card-remark {
  position: absolute;
  top: calc(50% + 60px); /* 根据 .card-num 的位置再加上一定的间距调整 */
  left: 50%;
  transform: translateX(-50%);
  width: 100%;
  margin-top: 5px;
  font-size: 1.3vh;
  color: rgba(127, 255, 255, 0.75);
}


button {
  color: rgba(127, 255, 255, 0.75);
  background: transparent;
  outline: 1px solid rgba(127, 255, 255, 0.75);
  border: 0;
  padding: 1.6vh 4vh;
  margin: 0 4.6vh;
  font-size: 2vh;
  font-weight: bold;
  cursor: pointer;
}

button:hover {
  background-color: rgba(0, 255, 255, 0.5);
}

button:active {
  color: #000000;
  background-color: rgba(0, 255, 255, 0.75);
}

.highlight {
  background-color: rgba(253, 105, 0, 0.95) !important;
  box-shadow: 0 0 12px rgba(253, 105, 0, 0.95);
  border: 1px solid rgba(253, 105, 0, 0.25);
}

.highlight.element .name {
  text-shadow: 0 0 16px rgba(255, 255, 255, 0.95);
}

.prize.element .name {
  text-shadow: none;
}

.prize.element {
  transition: background-color 1.5s ease-in 0.3s;
  background-color: rgba(253, 105, 0, 0.85) !important;
  box-shadow: 0 0 12px rgba(253, 105, 0, 0.95);
}

.prize .card-name,
.prize .card-num,
.prize .card-img,
.highlight .card-name,
.highlight .name,
.highlight .card-img {
  color: rgba(255, 255, 255, 0.85);
}

.dan-mu {
  visibility: hidden;
  position: fixed;
  z-index: -1;
  font-size: 12px;
  top: 1vh;
  left: 0;
  padding: 0 1.2vh;
  height: 2.2vh;
  line-height: 2.2vh;
  border-radius: 1vh;
  box-sizing: border-box;
  background-color: rgba(0, 127, 127, 0.37);
  box-shadow: 0 0 4px rgba(0, 255, 255, 0.5);
  border: 1px solid rgba(127, 255, 255, 0.25);
  color: rgba(127, 255, 255, 0.75);
}

.dan-mu.active {
  visibility: visible;
}

#prizeBar {
  position: fixed;
  left: 0;
  padding-left: 1.2vh;
  top: 1.2vh;
  z-index: 2;
}

.prize-list {
  margin: 0;
  padding: 0;
  list-style: none;
}

.prize-item {
  padding: 9px;
  margin: 6px 0;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 4px;
  flex-wrap: nowrap;
  background-color: rgba(0, 127, 127, 0.37);
  border: 1px solid rgba(127, 255, 255, 0.25);
  color: rgba(127, 255, 255, 0.75);
  width: 30vh;
  height: 10vh;
  box-sizing: border-box;
  transition: transform 0.5s ease-in;
}

.prize-item .prize-img {
  width: 8vh;
  height: 8vh;
  margin-right: 1.2vh;
  border-radius: 50%;
  background-color: #fff;
  text-shadow: 0 0 1vh rgba(0, 255, 255, 0.95);
  overflow: hidden;
}

.prize-img img {
  width: 90%;
  height: 90%;
  position: relative;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
}

.prize-text {
  flex: 1;
}

.prize-title {
  margin: 4px 0;
  font-size: 1.8vh;
}

.prize-count {
  padding: 4px 0;
  position: relative;
}

.prize-count .progress {
  height: 1.8vh;
  background: rgba(0, 0, 0, 0.5);
  padding: 1px;
  overflow: visible;
  border-radius: 1vh;
}

.progress .progress-bar {
  border-radius: 1.8vh;
  position: relative;
  animation: animate-positive 2s;
  background-color: #d9534f;
  height: 1.8vh;
  -webkit-transition: width 0.6s ease;
  -o-transition: width 0.6s ease;
  transition: width 0.6s ease;
}

.progress-bar.active {
  animation: reverse progress-bar-stripes 0.4s linear infinite,
  animate-positive 2s;
}

.progress-bar-striped {
  background-image: -webkit-linear-gradient(45deg,
  rgba(255, 255, 255, 0.15) 25%,
  transparent 25%,
  transparent 50%,
  rgba(255, 255, 255, 0.15) 50%,
  rgba(255, 255, 255, 0.15) 75%,
  transparent 75%,
  transparent);
  background-image: -o-linear-gradient(45deg,
  rgba(255, 255, 255, 0.15) 25%,
  transparent 25%,
  transparent 50%,
  rgba(255, 255, 255, 0.15) 50%,
  rgba(255, 255, 255, 0.15) 75%,
  transparent 75%,
  transparent);
  background-image: linear-gradient(45deg,
  rgba(255, 255, 255, 0.15) 25%,
  transparent 25%,
  transparent 50%,
  rgba(255, 255, 255, 0.15) 50%,
  rgba(255, 255, 255, 0.15) 75%,
  transparent 75%,
  transparent);
  -webkit-background-size: 8px 8px;
  background-size: 8px 8px;
}

@-webkit-keyframes animate-positive {
  0% {
    width: 0;
  }
}

@keyframes animate-positive {
  0% {
    width: 0;
  }
}

@-webkit-keyframes progress-bar-stripes {
  from {
    background-position: 8px 0;
  }

  to {
    background-position: 0 0;
  }
}

@-o-keyframes progress-bar-stripes {
  from {
    background-position: 8px 0;
  }

  to {
    background-position: 0 0;
  }
}

@keyframes progress-bar-stripes {
  from {
    background-position: 8px 0;
  }

  to {
    background-position: 0 0;
  }
}

.prize-count-left {
  position: absolute;
  color: #fff;
  right: 9px;
  font-size: 1.8vh;
  line-height: 1.6vh;
  top: 50%;
  transform: translateY(-50%);
}

.shine {
  box-shadow: 0 0 15px 0 rgba(0, 255, 255, 0.5);
  transform: scale(1.2);
  transform-origin: left center;
  position: relative;
  overflow: hidden;
}

.done {
  position: relative;
}

.done:after {
  content: "";
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background-color: rgba(0, 0, 0, 0.5);
  cursor: not-allowed;
}


.shine span {
  position: absolute;
  display: block
}

.shine span:nth-child(1) {
  top: 0;
  left: 0;
  width: 100%;
  height: 2px;
  background: linear-gradient(90deg, transparent, #03e9f4);
  animation: animate1 1s linear infinite
}

@keyframes animate1 {
  0% {
    left: -100%
  }

  50%,
  100% {
    left: 100%
  }
}

.shine span:nth-child(2) {
  top: -100%;
  right: 0;
  width: 2px;
  height: 100%;
  background: linear-gradient(180deg, transparent, #03e9f4);
  animation: animate2 1s linear infinite;
  animation-delay: .25s
}

@keyframes animate2 {
  0% {
    top: -100%
  }

  50%,
  100% {
    top: 100%
  }
}

.shine span:nth-child(3) {
  bottom: 0;
  right: 0;
  width: 100%;
  height: 2px;
  background: linear-gradient(270deg, transparent, #03e9f4);
  animation: animate3 1s linear infinite;
  animation-delay: .50s
}

@keyframes animate3 {
  0% {
    right: -100%
  }

  50%,
  100% {
    right: 100%
  }
}

.shine span:nth-child(4) {
  bottom: -100%;
  left: 0;
  width: 2px;
  height: 100%;
  background: linear-gradient(360deg, transparent, #03e9f4);
  animation: animate4 1s linear infinite;
  animation-delay: .75s
}

@keyframes animate4 {
  0% {
    bottom: -100%
  }

  50%,
  100% {
    bottom: 100%
  }
}


.shine.prize-item {
  /* width: 24vh; */
  margin: 1.8vh 0;
}

.prize-mess {
  color: #fff;
  line-height: 5vh;
  font-size: 1.6vh;
  margin: 2.4vh 0;
}

.prize-shine {
  font-size: 5vh;
  font-weight: bold;
  color: #db5c58;
  vertical-align: middle;
  padding: 0 6px;
}

.qipao-container {
  position: fixed;
  right: 0;
  top: 10vh;
  bottom: 1vh;
  width: 24vh;
  z-index: 2;
}

.qipao {
  width: 100%;
  padding: 1.8vh 1.4vh;
  line-height: 1.414;
  margin: 4px 0;
  box-sizing: border-box;
  font-size: 14px;
  background-color: rgba(127, 255, 255, 0.25);
  color: rgba(127, 255, 255, 0.75);
}

.music {
  position: fixed;
  top: 3vh;
  right: 4vh;
  z-index: 5;
}

.music-item {
  display: block !important;
  opacity: 0;
}

.music-box {
  width: 5vh;
  height: 5vh;
  border-radius: 50%;
  text-align: center;
  line-height: 5vh;
  font-size: 1.4vh;
  color: #fff;
  cursor: pointer;
  background-color: rgba(253, 105, 0, 0.9);
  border: 1px solid rgba(255, 255, 255, 0.5);
}

.rotate-active {
  animation: rotate 4s linear infinite;
}

@keyframes rotate {
  from {
    transform: rotate(0);
  }

  to {
    transform: rotate(360deg);
  }
}

.margin-l-40 {
  margin-left: 40px;
}

.fixed-bar {
  position: fixed;
  bottom: 20px;
  right: 20px;
}

.fixed-btn {
  margin: 20px 0 0;
  width: 200px;
  text-align: center;
  display: block;
}

.lottery {
  animation: breath 1.6s linear infinite;
  box-shadow: 0px 0px 15px rgb(127 255 255 / 75%);
}

@keyframes breath {
  0% {
    transform: scale(0.9);
    opacity: 0.8;
  }

  50% {
    transform: scale(1.1);
    opacity: 1;
  }

  100% {
    transform: scale(0.9);
    opacity: 0.8;
  }
}
</style>
