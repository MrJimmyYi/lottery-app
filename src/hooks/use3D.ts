// Camera
import * as THREE from "three";
import {CSS3DObject, CSS3DRenderer} from 'three/addons/renderers/CSS3DRenderer.js';
import {TrackballControls} from "three/addons/controls/TrackballControls.js";
import TWEEN, {Tween} from "@tweenjs/tween.js";
import {DURATION, ROTATE_LOOP, ROTATE_TIME, Target} from "@/types";
import {Euler} from "three";


export const Create3D = (): {camera: THREE.PerspectiveCamera, scene: THREE.Scene, renderer: CSS3DRenderer, controls: TrackballControls} => {
    let camera = new THREE.PerspectiveCamera(40, window.innerWidth / window.innerHeight, 1, 10000);
    camera.position.z = 3000;

// Scene
    let scene = new THREE.Scene();

    // Renderer
    let renderer = new CSS3DRenderer();
    renderer.setSize(window.innerWidth, window.innerHeight);

    // Controls
    let controls = new TrackballControls(camera, renderer.domElement);
    controls.rotateSpeed = 0.5;
    controls.minDistance = 500;
    controls.maxDistance = 6000;
    controls.addEventListener("change", ()=>{renderer.render(scene, camera)});
    return {camera, scene, renderer, controls}
}

/**
 * 渲染地球等
 */
export const Transform = (targetstr: string, camera: THREE.PerspectiveCamera, scene: THREE.Scene, renderer: CSS3DRenderer, targets: Target, threeDCards: Array<CSS3DObject>): void => {
    // TWEEN.removeAll(); // 如果需要使用，确保 TWEEN 库定义包含 removeAll 方法
    let tmpTargets: Array<THREE.Object3D>;

    if (targetstr === "sphere") {
        tmpTargets = targets.sphere;
    } else if (targetstr === "table") {
        tmpTargets = targets.table;
    } else {
        return;
    }
    for (let i = 0; i < threeDCards.length; i++) {
        let object = threeDCards[i];
        let target = tmpTargets[i];
        new TWEEN.Tween(object.position)
            .to(
                {
                    x: target.position.x,
                    y: target.position.y,
                    z: target.position.z,
                },
                Math.random() * DURATION + DURATION
            )
            .easing(TWEEN.Easing.Exponential.InOut)
            .start();

        new TWEEN.Tween(object.rotation)
            .to(
                {
                    x: target.rotation.x,
                    y: target.rotation.y,
                    z: target.rotation.z,
                },
                Math.random() * DURATION + DURATION
            )
            .easing(TWEEN.Easing.Exponential.InOut)
            .start();
    }

    new TWEEN.Tween({})
        .to({}, DURATION * 2)
        .onUpdate(()=>{renderer.render(scene, camera)})
        .start();
};

// Function to rotate the ball
export const RotateBall = (camera: THREE.PerspectiveCamera, scene: THREE.Scene, renderer: CSS3DRenderer, onTweenCreated: (tween: Tween<Euler>) => void):Promise<void>  => {
    return new Promise((resolve, reject) => {
        if (!scene) {
            reject('Scene not initialized');
            return;
        }
        scene.rotation.y = 0;
        let rotateObj = new TWEEN.Tween(scene.rotation)
            .to({ y: Math.PI * 6 * ROTATE_LOOP }, ROTATE_TIME * ROTATE_LOOP)
            .onUpdate(()=>{renderer.render(scene, camera)})
            // .easing(TWEEN.Easing.Linear) // Uncomment or adjust as needed
            .start()
            .onStop(() => {
                scene.rotation.y = 0;
                resolve();
            })
            .onComplete(() => {
                resolve();
            });
        onTweenCreated(rotateObj);
    });
};
