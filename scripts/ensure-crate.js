const INTERVAL = 2000;
const MAX_RETRY = 10;

const { spawn } = require("child_process");
const fsp = require("fs/promises");

async function ensureCrate(pkgName, version) {
  const command = spawn("curl", [
    `https://crates.io/api/v1/crates/${pkgName}/${version}`,
  ]);

  const data = await new Promise((resolve, reject) => {
    let data = "";
    command.stdout.on("data", (d) => {
      data += d.toString();
    });

    command.on("close", function (code) {
      if (code === 0) resolve(data);
      else reject(new Error(`curl exited with code ${code}`));
    });
    command.on("error", function (err) {
      reject(err);
    });
  });

  let res;
  try {
    res = JSON.parse(data);
  } catch (error) {
    res = null;
  }

  if (res) {
    if (res.version && res.version.crate === pkgName) {
      return;
    } else {
      throw new Error(`${pkgName}:${version} error: ${JSON.stringify(res)}`);
    }
  } else {
    throw new Error(`${pkgName}:${version} not available`);
  }
}

async function main() {
  const version = process.argv[2];

  if (!version) {
    throw new Error(`Parameter version is missing`);
  }

  const toml = await fsp.readFile("./Cargo.toml", "utf-8");
  const pkgName = toml.match(/name\s*=\s*"(.+)"/)?.[1];

  if (!pkgName) {
    throw new Error(`No valid Cargo.toml: ${process.cwd()}`);
  }

  console.log(`Checking crate ${pkgName}:${version}`);

  let ok = false;

  for (let i = 0; i < MAX_RETRY; i++) {
    if (i > 0) {
      await new Promise((resolve) => setTimeout(resolve, INTERVAL));
    }

    try {
      await ensureCrate(pkgName, version);
      ok = true;
      console.log(`[${i}] OK for ${pkgName}:${version}`);
      break;
    } catch (err) {
      console.log(`[${i}] Error for ${pkgName}:${version} ${err.message}`);
    }
  }

  if (!ok) process.exitCode = 2;
}

main().catch((err) => {
  process.exitCode = 1;
  console.error(err);
});
