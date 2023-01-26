import fs from "fs";
import path from "path";
import crypto from "crypto";
import process from "process";

let zip_directory = "/home/velvetremedy/Stuff/lat_zips/";

async function mane() {
  process.chdir(zip_directory);
  let zips = find_files_in_dir(`./`, ".zip");
  let duplicates = comparator(zips);
  duplicates.sort();
  console.log(duplicates);
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

function comparator(images: string[]) {
  let results: string[][] = [];
  for (let image of images) {
    let zip_hash = get_hash(zip_directory + image);
    let zip = [zip_hash, image]
    results.push(zip);
  }
  return results;
}

function get_hash(file: string) {
  const file_buffer = fs.readFileSync(file);
  const hash_sum = crypto.createHash("sha256");
  hash_sum.update(file_buffer);
  return hash_sum.digest("hex");
}

mane();
