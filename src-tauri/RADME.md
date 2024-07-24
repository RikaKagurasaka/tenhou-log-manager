tenhou log extraction

```js
prompt("Copy following",
    new Array(~~localStorage.lognext).fill(undefined).map(
        (_, i) => JSON.parse(localStorage[`log${i}`])['log']).join(','))
```