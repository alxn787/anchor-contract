import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Fanch } from "../target/types/fanch";
import { expect } from "chai";

describe("fanch", () => {
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.fanch as Program<Fanch>;
  const newAccount = anchor.web3.Keypair.generate();

  it("Is initialized!", async () => {
    // Add your test here.
     const tx = await program.methods.init(10)
      .accounts({
        account: newAccount.publicKey,
        signer: anchor.getProvider().wallet.publicKey,
      })
      .signers([newAccount])
      .rpc();
    console.log("Your transaction signature", tx);
    const account = await program.account.dataShape.fetch(newAccount.publicKey)
    const count = account.num;
    console.log(count);
    if(count!=10)throw new Error('failed')

  });

  it("Double!", async () => {
    // Add your test here.
     const tx = await program.methods.double()
      .accounts({
        account: newAccount.publicKey,
        signer: anchor.getProvider().wallet.publicKey,
      })
      .rpc();
    console.log("Your transaction signature", tx);
    const account = await program.account.dataShape.fetch(newAccount.publicKey)
    const count = account.num;
    console.log(count);
    if(count!=20)throw new Error('failed')

  });

    it("Add!", async () => {
    // Add your test here.
     const tx = await program.methods.add(5)
      .accounts({
        account: newAccount.publicKey,
        signer: anchor.getProvider().wallet.publicKey,
      })
      .rpc();
    console.log("Your transaction signature", tx);
    const account = await program.account.dataShape.fetch(newAccount.publicKey)
    const count = account.num;
    console.log(count);
    if(count!=25)throw new Error('failed')

  });

   
});
