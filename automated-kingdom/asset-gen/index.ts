import { readdirSync } from "fs"
import Jimp from "jimp"

const basePath = "../assets/sprites/workers"
const colors = ["blue", "green", "red", "yellow"]

const hexToJimp = (hex: string) => {
    const r = parseInt(hex.slice(1, 3), 16)
    const g = parseInt(hex.slice(3, 5), 16)
    const b = parseInt(hex.slice(5, 7), 16)
    return Jimp.rgbaToInt(r, g, b, 255)
}

const colorReplace: {
    [blueColor: number]: {
        green: number
        red: number
        yellow: number
    }
} = {
    [hexToJimp("#116ec9")]: {
        green: hexToJimp("#293900"),
        red: hexToJimp("#3a0000"),
        yellow: hexToJimp("#3a3a00"),
    },
    [hexToJimp("1b81e6")]: {
        green: hexToJimp("#293900"),
        red: hexToJimp("#3a0000"),
        yellow: hexToJimp("#3a3a00"),
    },
    [hexToJimp("0d87ff")]: {
        green: hexToJimp("#293900"),
        red: hexToJimp("#3a0000"),
        yellow: hexToJimp("#3a3a00"),
    },
}

const files = readdirSync(`${basePath}/${colors[0]}`)

// Jimp.read(path)
//     .then((image) => {
//         image.scan(0, 0, image.bitmap.width, image.bitmap.height, (x, y, idx) => {
//             const color = image.getPixelColor(x, y)
//             const newColor = colorReplace[color]

//             if (newColor) {
//                 image.setPixelColor(newColor, x, y)
//             }
//         })
//         return image.writeAsync("./output.png")
//     })
//     .then(() => {
//         console.log("Image successfully processed!")
//     })
//     .catch((err) => {
//         console.error(err)
//     })
