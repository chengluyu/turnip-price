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
    const turnip = wasm.calculate(currentPattern, seed);
    
    // Update the price table.
    document.getElementById('buying-price').innerText = turnip.buying_price().toString();
    const prices = new Int32Array(memory.buffer, turnip.selling_prices(), 12);
    prices.forEach((value, i) => {
        document.getElementById(`selling-price-${i}`).innerText = value.toString();
    });
    // Update the next pattern.
    const nextPattern = turnip.pattern();
    document.getElementById('next-pattern').innerText = `${nextPattern} (${explanation[nextPattern]})`;
};