import fs from "fs";
import path from "path";
import { chdir } from "process";

let minecraft_directory = "/home/velvetremedy/.minecraft/versions/1.19.3/1.19.3/assets/";
let pack_directory = "/home/velvetremedy/.minecraft/resourcepacks/Love-and-Tolerance/";

async function mane() {
  process.chdir(minecraft_directory);
  let images = find_files_in_dir(`./`, ".png");
  let missing_images = comparator(images);
};

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

function comparator(images: string[]) {
  let results: string[] = [];
  for (let image of images) {
    if (!fs.existsSync(pack_directory + image)) { 
      console.log(image);
    }
  }
  return results;
};

mane();
