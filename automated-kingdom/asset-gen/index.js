import { readdirSync } from "fs";
import Jimp from "jimp";
const start = performance.now();
const basePath = "./";
const outPath = "../assets/sprites/workers";
/**
 * First color is the base color, the rest are the colors to replace it with
 */
const colors = ["blue", "green", "red", "yellow"];
/**
 * Converts a hex string to Jimp color (number)
 */
const htj = (hex) => {
    const r = parseInt(hex.slice(1, 3), 16);
    const g = parseInt(hex.slice(3, 5), 16);
    const b = parseInt(hex.slice(5, 7), 16);
    return Jimp.rgbaToInt(r, g, b, 255);
};
const colorReplace = {
    [htj("#116ec9")]: {
        green: htj("#1ec724"),
        red: htj("#e64237"),
        yellow: htj("#c9c638"),
    },
    [htj("#1b81e6")]: {
        green: htj("#67f06c"),
        red: htj("#f5695f"),
        yellow: htj("#e8e44d"),
    },
    [htj("#0d87ff")]: {
        green: htj("#57e65c"),
        red: htj("#fa5b50"),
        yellow: htj("#fffb54"),
    },
};
for (const baseFile of readdirSync(`${basePath}/${colors[0]}`)) {
    const base = `${basePath}/${colors[0]}/${baseFile}`;
    for (const color of colors) {
        Jimp.read(base)
            .then((image) => {
            image.scan(0, 0, image.bitmap.width, image.bitmap.height, function (_x, _y, i) {
                const r = this.bitmap.data[i + 0];
                const g = this.bitmap.data[i + 1];
                const b = this.bitmap.data[i + 2];
                const col = Jimp.rgbaToInt(r, g, b, 255);
                const newColor = colorReplace?.[col]?.[color];
                if (newColor) {
                    const nc = Jimp.intToRGBA(newColor);
                    this.bitmap.data[i + 0] = nc.r;
                    this.bitmap.data[i + 1] = nc.g;
                    this.bitmap.data[i + 2] = nc.b;
                }
            });
            image.write(`${outPath}/${color}/${baseFile}`);
        })
            .catch((error) => {
            console.error(error);
        });
    }
}
console.log(`Done!, took ${(performance.now() - start).toFixed(2)}ms`);
//# sourceMappingURL=index.js.map