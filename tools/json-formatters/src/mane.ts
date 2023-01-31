import fs from "fs";
import path from "path";
import process from "process";

import { json_fmt } from "./json-fmt";
import { json_minify } from "./json-minify";

let pack_directory =
  "/home/velvetremedy/.minecraft/resourcepacks/Love-and-Tolerance/";

let fmt_args = ["-fmt", "-f", "-format"];
let mini_args = ["-min", "-m", "-minify", "-mini"];

async function mane() {
  check_dir(pack_directory);
  process.chdir(pack_directory);
  let json_files = find_files_in_dir(`./`, ".json");
  let mcmeta_files = find_files_in_dir(`./`, ".mcmeta");
  console.log(json_files, mcmeta_files);
}

function check_dir(dir: string) {
  if (!fs.existsSync(dir)) {
    console.error(
      `Failed to find directory: ${dir}, please make sure you entered a valid path.`
    );
    process.exit(1);
  }
}

function find_files_in_dir(startPath: string, filter: string) {
  let results: string[] = [];
  if (!fs.existsSync(startPath)) {
    console.log("no dir ", startPath);
    throw Error;
  }
  let files = fs.readdirSync(startPath);
  for (let i = 0; i < files.length; i++) {
    let filename = path.join(startPath, files[i]);
    let stat = fs.lstatSync(filename);
    if (stat.isDirectory()) {
      results = results.concat(find_files_in_dir(filename, filter));
    } else if (filename.indexOf(filter) >= 0) {
      results.push(filename);
    }
  }
  return results;
}

mane();
