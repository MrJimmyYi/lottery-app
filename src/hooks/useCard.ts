import {
    CardInfo,
    COLUMN_COUNT,
    COMPANY,
    Position, RESET_DURATION,
    RESOLUTION,
    ROW_COUNT,
    SELECT_DURATION, SelectedCardLocation,
    Target,
    TOTAL_CARDS,
    WIDTH
} from "@/types";
import {CSS3DObject, CSS3DRenderer} from 'three/addons/renderers/CSS3DRenderer.js';
import * as THREE from "three";
import TWEEN from "@tweenjs/tween.js";

const createElement = (css: string = "", text: string = "", tagName: string=""): HTMLElement => {
    let dom = document.createElement(tagName);
    dom.className = css;
    dom.innerHTML = text;
    return dom;
};

// createCard 函数转换
export const CreateCard = (card: CardInfo, isBold: boolean, id: number | string): HTMLElement => {
    let element = createElement("element", "", "div"); // 假设初始 css 类名为 "element"
    element.id = `card-${id}`;
    element.className = isBold ? "element lightitem highlight" : "element";
    element.style.backgroundColor = `rgba(0,127,127,${Math.random() * 0.7 + 0.25})`;

    element.appendChild(createElement("card-close-btn", "x", "div"));
    element.appendChild(createElement("company", COMPANY, "div"));
    element.appendChild(createElement("name", card.name, "div"));
    element.appendChild(createElement("details", card.num, "div"));
    return element;
};


/**
 * 切换名牌人员信息
 */
const changeCard = (threeDCards: Array<CSS3DObject>, cardIndex: number, card: CardInfo) => {
    let cardEle = threeDCards[cardIndex].element;
    cardEle.innerHTML = `<div class="card-close-btn">x</div><div class="company">${COMPANY}</div><div class="name">${card.name}</div><div class="details">${card.num || ""}</div>`;
}

/**
 * 随机切换
 */
const random = (num: number) => {
    // Math.floor取到0-num-1之间数字的概率是相等的
    return Math.floor(Math.random() * num);
}

// Dummy placeholder for other functions
export const InitCards = (isBold: boolean, index: number, scene:THREE.Scene, position:Position, allCardsArray: Array<CardInfo>, highlightCell: string[]): { targets: Target; threeDCards: Array<CSS3DObject> } => {
    // Initialization logic for cards
    let threeDCards: Array<CSS3DObject> = [];
    let targets: Target = { table: [], sphere: [] };
    let length = allCardsArray.length
    for (let i = 0; i < ROW_COUNT; i++) {
        for (let j = 0; j < COLUMN_COUNT; j++) {
            isBold = highlightCell.includes(j + "-" + i);
            let element = CreateCard(
                allCardsArray[index % length],
                isBold,
                index
            );
            let object = new CSS3DObject(element);
            object.position.x = Math.random() * 4000 - 2000;
            object.position.y = Math.random() * 4000 - 2000;
            object.position.z = Math.random() * 4000 - 2000;
            scene.add(object);
            threeDCards.push(object);
            let tableObject = new THREE.Object3D();
            tableObject.position.x = j * WIDTH - position.x;
            tableObject.position.y = -(i * 180) + position.y;
            targets.table.push(tableObject);
            index++;
        }
    }

    // 创建球形布局
    let vector = new THREE.Vector3();
    for (let i = 0, l = threeDCards.length; i < l; i++) {
        let phi = Math.acos(-1 + (2 * i) / l);
        let theta = Math.sqrt(l * Math.PI) * phi;
        let object = new THREE.Object3D();
        object.position.setFromSphericalCoords(800 * RESOLUTION, phi, theta);
        vector.copy(object.position).multiplyScalar(2);
        object.lookAt(vector);
        targets.sphere.push(object);
    }

    return {threeDCards, targets}
}

/**
 * 随机切换背景和人员信息
 */
export const ShineCard = (threeDCards: Array<CSS3DObject>, allCardsArray: Array<CardInfo>):NodeJS.Timeout => {
    let maxCard = 10,
        maxCardsArray;
    let shineCard = 10 + random(maxCard);
    let shineCardsArray = Object.values(allCardsArray)
    let intervalId =  setInterval(() => {
        maxCardsArray = allCardsArray.length;
        for (let i = 0; i < shineCard; i++) {
            let index = random(maxCardsArray),
                cardIndex = random(TOTAL_CARDS);
            let card = threeDCards[cardIndex].element;
            card.style.backgroundColor = "rgba(0,127,127," + (Math.random() * 0.7 + 0.25) + ")";
            changeCard(threeDCards, cardIndex, shineCardsArray[index]);
        }
    }, 1000);
    return intervalId;
}


