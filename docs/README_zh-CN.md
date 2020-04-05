# 大头菜价格

《集合啦！动物森友会》大头菜价格计算器（Rust 和 WebAssembly 版）。点击[这里](https://turnip-price.now.sh)访问在线版的计算器。

本项目非常感谢Twitter网友[@_Ninji](https://twitter.com/_Ninji)，如果他没有反编译游戏代码并且发布了一个[C++版本的算法](https://gist.github.com/Treeki/85be14d297c80c8b3c0a76375743325b)，那么根本就不会有本项目。

## 🚀 使用

你可以用 npm 或 Yarn 来安装这个包。

```bash
npm i turnip-price
# or
yarn add turnip-price
```

记得确认一下你的目标环境是支持 WebAssembly 的。安装完后，可以用下面的方式来计算大头菜价格。

```js
import * as wasm from "turnip-price";
import { memory } from 'turnip-price/turnip_price_bg';

function predict(whatPattern, seed) {
  const prediction = wasm.predict(whatPattern, seed);
  const prices = new Int32Array(memory.buffer, prediction.prices(), 14);
  return prices; // 长度为 14
}
```

关于 `prices` 数组的解释:

* 第一个元素是你在周日上午的买入价；
* 第二个元素没有用；
* 第二、四、六、八、十、十二个元素分别表示了周一到周六这几天上午的卖出价格；
* 第三、五、七、九、十一、十三个元素分别表示了周一到周六这几天下午的卖出价格。

## 🤔 问答

### 📈 如何使用这个库来预测我岛上的大头菜价格？

很遗憾现在还不行。因为此算法需要游戏内的随机数种子。

### 🕸️🦀️ 为什么使用 WebAssembly?

因为提供的算法中需要把无符号 32 位整数重新解释为 IEEE 754 的 32 位浮点数。用 JavaScript 手动实现微麻烦，干脆就用 Rust 写并编译到 WebAssembly 好了。

## 🔍 发现

### 📚 我可以枚举所有组合吗？

可以，一共有 4 × 2 ^ 32 = 17,179,869,184` 种组合。在我的电脑上遍历所有情况只需四分钟（使用了四个线程）。

### 💰 可能的最高价是多少？

我遍历了所有组合，在每种模式下得到了下面的结果。

* 在模式 0 (即代码中 `what_pattern` 设为 0), 最高价为 660 铃钱，此时种子为 326。
* 在模式 1 (即代码中 `what_pattern` 设为 1), 最高价为 660 铃钱，此时种子为 326。
* 在模式 2 (即代码中 `what_pattern` 设为 2), 最高价为 660 铃钱，此时种子为 326。
* 在模式 3 (即代码中 `what_pattern` 设为 3), 最高价为 660 铃钱，此时种子为 9772。

所以大头菜可能的最高价格是 660 铃钱。

### 📈 每周最高卖出价分布

我遍历所有组合后画了一张图。

![The Histogram of Weekly Highest Price of Turnips](weekly-highest-distribution.svg)
