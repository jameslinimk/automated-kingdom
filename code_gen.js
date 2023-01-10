import { readFileSync, writeFileSync } from "fs";
export const capitalize = (str) => str.charAt(0).toUpperCase() + str.slice(1);
export const pascalToSnakeCase = (str) => str.replace(/([a-z])([A-Z])/g, "$1_$2").toLowerCase();
export const colors = ["blue", "red", "green", "yellow"];
export const names = [
    "Icon",
    "IdleDown",
    "IdleUp",
    "IdleLeft",
    "IdleRight",
    "WalkDown",
    "WalkUp",
    "WalkLeft",
    "WalkRight",
];
export const codeGen = (filePath, handler) => {
    const file = readFileSync(filePath, "utf8");
    let newFile = file.split("\n");
    newFile.forEach((line, i) => {
        if (!line)
            return;
        const spaces = line.length - line.trimStart().length;
        if (!line.includes("// [code-gen]"))
            return;
        const name = line.trim().split("// [code-gen]")[1].trim();
        const newText = handler(name);
        if (newText) {
            newFile[i] = newFile[i] + "\n" + " ".repeat(spaces) + newText.replaceAll("\n", `\n${" ".repeat(spaces)}`);
            while (newFile[i + 1] && !newFile[i + 1].includes("// [code-gen] end")) {
                newFile[i + 1] = null;
                i++;
            }
        }
    });
    writeFileSync(filePath, newFile.filter((l) => l).join("\n"));
    // console.log(newFile.filter((l) => l).join("\n"))
};
//# sourceMappingURL=code_gen.js.map