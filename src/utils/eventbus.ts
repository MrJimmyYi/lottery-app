// eventBus.js
import mitt from 'mitt';
export const bus = mitt();

bus.emitAsync = function(event, ...args) {
    const listeners = bus.all.get(event);
    if (listeners && listeners.size > 0) { // 使用size属性来检查Set对象的元素数量
        const promises = Array.from(listeners).map(listener =>
            Promise.resolve(listener(...args))
        );
        return Promise.all(promises);
    } else {
        return Promise.resolve([]);
    }
};
