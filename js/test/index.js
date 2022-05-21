import { Connection } from "@solana/web3.js";
import { readKeypairFromFile } from "../lib/readKeypairFromFile.js";
import { createPost, getPostData, findAllPost, post } from "../index.js";

// mainnet
// const rpcUrl = 'https://solana-api.projectserum.com/';
// devnet
// const rpcUrl = 'https://api.devnet.solana.com';
// local
const rpcUrl = "http://localhost:8899/";

// comm
const connection = new Connection(rpcUrl, "finalized");
const idPath = "/home/alex/.config/solana/id.json";

// key
var postKey = "";

async function getPayer() {
  let keypair = await readKeypairFromFile(idPath);
  console.log("read payer", keypair.publicKey.toBase58());
  return keypair;
}

function wait(ms) {
  return new Promise((resolve) => setTimeout(() => resolve(), ms));
}

// test
async function main() {
  try {
    let payer = await getPayer();
    // find post
    {
      let list = await findAllPost(connection);
      if (list.length > 0) {
        postKey = list[0].pubkey.toBase58();
        console.log("post exist", postKey);
      }
    }
    // if post is null, start init
    if (postKey == "") {
      // create and init post
      {
        let res = await post(
          connection,
          payer,
          "Post test content 1234567890 !@#$%^&*() qwertyuiop asdfghjkl zxcvbnm QWERTYUIOP ASDFGHJKL ZXCVBNMPost test content 1234567890 !@#$%^&*() qwertyuiop asdfghjkl zxcvbnm QWERTYUIOP ASDFGHJKL ZXCVBNMPost test content 1234567890 !@#$%^&*() qwertyuiop asdfghjkl zxcvbnm QWERTYUIOP ASDFGHJKL ZXCVBNMPost test content 1234567890 !@#$%^&*() qwertyuiop asdfghjkl zxcvbnm QWERTYUIOP ASDFGHJKL ZXCVBNMPost test content 1234567890 !@#$%^&*() qwertyuiop asdfghjkl zxcvbnm QWERTYUIOP ASDFGHJKL ZXCVBNMPost test content 1234567890 !@#$%^&*() qwertyuiop asdfghjkl zxcvbnm QWERTYUIOP ASDFGHJKL ZXCVBNMPost test content 1234567890 !@#$%^&*() qwertyuiop asdfghjkl zxcvbnm QWERTYUIOP ASDFGHJKL ZXCVBNMPost test content 1234567890 !@#$%^&*() qwertyuiop asdfghjkl zxcvbnm QWERTYUIOP ASDFGHJKL ZXCVBNMPost test content 1234567890 !@#$%^&*() qwertyuiop asdfghjkl zxcvbnm QWERTYUIOP ASDFGHJKL ZXCVBNMPost test content 1234567890 !@#$%^&*() qwertyuiop asdfghjkl zxcvbnm QWERTYUIOP ASDFGHJKL ZXCVBNMPost test content 1234567890 !@#$%^&*() qwertyuiop asdfghjkl zxcvbnm QWERTYUIOP ASDFGHJKL ZXCVBNMPost test content 1234567890 !@#$%^&*() qwertyuiop asdfghjkl zxcvbnm QWERTYUIOP ASDFGHJKL ZXCVBNMPost test content 1234567890 !@#$%^&*() qwertyuiop asdfghjkl zxcvbnm QWERTYUIOP ASDFGHJKL ZXCVBNMPost test content 1234567890 !@#$%^&*() qwertyuiop asdfghjkl zxcvbnm QWERTYUIOP ASDFGHJKL ZXCVBNMPost test content 1234567890 !@#$%^&*() qwertyuiop asdfghjkl zxcvbnm QWERTYUIOP ASDFGHJKL ZXCVBNMPost test content 1234567890 !@#$%^&*() qwertyuiop asdfghjkl zxcvbnm QWERTYUIOP ASDFGHJKL ZXCVBNMPost test content 1234567890 !@#$%^&*() qwertyuiop asdfghjkl zxcvbnm QWERTYUIOP ASDFGHJKL ZXCVBNMPost test content 1234567890 !@#$%^&*() qwertyuiop asdfghjkl zxcvbnm QWERTYUIOP ASDFGHJKL ZXCVBNMPost test content 1234567890 !@#$%^&*() qwertyuiop asdfghjkl zxcvbnm QWERTYUIOP ASDFGHJKL ZXCVBNMPost test content 1234567890 !@#$%^&*() qwertyuiop asdfghjkl zxcvbnm QWERTYUIOP ASDFGHJKL ZXCVBNMPost test content 1234567890 !@#$%^&*() qwertyuiop asdfghjkl zxcvbnm QWERTYUIOP ASDFGHJKL ZXCVBNMPost test content 1234567890 !@#$%^&*() qwertyuiop asdfghjkl zxcvbnm QWERTYUIOP ASDFGHJKL ZXCVBNMPost test content 1234567890 !@#$%^&*() qwertyuiop asdfghjkl zxcvbnm QWERTYUIOP ASDFGHJKL ZXCVBNMPost test content 1234567890 !@#$%^&*() qwertyuiop asdfghjkl zxcvbnm QWERTYUIOP ASDFGHJKL ZXCVBNMPost test content 1234567890 !@#$%^&*() qwertyuiop asdfghjkl zxcvbnm QWERTYUIOP ASDFGHJKL ZXCVBNMPost test content 1234567890 !@#$%^&*() qwertyuiop asdfghjkl zxcvbnm QWERTYUIOP ASDFGHJKL ZXCVBNMPost test content 1234567890 !@#$%^&*() qwertyuiop asdfghjkl zxcvbnm QWERTYUIOP ASDFGHJKL ZXCVBNMPost test content 1234567890 !@#$%^&*() qwertyuiop asdfghjkl zxcvbnm QWERTYUIOP ASDFGHJKL ZXCVBNMPost test content 1234567890 !@#$%^&*() qwertyuiop asdfghjkl zxcvbnm QWERTYUIOP ASDFGHJKL ZXCVBNMPost test content 1234567890 !@#$%^&*() qwertyuiop asdfghjkl zxcvbnm QWERTYUIOP ASDFGHJKL ZXCVBNMPost test content 1234567890 !@#$%^&*() qwertyuiop asdfghjkl zxcvbnm QWERTYUIOP ASDFGHJKL ZXCVBNMPost test content 1234567890 !@#$%^&*() qwertyuiop asdfghjkl zxcvbnm QWERTYUIOP ASDFGHJKL ZXCVBNMPost test content 1234567890 !@#$%^&*() qwertyuiop asdfghjkl zxcvbnm QWERTYUIOP ASDFGHJKL ZXCVBNMPost test content 1234567890 !@#$%^&*() qwertyuiop asdfghjkl zxcvbnm QWERTYUIOP ASDFGHJKL ZXCVBNMPost test content 1234567890 !@#$%^&*() qwertyuiop asdfghjkl zxcvbnm QWERTYUIOP ASDFGHJKL ZXCVBNMPost test content 1234567890 !@#$%^&*() qwertyuiop asdfghjkl zxcvbnm QWERTYUIOP ASDFGHJKL ZXCVBNMPost test content 1234567890 !@#$%^&*() qwertyuiop asdfghjkl zxcvbnm QWERTYUIOP ASDFGHJKL ZXCVBNMPost test content 1234567890 !@#$%^&*() qwertyuiop asdfghjkl zxcvbnm QWERTYUIOP ASDFGHJKL ZXCVBNMPost test content 1234567890 !@#$%^&*() qwertyuiop asdfghjkl zxcvbnm QWERTYUIOP ASDFGHJKL ZXCVBNMPost test content 1234567890 !@#$%^&*() qwertyuiop asdfghjkl zxcvbnm QWERTYUIOP ASDFGHJKL ZXCVBNMPost test content 1234567890 !@#$%^&*() qwertyuiop asdfghjkl zxcvbnm QWERTYUIOP ASDFGHJKL ZXCVBNMPost test content 1234567890 !@#$%^&*() qwertyuiop asdfghjkl zxcvbnm QWERTYUIOP ASDFGHJKL ZXCVBNMPost test content 1234567890 !@#$%^&*() qwertyuiop asdfghjkl zxcvbnm QWERTYUIOP ASDFGHJKL ZXCVBNMPost test content 1234567890 !@#$%^&*() qwertyuiop asdfghjkl zxcvbnm QWERTYUIOP ASDFGHJKL ZXCVBNMPost test content 1234567890 !@#$%^&*() qwertyuiop asdfghjkl zxcvbnm QWERTYUIOP ASDFGHJKL ZXCVBNM"
        );
        if (res.code == 1) {
          postKey = res.data;
          console.log("init post ok", res.data);
        } else {
          console.error(res);
          return res;
        }
      }
    }
    // get post data, check if the data is  right
    {
      let res = await getPostData(connection, postKey);
      if (res.code == 1) {
        console.log("get post data", res.data);
      }
    }
  } catch (err) {
    console.error(err);
  }
}

main();
