import { dirname, resolve } from "path"
import { fileURLToPath } from "url"
import { workerColors, workerTextureNames } from "../../ak-server/code-gen/types_game.js"
import { capitalize, codeGen, pascalToSnake } from "../../code_gen.js"

const __filename = fileURLToPath(import.meta.url)
const __dirname = dirname(__filename)

export default () => {
    codeGen(resolve(__dirname, "../src/game.rs"), (name) => {
        let newText = ""
        switch (name) {
            case "workers": {
                newText = workerColors
                    .map((color) =>
                        workerTextureNames
                            .map(
                                (name) =>
                                    `Texture::${capitalize(color)}Worker${name} => "workers/${color}/${pascalToSnake(
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
}
