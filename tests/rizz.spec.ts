import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Rizz } from "../target/types/rizz";

describe("rizz", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Rizz as Program<Rizz>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
