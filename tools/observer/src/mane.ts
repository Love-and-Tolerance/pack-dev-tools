import { execSync } from "child_process";
import fs from "fs";
import process from "process";

const old_release = "/home/velvetremedy/Stuff/previous-release/";
const new_release = "/home/velvetremedy/Stuff/new-release/";

let added: string[] = [];
let renamed: string[] = [];
let changed: string[] = [];
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
  execute_command(`rm -rf ./*`);
  generate_changelog();
}

function check_dir(dir: string) {
  if (!fs.existsSync(dir)) {
    console.error(
      `Failed to find directory: ${dir}, please make sure you entered a valid path.`
    );
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

function generate_changelog() {
  let file_data: string[] = [];
  file_data.push("## Changelog");
  file_data.push("");
  if (added[0] != undefined) {
    file_data.push("### Added");
    for (let change of added) {
      file_data.push("- `" + change + "`");
    }
    file_data.push("");
  }
  if (renamed[0] != undefined) {
    file_data.push("### Renamed / Moved");
    for (let change of renamed) {
      let before = change.split("->")[0].trim();
      let after = change.split("->")[1].trim();
      file_data.push("- `" + before + "` -> `" + after + "`");
    }
    file_data.push("");
  }
  if (changed[0] != undefined) {
    file_data.push("### Modified");
    for (let change of changed) {
      file_data.push("- `" + change + "`");
    }
    file_data.push("");
  }
  if (deleted[0] != undefined) {
    file_data.push("### Removed");
    for (let change of deleted) {
      file_data.push("- `" + change + "`");
    }
    file_data.push("");
  }
  fs.writeFileSync("./changelog.md", file_data.join("\n"));
}

mane();
