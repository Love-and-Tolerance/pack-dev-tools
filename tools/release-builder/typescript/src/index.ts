import fs from "fs";
import path from "path";
import { simpleGit as git, CleanOptions } from "simple-git";
import { execSync } from "child_process";

git().clean(CleanOptions.FORCE);

const javaAssets = fs.readFileSync("./assets/java.json", "utf8");
const java = JSON.parse(javaAssets);
const bedrockAssets = fs.readFileSync("./assets/bedrock.json", "utf8");
const bedrock = JSON.parse(bedrockAssets);

let urls = new Set<string>();
let absolute = path.resolve("./");

async function mane() {
  checkGit();
  checkOptipng();
  mkDirs();
  getJavaUrls(urls);
  getBedrockUrls(urls);
  cloneRepos();
  let packIds = findIdentities(java);
  let packs = await getPackData(urls);
  optimizeImages(packs);
}

async function checkGit() {
  const { installed } = await git().version();
  if (!installed) {
    throw new Error(`Exit: "git" not available.`);
  }
}

function checkOptipng() {
  try {
    execSync('which "optipng"');
  } catch (err) {
    throw new Error(`Exit: "optipng" is not installed.`);
  }
}

function cloneRepos() {
  urls.forEach(url => {
    let name = url.split("/").pop();
    git().clone(url, absolute + "/builder/repos/" + name);
  });
}

function findIdentities(java: any): string[][] {
  let ids = new Array<Array<string>>();
  for (let addon of java.repos.addons) {
    let subId: string[] = [];
    let id: string;
    addon.variants.forEach(function(variant: any) {
      id = variant.id;
      subId.push(id);
    });
    ids.push(subId);
  }
  return generateIdentities(ids);
}

function generateIdentities(args: string[][]) {
  var indentities: string[][] = [],
    max = args.length - 1;
  function helper(arr: string[], i: number) {
    for (var j = 0, l = args[i].length; j < l; j++) {
      var id = arr.slice(0);
      id.push(args[i][j]);
      if (i == max) indentities.push(id);
      else helper(id, i + 1);
    }
  }
  helper([], 0);
  return indentities;
}

function getJavaUrls(urls: Set<string>): Set<string> {
  urls.add(java.repos.base.url);
  for (let addon of java.repos.addons) {
    addon.variants.forEach((variant: { url: string; }) => {
      if (variant.url) urls.add(variant.url);
    });
  }
  return urls;
}

function getBedrockUrls(urls: Set<string>): Set<string> {
  urls.add(bedrock.repos.base.url);
  bedrock.repos.addons.forEach((_: any, num: number) => {
    urls.add(bedrock.repos.addons[num].url);
  });
  return urls;
}

function mkDirs() {
  let dirs = ["repos", "tmp", "zip-dir", "zip-dir-bedrock"];
  for (let dir of dirs) {
    fs.mkdir(`./builder/${dir}`, { recursive: true }, (err) => {
      if (err) throw err;
    });
  }
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
  let result = await git(absolute + "/builder/repos/" + name).branch();
  return result.current;
}

async function getBranches(name: string) {
  let result = await git(absolute + "/builder/repos/" + name).branch();
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
    process.chdir(absolute + "/builder/repos/" + pack.name);
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
      };
      checkoutBranch(pack.defaultbranch);
    }
    process.chdir(absolute);
  })
}

function optimize(name: string) {
  let images = findFilesInDir(absolute + "/builder/repos/" + name, ".png")
  images.forEach(function (file) {
    if (file.endsWith(".png")) {
      try {
        execSync(`optipng -o7 -fix ${file}`);
      } catch (err) {
        throw new Error(`Failed to optimize image.`);
      }
    }
  })
}

async function checkoutBranch(branch: string) {
  try {
    execSync(`git checkout ${branch}`);
  } catch (error) {
    throw new Error(`Failed to checkout branch.`)
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