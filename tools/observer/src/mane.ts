import { execSync } from "child_process";
import { existsSync } from "fs";
import process from "process";

const old_release =
  "/home/velvetremedy/Stuff/previous-release/";
const new_release =
  "/home/velvetremedy/Stuff/new-release/";

let added: string[]  = [];
let renamed: string[]  = [];
let changed: string[]  = [];
let deleted: string[] = [];

async function mane() {
  check_dir(old_release);
  check_dir(new_release);
  const programs = ["git", "rsync"];
  check_installed(programs);
  execute_command(`rm -rf ./pack; mkdir pack`);
  process.chdir("./pack");
  execute_command(`rsync -avP ${old_release} ./`);
  execute_command(`git init`);
  execute_command(`git add * && git commit -m "previous release"`);
  execute_command('rm -r `ls | grep -v "./.git"`');
  execute_command(`rsync -avP ${new_release} ./`);
  execute_command(`git add -A`);
  let changes = get_changes() as string;
  separate_changes(changes);
  console.log(added, renamed, changed, deleted);
}

function check_dir(dir: string) {
    if (!existsSync(dir)) {
      console.error(`Failed to find directory: ${dir}, please make sure you entered a valid path.`);
      process.exit(1);
    }
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

function get_changes() {
  try {
    return execSync(`git status -s`).toString();
  } catch (err) {
    console.error(err);
  }
}

function separate_changes(changes: string) {
  let changes_array = changes.split("\n");
  for (let change of changes_array) {
    if (change.charAt(0) == "A") {
      added.push(change.slice(3, change.length));
    } else if (change.charAt(0) == "R") {
      renamed.push(change.slice(3, change.length));
    } else if (change.charAt(0) == "M") {
      changed.push(change.slice(3, change.length));
    } else if (change.charAt(0) == "D") {
      deleted.push(change.slice(3, change.length));
    }
  }
}

mane();
