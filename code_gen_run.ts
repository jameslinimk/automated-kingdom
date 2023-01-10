import { exec as _exec } from "child_process"
import { readdirSync, statSync } from "fs"
import { join } from "path"
import { promisify } from "util"
const exec = promisify(_exec)

const explore = async (path: string) => {
    const folders = readdirSync(path)
    for (const folder of folders) {
        const stat = statSync(join(path, folder))
        if (stat.isDirectory()) {
            if (folder === "node_modules" || folder === ".git" || folder === "target" || folder === ".history") {
                continue
            }

            if (folder === "code-gen") {
                for (const file of readdirSync(join(path, folder))) {
                    if (!file.endsWith(".js")) continue
                    console.log(`Running "cd ${join(path, folder)} && node ${file}"`)
                    await exec(`cd ${join(path, folder)} && node ${file}`)
                }
                continue
            }

            explore(join(path, folder))
        }
    }
}
await explore("./")
await exec("cargo fmt")
