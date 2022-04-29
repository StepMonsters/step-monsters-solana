import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { StepMonstersSolana } from "../target/types/step_monsters_solana";

describe("step-monsters-solana", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.StepMonstersSolana as Program<StepMonstersSolana>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
