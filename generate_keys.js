const fs = require('fs');
const { Keypair } = require('@solana/web3.js');
const path = require('path');

const dir = path.join(__dirname, 'tests', 'fixtures');
if (!fs.existsSync(dir)) {
    fs.mkdirSync(dir, { recursive: true });
}

['manager', 'jitosol_mint', 'usdg_mint', 'sol_mint'].forEach(name => {
  const kp = Keypair.generate();
  const secret = Array.from(kp.secretKey);
  fs.writeFileSync(path.join(dir, `${name}.json`), JSON.stringify(secret));
});

console.log("Keys generated securely on host in tests/fixtures/");
