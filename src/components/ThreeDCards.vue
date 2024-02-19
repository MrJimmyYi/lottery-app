<template>
  <div id="container" ref="containerRef"></div>
</template>

<script setup lang="ts">
import { onMounted, onBeforeUnmount, ref, watch, defineProps } from 'vue';
import * as THREE from 'three';
import { CSS3DRenderer, CSS3DObject } from 'three/addons/renderers/CSS3DRenderer.js';
import { TrackballControls } from 'three/addons/controls/TrackballControls.js';
import TWEEN, {Tween} from '@tweenjs/tween.js';
import {CreateHeartHighlight } from "@/hooks/useMatrix";
import { bus } from "@/utils/eventbus";
import {CardInfo, COLUMN_COUNT, ROW_COUNT, Target, WIDTH} from "@/types";
//import {GetLuckNum} from "@/utils/lottery";
import {InitCards, ResetCard, SelectCard, ShineCard} from "@/hooks/useCard";
import {Create3D, RotateBall, Transform} from "@/hooks/use3D";
import {Euler} from "three";
import {GetLuckNum} from "@/utils/lottery";


// Define props
const props = defineProps<{
  showTable: boolean;
}>();

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
  {"num":"000019","name": "伊万10", "img": "123"}, {"num":"000020","name": "伊万11", "img": "123"},
  {"num":"000019","name": "伊万10", "img": "123"}, {"num":"000020","name": "伊万11", "img": "123"},
  {"num":"000019","name": "伊万10", "img": "123"}, {"num":"000020","name": "伊万11", "img": "123"},
  {"num":"000019","name": "伊万10", "img": "123"}, {"num":"000020","name": "伊万11", "img": "123"}];
const containerRef = ref<HTMLElement | null>(null);
let excludeIndex: Array<number> = [];
//let tempSelectedIndex = [];

let renderer: CSS3DRenderer;
let camera: THREE.PerspectiveCamera;
let scene: THREE.Scene;
let controls: TrackballControls ;
let intervalId: NodeJS.Timeout;
let threeDCards: Array<CSS3DObject> = [];
let targets: Target = { table: [], sphere: [] };
let rotateObj: Tween<Euler>;

onMounted(() => {
    if (containerRef.value) {
      init();
      window.addEventListener("resize", onWindowResize, false);
    }
    bus.on("Drawing", (action: unknown)=> {drawing(action as boolean)})
});

onBeforeUnmount(() => {
  if (containerRef.value && renderer) {
    containerRef.value.removeChild(renderer.domElement);
  }
  bus.off("Drawing");
  if (intervalId) clearInterval(intervalId);
  window.onresize = null;
});

watch(() => props.showTable, (newVal) => {
  transTableOrSphere(newVal);
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

  // 创建 camera, scene, renderer
  ({camera, scene, renderer, controls} = Create3D());
  // 创建targets, threeDCards
  ({targets, threeDCards} = InitCards(isBold, index, scene, position, allCardsArray, highlightCell));

  if (containerRef.value) containerRef.value.appendChild(renderer.domElement);
  transTableOrSphere(props.showTable);
  animate();
  intervalId = ShineCard(threeDCards, allCardsArray);
}
const drawing =  (action: boolean) => {
  if (action) {
     ResetCard(targets, excludeIndex, threeDCards, camera, scene, renderer).then(() => {
      RotateBall(camera, scene, renderer, (tween) => {
        rotateObj = tween; // 在这里设置全局变量rotateObj
      }).then(() => {
        console.log('Rotation complete');
      }).catch(error => {
        console.error(error);
      });
    });
  } else {
    rotateObj?.stop();
    if(intervalId) clearInterval(intervalId);
    let luckCards = GetLuckNum(allCardsArray, excludeIndex, 5);
    excludeIndex = excludeIndex.concat(luckCards)
    SelectCard(threeDCards, luckCards, allCardsArray, camera, scene, renderer)
  }
}

const animate = (): void => {
  requestAnimationFrame(animate);
  TWEEN.update();
  controls.update();
  renderer.render(scene, camera);
}

const onWindowResize = (): void => {
  camera.aspect = window.innerWidth / window.innerHeight;
  camera.updateProjectionMatrix();
  renderer.setSize(window.innerWidth, window.innerHeight);
  renderer.render(scene, camera);
}

const transTableOrSphere = (showTable:boolean):void => {
  if(showTable){
    Transform("table", camera, scene, renderer, targets, threeDCards);
  } else {
    Transform("sphere", camera, scene, renderer,targets, threeDCards);
  }
}

</script>

<style scoped>
/* Your styles here */
</style>
