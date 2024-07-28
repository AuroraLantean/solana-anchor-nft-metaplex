import * as anchor from "@coral-xyz/anchor";
import type { Program } from "@coral-xyz/anchor";
import type { Nft } from "../target/types/nft";

describe("nft", () => {
	// Configure the client to use the local cluster.
	anchor.setProvider(anchor.AnchorProvider.env());

	const program = anchor.workspace.Nft as Program<Nft>;

	it("Is initialized!", async () => {
		// Add your test here.
		//const tx = await program.methods.createSingleNft().rpc();
		//console.log("Your transaction signature", tx);
	});
});
