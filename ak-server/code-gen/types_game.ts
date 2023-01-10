import { dirname, resolve } from "path"
import { fileURLToPath } from "url"
import { capitalize, codeGen } from "../../code_gen.js"

const __filename = fileURLToPath(import.meta.url)
const __dirname = dirname(__filename)

export const workerColors = ["blue", "red", "green", "yellow"]
export const workerTextureNames = [
    "Icon",
    "IdleDown",
    "IdleUp",
    "IdleLeft",
    "IdleRight",
    "WalkDown",
    "WalkUp",
    "WalkLeft",
    "WalkRight",
]

export default () => {
    codeGen(resolve(__dirname, "../src/types_game.rs"), (name) => {
        let newText = ""
        switch (name) {
            case "workers": {
                newText = workerColors
                    .map((color) => workerTextureNames.map((name) => `${capitalize(color)}Worker${name},`).join(" "))
                    .join("\n")
                break
            }
        }
        return newText
    })
}
