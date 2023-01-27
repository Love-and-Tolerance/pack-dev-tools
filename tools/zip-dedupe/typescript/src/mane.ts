import fs from "fs";
import path from "path";
import crypto from "crypto";
import process from "process";

let zip_directory = "/home/velvetremedy/Stuff/lat_zips/";

async function mane() {
  process.chdir(zip_directory);
  let zips = find_files_in_dir(`./`, ".zip");
  let duplicates = comparator(zips);
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

function comparator(zips: string[]) {
  let results: string[][] = [];
  let duplicates: string[][] = [];
  for (let zip of zips) {
    let zip_hash = get_hash(zip_directory + zip);
    let file = [zip_hash, zip]
    results.push(file);
  }
  results.sort();
  for (let i = 1; i < results.length; i++) {
    if (results[i][0] === results[i - 1][0]) {
      duplicates.push(results[i - 1]);
      duplicates.push(results[i]);
    }
  }
  return new Set(duplicates);
}

function get_hash(file: string) {
  const file_buffer = fs.readFileSync(file);
  const hash_sum = crypto.createHash("sha256");
  hash_sum.update(file_buffer);
  return hash_sum.digest("hex");
}

mane();
