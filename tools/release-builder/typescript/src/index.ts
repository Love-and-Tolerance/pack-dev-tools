import fs from "fs";
import path from "path";
import { simpleGit as git, CleanOptions } from "simple-git";
import { execSync } from "child_process";

async function mane() {
  const java = await getJsonData("java");
  const bedrock = await getJsonData("bedrock");
  git().clean(CleanOptions.FORCE);
  checkGit();
  checkOxipng();
  checkDir();
  mkDirs();
  const javaUrls = getJavaUrls(java);
  const bedrockUrls = getBedrockUrls(bedrock);
  await cloneRepos(javaUrls);
  await cloneRepos(bedrockUrls);
  const javaPacks = await getPackData(javaUrls);
  const bedrockPacks = await getPackData(bedrockUrls);
  //optimizeImages(packs);
}

async function getJsonData(version: string) {
  return await fetch(
    `https://raw.githubusercontent.com/Love-and-Tolerance/pack-builder-assets/mane/assets/${version}.json`
  ).then((res) => res.json());
}

async function checkGit() {
  const { installed } = await git().version();
  if (!installed) {
    throw new Error(`Exit: "git" not available.`);
  }
}

function checkOxipng() {
  try {
    execSync('which "oxipng"');
  } catch (err) {
    throw new Error(`Exit: "oxipng" is not installed.`);
  }
}

function checkDir() {
  if (fs.existsSync("./builder")) {
    fs.rmSync("./builder", { recursive: true, force: true });
  }
}

function mkDirs() {
  const dirs = ["repos", "tmp", "zip-dir", "zip-dir-bedrock"];
  for (let dir of dirs) {
    fs.mkdir(`./builder/${dir}`, { recursive: true }, (err) => {
      if (err) throw err;
    });
  }
}

function getJavaUrls(java: any): Set<string> {
  let urls = new Set<string>();
  urls.add(java.repos.base.url);
  let addonTypes = ["exclusive", "regular", "mods"];
  for (let type of addonTypes) {
    for (let addon of java.repos.addons[type]) {
      if (type == "exclusive") {
        addon.variants.forEach((variant: { url: string }) => {
          if (variant.url) urls.add(variant.url);
        });
      } else {
        if (addon.url) urls.add(addon.url);
      }
    }
  }
  return urls;
}

function getBedrockUrls(bedrock: any): Set<string> {
  let urls = new Set<string>();
  urls.add(bedrock.repos.base.url);
  bedrock.repos.addons.forEach((_: any, num: number) => {
    urls.add(bedrock.repos.addons[num].url);
  });
  return urls;
}

async function cloneRepos(urls: Set<string>) {
  let promises = [...urls].map((url) => {
    let name = url.split("/").pop();
    return git().clone(url, path.resolve("./builder/repos/" + name));
  });
  await Promise.all(promises);
}

async function getPackData(urls: Set<string>) {
  let packs: Array<{
    name: string;
    defaultbranch: string;
    branches: string[];
  }> = [];
  for (let url of urls) {
    let name = url.split("/").pop()!;
    let defaultbranch = await getDefaultBranch(name);
    let branches = await getBranches(name);
    packs.push({
      name,
      defaultbranch,
      branches,
    });
  }
  return packs;
}

async function getDefaultBranch(name: string) {
  let result = await git(path.resolve("./builder/repos/" + name)).branch();
  return result.current;
}

async function getBranches(name: string) {
  let result = await git(path.resolve("./builder/repos/" + name)).branch();
  let allbranches: string[] = [];
  for (let branch of result.all) {
    allbranches.push(branch.split("/").pop()!);
  }
  return [...new Set(allbranches)];
}

function findFilesInDir(startPath: any, filter: any) {
  var results: string[] = [];

  if (!fs.existsSync(startPath)) {
    console.log("no dir ", startPath);
    throw Error;
  }

  var files = fs.readdirSync(startPath);
  for (var i = 0; i < files.length; i++) {
    var filename = path.join(startPath, files[i]);
    var stat = fs.lstatSync(filename);
    if (stat.isDirectory()) {
      results = results.concat(findFilesInDir(filename, filter)); //recurse
    } else if (filename.indexOf(filter) >= 0) {
      results.push(filename);
    }
  }
  return results;
}

function optimizeImages(packs: any[]) {
  packs.forEach(function (pack) {
    process.chdir(path.resolve("./builder/repos/" + pack.name));
    if (pack.branches.length === 1) {
      optimize(pack.name);
      commitOptimizedImages();
    } else {
      optimize(pack.name);
      commitOptimizedImages();
      for (var i = 0; i < pack.branches.length; i++) {
        if (pack.branches[i] != pack.defaultbranch) {
          checkoutBranch(pack.branches[i]);
          optimize(pack.name);
          commitOptimizedImages();
        }
      }
      checkoutBranch(pack.defaultbranch);
    }
    process.chdir(process.cwd());
  });
}

function optimize(name: string) {
  let images = findFilesInDir(path.resolve("./builder/repos/" + name), ".png");
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

async function checkoutBranch(branch: string) {
  try {
    execSync(`git checkout ${branch}`);
  } catch (error) {
    throw new Error(`Failed to checkout branch.`);
  }
}

function commitOptimizedImages() {
  try {
    execSync(`git add * && git commit -m "optimize images"`);
  } catch (err) {
    throw new Error("Failed to commit changes.");
  }
}

mane();
