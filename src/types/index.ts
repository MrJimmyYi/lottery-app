import * as THREE from "three";

export const COLUMN_COUNT = 15;
export const ROW_COUNT = 8;
export const TOTAL_CARDS = ROW_COUNT * COLUMN_COUNT;
export const COMPANY = "Jimmy";
export const WIDTH = 140;
export const RESOLUTION = 1;
export const DURATION = 2000;
export const SELECT_DURATION = 600;
export const RESET_DURATION = 500;
export const ROTATE_TIME = 1000;
export const ROTATE_LOOP = 10;
export const MAX_RANDOM_NUM = 1000000;

export enum PAGES_OPTION {
    HOME = "home",
    MANAGEMENT = "management",
    LUCKY_USERS = "luckyUsers"
}


export enum DRAW_STATUS {
    TABLE = "table",
    READY = "ready",
    DRAWING = "drawing",
    DOWN = "down"
}


export interface Position {
    x: number;
    y: number;
}

// 定义 Card 接口
export interface Card {
    name: string;
    num: string; // 或者 number，取决于实际用途
}

export interface Starts {
    x: number;
    y: number;
    z: number;
    o: string;
};

export interface CardInfo {
    num: string;
    name: string;
    img: string;
}

export interface Target {
    table: Array<THREE.Object3D>,
    sphere: Array<THREE.Object3D>
}

export interface SelectedCardLocation {
    x: number,
    y: number
}

export interface Prize {
    id: number,
    range: string,
    title: string,
    imgSrc: string,
    altText: string,
    count: number,
    total: number,
    perCount: number
}