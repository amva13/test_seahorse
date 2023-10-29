import * as anchor from "@project-serum/anchor";
import { Program, web3, BN } from "@project-serum/anchor";
import { TestSeahorse } from "../target/types/test_seahorse";

const assert = require("assert");

describe("test_seahorse", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.TestSeahorse as Program<TestSeahorse>;
  const programProvider = program.provider as anchor.AnchorProvider;

  const owner = provider.wallet.publicKey;
  const calculator = web3.PublicKey.findProgramAddressSync(
    [Buffer.from('Calculator'), owner.toBuffer()],
    program.programId
  )[0];

  it("Is initialized!", async () => {
    // Add your test here.
    const calc = anchor.web3.Keypair.generate();
    // const calc_calculator = web3.PublicKey.findProgramAddressSync(
    //   [Buffer.from('Calculator'), calc.publicKey.toBuffer()],
    //   program.programId
    // )[0]; 
    const tr = await program.methods.initCalculator().accounts({ 
      owner: calc.publicKey,
    }).instruction()
    console.log(`Instruction ${tr}`);
    const tx = new web3.Transaction()
    tx.add(tr)
    await programProvider
    .sendAndConfirm(tx, [calc])
    .then(() => assert.ok(false)) // Error on success, we want a failure
    .catch(console.log);
    // console.log("Your transaction signature", tx);
  });

  it('Does some operations', async () => {
    const add2 = await program.methods
      .doOperation({ add: true }, new BN(2))
      .accounts({ owner, calculator })
      .instruction()
  
  
    const mul3 = await program.methods
      .doOperation({ mul: true }, new BN(3))
      .accounts({ owner, calculator })
      .instruction()
  
  
    const sub1 = await program.methods
      .doOperation({ sub: true }, new BN(1))
      .accounts({ owner, calculator })
      .instruction()
  
  
    const tx = new web3.Transaction()
    tx.add(add2, mul3, sub1)
    await provider.sendAndConfirm(tx)
  
  
    // Get the calculator's on-chain data
    const calculatorAccount = await program.account.testSeahorse.fetch(calculator)
  
  
    assert.ok(calculatorAccount.display.toNumber() === 5)
  })
  it('Prevents fraudulent transactions', async () => {
    let hackerman = new web3.Keypair()
  
  
    let shouldFail = await program.methods
      .resetCalculator()
      .accounts({
        owner: hackerman.publicKey,
        calculator,
      })
      .instruction()
  
  
    let tx = new web3.Transaction()
    tx.add(shouldFail)
    await provider
      .sendAndConfirm(tx, [hackerman])
      .then(() => assert.ok(false)) // Error on success, we want a failure
      .catch(console.log)
  })
});