export const SelectCard = (threeDCards: Array<CSS3DObject>, selectedIndex:Array<Number>, allCardsArray:Array<CardInfo>, camera: THREE.PerspectiveCamera, scene: THREE.Scene, renderer: CSS3DRenderer, locates: Array<SelectedCardLocation>=[])=>{
    //停止旋转
    //rotateObj.stop();
    //rotate = false
    let width = WIDTH,
        tag = -(selectedIndex.length - 1) / 2;

    //locates长度为0此时是结束抽奖后首次需要现实的卡片
    if(locates.length ===0 ) {
        // 计算位置信息, 大于5个分两排显示
        if (selectedIndex.length > 5) {
            let yPosition = [-87, 87],
                l = selectedIndex.length,
                mid = Math.ceil(l / 2);
            tag = -(mid - 1) / 2;
            for (let i = 0; i < mid; i++) {
                locates.push({
                    x: tag * width * RESOLUTION,
                    y: yPosition[0] * RESOLUTION
                });
                tag++;
            }
            tag = -(l - mid - 1) / 2;
            for (let i = mid; i < l; i++) {
                console.log("y", yPosition[1] * RESOLUTION);
                locates.push({
                    x: tag * width * RESOLUTION,
                    y: yPosition[1] * RESOLUTION
                });
                tag++;
            }
        } else {
            for (let i = selectedIndex.length; i > 0; i--) {
                locates.push({
                    x: tag * width * RESOLUTION,
                    y: 0 * RESOLUTION
                });
                tag++;
            }
        }
    }
    selectedIndex.forEach((cardIndex, index) => {
        changeCard(threeDCards, Number(cardIndex), allCardsArray[Number(cardIndex)]);
        let object = threeDCards[Number(cardIndex)];
        new TWEEN.Tween(object.position)
            .to(
                {
                    x: locates[index].x,
                    y: locates[index].y * RESOLUTION,
                    z: 2200
                },
                Math.random() * SELECT_DURATION + SELECT_DURATION
            )
            .easing(TWEEN.Easing.Exponential.InOut)
            .start();
        new TWEEN.Tween(object.rotation)
            .to(
                {
                    x: 0,
                    y: 0,
                    z: 0
                },
                Math.random() * SELECT_DURATION + SELECT_DURATION
            )
            .easing(TWEEN.Easing.Exponential.InOut)
            .start();
        object.element.classList.add("prize");
        tag++;
    });

    new TWEEN.Tween({})
        .to({}, SELECT_DURATION * 2)
        .onUpdate(()=>{renderer.render(scene, camera)})
        .start()
        .onComplete(() => {
            // 动画结束后可以操作
            //setLotteryStatus();
        });
}


/**
 * 重置抽奖牌内容
 */
export const ResetCard = (targets: Target, selectedIndex: Array<Number>, threeDCards: Array<CSS3DObject>, camera: THREE.PerspectiveCamera, scene: THREE.Scene, renderer: CSS3DRenderer ):Promise<void> => {
    if (selectedIndex.length === 0) {
        return Promise.resolve();
    }
    selectedIndex.forEach(index => {
        let object = threeDCards[Number(index)],
            target = targets.sphere[Number(index)];
        new TWEEN.Tween(object.position)
            .to(
                {
                    x: target.position.x,
                    y: target.position.y,
                    z: target.position.z
                },
                Math.random() * RESET_DURATION + RESET_DURATION
            )
            .easing(TWEEN.Easing.Exponential.InOut)
            .start();
        new TWEEN.Tween(object.rotation)
            .to(
                {
                    x: target.rotation.x,
                    y: target.rotation.y,
                    z: target.rotation.z
                },
                Math.random() * RESET_DURATION + RESET_DURATION
            )
            .easing(TWEEN.Easing.Exponential.InOut)
            .start();
    });
    return new Promise((resolve) => {
        new TWEEN.Tween({})
            .to({}, RESET_DURATION * 2)
            .onUpdate(()=>{renderer.render(scene, camera)})
            .start()
            .onComplete(() => {
                selectedIndex.forEach(index => {
                    let object = threeDCards[Number(index)];
                    object.element.classList.remove("prize");
                });
                resolve();
            });
    });
}