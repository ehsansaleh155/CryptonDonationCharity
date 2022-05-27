import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { CryptonDonationCharity } from "../target/types/crypton_donation_charity";

describe("CryptonDonationCharity", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.CryptonDonationCharity as Program<CryptonDonationCharity>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
