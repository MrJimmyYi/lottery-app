export
// 定义 NUMBER_MATRIX 的类型
type NumberMatrix = Array<Array<[number, number]>>;
const NUMBER_MATRIX: NumberMatrix = [
    [
        // 0
        [0, 0],
        [1, 0],
        [2, 0],
        [0, 1],
        [2, 1],
        [0, 2],
        [2, 2],
        [0, 3],
        [2, 3],
        [0, 4],
        [1, 4],
        [2, 4]
    ],
    [
        // 1
        [1, 0],
        [0, 1],
        [1, 1],
        [1, 2],
        [1, 3],
        [0, 4],
        [1, 4],
        [2, 4]
    ],
    [
        // 2
        [0, 0],
        [1, 0],
        [2, 0],
        [2, 1],
        [0, 2],
        [1, 2],
        [2, 2],
        [0, 3],
        [0, 4],
        [1, 4],
        [2, 4]
    ],
    [
        // 3
        [0, 0],
        [1, 0],
        [2, 0],
        [2, 1],
        [0, 2],
        [1, 2],
        [2, 2],
        [2, 3],
        [0, 4],
        [1, 4],
        [2, 4]
    ],
    [
        // 4
        [0, 0],
        [2, 0],
        [0, 1],
        [2, 1],
        [0, 2],
        [1, 2],
        [2, 2],
        [2, 3],
        [2, 4]
    ],
    [
        // 5
        [0, 0],
        [1, 0],
        [2, 0],
        [0, 1],
        [0, 2],
        [1, 2],
        [2, 2],
        [2, 3],
        [0, 4],
        [1, 4],
        [2, 4]
    ],
    [
        // 6
        [0, 0],
        [1, 0],
        [2, 0],
        [0, 1],
        [0, 2],
        [1, 2],
        [2, 2],
        [0, 3],
        [2, 3],
        [0, 4],
        [1, 4],
        [2, 4]
    ],
    [
        // 7
        [0, 0],
        [1, 0],
        [2, 0],
        [2, 1],
        [2, 2],
        [2, 3],
        [2, 4]
    ],
    [
        // 8
        [0, 0],
        [1, 0],
        [2, 0],
        [0, 1],
        [2, 1],
        [0, 2],
        [1, 2],
        [2, 2],
        [0, 3],
        [2, 3],
        [0, 4],
        [1, 4],
        [2, 4]
    ],
    [
        // 9
        [0, 0],
        [1, 0],
        [2, 0],
        [0, 1],
        [2, 1],
        [0, 2],
        [1, 2],
        [2, 2],
        [2, 3],
        [0, 4],
        [1, 4],
        [2, 4]
    ]
];

// 定义 HEART_MATRIX 的类型
type HeartMatrix = number[][];

const HEART_MATRIX: HeartMatrix = [
    [0, 0, 1, 0, 0, 0, 1, 0, 0],
    [0, 1, 0, 1, 0, 1, 0, 1, 0],
    [1, 0, 0, 0, 1, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 1],
    [0, 1, 0, 0, 0, 0, 0, 1, 0],
    [0, 0, 1, 0, 0, 0, 1, 0, 0],
    [0, 0, 0, 1, 0, 1, 0, 0, 0],
    [0, 0, 0, 0, 1, 0, 0, 0, 0]
];

const CreateHeartHighlight = (): string[] => {
    let xoffset: number = 3,
        highlight: string[] = [];
    for (let y: number = 0; y < HEART_MATRIX.length; y++) {
        for (let x: number = 0; x < HEART_MATRIX[y].length; x++) {
            if (HEART_MATRIX[y][x] === 1) {
                highlight.push(`${x + xoffset}-${y}`);
            }
        }
    }
    return highlight;
};

const RemoveHighlight = ()=> {
    document.querySelectorAll(".highlight").forEach(node => {
        node.classList.remove("highlight");
    });
}

// 导出模块的 TypeScript 写法
export { NUMBER_MATRIX, HEART_MATRIX, CreateHeartHighlight, RemoveHighlight };
