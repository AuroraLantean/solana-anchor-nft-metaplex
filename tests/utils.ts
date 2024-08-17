import * as anchor from "@coral-xyz/anchor";
import {
	createMint,
	getOrCreateAssociatedTokenAccount,
	TOKEN_2022_PROGRAM_ID,
} from "@solana/spl-token";
import type { Keypair, PublicKey } from "@solana/web3.js";
export const ll = console.log;
export const bn = (num: number | string) => new anchor.BN(num);
