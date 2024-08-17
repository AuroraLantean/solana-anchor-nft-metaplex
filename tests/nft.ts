import * as anchor from "@coral-xyz/anchor";
import type { Program } from "@coral-xyz/anchor";
import {
	getAssociatedTokenAddress,
	type Account,
	TOKEN_2022_PROGRAM_ID,
	ASSOCIATED_TOKEN_PROGRAM_ID,
} from "@solana/spl-token";
import { type TokenAmount, Keypair, PublicKey } from "@solana/web3.js";
import {
	findMasterEditionPda,
	findMetadataPda,
	mplTokenMetadata,
} from "@metaplex-foundation/mpl-token-metadata";
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults";
//import { mplCandyMachine } from "@metaplex-foundation/mpl-candy-machine";
import { publicKey, type Context } from "@metaplex-foundation/umi";
//import { u32 } from "@metaplex-foundation/umi/serializers";
import type { Nft } from "../target/types/nft";
import { walletAdapterIdentity } from "@metaplex-foundation/umi-signer-wallet-adapters";
import { bn } from "./utils";

const ll = console.log;
//const payer = new Keypair();
//const adam = new Keypair();
//const bob = new Keypair();

let mint9: PublicKey;
let authAta9: Account;
let toAta: Account;
let authAta: Account;
//let adamAta9: Account;
let fromAta: anchor.Address;

describe("nft", () => {
	// anchor.AnchorProvider.env() or .local()
	const provider = anchor.AnchorProvider.local();
	anchor.setProvider(provider);
	const authWallet = provider.wallet;
	const auth = authWallet.publicKey;
	ll("auth:", auth.toBase58());
	const program = anchor.workspace.Nft as Program<Nft>;
	const pgid = program.programId;

	const TOKEN_METADATA_PROGRAM_ID = new PublicKey(
		"metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s",
	); // metaplex metadata program id

	const umi = createUmi("https://api.devnet.solana.com")
		.use(walletAdapterIdentity(authWallet))
		.use(mplTokenMetadata()); //.use(mplCandyMachine());

	const mintKp = anchor.web3.Keypair.generate();
	const mint9 = mintKp.publicKey;
	ll("mint9:", mint9.toBase58());

	const tokenAccount = anchor.utils.token.associatedAddress({
		mint: mintKp.publicKey,
		owner: authWallet.publicKey,
	});
	ll(`tokenAccount: ${tokenAccount}`);
	//const authAta0 = await getAssociatedTokenAddress(mint9, auth);
	//ll("authAta0:", authAta0.toBase58());

	const metadataAccount = anchor.web3.PublicKey.findProgramAddressSync(
		[
			Buffer.from("metadata"),
			TOKEN_METADATA_PROGRAM_ID.toBuffer(),
			mintKp.publicKey.toBuffer(),
		],
		TOKEN_METADATA_PROGRAM_ID,
	)[0];
	ll(`metadataAccount: ${metadataAccount}`);

	const masterEditionAccount = anchor.web3.PublicKey.findProgramAddressSync(
		[
			Buffer.from("metadata"),
			TOKEN_METADATA_PROGRAM_ID.toBuffer(),
			mintKp.publicKey.toBuffer(),
			Buffer.from("edition"),
		],
		TOKEN_METADATA_PROGRAM_ID,
	)[0];
	ll(`Master edition address: ${masterEditionAccount}`);

	const id = bn(0);
	const metadata = {
		name: "Kobeni",
		symbol: "kBN",
		uri: "https://raw.githubusercontent.com/687c/solana-nft-native-client/main/metadata.json",
	};
	const price = 100.32;
	const cant = bn(150);

	it("mint NFT", async () => {
		ll("mintNFT...");

		//cannot make auth ATA when mint does not exist yet...

		ll("ASSOCIATED_TOKEN_PROGRAM_ID:", ASSOCIATED_TOKEN_PROGRAM_ID.toBase58());
		ll("TOKEN_2022_PROGRAM_ID:", TOKEN_2022_PROGRAM_ID.toBase58());
		ll("TOKEN_METADATA_PROGRAM_ID:", TOKEN_METADATA_PROGRAM_ID.toBase58());
		ll("systemProgram:", anchor.web3.SystemProgram.programId.toBase58());
		ll("rent:", anchor.web3.SYSVAR_RENT_PUBKEY.toBase58());

		const tx = await program.methods
			.createSingleNft(
				id,
				metadata.name,
				metadata.symbol,
				metadata.uri,
				price,
				cant,
			)
			.accounts({
				//authority: auth,
				//payer: auth,
				tokenAccount,
				associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
				tokenProgram: TOKEN_2022_PROGRAM_ID,
				metadataProgram: TOKEN_METADATA_PROGRAM_ID,
				masterEditionAccount,
				metadataAccount,
				systemProgram: anchor.web3.SystemProgram.programId,
				rent: anchor.web3.SYSVAR_RENT_PUBKEY,
			})
			//.signers([mintKp])
			.rpc();

		ll(`mint nft tx: https://explorer.solana.com/tx/${tx}?cluster=devnet`);
		ll(
			`minted nft: https://explorer.solana.com/address/${mint9.toBase58()}?cluster=devnet`,
		);
	});
});
