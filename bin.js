#!/usr/bin/env node
var __commonJS = (cb, mod) => () => (mod || cb((mod = { exports: {} }).exports, mod), mod.exports);

// bum.darwin-arm64.node
var require_bum_darwin_arm64 = __commonJS((exports2, module2) => {
  module2.exports = require("./bum.darwin-arm64-7xvffnqw.node");
});

// index.js
var require_bum = __commonJS((exports2, module2) => {
  var __dirname = "/Users/rthionanda/Learn/bum";
  var { existsSync, readFileSync } = require("fs");
  var { join } = require("path");
  var { platform, arch } = process;
  var nativeBinding = null;
  var localFileExisted = false;
  var loadError = null;
  function isMusl() {
    if (!process.report || typeof process.report.getReport !== "function") {
      try {
        const lddPath = require("child_process").execSync("which ldd").toString().trim();
        return readFileSync(lddPath, "utf8").includes("musl");
      } catch (e) {
        return true;
      }
    } else {
      const { glibcVersionRuntime } = process.report.getReport().header;
      return !glibcVersionRuntime;
    }
  }
  switch (platform) {
    case "android":
      switch (arch) {
        case "arm64":
          localFileExisted = existsSync(join(__dirname, "bum.android-arm64.node"));
          try {
            if (localFileExisted) {
              nativeBinding = (()=>{throw new Error("Cannot require module "+"./bum.android-arm64.node");})();
            } else {
              nativeBinding = (()=>{throw new Error("Cannot require module "+"@owenizedd/bum-android-arm64");})();
            }
          } catch (e) {
            loadError = e;
          }
          break;
        case "arm":
          localFileExisted = existsSync(join(__dirname, "bum.android-arm-eabi.node"));
          try {
            if (localFileExisted) {
              nativeBinding = (()=>{throw new Error("Cannot require module "+"./bum.android-arm-eabi.node");})();
            } else {
              nativeBinding = (()=>{throw new Error("Cannot require module "+"@owenizedd/bum-android-arm-eabi");})();
            }
          } catch (e) {
            loadError = e;
          }
          break;
        default:
          throw new Error(`Unsupported architecture on Android ${arch}`);
      }
      break;
    case "win32":
      switch (arch) {
        case "x64":
          localFileExisted = existsSync(join(__dirname, "bum.win32-x64-msvc.node"));
          try {
            if (localFileExisted) {
              nativeBinding = (()=>{throw new Error("Cannot require module "+"./bum.win32-x64-msvc.node");})();
            } else {
              nativeBinding = (()=>{throw new Error("Cannot require module "+"@owenizedd/bum-win32-x64-msvc");})();
            }
          } catch (e) {
            loadError = e;
          }
          break;
        case "ia32":
          localFileExisted = existsSync(join(__dirname, "bum.win32-ia32-msvc.node"));
          try {
            if (localFileExisted) {
              nativeBinding = (()=>{throw new Error("Cannot require module "+"./bum.win32-ia32-msvc.node");})();
            } else {
              nativeBinding = (()=>{throw new Error("Cannot require module "+"@owenizedd/bum-win32-ia32-msvc");})();
            }
          } catch (e) {
            loadError = e;
          }
          break;
        case "arm64":
          localFileExisted = existsSync(join(__dirname, "bum.win32-arm64-msvc.node"));
          try {
            if (localFileExisted) {
              nativeBinding = (()=>{throw new Error("Cannot require module "+"./bum.win32-arm64-msvc.node");})();
            } else {
              nativeBinding = (()=>{throw new Error("Cannot require module "+"@owenizedd/bum-win32-arm64-msvc");})();
            }
          } catch (e) {
            loadError = e;
          }
          break;
        default:
          throw new Error(`Unsupported architecture on Windows: ${arch}`);
      }
      break;
    case "darwin":
      localFileExisted = existsSync(join(__dirname, "bum.darwin-universal.node"));
      try {
        if (localFileExisted) {
          nativeBinding = (()=>{throw new Error("Cannot require module "+"./bum.darwin-universal.node");})();
        } else {
          nativeBinding = (()=>{throw new Error("Cannot require module "+"@owenizedd/bum-darwin-universal");})();
        }
        break;
      } catch {}
      switch (arch) {
        case "x64":
          localFileExisted = existsSync(join(__dirname, "bum.darwin-x64.node"));
          try {
            if (localFileExisted) {
              nativeBinding = (()=>{throw new Error("Cannot require module "+"./bum.darwin-x64.node");})();
            } else {
              nativeBinding = (()=>{throw new Error("Cannot require module "+"@owenizedd/bum-darwin-x64");})();
            }
          } catch (e) {
            loadError = e;
          }
          break;
        case "arm64":
          localFileExisted = existsSync(join(__dirname, "bum.darwin-arm64.node"));
          try {
            if (localFileExisted) {
              nativeBinding = require_bum_darwin_arm64();
            } else {
              nativeBinding = (()=>{throw new Error("Cannot require module "+"@owenizedd/bum-darwin-arm64");})();
            }
          } catch (e) {
            loadError = e;
          }
          break;
        default:
          throw new Error(`Unsupported architecture on macOS: ${arch}`);
      }
      break;
    case "freebsd":
      if (arch !== "x64") {
        throw new Error(`Unsupported architecture on FreeBSD: ${arch}`);
      }
      localFileExisted = existsSync(join(__dirname, "bum.freebsd-x64.node"));
      try {
        if (localFileExisted) {
          nativeBinding = (()=>{throw new Error("Cannot require module "+"./bum.freebsd-x64.node");})();
        } else {
          nativeBinding = (()=>{throw new Error("Cannot require module "+"@owenizedd/bum-freebsd-x64");})();
        }
      } catch (e) {
        loadError = e;
      }
      break;
    case "linux":
      switch (arch) {
        case "x64":
          if (isMusl()) {
            localFileExisted = existsSync(join(__dirname, "bum.linux-x64-musl.node"));
            try {
              if (localFileExisted) {
                nativeBinding = (()=>{throw new Error("Cannot require module "+"./bum.linux-x64-musl.node");})();
              } else {
                nativeBinding = (()=>{throw new Error("Cannot require module "+"@owenizedd/bum-linux-x64-musl");})();
              }
            } catch (e) {
              loadError = e;
            }
          } else {
            localFileExisted = existsSync(join(__dirname, "bum.linux-x64-gnu.node"));
            try {
              if (localFileExisted) {
                nativeBinding = (()=>{throw new Error("Cannot require module "+"./bum.linux-x64-gnu.node");})();
              } else {
                nativeBinding = (()=>{throw new Error("Cannot require module "+"@owenizedd/bum-linux-x64-gnu");})();
              }
            } catch (e) {
              loadError = e;
            }
          }
          break;
        case "arm64":
          if (isMusl()) {
            localFileExisted = existsSync(join(__dirname, "bum.linux-arm64-musl.node"));
            try {
              if (localFileExisted) {
                nativeBinding = (()=>{throw new Error("Cannot require module "+"./bum.linux-arm64-musl.node");})();
              } else {
                nativeBinding = (()=>{throw new Error("Cannot require module "+"@owenizedd/bum-linux-arm64-musl");})();
              }
            } catch (e) {
              loadError = e;
            }
          } else {
            localFileExisted = existsSync(join(__dirname, "bum.linux-arm64-gnu.node"));
            try {
              if (localFileExisted) {
                nativeBinding = (()=>{throw new Error("Cannot require module "+"./bum.linux-arm64-gnu.node");})();
              } else {
                nativeBinding = (()=>{throw new Error("Cannot require module "+"@owenizedd/bum-linux-arm64-gnu");})();
              }
            } catch (e) {
              loadError = e;
            }
          }
          break;
        case "arm":
          if (isMusl()) {
            localFileExisted = existsSync(join(__dirname, "bum.linux-arm-musleabihf.node"));
            try {
              if (localFileExisted) {
                nativeBinding = (()=>{throw new Error("Cannot require module "+"./bum.linux-arm-musleabihf.node");})();
              } else {
                nativeBinding = (()=>{throw new Error("Cannot require module "+"@owenizedd/bum-linux-arm-musleabihf");})();
              }
            } catch (e) {
              loadError = e;
            }
          } else {
            localFileExisted = existsSync(join(__dirname, "bum.linux-arm-gnueabihf.node"));
            try {
              if (localFileExisted) {
                nativeBinding = (()=>{throw new Error("Cannot require module "+"./bum.linux-arm-gnueabihf.node");})();
              } else {
                nativeBinding = (()=>{throw new Error("Cannot require module "+"@owenizedd/bum-linux-arm-gnueabihf");})();
              }
            } catch (e) {
              loadError = e;
            }
          }
          break;
        case "riscv64":
          if (isMusl()) {
            localFileExisted = existsSync(join(__dirname, "bum.linux-riscv64-musl.node"));
            try {
              if (localFileExisted) {
                nativeBinding = (()=>{throw new Error("Cannot require module "+"./bum.linux-riscv64-musl.node");})();
              } else {
                nativeBinding = (()=>{throw new Error("Cannot require module "+"@owenizedd/bum-linux-riscv64-musl");})();
              }
            } catch (e) {
              loadError = e;
            }
          } else {
            localFileExisted = existsSync(join(__dirname, "bum.linux-riscv64-gnu.node"));
            try {
              if (localFileExisted) {
                nativeBinding = (()=>{throw new Error("Cannot require module "+"./bum.linux-riscv64-gnu.node");})();
              } else {
                nativeBinding = (()=>{throw new Error("Cannot require module "+"@owenizedd/bum-linux-riscv64-gnu");})();
              }
            } catch (e) {
              loadError = e;
            }
          }
          break;
        case "s390x":
          localFileExisted = existsSync(join(__dirname, "bum.linux-s390x-gnu.node"));
          try {
            if (localFileExisted) {
              nativeBinding = (()=>{throw new Error("Cannot require module "+"./bum.linux-s390x-gnu.node");})();
            } else {
              nativeBinding = (()=>{throw new Error("Cannot require module "+"@owenizedd/bum-linux-s390x-gnu");})();
            }
          } catch (e) {
            loadError = e;
          }
          break;
        default:
          throw new Error(`Unsupported architecture on Linux: ${arch}`);
      }
      break;
    default:
      throw new Error(`Unsupported OS: ${platform}, architecture: ${arch}`);
  }
  if (!nativeBinding) {
    if (loadError) {
      throw loadError;
    }
    throw new Error(`Failed to load native binding`);
  }
  var { run } = nativeBinding;
  module2.exports.run = run;
});

// bin.ts
var { run } = require_bum();
var args = process.argv.slice(2);
try {
  run(args);
} catch (e) {
  console.error(e);
  process.exit(1);
}
