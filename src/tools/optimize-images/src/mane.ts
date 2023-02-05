import fs from "fs";
import path from "path";
import { execSync } from "child_process";

let pack_dir = "/home/velvetremedy/Stuff/new-release/";

async function mane() {
  let programs = ["oxipng"];
  check_installed(programs);
  optimize_images(pack_dir);
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

function optimize_images(name: string) {
  let images = find_files_in_dir(name, ".png");
  images.forEach(function (file) {
    if (file.endsWith(".png")) {
      try {
        execSync(`oxipng -o 6 -i 1 --strip safe ${file} --fix`);
      } catch (err) {
        throw new Error(`Failed to optimize image.`);
      }
    }
  });
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
