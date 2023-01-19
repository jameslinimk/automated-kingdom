import { exec } from "child_process"
import { resolve } from "path"

const exe = resolve("../target/release/automated-kingdom.exe")
const out = resolve("out.zip")

const SevenZipPath = "C:\\Program Files\\7-Zip"

exec(
    `.\\7z.exe a -tzip ${out} ${exe}`,
    {
        cwd: SevenZipPath,
    },
    (error, stdout, stderr) => {
        if (error) {
            console.log(`error: ${error.message}`)
            return
        }
        if (stderr) {
            console.log(`stderr: ${stderr}`)
            return
        }

        console.log(stdout)
    }
)
