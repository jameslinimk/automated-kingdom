import { resolve } from "path"
import { capitalize, codeGen, colors, names, pascalToSnakeCase } from "../../code_gen.js"

const start = performance.now()

codeGen(resolve("../src/game.rs"), (name) => {
    let newText = ""
    switch (name) {
        case "workers": {
            newText = colors
                .map((color) =>
                    names
                        .map(
                            (name) =>
                                `Texture::${capitalize(color)}Worker${name} => "workers/${color}/${pascalToSnakeCase(
                                    name
                                )}.png",`
                        )
                        .join(" ")
                )
                .join("\n")
            break
        }
    }
    return newText
})

console.log(`Done!, took ${(performance.now() - start).toFixed(2)}ms`)
