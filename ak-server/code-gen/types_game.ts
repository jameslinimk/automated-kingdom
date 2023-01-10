import { resolve } from "path"
import { capitalize, codeGen, colors, names } from "../../code_gen.js"

const start = performance.now()

codeGen(resolve("../src/types_game.rs"), (name) => {
    let newText = ""
    switch (name) {
        case "workers": {
            newText = colors
                .map((color) => names.map((name) => `${capitalize(color)}Worker${name},`).join(" "))
                .join("\n")
            break
        }
    }
    return newText
})

console.log(`Done!, took ${(performance.now() - start).toFixed(2)}ms`)
