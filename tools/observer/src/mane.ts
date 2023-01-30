import fs from "fs";
import fse from "fs-extra";
import { simpleGit as git, CleanOptions } from "simple-git";
import { execSync } from "child_process";

const old_release =
  "/home/velvetremedy/Stuff/lat_zips/resource-packs/beta-versions/Arekuzu-test-1/";
const new_release =
  "/home/velvetremedy/Stuff/lat_zips/resource-packs/beta-versions/Arekuzu-test-2/";

async function mane() {
  git().clean(CleanOptions.FORCE);
  const programs = ["git"];
  check_installed(programs);
  await remove_pack_dir();
  fs.mkdirSync("./pack");
  await copy_files(old_release, false);
  process.chdir("./pack");
  await git_int();
  git_commit();
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

async function remove_pack_dir() {
  if (fs.existsSync("./pack")) {
    try {
      await fse.rm("./pack/", { recursive: true, force: true });
    } catch (err) {
      console.error(err);
    }
  }
}

async function copy_files(source: string, overwrite: boolean) {
  try {
    fse.copySync(source, "./pack/", { overwrite });
  } catch (err) {
    console.error(err);
  }
}

async function git_int() {
  try {
    await git().init();
  } catch (err) {
    console.error(err);
  }
}

function git_commit() {
  try {
    execSync(`git add * && git commit -m "previous release"`);
  } catch (err) {
    console.error(err);
  }
}

mane();
