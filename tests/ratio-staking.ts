import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { RatioStaking } from "../target/types/ratio_staking";

describe("ratio-staking", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.RatioStaking as Program<RatioStaking>;

  it("Is initialized!", async () => {
    // Add your test here.
    /* const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx); */
  });
});
