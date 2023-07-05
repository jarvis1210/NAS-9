import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Nas9 } from "../target/types/nas_9";

describe("nas_9", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Nas9 as Program<Nas9>;
  const provider = anchor.getProvider();

  it("Is initialized!", async () => {

    const [counterKey, bump] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("NAS COUNTER"), provider.publicKey.toBuffer()],
      program.programId
    )

    // Add your test here.
    const tx = await program.methods.initialize()
    .accounts({
      counter: counterKey,
      owner: provider.publicKey,
    })
    .rpc();



    const dataAccount = await program.account.counter.fetch(counterKey);

    const incTx = await program.methods.increment().accounts({
      counter: counterKey,
      owner: provider.publicKey,
    }).rpc();

    const dataAccount2 = await program.account.counter.fetch(counterKey);


    console.log("Your data is", dataAccount.data);
    console.log("Your data is", dataAccount2.data);
    console.log("Your transaction signature", tx);
    console.log("Your transaction signature", incTx);
  });
});
