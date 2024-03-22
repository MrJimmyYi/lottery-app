import {MAX_RANDOM_NUM} from "@/types";

export const GetLuckNum = (
    allCardsArray: any[],
    selectedIndex: number[],
    n: number
): number[] => {
    let indexArray = Array.from(allCardsArray.keys()); // 创建索引数组
    let start = 0;
    let selectedIndices = new Set<number>();
    let luckResIndex: number[] = [];
    while (start < n) {
        //设置如果超过最大值，则返回空数组
        if(start > MAX_RANDOM_NUM){
            return [];
        }
        // 随机选取一个索引
        let randomIndex = Math.floor(Math.random() * indexArray.length);
        if (selectedIndices.has(randomIndex) || selectedIndex.includes(randomIndex)) {
            continue;
        }
        selectedIndices.add(randomIndex);
        luckResIndex.push(randomIndex);
        start++;
    }
    return luckResIndex;
};


