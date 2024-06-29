async function init (){

        const importObject = {
                console: {
                        log: () => {
                                console.log("Just logging something!");
                                },
                        error: ()=> {
                                console.log("I am just error");
                        }
                }
        }

        const response = await fetch("sum.wasm")
        const buffer = await response.arrayBuffer();
        const wasm = await WebAssembly.instantiate(buffe, importObject);

        const sumFunction = wasm.instance.exports.sum;
        const result = sumFunction(80,100);
        console.log(result);
}