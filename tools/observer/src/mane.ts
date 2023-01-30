import { execSync } from "child_process";
import process from "process";

const old_release =
  "/home/velvetremedy/Stuff/lat_zips/resource-packs/beta-versions/Arekuzu-test-1/";
const new_release =
  "/home/velvetremedy/Stuff/lat_zips/resource-packs/beta-versions/Arekuzu-test-2/";

async function mane() {
  const programs = ["git", "rsync"];
  check_installed(programs);
  execute_command(`rm -rf ./pack; mkdir pack`);
  process.chdir("./pack");
  execute_command(`rsync -avP ${old_release} ./`);
  execute_command(`git init`);
  execute_command(`git add * && git commit -m "previous release"`);
}

function check_installed(programs: string[]) {
  for (let program of programs) {
    try {
      execSync(`which "${program}"`);
    } catch (err) {
      throw new Error(`Exit: "${program}" is not installed.`);
    }
  }
}

function execute_command(command: string) {
  try {
    execSync(command);
  } catch (err) {
    console.error(err);
  }
}

mane();
