import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { FiveoutofninechessRs } from "../target/types/fiveoutofninechess_rs";

describe("fiveoutofninechess-rs", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.FiveoutofninechessRs as Program<FiveoutofninechessRs>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.rpc.initialize({});
    console.log("Your transaction signature", tx);
  });
});
