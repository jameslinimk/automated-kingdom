import { readFileSync, writeFileSync } from "fs"

export const capitalize = (str: string) => str.charAt(0).toUpperCase() + str.slice(1)
export const pascalToSnake = (str: string) => str.replace(/([a-z])([A-Z])/g, "$1_$2").toLowerCase()

export const codeGen = (filePath: string, handler: (name: string) => string) => {
    const file = readFileSync(filePath, "utf8")

    let newFile = file.split("\n")
    newFile.forEach((line, i) => {
        if (!line) return

        const spaces = line.length - line.trimStart().length

        if (!line.includes("// [code-gen]")) return
        const name = line.trim().split("// [code-gen]")[1].trim()

        const newText = handler(name)
        if (newText) {
            newFile[i] = newFile[i] + "\n" + " ".repeat(spaces) + newText.replaceAll("\n", `\n${" ".repeat(spaces)}`)
            while (newFile[i + 1] && !newFile[i + 1].includes("// [code-gen] end")) {
                newFile[i + 1] = null
                i++
            }
        }
    })

    writeFileSync(
        filePath,
        newFile
            .filter((l) => l !== null)
            .join("\n")
            .replace(/\r?\n/g, "\r\n")
    )
    // console.log(newFile.filter((l) => l).join("\n"))
}
