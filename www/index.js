import * as wasm from "turnip-price";
import { memory } from 'turnip-price/turnip_price_bg';
import random from 'lodash.random';

const explanation = [
    'high, decreasing, high, decreasing, high',
    'decreasing middle, high spike, random low',
    'consistently decreasing',
    'decreasing, spike, decreasing',
];

document.getElementById('generate-seed').onclick = () => {
    document.getElementById('seed').value = random(0, 4294967295);
};

document.getElementById('turnip-price-form').onsubmit = e => {
    e.preventDefault();
    const currentPattern = parseInt(document.getElementById('pattern').value, 10);
    const seed = parseInt(document.getElementById('seed').value, 10);
    const prediction = wasm.predict(currentPattern, seed);
    
    // Update the price table.
    const prices = new Int32Array(memory.buffer, prediction.prices(), 14);
    for (let i = 0; i < 14; i++) {
        if (i === 1) {
            continue;
        }
        document.getElementById(`item-${i}`).innerText = prices[i].toString();
    }

    // Update the next pattern.
    const nextPattern = prediction.pattern();
    document.getElementById('next-pattern').innerText = `${nextPattern} (${explanation[nextPattern]})`;
};