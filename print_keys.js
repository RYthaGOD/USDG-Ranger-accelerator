const fs = require('fs');
const { Keypair } = require('@solana/web3.js');

const load = name => {
  const p = `./tests/fixtures/${name}.json`;
  return Keypair.fromSecretKey(new Uint8Array(JSON.parse(fs.readFileSync(p, 'utf-8')))).publicKey.toBase58();
};

const output = `Manager: ${load('manager')}
JitoSOL Mint: ${load('jitosol_mint')}
USDG Mint: ${load('usdg_mint')}
SOL Mint: ${load('sol_mint')}`;

fs.writeFileSync('./tests/fixtures/keypair_notes.txt', output);
console.log(output);
