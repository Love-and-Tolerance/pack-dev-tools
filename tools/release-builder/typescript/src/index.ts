import fs from "fs";
import path from "path";
import { simpleGit as git, CleanOptions } from "simple-git";
import { execSync } from "child_process";
import { version } from "os";

let reposDir = "./builder/repos";

async function mane() {
  const java = await getJsonData("java");
  const bedrock = await getJsonData("bedrock");
  const javaVersion = java.repos.base.version;
  const javaFormat = java.repos.base.pack_format;
  const bedrockVersion = bedrock.repos.base.version;
  git().clean(CleanOptions.FORCE);
  const programs = ["git", "zip", "oxipng"];
  checkInstalled(programs);
  checkDir();
  mkDirs();
  const javaUrls = getJavaUrls(java);
  const bedrockUrls = getBedrockUrls(bedrock);
  await cloneRepos(javaUrls);
  await cloneRepos(bedrockUrls);
  const javaPacks = await getPackData(javaUrls);
  const bedrockPacks = await getPackData(bedrockUrls);
  generatePacks(javaPacks, javaVersion, "zip", "java", javaFormat);
  generatePacks(bedrockPacks, bedrockVersion, "mcpack", "bedrock");
  const optimize = true;
  generatePacks(javaPacks, javaVersion, "zip", "java", javaFormat, optimize);
  generatePacks(
    bedrockPacks,
    bedrockVersion,
    "mcpack",
    "bedrock",
    undefined,
    optimize
  );
}

async function getJsonData(version: string) {
  return await fetch(
    `https://raw.githubusercontent.com/Love-and-Tolerance/pack-builder-assets/mane/assets/${version}.json`
  ).then((res) => res.json());
}

function checkInstalled(programs: string[]) {
  for (let program of programs) {
    try {
      execSync(`which "${program}"`);
    } catch (err) {
      throw new Error(`Exit: "${program}" is not installed.`);
    }
  }
}

function checkDir() {
  if (fs.existsSync("./builder")) {
    fs.rmSync("./builder", { recursive: true, force: true });
  }
}

function mkDirs() {
  const dirs = ["repos", "zip-dir"];
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
    let name = url.split("/").pop()!;
    return git().clone(url, path.resolve(reposDir, name));
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
  let result = await git(path.resolve(reposDir, name)).branch();
  return result.current;
}

async function getBranches(name: string) {
  let result = await git(path.resolve(reposDir, name)).branch();
  let allbranches: string[] = [];
  for (let branch of result.all) {
    allbranches.push(branch.split("/").pop()!);
  }
  return [...new Set(allbranches)];
}

function generatePacks(
  packs: Array<{
    name: string;
    defaultbranch: string;
    branches: string[];
  }>,
  version: string,
  extension: string,
  platform: string,
  format?: string,
  optimize?: boolean
) {
  packs.forEach(function (pack) {
    process.chdir(path.resolve(reposDir, pack.name));
    for (var i = 0; i < pack.branches.length; i++) {
      let branch: string;
      if (pack.branches[i] != pack.defaultbranch) {
        checkoutBranch(pack.branches[i]);
        branch = pack.branches[i];
      } else {
        branch = pack.defaultbranch;
      }
      if (optimize) {
        optimizeImages(pack.name);
      }
      generateZip(
        pack.name,
        version,
        branch,
        extension,
        platform,
        format,
        !optimize
      );
    }
    checkoutBranch(pack.defaultbranch);
    process.chdir("../../..");
  });
}

function generateZip(
  name: string,
  version: string,
  branch: string,
  extension: string,
  platform: string,
  format?: string,
  source?: boolean
) {
  if (!source && platform == "java") {
    const filename = `L_T-${version}-format.${format}-${name}-${branch}.${extension}`;
    zipPack(filename);
  } else if (source && platform == "java") {
    const filename = `L_T-${version}-format.${format}-${name}-${branch}-source.${extension}`;
    zipPack(filename);
  } else if (!source && platform == "bedrock") {
    const filename = `L_T-${version}-${name}-${branch}.${extension}`;
    zipPack(filename);
  } else if (source && platform == "bedrock") {
    const filename = `L_T-${version}-${name}-${branch}-source.${extension}`;
    zipPack(filename);
  }
}

function zipPack(filename: string) {
  try {
    execSync(`zip -rq9 ../../zip-dir/${filename} *`);
  } catch (error) {
    throw new Error(`Failed to zip pack.`);
  }
  console.log(filename);
}

function optimizeImages(name: string) {
  let images = findFilesInDir(`./`, ".png");
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
      results = results.concat(findFilesInDir(filename, filter));
    } else if (filename.indexOf(filter) >= 0) {
      results.push(filename);
    }
  }
  return results;
}

async function checkoutBranch(branch: string) {
  try {
    execSync(`git switch ${branch} --discard-changes`);
  } catch (error) {
    throw new Error(`Failed to checkout branch.`);
  }
}

mane();
