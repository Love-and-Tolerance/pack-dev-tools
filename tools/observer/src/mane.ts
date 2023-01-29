import fs from "fs";
import path from "path";
import crypto from "crypto";
import process from "process";

let old_release =
  "/home/velvetremedy/Stuff/lat_zips/resource-packs/beta-versions/Arekuzu-test-1/";
let new_release =
  "/home/velvetremedy/Stuff/lat_zips/resource-packs/beta-versions/Arekuzu-test-2/";

async function mane() {
  process.chdir(old_release);
  let old_files = find_files_in_dir(`./`, "");
  process.chdir(new_release);
  let new_files = find_files_in_dir(`./`, "");
  let changes = comparator(old_files, new_files);
  console.log(changes);
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

function comparator(old_files: string[], new_files: string[]) {
  let added: string[] = [];
  let changed: string[] = [];
  let removed: string[] = [];
  let unchanged: string[] = [];
  let copied: string[] = [];
  let old_hashes = get_hashes(old_release, old_files);
  let new_hashes = get_hashes(new_release, new_files);
  for (let i in old_files) {
    if (
      new_files.includes(old_files[i]) &&
      new_hashes.includes(old_hashes[i]) &&
      new_files.indexOf(old_files[i]) === new_hashes.indexOf(old_hashes[i])
    ) {
      unchanged.push(old_files[i]);
      //new_files.splice(new_files.indexOf(old_files[i]), 1);
      //new_hashes.splice(new_hashes.indexOf(old_hashes[i]), 1);
    } else if (
      !new_files.includes(old_files[i]) &&
      !new_hashes.includes(old_hashes[i])
    ) {
      removed.push(old_files[i]);
    }
  }
  for (let i in new_files) {
    if (
      !old_hashes.includes(new_hashes[i]) &&
      !old_files.includes(new_files[i])
    ) {
      added.push(new_files[i]);
    } else if (
      !old_hashes.includes(new_hashes[i]) &&
      old_files.includes(new_files[i]) &&
      new_hashes.filter((x) => x == new_hashes[i]).length == 1
    ) {
      changed.push(new_files[i]);
    } else if (
      old_hashes.includes(new_hashes[i]) &&
      !old_files.includes(new_files[i]) &&
      new_hashes.filter((x) => x == new_hashes[i]).length > 1 &&
      !copied.includes(old_files[old_hashes.indexOf(new_hashes[i])])
    ) {
      let new_index = old_hashes.indexOf(new_hashes[i]);
      let new_locations = new_files.filter(
        (file) =>
          file != old_files[new_index] &&
          new_hashes[i] == new_hashes[new_files.indexOf(file)]
      );
      changed.push(
        old_files[new_index] + " has been copied to " + new_locations.join(", ")
      );
      copied.push(old_files[new_index]);
    } else if (
      old_hashes.includes(new_hashes[i]) &&
      !old_files.includes(new_files[i])
    ) {
      let new_index = old_hashes.indexOf(new_hashes[i]);
      changed.push(
        old_files[new_index] + " has been renamed to " + new_files[i]
      );
    }
  }
  console.log(new_files, new_hashes);
  return [added, changed, removed];
}

function get_hashes(location: string, files: string[]) {
  let hashes: string[] = [];
  for (let file of files) {
    const file_buffer = fs.readFileSync(location + file);
    const hash_sum = crypto.createHash("sha256");
    hash_sum.update(file_buffer);
    hashes.push(hash_sum.digest("hex"));
  }
  return hashes;
}

mane();
